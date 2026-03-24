use crate::analyzers::finding::{Category, Finding, Severity};
use crate::parsers::ParsedBundle;

/// Check environment sizing from parsed bundle data.
pub fn check_sizing(bundle: &ParsedBundle) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Check for high event volume indicators
    check_event_volume(bundle, &mut findings);

    // Check for memory/resource constraints in logs
    check_resource_constraints(bundle, &mut findings);

    findings
}

fn check_event_volume(bundle: &ParsedBundle, findings: &mut Vec<Finding>) {
    // Check MetricsExtension ETW entries for Level 2 (error) volume
    let error_events: Vec<_> = bundle
        .event_entries
        .iter()
        .filter(|e| e.level == Some(2))
        .collect();

    if error_events.len() > 100 {
        findings.push(Finding {
            rule_id: "SZ-004".to_string(),
            name: "High Error Event Volume".to_string(),
            severity: Severity::Warning,
            category: Category::Sizing,
            description: format!(
                "Found {} Level-2 (error) events in MetricsExtension ETW traces. \
                 High error volume may indicate the agent is overwhelmed or misconfigured.",
                error_events.len()
            ),
            evidence: error_events
                .iter()
                .take(3)
                .map(|e| {
                    format!(
                        "{}: {}",
                        e.timestamp.as_deref().unwrap_or("unknown time"),
                        &e.message[..e.message.len().min(200)]
                    )
                })
                .collect(),
            remediation: "Review VM sizing. Consider upgrading to a larger SKU or reducing \
                         data collection scope in the DCR."
                .to_string(),
            doc_link: Some("https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance".to_string()),
        });
    }
}

fn check_resource_constraints(bundle: &ParsedBundle, findings: &mut Vec<Finding>) {
    use crate::parsers::common::Patterns;
    use regex::Regex;
    use std::sync::LazyLock;

    static MEMORY_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)(out.of.memory|OOM|memory.pressure|low.memory|insufficient.memory)")
            .unwrap()
    });

    static DISK_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)(disk.full|no.space|insufficient.disk|disk.space|write.fail.*disk)")
            .unwrap()
    });

    static CPU_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)(high.cpu|cpu.throttl|cpu.usage.*9[0-9]%|cpu.usage.*100%)")
            .unwrap()
    });

    let _ = Patterns::version_pattern(); // ensure lazy init

    // Memory checks
    let mem_evidence: Vec<_> = bundle
        .log_lines
        .iter()
        .filter(|l| MEMORY_RE.is_match(&l.content))
        .take(5)
        .map(|l| format!("{}:{}: {}", l.file, l.line_num, l.content))
        .collect();

    if !mem_evidence.is_empty() {
        findings.push(Finding {
            rule_id: "SZ-001".to_string(),
            name: "Low Memory Detected".to_string(),
            severity: Severity::Warning,
            category: Category::Sizing,
            description: "Log entries indicate memory pressure on this machine. \
                         Low memory can cause AMA to drop events or crash."
                .to_string(),
            evidence: mem_evidence,
            remediation: "Ensure the VM has at least 256 MB free memory. Consider upgrading \
                         the VM SKU or reducing data collection scope."
                .to_string(),
            doc_link: Some("https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance".to_string()),
        });
    }

    // Disk checks
    let disk_evidence: Vec<_> = bundle
        .log_lines
        .iter()
        .filter(|l| DISK_RE.is_match(&l.content))
        .take(5)
        .map(|l| format!("{}:{}: {}", l.file, l.line_num, l.content))
        .collect();

    if !disk_evidence.is_empty() {
        findings.push(Finding {
            rule_id: "SZ-003".to_string(),
            name: "Disk Space Constraint".to_string(),
            severity: Severity::Warning,
            category: Category::Sizing,
            description: "Log entries indicate disk space issues. AMA requires adequate \
                         disk space for log buffering and data store."
                .to_string(),
            evidence: disk_evidence,
            remediation: "Free disk space on the data store partition or expand the disk."
                .to_string(),
            doc_link: None,
        });
    }

    // CPU checks
    let cpu_evidence: Vec<_> = bundle
        .log_lines
        .iter()
        .filter(|l| CPU_RE.is_match(&l.content))
        .take(5)
        .map(|l| format!("{}:{}: {}", l.file, l.line_num, l.content))
        .collect();

    if !cpu_evidence.is_empty() {
        findings.push(Finding {
            rule_id: "SZ-002".to_string(),
            name: "High CPU Utilization".to_string(),
            severity: Severity::Warning,
            category: Category::Sizing,
            description: "Log entries indicate high CPU usage by AMA processes. This may \
                         cause delayed data collection or event drops."
                .to_string(),
            evidence: cpu_evidence,
            remediation: "Consider upgrading to a VM SKU with more CPU cores or reducing \
                         data collection volume."
                .to_string(),
            doc_link: Some("https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-performance".to_string()),
        });
    }
}
