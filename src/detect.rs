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
    pub timeline_points: Vec<u64>,
    pub filter: FilterState,
}

impl TuiAnalysis {
    pub fn refresh_dashboard_data(&mut self) {
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
        self.timeline_points = self
            .event_store
            .timeline(&self.filter)
            .into_iter()
            .map(|bucket| u64::try_from(bucket.warning_or_higher).unwrap_or(u64::MAX))
            .collect();
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
        Some(Platform::Windows) => {
            parsers::windows::enrich_windows_data(&mut bundle, extracted.path());
        }
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
            parsers::windows::enrich_windows_data(&mut bundle, extracted_bundle.path());
        }
        Some(Platform::Linux) => {
            parsers::linux::enrich_linux_data(&mut bundle, extracted_bundle.path());
        }
        None => tracing::warn!("Could not detect platform — running generic analysis"),
    }

    let report = finalize_report(path, &bundle)?;
    let os = bundle.platform.map(OsKind::from).unwrap_or(OsKind::Linux);
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
        timeline_points: Vec::new(),
        filter,
    };
    analysis.refresh_dashboard_data();
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_dir() -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        path.push(format!("amadiag-detect-test-{unique}"));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn refresh_dashboard_data_updates_timeline_points_for_filter_changes() {
        let dir = create_temp_dir();
        let older_ts = (Utc::now() - Duration::hours(2)).format("%Y-%m-%dT%H:%M:%SZ");
        let recent_ts = (Utc::now() - Duration::minutes(5)).format("%Y-%m-%dT%H:%M:%SZ");

        std::fs::write(
            dir.join("ama.log"),
            format!(
                "{older_ts} ERROR older endpoint unreachable\n{recent_ts} ERROR recent endpoint unreachable\n"
            ),
        )
        .unwrap();

        let event_store = EventStore::from_bundle_dir(&dir, OsKind::Linux).unwrap();
        let mut analysis = TuiAnalysis {
            report: DiagnosticReport::new(dir.display().to_string()),
            extracted_bundle: crate::input::prepare_bundle(&dir).unwrap(),
            events: event_store.events().to_vec(),
            event_store,
            grouped_findings: Vec::new(),
            timeline_points: Vec::new(),
            filter: FilterState {
                time_filter: crate::model::filter::TimeFilter::All,
                ..FilterState::default()
            },
        };

        analysis.refresh_dashboard_data();
        assert_eq!(analysis.timeline_points.len(), 2);

        analysis.filter.time_filter = crate::model::filter::TimeFilter::LastMinutes(15);
        analysis.refresh_dashboard_data();
        assert_eq!(analysis.timeline_points.len(), 1);

        let _ = std::fs::remove_dir_all(dir);
    }
}
