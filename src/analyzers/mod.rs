pub mod finding;
pub mod rules;
pub mod sizing;

use crate::parsers::ParsedBundle;
use finding::{DiagnosticReport, Finding};

/// Run all analysis rules against a parsed bundle and produce findings.
pub fn analyze(bundle: &ParsedBundle, report: &mut DiagnosticReport) -> anyhow::Result<()> {
    // Load and run built-in detection rules
    let rule_defs = rules::load_builtin_rules()?;
    let mut findings = rules::evaluate_rules(&rule_defs, bundle);

    // Run sizing analysis
    let sizing_findings = sizing::check_sizing(bundle);
    findings.extend(sizing_findings);

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

    let pattern_checks: Vec<(&regex::Regex, &str, &str, Category, Severity, &str)> = vec![
        (
            Patterns::imds_error(),
            "IMDS-001",
            "IMDS Connectivity Failure",
            Category::Connectivity,
            Severity::Critical,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#verify-imds-connectivity",
        ),
        (
            Patterns::auth_token_error(),
            "IDENTITY-002",
            "Authentication Token Error",
            Category::Identity,
            Severity::Critical,
            "https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/qs-configure-portal-windows-vm",
        ),
        (
            Patterns::connectivity_error(),
            "CONN-001",
            "Network Connectivity Failure",
            Category::Connectivity,
            Severity::Critical,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration",
        ),
        (
            Patterns::service_crash(),
            "AGENT-001",
            "Agent Service Crash or Unexpected Termination",
            Category::AgentNotRunning,
            Severity::Critical,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm",
        ),
        (
            Patterns::dcr_error(),
            "DCR-001",
            "DCR Configuration Error",
            Category::Dcr,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/essentials/data-collection-rule-overview",
        ),
        (
            Patterns::extension_error(),
            "INSTALL-001",
            "Extension Provisioning Failure",
            Category::Installation,
            Severity::Critical,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm#step-1-verify-that-the-extension-was-installed-properly",
        ),
        (
            Patterns::syslog_error(),
            "SYSLOG-001",
            "Syslog/CEF Forwarding Failure",
            Category::Syslog,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
        ),
        (
            Patterns::metrics_extension_error(),
            "METRICS-001",
            "MetricsExtension Error",
            Category::MetricsExtension,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-vm",
        ),
        (
            Patterns::arc_agent_error(),
            "ARC-001",
            "Arc Agent Error",
            Category::ArcAgent,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-windows-arc",
        ),
        (
            Patterns::oom_killer(),
            "OOM-001",
            "OOM Killer Terminated Agent Process",
            Category::Sizing,
            Severity::Critical,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance",
        ),
        (
            Patterns::fluentbit_error(),
            "FLUENTBIT-001",
            "Fluentbit Engine Error",
            Category::Syslog,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
        ),
        (
            Patterns::mdsd_qos_failure(),
            "QOS-001",
            "MDSD QoS Upload Failure",
            Category::Connectivity,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm",
        ),
        (
            Patterns::throttling(),
            "THROTTLE-001",
            "Data Ingestion Throttling",
            Category::Syslog,
            Severity::Warning,
            "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting",
        ),
    ];

    for (pattern, rule_id, name, category, severity, doc_link) in &pattern_checks {
        let mut evidence = Vec::new();
        for line in &bundle.log_lines {
            if pattern.is_match(&line.content) {
                evidence.push(format!("{}:{}: {}", line.file, line.line_num, line.content));
                if evidence.len() >= 5 {
                    break; // Cap evidence per finding
                }
            }
        }

        if !evidence.is_empty() {
            findings.push(Finding {
                rule_id: rule_id.to_string(),
                name: name.to_string(),
                severity: *severity,
                category: category.clone(),
                description: format!(
                    "Detected {} pattern match(es) in log files.",
                    evidence.len()
                ),
                evidence,
                remediation: format!("See documentation: {doc_link}"),
                doc_link: Some(doc_link.to_string()),
            });
        }
    }

    findings
}
