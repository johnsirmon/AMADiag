use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use std::sync::LazyLock;

/// A parsed log line with metadata.
#[derive(Debug, Clone)]
pub struct LogLine {
    pub file: String,
    pub line_num: usize,
    pub level: LogLevel,
    pub timestamp: Option<DateTime<Utc>>,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Unknown,
}

static ERROR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(error|fatal|fail(ed|ure)?|exception)\b").unwrap());
static WARN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(warn(ing)?|caution)\b").unwrap());
static INFO_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(info|information)\b").unwrap());
static DEBUG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bdebug\b").unwrap());
static RFC3339_TS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<ts>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2}))")
        .unwrap()
});
static SPACE_TS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<ts>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}(?:\.\d+)?)").unwrap());
static SLASH_TS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<ts>\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2}(?:\.\d+)?)").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampKind {
    DateTime(DateTime<Utc>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractedTimestamp {
    pub kind: TimestampKind,
}

/// Parse raw text into classified log lines.
pub fn parse_log_lines(content: &str, file_name: &str) -> Vec<LogLine> {
    content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let level = classify_line(line);
            LogLine {
                file: file_name.to_string(),
                line_num: i + 1,
                level,
                timestamp: extract_timestamp(line).map(|value| match value.kind {
                    TimestampKind::DateTime(ts) => ts,
                }),
                content: line.to_string(),
            }
        })
        .collect()
}

pub fn classify_line(line: &str) -> LogLevel {
    if ERROR_RE.is_match(line) {
        LogLevel::Error
    } else if WARN_RE.is_match(line) {
        LogLevel::Warning
    } else if INFO_RE.is_match(line) {
        LogLevel::Info
    } else if DEBUG_RE.is_match(line) {
        LogLevel::Debug
    } else {
        LogLevel::Unknown
    }
}

pub fn extract_timestamp(line: &str) -> Option<ExtractedTimestamp> {
    if let Some(captures) = RFC3339_TS_RE.captures(line) {
        let timestamp = captures.name("ts")?.as_str();
        if let Ok(parsed) = DateTime::parse_from_rfc3339(timestamp) {
            return Some(ExtractedTimestamp {
                kind: TimestampKind::DateTime(parsed.with_timezone(&Utc)),
            });
        }
    }

    if let Some(captures) = SPACE_TS_RE.captures(line) {
        let timestamp = captures.name("ts")?.as_str();
        for format in ["%Y-%m-%d %H:%M:%S%.f", "%Y-%m-%d %H:%M:%S"] {
            if let Ok(parsed) = NaiveDateTime::parse_from_str(timestamp, format) {
                return Some(ExtractedTimestamp {
                    kind: TimestampKind::DateTime(parsed.and_utc()),
                });
            }
        }
    }

    if let Some(captures) = SLASH_TS_RE.captures(line) {
        let timestamp = captures.name("ts")?.as_str();
        for format in ["%Y/%m/%d %H:%M:%S%.f", "%Y/%m/%d %H:%M:%S"] {
            if let Ok(parsed) = NaiveDateTime::parse_from_str(timestamp, format) {
                return Some(ExtractedTimestamp {
                    kind: TimestampKind::DateTime(parsed.and_utc()),
                });
            }
        }
    }

    None
}

/// Common regex patterns for AMA diagnostic log analysis.
pub struct Patterns;

impl Patterns {
    pub fn imds_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(IMDS|169\.254\.169\.254).*(unreachable|timeout|\bfail(ed|ure)?\b|\berror\b|refused)")
                .unwrap()
        });
        &RE
    }

    pub fn auth_token_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(managed.identity|MSI|auth.?token).*(fail|error|missing|absent|expired)",
            )
            .unwrap()
        });
        &RE
    }

    pub fn connectivity_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(AMCS|handler\.control|ingest\.monitor|ods\.opinsights|monitor\.azure\.com|global\.handler).*(\bfail(ed|ure)?\b|\berror\b|refused|timeout|unreachable)")
                .unwrap()
        });
        &RE
    }

    pub fn service_crash() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(crash(ed|ing)?|terminated unexpectedly|service.*(stopped|failed|dead)|process.exited.*(error|abnormal|unexpected))",
            )
            .unwrap()
        });
        &RE
    }

    pub fn dcr_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(DCR|data.collection.rule).*(not.found|missing|invalid|error|fail)")
                .unwrap()
        });
        &RE
    }

    pub fn extension_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(extension|provisioning).*(\bfailed\b|\bfailure\b|\btimeout\b|not.installed|provision.*(error|fail))").unwrap()
        });
        &RE
    }

    pub fn syslog_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(rsyslog|syslog-ng|syslog|CEF).*(fail|error|not.running|stopped|refused)",
            )
            .unwrap()
        });
        &RE
    }

    pub fn metrics_extension_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(MetricsExtension|ME\b).*(\berror\b|\bfail(ed|ure)?\b|Level\s*2)").unwrap()
        });
        &RE
    }

    pub fn arc_agent_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(himds|connected.machine|arc.agent|azcmagent).*(fail|error|not.running|stopped)")
                .unwrap()
        });
        &RE
    }

    pub fn version_pattern() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(?:version|ver)[:\s]*(\d+\.\d+\.\d+(?:\.\d+)?)").unwrap()
        });
        &RE
    }

    pub fn oom_killer() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(oom-kill|Out of memory.*Killed process).*(mdsd|amacoreagent|azuremonitor)",
            )
            .unwrap()
        });
        &RE
    }

    pub fn fluentbit_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)\[error\].*(fluentbit|fluent.bit|td-agent)").unwrap()
        });
        &RE
    }

    pub fn mdsd_qos_failure() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(SuccessCount\s*=\s*0|FailCount\s*=\s*[1-9])").unwrap()
        });
        &RE
    }

    pub fn throttling() -> &'static Regex {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)Throttling ingestion").unwrap());
        &RE
    }

    pub fn guest_agent_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(walinuxagent|waagent|guest.agent).*(fail|error|stopped|not.running|dead)",
            )
            .unwrap()
        });
        &RE
    }

    pub fn systemd_service_failure() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r"(?i)(azuremonitoragent|azuremonitor-coreagent).*(failed|inactive|dead|not.running)",
            )
            .unwrap()
        });
        &RE
    }

    pub fn disk_full() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(no space left|disk full|cannot write|ENOSPC)").unwrap()
        });
        &RE
    }

    pub fn cgroup_oom() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)CONSTRAINT_MEMCG.*(mdsd|amacoreagent|azuremonitor)").unwrap()
        });
        &RE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_error_lines() {
        assert_eq!(classify_line("FATAL: disk full"), LogLevel::Error);
        assert_eq!(classify_line("Operation failed"), LogLevel::Error);
        assert_eq!(classify_line("exception occurred"), LogLevel::Error);
    }

    #[test]
    fn classify_warning_lines() {
        assert_eq!(classify_line("WARNING: low disk"), LogLevel::Warning);
        assert_eq!(classify_line("caution: retry"), LogLevel::Warning);
    }

    #[test]
    fn classify_unknown_lines() {
        assert_eq!(classify_line("all systems nominal"), LogLevel::Unknown);
        assert_eq!(classify_line(""), LogLevel::Unknown);
    }

    #[test]
    fn parse_log_lines_basic() {
        let content = "line one\nerror occurred\nwarning low memory\ninfo line";
        let lines = parse_log_lines(content, "test.log");
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0].level, LogLevel::Unknown);
        assert_eq!(lines[1].level, LogLevel::Error);
        assert_eq!(lines[2].level, LogLevel::Warning);
        assert_eq!(lines[0].line_num, 1);
        assert_eq!(lines[0].file, "test.log");
        assert!(lines[0].timestamp.is_none());
    }

    #[test]
    fn extract_rfc3339_timestamp() {
        let timestamp = extract_timestamp("2026-03-26T10:15:00Z error")
            .map(|value| match value.kind {
                TimestampKind::DateTime(ts) => ts,
            })
            .unwrap();
        assert_eq!(timestamp.to_rfc3339(), "2026-03-26T10:15:00+00:00");
    }

    #[test]
    fn extract_space_separated_timestamp() {
        let timestamp = extract_timestamp("2026-03-26 10:15:00.123 warning")
            .map(|value| match value.kind {
                TimestampKind::DateTime(ts) => ts,
            })
            .unwrap();
        assert_eq!(
            timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2026-03-26 10:15:00"
        );
    }

    #[test]
    fn extract_slash_separated_timestamp() {
        let timestamp = extract_timestamp("2026/03/26 10:15:00 warning")
            .map(|value| match value.kind {
                TimestampKind::DateTime(ts) => ts,
            })
            .unwrap();
        assert_eq!(
            timestamp.format("%Y/%m/%d %H:%M:%S").to_string(),
            "2026/03/26 10:15:00"
        );
    }

    #[test]
    fn imds_error_matches() {
        assert!(Patterns::imds_error().is_match("IMDS endpoint unreachable"));
        assert!(Patterns::imds_error().is_match("169.254.169.254 timeout"));
        assert!(!Patterns::imds_error().is_match("all systems normal"));
    }

    #[test]
    fn auth_token_error_matches() {
        assert!(Patterns::auth_token_error().is_match("managed identity token failed"));
        assert!(Patterns::auth_token_error().is_match("MSI auth token expired"));
        assert!(!Patterns::auth_token_error().is_match("token refresh succeeded"));
    }

    #[test]
    fn connectivity_error_matches() {
        assert!(Patterns::connectivity_error().is_match("AMCS endpoint connection refused"));
        assert!(Patterns::connectivity_error().is_match("network timeout on ingestion"));
        assert!(!Patterns::connectivity_error().is_match("connected successfully"));
    }

    #[test]
    fn service_crash_matches() {
        assert!(Patterns::service_crash().is_match("process exited unexpectedly"));
        assert!(Patterns::service_crash().is_match("service stopped"));
        assert!(!Patterns::service_crash().is_match("service started"));
    }

    #[test]
    fn dcr_error_matches() {
        assert!(Patterns::dcr_error().is_match("DCR not found for workspace"));
        assert!(Patterns::dcr_error().is_match("data collection rule invalid"));
        assert!(!Patterns::dcr_error().is_match("DCR applied successfully"));
    }

    #[test]
    fn extension_error_matches() {
        assert!(Patterns::extension_error().is_match("extension provisioning failed"));
        assert!(!Patterns::extension_error().is_match("extension installed"));
    }

    #[test]
    fn syslog_error_matches() {
        assert!(Patterns::syslog_error().is_match("rsyslog not running"));
        assert!(Patterns::syslog_error().is_match("syslog-ng stopped"));
        assert!(!Patterns::syslog_error().is_match("rsyslog is active"));
    }

    #[test]
    fn metrics_extension_error_matches() {
        assert!(Patterns::metrics_extension_error().is_match("MetricsExtension error in upload"));
        assert!(!Patterns::metrics_extension_error().is_match("MetricsExtension healthy"));
    }

    #[test]
    fn arc_agent_error_matches() {
        assert!(Patterns::arc_agent_error().is_match("azcmagent not running"));
        assert!(Patterns::arc_agent_error().is_match("connected machine agent failed"));
        assert!(!Patterns::arc_agent_error().is_match("arc agent connected"));
    }

    #[test]
    fn version_pattern_matches() {
        let re = Patterns::version_pattern();
        let caps = re.captures("Version: 1.24.3.0").unwrap();
        assert_eq!(&caps[1], "1.24.3.0");

        let caps = re.captures("ver: 2.0.1").unwrap();
        assert_eq!(&caps[1], "2.0.1");
    }
}
