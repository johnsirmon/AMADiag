use crate::model::diagnostic::{Category, Severity};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFilter {
    LastMinutes(u32),
    LastHours(u32),
    Custom {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    All,
}

impl TimeFilter {
    pub fn label(&self) -> String {
        match self {
            Self::LastMinutes(value) => format!("Last {value}m"),
            Self::LastHours(value) => format!("Last {value}h"),
            Self::Custom { start, end } => {
                format!("{} - {}", start.format("%Y-%m-%d %H:%M"), end.format("%H:%M"))
            }
            Self::All => "All".to_string(),
        }
    }

    pub fn contains(&self, ts: Option<DateTime<Utc>>, latest: Option<DateTime<Utc>>) -> bool {
        match self {
            Self::All => true,
            Self::Custom { start, end } => ts.is_some_and(|value| value >= *start && value <= *end),
            Self::LastMinutes(minutes) => {
                if let (Some(value), Some(anchor)) = (ts, latest) {
                    value >= anchor - Duration::minutes(i64::from(*minutes))
                } else {
                    false
                }
            }
            Self::LastHours(hours) => {
                if let (Some(value), Some(anchor)) = (ts, latest) {
                    value >= anchor - Duration::hours(i64::from(*hours))
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterState {
    pub time_filter: TimeFilter,
    pub min_severity: Severity,
    pub categories: BTreeSet<Category>,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            time_filter: TimeFilter::LastHours(24),
            min_severity: Severity::Medium,
            categories: BTreeSet::new(),
        }
    }
}

impl FilterState {
    pub fn allows_category(&self, category: Category) -> bool {
        self.categories.is_empty() || self.categories.contains(&category)
    }

    pub fn cycle_time_filter(&mut self) {
        self.time_filter = match self.time_filter {
            TimeFilter::LastMinutes(15) => TimeFilter::LastHours(1),
            TimeFilter::LastHours(1) => TimeFilter::LastHours(24),
            TimeFilter::LastHours(24) => TimeFilter::All,
            _ => TimeFilter::LastMinutes(15),
        };
    }
}
