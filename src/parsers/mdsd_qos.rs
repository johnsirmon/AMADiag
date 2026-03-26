use regex::Regex;
use std::sync::LazyLock;

/// A parsed entry from the mdsd.qos log showing upload metrics per blob type.
#[derive(Debug, Clone)]
pub struct MdsdQosEntry {
    pub blob_type: String,
    pub total_count: u64,
    pub success_count: u64,
    pub fail_count: u64,
    pub timestamp: Option<String>,
}

impl MdsdQosEntry {
    /// Returns true if all events were uploaded successfully.
    pub fn is_healthy(&self) -> bool {
        self.fail_count == 0 && self.success_count == self.total_count
    }

    /// Returns true if there were complete upload failures (nothing succeeded).
    pub fn is_total_failure(&self) -> bool {
        self.total_count > 0 && self.success_count == 0
    }
}

// mdsd.qos lines contain key=value pairs like:
//   TotalCount=42 SuccessCount=42 FailCount=0 ... DataType=LINUX_SYSLOG_BLOB
// The exact format varies across AMA versions, so we use regex extraction.
static BLOB_TYPE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:DataType|InputType)\s*=\s*(\S+)").unwrap());
static TOTAL_COUNT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)TotalCount\s*=\s*(\d+)").unwrap());
static SUCCESS_COUNT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)SuccessCount\s*=\s*(\d+)").unwrap());
static FAIL_COUNT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)FailCount\s*=\s*(\d+)").unwrap());
static TIMESTAMP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}").unwrap());

/// Parse mdsd.qos log content into structured QoS entries.
///
/// Each line that contains a blob type and count metrics is parsed.
/// Lines that don't match are silently skipped.
pub fn parse_mdsd_qos(content: &str) -> Vec<MdsdQosEntry> {
    content
        .lines()
        .filter_map(|line| {
            let blob_type = BLOB_TYPE_RE
                .captures(line)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())?;

            let total_count = TOTAL_COUNT_RE
                .captures(line)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let success_count = SUCCESS_COUNT_RE
                .captures(line)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let fail_count = FAIL_COUNT_RE
                .captures(line)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let timestamp = TIMESTAMP_RE.find(line).map(|m| m.as_str().to_string());

            Some(MdsdQosEntry {
                blob_type,
                total_count,
                success_count,
                fail_count,
                timestamp,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_healthy_syslog_qos() {
        let content = "2026-03-25T10:00:00 DataType=LINUX_SYSLOG_BLOB TotalCount=100 SuccessCount=100 FailCount=0\n";
        let entries = parse_mdsd_qos(content);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].blob_type, "LINUX_SYSLOG_BLOB");
        assert_eq!(entries[0].total_count, 100);
        assert_eq!(entries[0].success_count, 100);
        assert_eq!(entries[0].fail_count, 0);
        assert!(entries[0].is_healthy());
        assert!(!entries[0].is_total_failure());
    }

    #[test]
    fn parse_failing_cef_qos() {
        let content = "2026-03-25T10:05:00 DataType=SECURITY_CEF_BLOB TotalCount=50 SuccessCount=0 FailCount=50\n";
        let entries = parse_mdsd_qos(content);
        assert_eq!(entries.len(), 1);
        assert!(!entries[0].is_healthy());
        assert!(entries[0].is_total_failure());
    }

    #[test]
    fn parse_partial_failure() {
        let content = "DataType=GENERIC_PERF_BLOB TotalCount=200 SuccessCount=180 FailCount=20\n";
        let entries = parse_mdsd_qos(content);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].fail_count, 20);
        assert!(!entries[0].is_healthy());
        assert!(!entries[0].is_total_failure());
    }

    #[test]
    fn parse_mixed_lines() {
        let content = "\
2026-03-25T10:00:00 DataType=LINUX_SYSLOG_BLOB TotalCount=100 SuccessCount=100 FailCount=0
some random line without metrics
2026-03-25T10:05:00 DataType=SECURITY_CEF_BLOB TotalCount=50 SuccessCount=30 FailCount=20
";
        let entries = parse_mdsd_qos(content);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].blob_type, "LINUX_SYSLOG_BLOB");
        assert_eq!(entries[1].blob_type, "SECURITY_CEF_BLOB");
    }

    #[test]
    fn skip_non_qos_lines() {
        let content = "just a regular log line\nanother line\n";
        let entries = parse_mdsd_qos(content);
        assert!(entries.is_empty());
    }

    #[test]
    fn handles_inputtype_variant() {
        let content = "InputType=LINUX_SYSLOG_BLOB TotalCount=10 SuccessCount=10 FailCount=0\n";
        let entries = parse_mdsd_qos(content);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].blob_type, "LINUX_SYSLOG_BLOB");
    }
}
