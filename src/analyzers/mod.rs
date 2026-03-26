pub mod finding;
pub mod linux_analysis;
pub mod rules;
pub mod sizing;

use crate::parsers::ParsedBundle;
use finding::{DiagnosticReport, Finding, Platform};

/// Run all analysis rules against a parsed bundle and produce findings.
pub fn analyze(bundle: &ParsedBundle, report: &mut DiagnosticReport) -> anyhow::Result<()> {
    // Load and run built-in detection rules
    let rule_defs = rules::load_builtin_rules()?;
    let mut findings = rules::evaluate_rules(&rule_defs, bundle);

    // Run sizing analysis
    let sizing_findings = sizing::check_sizing(bundle);
    findings.extend(sizing_findings);

    // Run Linux-specific analysis if Linux data is available
    if bundle.platform == Some(Platform::Linux) {
        if let Some(linux_data) = &bundle.linux_data {
            let linux_findings = linux_analysis::check_linux(linux_data);
            findings.extend(linux_findings);
        }
    }

    // Sort findings: Critical first, then Warning, then Info
    findings.sort_by(|a, b| b.severity.cmp(&a.severity));

    report.findings = findings;
    Ok(())
}

/// Convenience: run pattern-based scanning across all log lines.
///
/// Uses `RegexSet` for single-pass matching: each log line is tested against
/// all 17 diagnostic patterns in one evaluation instead of 17 separate ones.
pub fn scan_log_patterns(bundle: &ParsedBundle) -> Vec<Finding> {
    use crate::parsers::common::Patterns;
    use finding::{Category, Severity};

    // Regex to detect IPv6 context — lines about IPv6-only failures are benign
    // on most Azure VMs and should not trigger connectivity/IMDS alerts.
    static IPV6_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(?i)(IPv6|fe80::|::[\da-f]{2,}|\[2[0-9a-f]{3}:)").unwrap()
    });

    // Lines with ErrorCode:0 in AMA Windows extension logs indicate SUCCESS
    // regardless of the log-level label. Skip these to avoid false positives.
    static ERRORCODE_ZERO_RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"ErrorCode:0\b").unwrap());

    struct PatternCheck {
        rule_id: &'static str,
        name: &'static str,
        category: Category,
        severity: Severity,
        doc_link: &'static str,
        platform: Option<Platform>,
        /// If true, skip lines containing IPv6 indicators.
        exclude_ipv6: bool,
    }

    // Index order MUST match DIAGNOSTIC_PATTERN_STRINGS in common.rs
    let pattern_checks: Vec<PatternCheck> = vec![
        PatternCheck {
            rule_id: "IMDS-001",
            name: "IMDS Connectivity Failure",
            category: Category::Connectivity,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#verify-imds-connectivity",
            platform: None,
            exclude_ipv6: true,
        },
        PatternCheck {
            rule_id: "IDENTITY-002",
            name: "Authentication Token Error",
            category: Category::Identity,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/qs-configure-portal-windows-vm",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "CONN-001",
            name: "Network Connectivity Failure",
            category: Category::Connectivity,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration",
            platform: None,
            exclude_ipv6: true,
        },
        PatternCheck {
            rule_id: "AGENT-001",
            name: "Agent Service Crash or Unexpected Termination",
            category: Category::AgentNotRunning,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "DCR-001",
            name: "DCR Configuration Error",
            category: Category::Dcr,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/essentials/data-collection-rule-overview",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "INSTALL-001",
            name: "Extension Provisioning Failure",
            category: Category::Installation,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "SYSLOG-001",
            name: "Syslog/CEF Forwarding Failure",
            category: Category::Syslog,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "METRICS-001",
            name: "MetricsExtension Error",
            category: Category::MetricsExtension,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "ARC-001",
            name: "Arc Agent Error",
            category: Category::ArcAgent,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-arc",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "OOM-001",
            name: "OOM Killer Terminated Agent Process",
            category: Category::Sizing,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "FLUENTBIT-001",
            name: "Fluentbit Engine Error",
            category: Category::Syslog,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "QOS-001",
            name: "MDSD QoS Upload Failure",
            category: Category::Connectivity,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "THROTTLE-001",
            name: "Data Ingestion Throttling",
            category: Category::Syslog,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "GUEST-001",
            name: "Linux Guest Agent Error",
            category: Category::Installation,
            severity: Severity::Warning,
            doc_link: "https://learn.microsoft.com/en-us/azure/virtual-machines/extensions/agent-linux",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "SERVICE-001",
            name: "AMA Systemd Service Failure",
            category: Category::AgentNotRunning,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "DISK-001",
            name: "Disk Space Exhaustion",
            category: Category::Sizing,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
            platform: None,
            exclude_ipv6: false,
        },
        PatternCheck {
            rule_id: "CGROUP-001",
            name: "Memory Cgroup OOM Kill",
            category: Category::Sizing,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
            platform: Some(Platform::Linux),
            exclude_ipv6: false,
        },
    ];

    let regex_set = Patterns::diagnostic_set();

    // Pre-filter: determine which pattern indices apply to this bundle's platform.
    let active_indices: Vec<usize> = pattern_checks
        .iter()
        .enumerate()
        .filter(|(_, check)| check.platform.map_or(true, |p| bundle.platform == Some(p)))
        .map(|(idx, _)| idx)
        .collect();

    // Single-pass: collect evidence per pattern using RegexSet
    let mut evidence_per_check: Vec<Vec<String>> = vec![Vec::new(); pattern_checks.len()];

    for line in &bundle.log_lines {
        // Check if all active patterns have hit the 5-evidence cap
        if active_indices
            .iter()
            .all(|&idx| evidence_per_check[idx].len() >= 5)
        {
            break;
        }

        let matches = regex_set.matches(&line.content);
        if !matches.matched_any() {
            continue;
        }

        for &idx in &active_indices {
            if !matches.matched(idx) || evidence_per_check[idx].len() >= 5 {
                continue;
            }

            let check = &pattern_checks[idx];
            if check.exclude_ipv6 && IPV6_RE.is_match(&line.content) {
                continue;
            }
            if ERRORCODE_ZERO_RE.is_match(&line.content) {
                continue;
            }

            evidence_per_check[idx]
                .push(format!("{}:{}: {}", line.file, line.line_num, line.content));
        }
    }

    // Build findings from collected evidence
    let mut findings = Vec::with_capacity(active_indices.len());
    for (idx, evidence) in evidence_per_check.into_iter().enumerate() {
        if evidence.is_empty() {
            continue;
        }
        let check = &pattern_checks[idx];
        findings.push(Finding {
            rule_id: check.rule_id.to_string(),
            name: check.name.to_string(),
            severity: check.severity,
            category: check.category.clone(),
            description: format!(
                "Detected {} pattern match(es) in log files.",
                evidence.len()
            ),
            evidence,
            remediation: format!("See documentation: {}", check.doc_link),
            doc_link: Some(check.doc_link.to_string()),
        });
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::scan_log_patterns;
    use crate::analyzers::finding::Platform;
    use crate::parsers::{common::parse_log_lines, ParsedBundle};
    use std::collections::HashMap;

    #[test]
    fn linux_only_log_patterns_are_skipped_for_windows_bundles() {
        let log_content = "\
azuremonitoragent service dead
rsyslog not running
walinuxagent stopped
SuccessCount = 0
";

        let bundle = ParsedBundle {
            platform: Some(Platform::Windows),
            files: HashMap::from([("agent.log".to_string(), log_content.to_string())]),
            log_lines: parse_log_lines(log_content, "agent.log"),
            ..Default::default()
        };

        let findings = scan_log_patterns(&bundle);

        assert!(!findings
            .iter()
            .any(|finding| finding.rule_id == "SERVICE-001"));
        assert!(!findings
            .iter()
            .any(|finding| finding.rule_id == "SYSLOG-001"));
        assert!(!findings
            .iter()
            .any(|finding| finding.rule_id == "GUEST-001"));
        assert!(!findings.iter().any(|finding| finding.rule_id == "QOS-001"));
    }

    #[test]
    fn linux_only_log_patterns_still_fire_for_linux_bundles() {
        let log_content = "\
azuremonitoragent service dead
rsyslog not running
walinuxagent stopped
SuccessCount = 0
";

        let bundle = ParsedBundle {
            platform: Some(Platform::Linux),
            files: HashMap::from([("agent.log".to_string(), log_content.to_string())]),
            log_lines: parse_log_lines(log_content, "agent.log"),
            ..Default::default()
        };

        let findings = scan_log_patterns(&bundle);

        assert!(findings
            .iter()
            .any(|finding| finding.rule_id == "SERVICE-001"));
        assert!(findings
            .iter()
            .any(|finding| finding.rule_id == "SYSLOG-001"));
        assert!(findings
            .iter()
            .any(|finding| finding.rule_id == "GUEST-001"));
        assert!(findings.iter().any(|finding| finding.rule_id == "QOS-001"));
    }
}
