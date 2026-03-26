use crate::model::diagnostic::{Category, DiagnosticEvent, EvidenceRef, OsKind, Severity, Status};
use crate::model::filter::FilterState;
use crate::parsers::common::{self, LogLevel, TimestampKind};
use crate::store::timeline::{build_timeline, TimelineBucket};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub const DEFAULT_STALE_LOG_AGE_DAYS: i64 = 180;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TimeRange {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

impl TimeRange {
    pub fn start(self) -> Option<DateTime<Utc>> {
        self.start
    }

    pub fn end(self) -> Option<DateTime<Utc>> {
        self.end
    }

    pub fn is_known(self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    fn include(&mut self, ts: DateTime<Utc>) {
        self.start = Some(self.start.map_or(ts, |current| current.min(ts)));
        self.end = Some(self.end.map_or(ts, |current| current.max(ts)));
    }

    fn include_range(&mut self, other: Self) {
        if let Some(start) = other.start {
            self.include(start);
        }
        if let Some(end) = other.end {
            self.include(end);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventStoreOptions {
    stale_log_cutoff: Option<DateTime<Utc>>,
}

impl Default for EventStoreOptions {
    fn default() -> Self {
        Self::last_days(DEFAULT_STALE_LOG_AGE_DAYS)
    }
}

impl EventStoreOptions {
    pub fn last_days(days: i64) -> Self {
        Self {
            stale_log_cutoff: (days > 0).then(|| Utc::now() - Duration::days(days)),
        }
    }

    fn should_skip_timestamped_file(self, latest_file_ts: DateTime<Utc>) -> bool {
        self.stale_log_cutoff
            .is_some_and(|cutoff| latest_file_ts < cutoff)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BundleDateSummary {
    discovered_range: TimeRange,
    analyzed_range: TimeRange,
    candidate_log_files: usize,
    timestamped_log_files: usize,
    stale_log_files_skipped: usize,
}

impl BundleDateSummary {
    pub fn discovered_range(&self) -> TimeRange {
        self.discovered_range
    }

    pub fn analyzed_range(&self) -> TimeRange {
        self.analyzed_range
    }

    pub fn candidate_log_files(&self) -> usize {
        self.candidate_log_files
    }

    pub fn timestamped_log_files(&self) -> usize {
        self.timestamped_log_files
    }

    pub fn stale_log_files_skipped(&self) -> usize {
        self.stale_log_files_skipped
    }
}

#[derive(Debug, Clone)]
pub struct EventStore {
    events: Vec<DiagnosticEvent>,
    time_index: BTreeMap<i64, Vec<usize>>,
    source_dir: PathBuf,
    date_summary: BundleDateSummary,
}

impl EventStore {
    pub fn from_bundle_dir(bundle_dir: &Path, os: OsKind) -> Result<Self> {
        Self::from_bundle_dir_with_options(bundle_dir, os, EventStoreOptions::default())
    }

    pub fn from_bundle_dir_with_options(
        bundle_dir: &Path,
        os: OsKind,
        options: EventStoreOptions,
    ) -> Result<Self> {
        let mut store = Self {
            events: Vec::new(),
            time_index: BTreeMap::new(),
            source_dir: bundle_dir.to_path_buf(),
            date_summary: BundleDateSummary::default(),
        };

        for entry in ignore::WalkBuilder::new(bundle_dir)
            .standard_filters(false)
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        {
            let path = entry.path();
            let relative_path = path
                .strip_prefix(bundle_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            if !is_candidate_log(&relative_path) {
                continue;
            }

            store.ingest_file(path, &relative_path, os, options)?;
        }

        Ok(store)
    }

    pub fn events(&self) -> &[DiagnosticEvent] {
        &self.events
    }

    pub fn source_dir(&self) -> &Path {
        &self.source_dir
    }

    pub fn date_summary(&self) -> &BundleDateSummary {
        &self.date_summary
    }

    pub fn latest_timestamp(&self) -> Option<DateTime<Utc>> {
        self.events.iter().filter_map(|event| event.ts).max()
    }

    pub fn filtered_event_indices(&self, filter: &FilterState) -> Vec<usize> {
        let latest = self.latest_timestamp();
        self.events
            .iter()
            .enumerate()
            .filter(|(_, event)| {
                event.severity >= filter.min_severity
                    && filter.allows_category(event.category)
                    && filter.time_filter.contains(event.ts, latest)
            })
            .map(|(index, _)| index)
            .collect()
    }

    pub fn timeline(&self, filter: &FilterState) -> Vec<TimelineBucket> {
        let indices = self.filtered_event_indices(filter);
        let events: Vec<_> = indices
            .into_iter()
            .filter_map(|index| self.events.get(index).cloned())
            .collect();
        build_timeline(&events, 5)
    }

    pub fn load_evidence_context(
        &self,
        evidence: &EvidenceRef,
        before: usize,
        after: usize,
    ) -> Result<Vec<String>> {
        let file_path = self.source_dir.join(&evidence.file_id);
        let file = std::fs::File::open(&file_path)?;
        let reader = BufReader::new(file);
        let start_line = evidence.line_range.start.saturating_sub(before).max(1);
        let end_line = evidence.line_range.end.saturating_add(after);

        let mut lines = Vec::new();
        for (index, line) in reader.lines().enumerate() {
            let line_number = index + 1;
            if line_number < start_line {
                continue;
            }
            if line_number > end_line {
                break;
            }

            let line = line?;
            let marker = if evidence.line_range.contains(&line_number) {
                '>'
            } else {
                ' '
            };
            lines.push(format!("{marker} {line_number:>6}  {line}"));
        }

        Ok(lines)
    }

    fn ingest_file(
        &mut self,
        path: &Path,
        relative_path: &str,
        os: OsKind,
        options: EventStoreOptions,
    ) -> Result<()> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let event_start = self.events.len();
        let mut touched_minutes = Vec::new();
        let mut file_range = TimeRange::default();

        for (index, line) in reader.lines().enumerate() {
            let line_number = index + 1;
            let line = line?;
            if let Some(candidate) = common::extract_timestamp(&line) {
                match candidate.kind {
                    TimestampKind::DateTime(value) => file_range.include(value),
                }
            }
            let Some(event) = classify_line(relative_path, line_number, &line, os) else {
                continue;
            };

            let event_index = self.events.len();
            if let Some(ts) = event.ts {
                let minute = ts.timestamp() / 60;
                self.time_index
                    .entry(minute)
                    .or_default()
                    .push(event_index);
                touched_minutes.push(minute);
            }
            self.events.push(event);
        }

        self.date_summary.candidate_log_files += 1;
        if file_range.is_known() {
            self.date_summary.timestamped_log_files += 1;
            self.date_summary.discovered_range.include_range(file_range);
        }

        if let Some(latest_file_ts) = file_range.end() {
            if options.should_skip_timestamped_file(latest_file_ts) {
                self.rollback_file_events(event_start, &touched_minutes);
                self.date_summary.stale_log_files_skipped += 1;
                return Ok(());
            }

            self.date_summary.analyzed_range.include_range(file_range);
        }

        Ok(())
    }

    fn rollback_file_events(&mut self, event_start: usize, touched_minutes: &[i64]) {
        self.events.truncate(event_start);

        let mut unique_minutes = touched_minutes.to_vec();
        unique_minutes.sort_unstable();
        unique_minutes.dedup();

        let mut empty_minutes = Vec::new();
        for minute in unique_minutes {
            if let Some(indices) = self.time_index.get_mut(&minute) {
                indices.retain(|index| *index < event_start);
                if indices.is_empty() {
                    empty_minutes.push(minute);
                }
            }
        }

        for minute in empty_minutes {
            self.time_index.remove(&minute);
        }
    }
}

fn classify_line(
    file_id: &str,
    line_number: usize,
    line: &str,
    os: OsKind,
) -> Option<DiagnosticEvent> {
    let log_level = common::classify_line(line);
    let timestamp = common::extract_timestamp(line).and_then(|candidate| match candidate.kind {
        TimestampKind::DateTime(value) => Some(value),
    });

    let mut category = Category::Other;
    let mut title = match log_level {
        LogLevel::Error => "Error log event",
        LogLevel::Warning => "Warning log event",
        LogLevel::Info => "Informational log event",
        LogLevel::Debug => "Debug log event",
        LogLevel::Unknown => "AMA log event",
    }
    .to_string();
    let mut severity = match log_level {
        LogLevel::Error => Severity::High,
        LogLevel::Warning => Severity::Medium,
        LogLevel::Info => Severity::Info,
        LogLevel::Debug | LogLevel::Unknown => Severity::Low,
    };
    let mut status = match severity {
        Severity::Critical | Severity::High => Status::Fail,
        Severity::Medium | Severity::Low => Status::Warn,
        Severity::Info => Status::Unknown,
    };
    let mut tags = Vec::new();
    let mut doc_link = None;

    if common::Patterns::imds_error().is_match(line)
        || common::Patterns::connectivity_error().is_match(line)
    {
        category = Category::Connectivity;
        title = "Connectivity failures to ingestion or control endpoint".to_string();
        severity = Severity::High;
        status = Status::Fail;
        tags.push("connectivity".to_string());
        doc_link = Some("https://learn.microsoft.com/en-us/azure/azure-monitor/agents/azure-monitor-agent-network-configuration".to_string());
    } else if common::Patterns::auth_token_error().is_match(line) {
        category = Category::IdentityAuth;
        title = "Managed identity or token acquisition issue".to_string();
        severity = Severity::Critical;
        status = Status::Fail;
        tags.push("identity".to_string());
        doc_link = Some("https://learn.microsoft.com/en-us/azure/active-directory/managed-identities-azure-resources/overview".to_string());
    } else if common::Patterns::service_crash().is_match(line)
        || common::Patterns::systemd_service_failure().is_match(line)
    {
        category = Category::Service;
        title = "AMA service crash or unexpected termination".to_string();
        severity = Severity::Critical;
        status = Status::Fail;
        tags.push("service".to_string());
    } else if common::Patterns::extension_error().is_match(line) {
        category = Category::Install;
        title = "Extension provisioning or installation failure".to_string();
        severity = Severity::High;
        status = Status::Fail;
        tags.push("installation".to_string());
    } else if common::Patterns::dcr_error().is_match(line) {
        category = Category::Dcr;
        title = "DCR configuration issue".to_string();
        severity = Severity::Medium;
        status = Status::Warn;
        tags.push("dcr".to_string());
    } else if common::Patterns::syslog_error().is_match(line) {
        category = Category::Syslog;
        title = "Syslog forwarding problem".to_string();
        severity = Severity::Medium;
        status = Status::Warn;
        tags.push("syslog".to_string());
    } else if common::Patterns::fluentbit_error().is_match(line) {
        category = Category::FluentBit;
        title = "FluentBit processing or forwarding error".to_string();
        severity = Severity::Medium;
        status = Status::Warn;
        tags.push("fluentbit".to_string());
    } else if common::Patterns::mdsd_qos_failure().is_match(line) {
        category = Category::Mdsd;
        title = "MDSD upload health issue".to_string();
        severity = Severity::Medium;
        status = Status::Warn;
        tags.push("mdsd".to_string());
    } else if common::Patterns::arc_agent_error().is_match(line) {
        category = Category::ArcExtension;
        title = "Azure Arc extension or HIMDS issue".to_string();
        severity = Severity::Medium;
        status = Status::Warn;
        tags.push("arc".to_string());
    } else if common::Patterns::oom_killer().is_match(line)
        || common::Patterns::cgroup_oom().is_match(line)
        || common::Patterns::disk_full().is_match(line)
    {
        category = Category::CpuMemory;
        title = "Resource pressure impacting AMA".to_string();
        severity = Severity::Critical;
        status = Status::Fail;
        tags.push("resource-pressure".to_string());
    } else if !matches!(log_level, LogLevel::Error | LogLevel::Warning) {
        return None;
    }

    Some(DiagnosticEvent {
        ts: timestamp,
        os,
        category,
        severity,
        status,
        title,
        summary: summarize(line),
        details: Some(line.to_string()),
        evidence: vec![EvidenceRef {
            file_id: file_id.to_string(),
            line_range: line_number..line_number + 1,
            preview: summarize(line),
        }],
        tags,
        rule_id: None,
        remediation: None,
        doc_link,
    })
}

fn summarize(line: &str) -> String {
    const MAX_LEN: usize = 160;
    if line.len() <= MAX_LEN {
        line.to_string()
    } else {
        format!("{}...", &line[..MAX_LEN.saturating_sub(3)])
    }
}

fn is_candidate_log(relative_path: &str) -> bool {
    let lower = relative_path.to_lowercase();
    lower.ends_with(".log")
        || lower.ends_with(".txt")
        || lower.ends_with(".err")
        || lower.ends_with(".warn")
        || lower.ends_with(".info")
        || lower.ends_with(".out")
        || lower.contains("mdsd")
        || lower.contains("fluentbit")
        || lower.contains("waagent")
        || lower.contains("extension")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_dir() -> PathBuf {
        let mut path = std::env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        path.push(format!("amadiag-event-store-{unique}"));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn warning_and_error_lines_are_indexed() {
        let event = classify_line(
            "ama.log",
            4,
            "2026-03-26T10:00:00Z ERROR ingestion endpoint unreachable",
            OsKind::Linux,
        )
        .unwrap();

        assert_eq!(event.category, Category::Connectivity);
        assert_eq!(event.status, Status::Fail);
        assert!(event.ts.is_some());
    }

    #[test]
    fn stale_timestamped_logs_are_skipped_by_default() {
        let dir = create_temp_dir();
        let old_ts = (Utc::now() - Duration::days(DEFAULT_STALE_LOG_AGE_DAYS + 30))
            .format("%Y-%m-%dT%H:%M:%SZ");
        let recent_ts = (Utc::now() - Duration::days(1)).format("%Y-%m-%dT%H:%M:%SZ");

        std::fs::write(
            dir.join("old.log"),
            format!("{old_ts} ERROR old endpoint unreachable\n"),
        )
        .unwrap();
        std::fs::write(
            dir.join("recent.log"),
            format!("{recent_ts} ERROR recent endpoint unreachable\n"),
        )
        .unwrap();

        let store = EventStore::from_bundle_dir(&dir, OsKind::Linux).unwrap();
        let summary = store.date_summary();

        assert_eq!(summary.candidate_log_files(), 2);
        assert_eq!(summary.timestamped_log_files(), 2);
        assert_eq!(summary.stale_log_files_skipped(), 1);
        assert_eq!(store.events().len(), 1);
        assert_eq!(
            store.date_summary().analyzed_range().start(),
            store.date_summary().analyzed_range().end()
        );
        assert!(
            summary
                .discovered_range()
                .start()
                .zip(summary.analyzed_range().start())
                .is_some_and(|(discovered, analyzed)| discovered < analyzed)
        );

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn untimestamped_logs_remain_available() {
        let dir = create_temp_dir();
        std::fs::write(dir.join("untimestamped.log"), "ERROR ingestion endpoint unreachable\n")
            .unwrap();

        let store = EventStore::from_bundle_dir(&dir, OsKind::Linux).unwrap();

        assert_eq!(store.events().len(), 1);
        assert_eq!(store.date_summary().stale_log_files_skipped(), 0);
        assert!(!store.date_summary().discovered_range().is_known());

        let _ = std::fs::remove_dir_all(dir);
    }
}
