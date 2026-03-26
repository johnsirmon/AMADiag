use crate::analyzers;
use crate::analyzers::finding::{DiagnosticReport, Platform};
use crate::model::diagnostic::{DiagnosticEvent, FindingGroup, OsKind};
use crate::model::filter::FilterState;
use crate::parsers;
use crate::store::{event_store::EventStore, grouping};
use anyhow::Result;
use std::path::Path;

pub struct TuiAnalysis {
    pub report: DiagnosticReport,
    pub extracted_bundle: crate::input::ExtractedBundle,
    pub event_store: EventStore,
    pub events: Vec<DiagnosticEvent>,
    pub grouped_findings: Vec<FindingGroup>,
    pub filter: FilterState,
}

impl TuiAnalysis {
    pub fn refresh_groups(&mut self) {
        let latest = self.event_store.latest_timestamp();
        let indexed_event_count = self.event_store.events().len();
        let mut indices = self.event_store.filtered_event_indices(&self.filter);

        for (index, event) in self.events.iter().enumerate().skip(indexed_event_count) {
            if event.severity >= self.filter.min_severity
                && self.filter.allows_category(event.category)
                && self.filter.time_filter.contains(event.ts, latest)
            {
                indices.push(index);
            }
        }

        self.grouped_findings = grouping::group_events(&self.events, &indices);
    }
}

/// Full analysis pipeline: extract → parse → analyze → report.
pub fn analyze_bundle(path: &Path) -> Result<DiagnosticReport> {
    // 1. Extract bundle
    tracing::info!("Analyzing bundle: {}", path.display());
    let extracted = crate::input::prepare_bundle(path)?;

    // 2. Parse all files
    tracing::info!("Parsing bundle contents...");
    let mut bundle = parsers::parse_bundle(extracted.path())?;

    // Enrich with platform-specific data
    match bundle.platform {
        Some(Platform::Windows) => parsers::windows::enrich_windows_data(&mut bundle, extracted.path()),
        Some(Platform::Linux) => parsers::linux::enrich_linux_data(&mut bundle, extracted.path()),
        None => tracing::warn!("Could not detect platform — running generic analysis"),
    }

    finalize_report(path, &bundle)
}

pub fn analyze_bundle_for_tui(path: &Path) -> Result<TuiAnalysis> {
    tracing::info!("Analyzing bundle for TUI: {}", path.display());
    let extracted_bundle = crate::input::prepare_bundle(path)?;
    let mut bundle = parsers::parse_bundle(extracted_bundle.path())?;

    match bundle.platform {
        Some(Platform::Windows) => {
            parsers::windows::enrich_windows_data(&mut bundle, extracted_bundle.path())
        }
        Some(Platform::Linux) => parsers::linux::enrich_linux_data(&mut bundle, extracted_bundle.path()),
        None => tracing::warn!("Could not detect platform — running generic analysis"),
    }

    let report = finalize_report(path, &bundle)?;
    let os = bundle
        .platform
        .map(OsKind::from)
        .unwrap_or(OsKind::Linux);
    let event_store = EventStore::from_bundle_dir(extracted_bundle.path(), os)?;

    let mut events = event_store.events().to_vec();
    events.extend(
        report
            .findings
            .iter()
            .map(|finding| DiagnosticEvent::from_legacy_finding(finding, os)),
    );

    let filter = FilterState::default();
    let mut analysis = TuiAnalysis {
        report,
        extracted_bundle,
        event_store,
        events,
        grouped_findings: Vec::new(),
        filter,
    };
    analysis.refresh_groups();
    Ok(analysis)
}

fn finalize_report(path: &Path, bundle: &parsers::ParsedBundle) -> Result<DiagnosticReport> {
    let mut report = DiagnosticReport::new(path.display().to_string());
    report.files_analyzed = bundle.files.len();
    report.environment.platform = bundle.platform;
    report.environment.dcr_count = bundle.xml_configs.len();

    extract_environment_info(bundle, &mut report);
    tracing::info!("Running analysis rules...");
    analyzers::analyze(bundle, &mut report)?;

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
