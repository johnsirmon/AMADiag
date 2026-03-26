pub mod common;
pub mod event_table;
pub mod fluentbit_config;
pub mod linux;
pub mod mdsd_qos;
pub mod windows;
pub mod xml_config;

use crate::analyzers::finding::Platform;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;

/// Linux-specific parsed data extracted from a troubleshooter bundle.
#[derive(Debug, Clone, Default)]
pub struct LinuxData {
    /// Parsed mdsd.qos upload metrics per blob type.
    pub mdsd_qos_entries: Vec<mdsd_qos::MdsdQosEntry>,
    /// Parsed Fluentbit td-agent.conf configuration.
    pub fluentbit_config: Option<fluentbit_config::FluentbitConfig>,
    /// Parsed rsyslog forwarding rules for AMA.
    pub rsyslog_rules: Vec<linux::RsyslogForwardRule>,
    /// Parsed /etc/os-release fields.
    pub os_release: Option<linux::OsRelease>,
    /// Raw proxy.conf content if present.
    pub proxy_config: Option<String>,
}

/// Parsed data extracted from an AMA troubleshooter bundle.
#[derive(Debug, Default)]
pub struct ParsedBundle {
    pub platform: Option<Platform>,
    /// Key-value pairs of parsed files: logical name → content
    pub files: HashMap<String, String>,
    /// Parsed XML config data from mcsconfig
    pub xml_configs: Vec<xml_config::McsConfig>,
    /// Event table entries
    pub event_entries: Vec<event_table::EventEntry>,
    /// Log lines with matched severity/content
    pub log_lines: Vec<common::LogLine>,
    /// Linux-specific parsed data (populated only for Linux bundles).
    pub linux_data: Option<LinuxData>,
}

/// Parse all relevant files from an extracted bundle directory.
pub fn parse_bundle(bundle_dir: &Path) -> anyhow::Result<ParsedBundle> {
    struct FileResult {
        rel_path: String,
        content: String,
        xml_config: Option<xml_config::McsConfig>,
        event_entries: Vec<event_table::EventEntry>,
        log_lines: Vec<common::LogLine>,
    }

    let mut parsed = ParsedBundle {
        platform: detect_platform(bundle_dir),
        ..Default::default()
    };

    // Collect all file paths first, then process in parallel with rayon.
    // Uses the `ignore` crate (ripgrep ecosystem) — faster than walkdir.
    let file_paths: Vec<_> = ignore::WalkBuilder::new(bundle_dir)
        .standard_filters(false)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
        .map(|e| e.into_path())
        .collect();

    let results: Vec<FileResult> = file_paths
        .par_iter()
        .filter_map(|path| {
            let rel_path = path.strip_prefix(bundle_dir).unwrap_or(path);
            let rel_str = rel_path.to_string_lossy().to_string();

            let content = std::fs::read_to_string(path).ok()?;

            let xml_config = if is_mcsconfig_file(&rel_str) {
                xml_config::parse_mcsconfig(&content).ok()
            } else {
                None
            };

            let event_entries = if rel_str.ends_with(".csv") {
                event_table::parse_csv(&content).unwrap_or_default()
            } else {
                Vec::new()
            };

            let log_lines = if is_log_file(&rel_str) {
                common::parse_log_lines(&content, &rel_str)
            } else {
                Vec::new()
            };

            Some(FileResult {
                rel_path: rel_str,
                content,
                xml_config,
                event_entries,
                log_lines,
            })
        })
        .collect();

    // Merge results
    for result in results {
        if let Some(cfg) = result.xml_config {
            parsed.xml_configs.push(cfg);
        }
        parsed.event_entries.extend(result.event_entries);
        parsed.log_lines.extend(result.log_lines);
        parsed.files.insert(result.rel_path, result.content);
    }

    Ok(parsed)
}

fn detect_platform(dir: &Path) -> Option<Platform> {
    // Windows indicators
    let win_indicators = [
        "MonAgentHost",
        "AzureMonitorWindowsAgent",
        "WindowsAzure",
        "mcsconfig.lkg.xml",
    ];
    // Linux indicators
    let linux_indicators = [
        "mdsd",
        "AzureMonitorLinuxAgent",
        "rsyslog",
        "waagent",
        "fluentbit",
        "amacoreagent",
        "telegraf",
        "td-agent.conf",
        "os-release",
        "azuremonitoragent",
    ];

    let mut win_score = 0;
    let mut linux_score = 0;

    for entry in ignore::WalkBuilder::new(dir)
        .standard_filters(false)
        .max_depth(Some(4))
        .build()
        .filter_map(|e| e.ok())
    {
        let name = entry.file_name().to_string_lossy();
        let path_str = entry.path().to_string_lossy();

        for indicator in &win_indicators {
            if name.contains(indicator) || path_str.contains(indicator) {
                win_score += 1;
            }
        }
        for indicator in &linux_indicators {
            if name.contains(indicator) || path_str.contains(indicator) {
                linux_score += 1;
            }
        }
    }

    if win_score > linux_score {
        Some(Platform::Windows)
    } else if linux_score > win_score {
        Some(Platform::Linux)
    } else {
        None
    }
}

fn is_mcsconfig_file(rel_path: &str) -> bool {
    let lower = rel_path.to_lowercase();
    lower.contains("mcsconfig") && lower.ends_with(".xml")
}

fn is_log_file(rel_path: &str) -> bool {
    let lower = rel_path.to_lowercase();
    // Exclude non-log file types — HTML and Markdown may contain troubleshooter
    // output or documentation prose that triggers false pattern matches.
    if lower.ends_with(".html")
        || lower.ends_with(".htm")
        || lower.ends_with(".md")
        || lower.ends_with(".json")
        || lower.ends_with(".xml")
        || lower.ends_with(".csv")
    {
        return false;
    }
    lower.ends_with(".log")
        || lower.ends_with(".txt")
        || lower.contains("mdsd.err")
        || lower.contains("mdsd.warn")
        || lower.contains("mdsd.info")
        || lower.contains("mdsd.qos")
        || lower.contains("fluentbit.log")
        || lower.contains("amaca")
        || lower.contains("waagent.log")
        || lower.contains("extension.log")
        || lower.contains("troubleshooter")
}
