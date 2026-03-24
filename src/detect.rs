use crate::analyzers;
use crate::analyzers::finding::{DiagnosticReport, Platform};
use crate::parsers;
use anyhow::Result;
use std::path::Path;

/// Full analysis pipeline: extract → parse → analyze → report.
pub fn analyze_bundle(path: &Path) -> Result<DiagnosticReport> {
    // 1. Extract bundle
    tracing::info!("Analyzing bundle: {}", path.display());
    let (dir, _tmp) = crate::input::extract_bundle(path)?;

    // 2. Parse all files
    tracing::info!("Parsing bundle contents...");
    let mut bundle = parsers::parse_bundle(&dir)?;

    // Enrich with platform-specific data
    match bundle.platform {
        Some(Platform::Windows) => parsers::windows::enrich_windows_data(&mut bundle, &dir),
        Some(Platform::Linux) => parsers::linux::enrich_linux_data(&mut bundle, &dir),
        None => tracing::warn!("Could not detect platform — running generic analysis"),
    }

    // 3. Build report
    let mut report = DiagnosticReport::new(path.display().to_string());
    report.files_analyzed = bundle.files.len();
    report.environment.platform = bundle.platform;
    report.environment.dcr_count = bundle.xml_configs.len();

    // Extract environment info from IMDS/metadata if present
    extract_environment_info(&bundle, &mut report);

    // 4. Analyze
    tracing::info!("Running analysis rules...");
    analyzers::analyze(&bundle, &mut report)?;

    tracing::info!(
        "Analysis complete: {} findings ({} critical)",
        report.findings.len(),
        report.finding_count_by_severity(crate::analyzers::finding::Severity::Critical)
    );

    Ok(report)
}

/// Extract environment information from the parsed bundle.
fn extract_environment_info(bundle: &parsers::ParsedBundle, report: &mut DiagnosticReport) {
    // Look for IMDS metadata
    for (name, content) in &bundle.files {
        let name_lower = name.to_lowercase();

        // Try to parse IMDS JSON
        if name_lower.contains("imds") || name_lower.contains("metadata") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
                if let Some(compute) = json.get("compute") {
                    report.environment.vm_sku = compute
                        .get("vmSize")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    report.environment.os = compute
                        .get("osType")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    report.environment.hostname = compute
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    report.environment.resource_id = compute
                        .get("resourceId")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        }

        // Detect Arc
        if name_lower.contains("himds") || name_lower.contains("azcmagent") {
            report.environment.is_arc = Some(true);
        }

        // Try to extract AMA version from log lines
        if report.environment.ama_version.is_none() {
            let version_re = crate::parsers::common::Patterns::version_pattern();
            if name_lower.contains("azuremonitor") || name_lower.contains("ama") {
                if let Some(caps) = version_re.captures(content) {
                    if let Some(ver) = caps.get(1) {
                        report.environment.ama_version = Some(ver.as_str().to_string());
                    }
                }
            }
        }
    }
}
