use crate::analyzers::finding::{Category, Finding, Severity};
use crate::parsers::LinuxData;

/// Run Linux-specific analysis against parsed Linux data.
///
/// This produces findings based on structured data extracted from:
/// - mdsd.qos upload metrics
/// - Fluentbit td-agent.conf configuration
/// - rsyslog forwarding rules
pub fn check_linux(linux_data: &LinuxData) -> Vec<Finding> {
    let mut findings = Vec::new();

    check_mdsd_qos(linux_data, &mut findings);
    check_fluentbit_config(linux_data, &mut findings);
    check_rsyslog_forwarding(linux_data, &mut findings);

    findings
}

/// Analyze mdsd.qos upload metrics for failures.
fn check_mdsd_qos(data: &LinuxData, findings: &mut Vec<Finding>) {
    if data.mdsd_qos_entries.is_empty() {
        return;
    }

    // Check for total upload failures (SuccessCount == 0 but TotalCount > 0)
    for entry in &data.mdsd_qos_entries {
        if entry.is_total_failure() {
            findings.push(Finding {
                rule_id: "QOS-002".to_string(),
                name: format!("Complete Upload Failure for {}", entry.blob_type),
                severity: Severity::Critical,
                category: Category::Connectivity,
                description: format!(
                    "All {} events for {} failed to upload (0 of {} succeeded). \
                     Data is being collected but not reaching Azure Monitor.",
                    entry.total_count, entry.blob_type, entry.total_count
                ),
                evidence: vec![format!(
                    "{}: TotalCount={} SuccessCount={} FailCount={}",
                    entry.timestamp.as_deref().unwrap_or("unknown"),
                    entry.total_count,
                    entry.success_count,
                    entry.fail_count
                )],
                remediation: "Check network connectivity to the ingestion endpoint. \
                    Verify firewall and proxy settings. Review mdsd.err for connection errors."
                    .to_string(),
                doc_link: Some(
                    "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm"
                        .to_string(),
                ),
            });
            // One finding per blob type with total failure is sufficient
            break;
        }
    }

    // Check for partial upload failures (FailCount > 0 but not total failure)
    let partial_failures: Vec<_> = data
        .mdsd_qos_entries
        .iter()
        .filter(|e| e.fail_count > 0 && !e.is_total_failure())
        .collect();

    if !partial_failures.is_empty() {
        let total_fails: u64 = partial_failures.iter().map(|e| e.fail_count).sum();
        let evidence: Vec<String> = partial_failures
            .iter()
            .take(5)
            .map(|e| {
                format!(
                    "{}: {} FailCount={}/{}",
                    e.timestamp.as_deref().unwrap_or("unknown"),
                    e.blob_type,
                    e.fail_count,
                    e.total_count,
                )
            })
            .collect();

        findings.push(Finding {
            rule_id: "QOS-003".to_string(),
            name: "Partial Upload Failures Detected".to_string(),
            severity: Severity::Warning,
            category: Category::Connectivity,
            description: format!(
                "Found {} failed upload attempts across {} QoS entries. \
                 Some data may be lost or delayed.",
                total_fails,
                partial_failures.len()
            ),
            evidence,
            remediation: "Review mdsd.err and mdsd.warn for connection or throttling errors. \
                Check network connectivity and disk space."
                .to_string(),
            doc_link: Some(
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm"
                    .to_string(),
            ),
        });
    }
}

/// Validate Fluentbit configuration.
fn check_fluentbit_config(data: &LinuxData, findings: &mut Vec<Finding>) {
    let config = match &data.fluentbit_config {
        Some(c) => c,
        None => return,
    };

    // Check if there are no INPUT sections (nothing being collected)
    if config.inputs.is_empty() {
        findings.push(Finding {
            rule_id: "FBCFG-001".to_string(),
            name: "Fluentbit Has No Input Configuration".to_string(),
            severity: Severity::Warning,
            category: Category::Syslog,
            description: "The Fluentbit td-agent.conf has no [INPUT] sections. \
                No text or JSON log files are being tailed for collection."
                .to_string(),
            evidence: vec!["td-agent.conf: 0 INPUT sections found".to_string()],
            remediation: "Verify the DCR has custom log data sources configured. \
                The agent should generate INPUT sections automatically from the DCR. \
                Try restarting AMA: systemctl restart azuremonitoragent."
                .to_string(),
            doc_link: Some(
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/data-collection-log-text"
                    .to_string(),
            ),
        });
    }

    // Check if OUTPUT doesn't target mdsd
    if !config.outputs.is_empty() && !config.has_mdsd_output() {
        let output_names: Vec<String> = config
            .outputs
            .iter()
            .filter_map(|o| o.name.clone())
            .collect();
        findings.push(Finding {
            rule_id: "FBCFG-002".to_string(),
            name: "Fluentbit Not Sending to MDSD".to_string(),
            severity: Severity::Warning,
            category: Category::Syslog,
            description: "Fluentbit OUTPUT sections do not target the mdsd TCP socket \
                (port 28330). Collected log data is not reaching the upload pipeline."
                .to_string(),
            evidence: vec![format!(
                "Output plugins: {} (expected tcp:28330)",
                output_names.join(", ")
            )],
            remediation: "The td-agent.conf should have a TCP output to 127.0.0.1:28330. \
                This is auto-configured from the DCR — restart AMA to regenerate."
                .to_string(),
            doc_link: Some(
                "https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-troubleshoot-linux-vm"
                    .to_string(),
            ),
        });
    }

    // Info finding if debug logging is enabled
    if config.is_debug_logging() {
        findings.push(Finding {
            rule_id: "FBCFG-003".to_string(),
            name: "Fluentbit Debug Logging Enabled".to_string(),
            severity: Severity::Info,
            category: Category::Syslog,
            description: "Fluentbit Log_Level is set to 'debug'. This generates \
                high log volume and may impact performance. Disable after troubleshooting."
                .to_string(),
            evidence: vec!["td-agent.conf [SERVICE] Log_Level = debug".to_string()],
            remediation: "Set Log_Level back to 'info' in td-agent.conf and restart AMA.".to_string(),
            doc_link: None,
        });
    }
}

/// Validate rsyslog forwarding to AMA port 28330.
fn check_rsyslog_forwarding(data: &LinuxData, findings: &mut Vec<Finding>) {
    // Only relevant if syslog rules were found in the bundle
    if data.rsyslog_rules.is_empty() {
        // No rsyslog config found — this is covered by YAML rule SYSLOG-003
        return;
    }

    // Check if any rule forwards to port 28330
    let has_ama_forward = data
        .rsyslog_rules
        .iter()
        .any(|r| r.target_port == Some(28330));

    if !has_ama_forward {
        let evidence: Vec<String> = data
            .rsyslog_rules
            .iter()
            .take(5)
            .map(|r| r.raw_line.clone())
            .collect();

        findings.push(Finding {
            rule_id: "RSYSLOG-001".to_string(),
            name: "Rsyslog Not Forwarding to AMA".to_string(),
            severity: Severity::Warning,
            category: Category::Syslog,
            description: "Rsyslog forwarding rules were found but none forward to \
                port 28330 (the mdsd listening port). Syslog data will not reach AMA."
                .to_string(),
            evidence,
            remediation: "Verify /etc/rsyslog.d/10-azuremonitoragent.conf contains \
                a forwarding rule to @@127.0.0.1:28330. Restart rsyslog after changes."
                .to_string(),
            doc_link: Some(
                "https://learn.microsoft.com/en-us/azure/sentinel/cef-syslog-ama-troubleshooting"
                    .to_string(),
            ),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::fluentbit_config::{
        FluentbitConfig, FluentbitInput, FluentbitOutput, FluentbitService,
    };
    use crate::parsers::linux::RsyslogForwardRule;
    use crate::parsers::mdsd_qos::MdsdQosEntry;

    fn make_qos_entry(
        blob_type: &str,
        total: u64,
        success: u64,
        fail: u64,
    ) -> MdsdQosEntry {
        MdsdQosEntry {
            blob_type: blob_type.to_string(),
            total_count: total,
            success_count: success,
            fail_count: fail,
            timestamp: Some("2026-03-25T10:00:00".to_string()),
        }
    }

    #[test]
    fn detects_total_upload_failure() {
        let data = LinuxData {
            mdsd_qos_entries: vec![make_qos_entry("LINUX_SYSLOG_BLOB", 50, 0, 50)],
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().any(|f| f.rule_id == "QOS-002"));
    }

    #[test]
    fn detects_partial_upload_failure() {
        let data = LinuxData {
            mdsd_qos_entries: vec![make_qos_entry("LINUX_SYSLOG_BLOB", 100, 80, 20)],
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().any(|f| f.rule_id == "QOS-003"));
    }

    #[test]
    fn healthy_qos_produces_no_findings() {
        let data = LinuxData {
            mdsd_qos_entries: vec![make_qos_entry("LINUX_SYSLOG_BLOB", 100, 100, 0)],
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings
            .iter()
            .all(|f| f.rule_id != "QOS-002" && f.rule_id != "QOS-003"));
    }

    #[test]
    fn detects_no_fluentbit_inputs() {
        let data = LinuxData {
            fluentbit_config: Some(FluentbitConfig {
                inputs: vec![],
                outputs: vec![FluentbitOutput {
                    name: Some("tcp".to_string()),
                    port: Some(28330),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().any(|f| f.rule_id == "FBCFG-001"));
    }

    #[test]
    fn detects_fluentbit_not_targeting_mdsd() {
        let data = LinuxData {
            fluentbit_config: Some(FluentbitConfig {
                inputs: vec![FluentbitInput {
                    name: Some("tail".to_string()),
                    path: Some("/var/log/test.log".to_string()),
                    ..Default::default()
                }],
                outputs: vec![FluentbitOutput {
                    name: Some("file".to_string()),
                    path: Some("/tmp/out.log".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().any(|f| f.rule_id == "FBCFG-002"));
    }

    #[test]
    fn detects_debug_logging() {
        let data = LinuxData {
            fluentbit_config: Some(FluentbitConfig {
                service: FluentbitService {
                    log_level: Some("debug".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().any(|f| f.rule_id == "FBCFG-003"));
    }

    #[test]
    fn detects_rsyslog_not_forwarding_to_ama() {
        let data = LinuxData {
            rsyslog_rules: vec![RsyslogForwardRule {
                facility: Some("*.*".to_string()),
                target_host: Some("10.0.0.1".to_string()),
                target_port: Some(514),
                raw_line: "*.* @10.0.0.1:514".to_string(),
            }],
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().any(|f| f.rule_id == "RSYSLOG-001"));
    }

    #[test]
    fn rsyslog_forwarding_to_ama_is_healthy() {
        let data = LinuxData {
            rsyslog_rules: vec![RsyslogForwardRule {
                facility: Some("*.*".to_string()),
                target_host: Some("127.0.0.1".to_string()),
                target_port: Some(28330),
                raw_line: "*.* @@127.0.0.1:28330".to_string(),
            }],
            ..Default::default()
        };
        let findings = check_linux(&data);
        assert!(findings.iter().all(|f| f.rule_id != "RSYSLOG-001"));
    }
}
