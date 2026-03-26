use serde::Serialize;
use std::fmt;

/// Severity of a diagnostic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Critical => write!(f, "Critical"),
        }
    }
}

/// Category of a diagnostic finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Installation,
    AgentNotRunning,
    Identity,
    Connectivity,
    Dcr,
    PerformanceCounters,
    EventLog,
    Syslog,
    MetricsExtension,
    Heartbeat,
    ArcAgent,
    VersionMismatch,
    Sizing,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Installation => write!(f, "Installation"),
            Category::AgentNotRunning => write!(f, "Agent Not Running"),
            Category::Identity => write!(f, "Identity"),
            Category::Connectivity => write!(f, "Connectivity"),
            Category::Dcr => write!(f, "DCR Configuration"),
            Category::PerformanceCounters => write!(f, "Performance Counters"),
            Category::EventLog => write!(f, "Event Log Collection"),
            Category::Syslog => write!(f, "Syslog/CEF Forwarding"),
            Category::MetricsExtension => write!(f, "Metrics Extension"),
            Category::Heartbeat => write!(f, "Heartbeat / Data Gap"),
            Category::ArcAgent => write!(f, "Arc Agent"),
            Category::VersionMismatch => write!(f, "Version Mismatch"),
            Category::Sizing => write!(f, "Environment Sizing"),
        }
    }
}

/// A single diagnostic finding from the analysis.
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub name: String,
    pub severity: Severity,
    pub category: Category,
    pub description: String,
    pub evidence: Vec<String>,
    pub remediation: String,
    pub doc_link: Option<String>,
}

impl Finding {
    pub fn is_critical(&self) -> bool {
        self.severity == Severity::Critical
    }
}

/// Target platform for detection rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Linux,
}

impl Platform {
    pub fn as_str(self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::Linux => "linux",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Windows => write!(f, "Windows"),
            Platform::Linux => write!(f, "Linux"),
        }
    }
}

/// Environment information extracted from the bundle.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnvironmentInfo {
    pub os: Option<String>,
    pub platform: Option<Platform>,
    pub ama_version: Option<String>,
    pub vm_sku: Option<String>,
    pub is_arc: Option<bool>,
    pub dcr_count: usize,
    pub hostname: Option<String>,
    pub resource_id: Option<String>,
}

/// Complete diagnostic report.
#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticReport {
    pub environment: EnvironmentInfo,
    pub findings: Vec<Finding>,
    pub files_analyzed: usize,
    pub bundle_path: String,
}

impl DiagnosticReport {
    pub fn new(bundle_path: String) -> Self {
        Self {
            environment: EnvironmentInfo::default(),
            findings: Vec::new(),
            files_analyzed: 0,
            bundle_path,
        }
    }

    pub fn has_critical_findings(&self) -> bool {
        self.findings.iter().any(|f| f.is_critical())
    }

    pub fn finding_count_by_severity(&self, severity: Severity) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_finding(severity: Severity) -> Finding {
        Finding {
            rule_id: "TEST-001".to_string(),
            name: "Test Finding".to_string(),
            severity,
            category: Category::Connectivity,
            description: "A test finding".to_string(),
            evidence: vec!["test evidence".to_string()],
            remediation: "Fix it".to_string(),
            doc_link: None,
        }
    }

    #[test]
    fn finding_is_critical() {
        assert!(sample_finding(Severity::Critical).is_critical());
        assert!(!sample_finding(Severity::Warning).is_critical());
        assert!(!sample_finding(Severity::Info).is_critical());
    }

    #[test]
    fn severity_ordering() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Critical);
    }

    #[test]
    fn report_has_critical_findings() {
        let mut report = DiagnosticReport::new("test".to_string());
        assert!(!report.has_critical_findings());

        report.findings.push(sample_finding(Severity::Warning));
        assert!(!report.has_critical_findings());

        report.findings.push(sample_finding(Severity::Critical));
        assert!(report.has_critical_findings());
    }

    #[test]
    fn report_count_by_severity() {
        let mut report = DiagnosticReport::new("test".to_string());
        report.findings.push(sample_finding(Severity::Critical));
        report.findings.push(sample_finding(Severity::Critical));
        report.findings.push(sample_finding(Severity::Warning));
        report.findings.push(sample_finding(Severity::Info));

        assert_eq!(report.finding_count_by_severity(Severity::Critical), 2);
        assert_eq!(report.finding_count_by_severity(Severity::Warning), 1);
        assert_eq!(report.finding_count_by_severity(Severity::Info), 1);
    }

    #[test]
    fn severity_display() {
        assert_eq!(Severity::Critical.to_string(), "Critical");
        assert_eq!(Severity::Warning.to_string(), "Warning");
        assert_eq!(Severity::Info.to_string(), "Info");
    }

    #[test]
    fn category_display() {
        assert_eq!(Category::Installation.to_string(), "Installation");
        assert_eq!(Category::Dcr.to_string(), "DCR Configuration");
        assert_eq!(Category::Syslog.to_string(), "Syslog/CEF Forwarding");
    }

    #[test]
    fn platform_display() {
        assert_eq!(Platform::Windows.to_string(), "Windows");
        assert_eq!(Platform::Linux.to_string(), "Linux");
    }

    #[test]
    fn empty_report_no_critical() {
        let report = DiagnosticReport::new("empty".to_string());
        assert!(!report.has_critical_findings());
        assert_eq!(report.findings.len(), 0);
        assert_eq!(report.finding_count_by_severity(Severity::Critical), 0);
    }
}
