use crate::model::diagnostic::{DiagnosticEvent, Severity};
use chrono::{DateTime, Duration, Timelike, Utc};

#[derive(Debug, Clone)]
pub struct TimelineBucket {
    pub start: DateTime<Utc>,
    pub warning_or_higher: usize,
    pub critical: usize,
}

pub fn build_timeline(events: &[DiagnosticEvent], bucket_minutes: i64) -> Vec<TimelineBucket> {
    let mut buckets = Vec::new();
    if bucket_minutes <= 0 {
        return buckets;
    }

    for event in events.iter().filter(|event| event.ts.is_some()) {
        let ts = event.ts.expect("checked is_some");
        let minute = i64::from(ts.minute());
        let offset = minute % bucket_minutes;
        let bucket_start = ts
            - Duration::minutes(offset)
            - Duration::seconds(i64::from(ts.second()))
            - Duration::nanoseconds(i64::from(ts.nanosecond()));

        if let Some(bucket) = buckets.iter_mut().find(|bucket| bucket.start == bucket_start) {
            if event.severity >= Severity::Medium {
                bucket.warning_or_higher += 1;
            }
            if event.severity >= Severity::Critical {
                bucket.critical += 1;
            }
            continue;
        }

        buckets.push(TimelineBucket {
            start: bucket_start,
            warning_or_higher: usize::from(event.severity >= Severity::Medium),
            critical: usize::from(event.severity >= Severity::Critical),
        });
    }

    buckets.sort_by_key(|bucket| bucket.start);
    buckets
}

