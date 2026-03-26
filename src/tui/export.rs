use crate::analyzers::finding::DiagnosticReport;
use crate::model::diagnostic::FindingGroup;
use crate::reporters::{self, OutputFormat};
use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn export_report(report: &DiagnosticReport, format: OutputFormat) -> Result<PathBuf> {
    let path = default_export_path(Path::new(&report.bundle_path), format);
    export_report_to(report, format, &path.display().to_string())
}

pub fn export_report_to(
    report: &DiagnosticReport,
    format: OutputFormat,
    output_path: &str,
) -> Result<PathBuf> {
    let path = PathBuf::from(output_path.trim());
    let rendered = match format {
        OutputFormat::Markdown => reporters::markdown::render(report),
        OutputFormat::Json => reporters::json::render(report)?,
    };

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    std::fs::write(&path, rendered)?;
    Ok(path)
}

#[derive(Serialize)]
struct DashboardExport<'a> {
    report: &'a DiagnosticReport,
    time_filter: &'a str,
    category_filter: &'a str,
    visible_group_count: usize,
    selected_group: Option<&'a FindingGroup>,
    grouped_findings: &'a [FindingGroup],
}

pub fn export_dashboard_to(
    report: &DiagnosticReport,
    grouped_findings: &[FindingGroup],
    selected_group: Option<&FindingGroup>,
    time_filter: &str,
    category_filter: &str,
    format: OutputFormat,
    output_path: &str,
) -> Result<PathBuf> {
    let path = PathBuf::from(output_path.trim());
    let rendered = match format {
        OutputFormat::Markdown => render_dashboard_markdown(
            report,
            grouped_findings,
            selected_group,
            time_filter,
            category_filter,
        ),
        OutputFormat::Json => serde_json::to_string_pretty(&DashboardExport {
            report,
            time_filter,
            category_filter,
            visible_group_count: grouped_findings.len(),
            selected_group,
            grouped_findings,
        })?,
    };

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    std::fs::write(&path, rendered)?;
    Ok(path)
}

pub fn default_export_path(source_path: &Path, format: OutputFormat) -> PathBuf {
    let parent = source_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = source_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("amadiag-report");

    let extension = match format {
        OutputFormat::Markdown => "md",
        OutputFormat::Json => "json",
    };

    parent.join(format!("{stem}.amadiag.{extension}"))
}

fn render_dashboard_markdown(
    report: &DiagnosticReport,
    grouped_findings: &[FindingGroup],
    selected_group: Option<&FindingGroup>,
    time_filter: &str,
    category_filter: &str,
) -> String {
    let mut out = String::new();
    out.push_str("# AMADiag Ops Cockpit Export\n\n");
    out.push_str("## Scope\n\n");
    out.push_str(&format!("- **Bundle**: `{}`\n", report.bundle_path));
    out.push_str(&format!("- **Time filter**: {time_filter}\n"));
    out.push_str(&format!("- **Category filter**: {category_filter}\n"));
    out.push_str(&format!(
        "- **Visible grouped findings**: {}\n\n",
        grouped_findings.len()
    ));

    if let Some(group) = selected_group {
        out.push_str("## Selected finding\n\n");
        out.push_str(&format!("- **Title**: {}\n", group.title));
        out.push_str(&format!("- **Category**: {}\n", group.category));
        out.push_str(&format!("- **Severity**: {}\n", group.severity));
        out.push_str(&format!("- **Status**: {}\n", group.status));
        out.push_str(&format!("- **Event count**: {}\n\n", group.event_count()));
        out.push_str(&format!("{}\n\n", group.summary));

        if !group.likely_causes.is_empty() {
            out.push_str("### Likely causes\n\n");
            for cause in &group.likely_causes {
                out.push_str(&format!("- {cause}\n"));
            }
            out.push('\n');
        }

        if !group.suggested_actions.is_empty() {
            out.push_str("### Suggested actions\n\n");
            for action in &group.suggested_actions {
                out.push_str(&format!("- {action}\n"));
            }
            out.push('\n');
        }
    }

    out.push_str("## Grouped findings\n\n");
    out.push_str("| Severity | Category | Title | Events |\n");
    out.push_str("|---|---|---|---:|\n");
    for group in grouped_findings {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            group.severity,
            group.category,
            group.title,
            group.event_count()
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::diagnostic::{Category, EvidenceRef, FindingGroup, Severity as UiSeverity, Status};
    use chrono::Utc;

    #[test]
    fn builds_markdown_export_path() {
        let path = default_export_path(Path::new(r"C:\temp\bundle.zip"), OutputFormat::Markdown);
        assert_eq!(path, PathBuf::from(r"C:\temp\bundle.amadiag.md"));
    }

    #[test]
    fn falls_back_when_stem_missing() {
        let path = default_export_path(Path::new(""), OutputFormat::Json);
        assert!(path.ends_with("amadiag-report.amadiag.json"));
    }

    #[test]
    fn dashboard_markdown_includes_group_title() {
        let report = DiagnosticReport::new("bundle.zip".to_string());
        let groups = vec![FindingGroup {
            key: "connectivity".to_string(),
            category: Category::Connectivity,
            severity: UiSeverity::High,
            status: Status::Fail,
            title: "Connectivity failures".to_string(),
            summary: "Failed connections were observed.".to_string(),
            first_seen: Some(Utc::now()),
            last_seen: Some(Utc::now()),
            events: vec![0, 1],
            likely_causes: vec!["Proxy issue".to_string()],
            suggested_actions: vec!["Check endpoint access".to_string()],
            evidence: vec![EvidenceRef {
                file_id: "ama.log".to_string(),
                line_range: 1..2,
                preview: "failed".to_string(),
            }],
            doc_links: Vec::new(),
            rule_ids: Vec::new(),
        }];

        let rendered = render_dashboard_markdown(
            &report,
            &groups,
            groups.first(),
            "Last 24h",
            "All categories",
        );

        assert!(rendered.contains("Connectivity failures"));
        assert!(rendered.contains("Likely causes"));
    }
}
