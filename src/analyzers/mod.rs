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
pub fn scan_log_patterns(bundle: &ParsedBundle) -> Vec<Finding> {
    use crate::parsers::common::Patterns;
    use finding::{Category, Severity};

    let mut findings = Vec::new();

    struct PatternCheck {
        pattern: &'static regex::Regex,
        rule_id: &'static str,
        name: &'static str,
        category: Category,
        severity: Severity,
        doc_link: &'static str,
        platform: Option<Platform>,
    }

    let pattern_checks = vec![
        PatternCheck {
            pattern: Patterns::imds_error(),
            rule_id: "IMDS-001",
            name: "IMDS Connectivity Failure",
            category: Category::Connectivity,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#verify-imds-connectivity",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::auth_token_error(),
            rule_id: "IDENTITY-002",
            name: "Authentication Token Error",
            category: Category::Identity,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/qs-configure-portal-windows-vm",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::connectivity_error(),
            rule_id: "CONN-001",
            name: "Network Connectivity Failure",
            category: Category::Connectivity,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::service_crash(),
            rule_id: "AGENT-001",
            name: "Agent Service Crash or Unexpected Termination",
            category: Category::AgentNotRunning,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::dcr_error(),
            rule_id: "DCR-001",
            name: "DCR Configuration Error",
            category: Category::Dcr,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/essentials/data-collection-rule-overview",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::extension_error(),
            rule_id: "INSTALL-001",
            name: "Extension Provisioning Failure",
            category: Category::Installation,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::syslog_error(),
            rule_id: "SYSLOG-001",
            name: "Syslog/CEF Forwarding Failure",
            category: Category::Syslog,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::metrics_extension_error(),
            rule_id: "METRICS-001",
            name: "MetricsExtension Error",
            category: Category::MetricsExtension,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::arc_agent_error(),
            rule_id: "ARC-001",
            name: "Arc Agent Error",
            category: Category::ArcAgent,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-arc",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::oom_killer(),
            rule_id: "OOM-001",
            name: "OOM Killer Terminated Agent Process",
            category: Category::Sizing,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::fluentbit_error(),
            rule_id: "FLUENTBIT-001",
            name: "Fluentbit Engine Error",
            category: Category::Syslog,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::mdsd_qos_failure(),
            rule_id: "QOS-001",
            name: "MDSD QoS Upload Failure",
            category: Category::Connectivity,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::throttling(),
            rule_id: "THROTTLE-001",
            name: "Data Ingestion Throttling",
            category: Category::Syslog,
            severity: Severity::Warning,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::guest_agent_error(),
            rule_id: "GUEST-001",
            name: "Linux Guest Agent Error",
            category: Category::Installation,
            severity: Severity::Warning,
            doc_link: "https://learn.microsoft.com/en-us/azure/virtual-machines/extensions/agent-linux",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::systemd_service_failure(),
            rule_id: "SERVICE-001",
            name: "AMA Systemd Service Failure",
            category: Category::AgentNotRunning,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm",
            platform: Some(Platform::Linux),
        },
        PatternCheck {
            pattern: Patterns::disk_full(),
            rule_id: "DISK-001",
            name: "Disk Space Exhaustion",
            category: Category::Sizing,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
            platform: None,
        },
        PatternCheck {
            pattern: Patterns::cgroup_oom(),
            rule_id: "CGROUP-001",
            name: "Memory Cgroup OOM Kill",
            category: Category::Sizing,
            severity: Severity::Critical,
            doc_link:
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
            platform: Some(Platform::Linux),
        },
    ];

    for check in &pattern_checks {
        if let Some(platform) = check.platform {
            if bundle.platform != Some(platform) {
                continue;
            }
        }

        let mut evidence = Vec::new();
        for line in &bundle.log_lines {
            if check.pattern.is_match(&line.content) {
                evidence.push(format!("{}:{}: {}", line.file, line.line_num, line.content));
                if evidence.len() >= 5 {
                    break; // Cap evidence per finding
                }
            }
        }

        if !evidence.is_empty() {
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
