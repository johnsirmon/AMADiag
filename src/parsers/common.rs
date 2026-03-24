use regex::Regex;
use std::sync::LazyLock;

/// A parsed log line with metadata.
#[derive(Debug, Clone)]
pub struct LogLine {
    pub file: String,
    pub line_num: usize,
    pub level: LogLevel,
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
                content: line.to_string(),
            }
        })
        .collect()
}

fn classify_line(line: &str) -> LogLevel {
    if ERROR_RE.is_match(line) {
        LogLevel::Error
    } else if WARN_RE.is_match(line) {
        LogLevel::Warning
    } else {
        LogLevel::Unknown
    }
}

/// Common regex patterns for AMA diagnostic log analysis.
pub struct Patterns;

impl Patterns {
    pub fn imds_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(IMDS|169\.254\.169\.254).*(unreachable|timeout|fail|error|refused)")
                .unwrap()
        });
        &RE
    }

    pub fn auth_token_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(managed.identity|MSI|auth.?token).*(fail|error|missing|absent|expired)")
                .unwrap()
        });
        &RE
    }

    pub fn connectivity_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(connect|network|endpoint|AMCS|ingestion).*(fail|error|refused|timeout|unreachable)")
                .unwrap()
        });
        &RE
    }

    pub fn service_crash() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(crash|terminated unexpectedly|service.stopped|not.running|process.exited)")
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
            Regex::new(r"(?i)(extension|provisioning).*(fail|error|timeout|not.installed)")
                .unwrap()
        });
        &RE
    }

    pub fn syslog_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(rsyslog|syslog-ng|syslog|CEF).*(fail|error|not.running|stopped|refused)")
                .unwrap()
        });
        &RE
    }

    pub fn metrics_extension_error() -> &'static Regex {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?i)(MetricsExtension|ME\b).*(error|fail|Level\s*2)")
                .unwrap()
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
            Regex::new(r"(?i)(?:version|ver)[:\s]*(\d+\.\d+\.\d+(?:\.\d+)?)")
                .unwrap()
        });
        &RE
    }
}
