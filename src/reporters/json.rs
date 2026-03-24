use crate::analyzers::finding::DiagnosticReport;

/// Render a diagnostic report as JSON.
pub fn render(report: &DiagnosticReport) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(report)?)
}
