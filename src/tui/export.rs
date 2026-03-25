use crate::analyzers::finding::DiagnosticReport;
use crate::reporters::{self, OutputFormat};
use anyhow::Result;
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
