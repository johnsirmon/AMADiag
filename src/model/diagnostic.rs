use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsKind {
    Windows,
    Linux,
}

impl fmt::Display for OsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Windows => write!(f, "Windows"),
            Self::Linux => write!(f, "Linux"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn is_warning_or_higher(self) -> bool {
        self >= Self::Medium
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pass,
    Warn,
    Fail,
    Unknown,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => write!(f, "Pass"),
            Self::Warn => write!(f, "Warn"),
            Self::Fail => write!(f, "Fail"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Install,
    Service,
    Heartbeat,
    Connectivity,
    Dcr,
    Mdsd,
    FluentBit,
    Syslog,
    CustomLogs,
    CpuMemory,
    ArcExtension,
    IdentityAuth,
    ProxyTls,
    Config,
    Other,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Install => write!(f, "Install"),
            Self::Service => write!(f, "Service"),
            Self::Heartbeat => write!(f, "Heartbeat"),
            Self::Connectivity => write!(f, "Connectivity"),
            Self::Dcr => write!(f, "DCR"),
            Self::Mdsd => write!(f, "MDSD"),
            Self::FluentBit => write!(f, "FluentBit"),
            Self::Syslog => write!(f, "Syslog"),
            Self::CustomLogs => write!(f, "Custom Logs"),
            Self::CpuMemory => write!(f, "CPU/Memory"),
            Self::ArcExtension => write!(f, "Arc Extension"),
            Self::IdentityAuth => write!(f, "Identity/Auth"),
            Self::ProxyTls => write!(f, "Proxy/TLS"),
            Self::Config => write!(f, "Config"),
            Self::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub file_id: String,
    pub line_range: Range<usize>,
    pub preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticEvent {
    pub ts: Option<DateTime<Utc>>,
    pub os: OsKind,
    pub category: Category,
    pub severity: Severity,
    pub status: Status,
    pub title: String,
    pub summary: String,
    pub details: Option<String>,
    pub evidence: Vec<EvidenceRef>,
    pub tags: Vec<String>,
    pub rule_id: Option<String>,
    pub remediation: Option<String>,
    pub doc_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingGroup {
    pub key: String,
    pub category: Category,
    pub severity: Severity,
    pub status: Status,
    pub title: String,
    pub summary: String,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub events: Vec<usize>,
    pub likely_causes: Vec<String>,
    pub suggested_actions: Vec<String>,
    pub evidence: Vec<EvidenceRef>,
    pub doc_links: Vec<String>,
    pub rule_ids: Vec<String>,
}

impl FindingGroup {
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

impl From<crate::analyzers::finding::Platform> for OsKind {
    fn from(value: crate::analyzers::finding::Platform) -> Self {
        match value {
            crate::analyzers::finding::Platform::Windows => Self::Windows,
            crate::analyzers::finding::Platform::Linux => Self::Linux,
        }
    }
}

impl From<crate::analyzers::finding::Severity> for Severity {
    fn from(value: crate::analyzers::finding::Severity) -> Self {
        match value {
            crate::analyzers::finding::Severity::Critical => Self::Critical,
            crate::analyzers::finding::Severity::Warning => Self::Medium,
            crate::analyzers::finding::Severity::Info => Self::Info,
        }
    }
}

impl From<crate::analyzers::finding::Category> for Category {
    fn from(value: crate::analyzers::finding::Category) -> Self {
        use crate::analyzers::finding::Category as OldCategory;
        match value {
            OldCategory::Installation => Self::Install,
            OldCategory::AgentNotRunning => Self::Service,
            OldCategory::Identity => Self::IdentityAuth,
            OldCategory::Connectivity => Self::Connectivity,
            OldCategory::Dcr => Self::Dcr,
            OldCategory::PerformanceCounters => Self::Config,
            OldCategory::EventLog => Self::CustomLogs,
            OldCategory::Syslog => Self::Syslog,
            OldCategory::MetricsExtension => Self::Config,
            OldCategory::Heartbeat => Self::Heartbeat,
            OldCategory::ArcAgent => Self::ArcExtension,
            OldCategory::VersionMismatch => Self::Config,
            OldCategory::Sizing => Self::CpuMemory,
        }
    }
}

impl DiagnosticEvent {
    pub fn from_legacy_finding(finding: &crate::analyzers::finding::Finding, os: OsKind) -> Self {
        let severity = Severity::from(finding.severity);
        let status = match severity {
            Severity::Critical | Severity::High => Status::Fail,
            Severity::Medium | Severity::Low => Status::Warn,
            Severity::Info => Status::Unknown,
        };

        let evidence = finding
            .evidence
            .iter()
            .enumerate()
            .map(|(index, line)| EvidenceRef {
                file_id: "legacy".to_string(),
                line_range: index + 1..index + 2,
                preview: line.clone(),
            })
            .collect();

        Self {
            ts: None,
            os,
            category: Category::from(finding.category.clone()),
            severity,
            status,
            title: finding.name.clone(),
            summary: finding.description.clone(),
            details: Some(finding.remediation.clone()),
            evidence,
            tags: vec!["legacy-report".to_string()],
            rule_id: Some(finding.rule_id.clone()),
            remediation: Some(finding.remediation.clone()),
            doc_link: finding.doc_link.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_severity_maps_to_new_scale() {
        assert_eq!(
            Severity::from(crate::analyzers::finding::Severity::Critical),
            Severity::Critical
        );
        assert_eq!(
            Severity::from(crate::analyzers::finding::Severity::Warning),
            Severity::Medium
        );
    }
}
