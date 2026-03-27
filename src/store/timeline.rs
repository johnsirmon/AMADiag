use crate::model::diagnostic::{DiagnosticEvent, Severity};
use chrono::{DateTime, Duration, Timelike, Utc};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TimelineBucket {
    pub start: DateTime<Utc>,
    pub warning_or_higher: usize,
    pub critical: usize,
}

pub fn build_timeline(events: &[DiagnosticEvent], bucket_minutes: i64) -> Vec<TimelineBucket> {
    build_timeline_from_iter(events.iter(), bucket_minutes)
}

pub fn build_timeline_for_indices(
    events: &[DiagnosticEvent],
    indices: &[usize],
    bucket_minutes: i64,
) -> Vec<TimelineBucket> {
    build_timeline_from_iter(
        indices.iter().filter_map(|index| events.get(*index)),
        bucket_minutes,
    )
}

fn build_timeline_from_iter<'a>(
    events: impl IntoIterator<Item = &'a DiagnosticEvent>,
    bucket_minutes: i64,
) -> Vec<TimelineBucket> {
    if bucket_minutes <= 0 {
        return Vec::new();
    }

    let mut buckets = BTreeMap::new();
    for event in events {
        let Some(ts) = event.ts else {
            continue;
        };

        let bucket = buckets
            .entry(bucket_start(ts, bucket_minutes))
            .or_insert(TimelineBucket {
                start: bucket_start(ts, bucket_minutes),
                warning_or_higher: 0,
                critical: 0,
            });
        if event.severity >= Severity::Medium {
            bucket.warning_or_higher += 1;
        }
        if event.severity >= Severity::Critical {
            bucket.critical += 1;
        }
    }

    buckets.into_values().collect()
}

fn bucket_start(ts: DateTime<Utc>, bucket_minutes: i64) -> DateTime<Utc> {
    let minute = i64::from(ts.minute());
    let offset = minute % bucket_minutes;
    ts - Duration::minutes(offset)
        - Duration::seconds(i64::from(ts.second()))
        - Duration::nanoseconds(i64::from(ts.nanosecond()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::diagnostic::{Category, DiagnosticEvent, OsKind, Status};

    fn event(ts: &str, severity: Severity) -> DiagnosticEvent {
        DiagnosticEvent {
            ts: Some(ts.parse().unwrap()),
            os: OsKind::Linux,
            category: Category::Connectivity,
            severity,
            status: Status::Fail,
            title: "Timeline event".to_string(),
            summary: "Timeline event".to_string(),
            details: None,
            evidence: Vec::new(),
            tags: Vec::new(),
            rule_id: None,
            remediation: None,
            doc_link: None,
        }
    }

    #[test]
    fn indexed_timeline_matches_slice_timeline() {
        let events = vec![
            event("2026-03-26T10:01:05Z", Severity::Medium),
            event("2026-03-26T10:03:10Z", Severity::Critical),
            event("2026-03-26T10:09:00Z", Severity::Info),
        ];

        let all = build_timeline(&events, 5);
        let indexed = build_timeline_for_indices(&events, &[0, 1, 2], 5);

        assert_eq!(all.len(), indexed.len());
        assert_eq!(all[0].warning_or_higher, indexed[0].warning_or_higher);
        assert_eq!(all[0].critical, indexed[0].critical);
        assert_eq!(all[1].warning_or_higher, indexed[1].warning_or_higher);
        assert_eq!(all[1].critical, indexed[1].critical);
    }

    #[test]
    fn timeline_groups_into_sorted_buckets() {
        let events = vec![
            event("2026-03-26T10:09:00Z", Severity::Info),
            event("2026-03-26T10:01:05Z", Severity::Medium),
            event("2026-03-26T10:03:10Z", Severity::Critical),
        ];

        let buckets = build_timeline(&events, 5);

        assert_eq!(buckets.len(), 2);
        assert!(buckets[0].start < buckets[1].start);
        assert_eq!(buckets[0].warning_or_higher, 2);
        assert_eq!(buckets[0].critical, 1);
        assert_eq!(buckets[1].warning_or_higher, 0);
        assert_eq!(buckets[1].critical, 0);
    }
}
