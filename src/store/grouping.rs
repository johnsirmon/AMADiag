use crate::model::diagnostic::{Category, DiagnosticEvent, EvidenceRef, FindingGroup, Status};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Advice {
    pub likely_causes: Vec<String>,
    pub suggested_actions: Vec<String>,
}

pub fn group_events(events: &[DiagnosticEvent], indices: &[usize]) -> Vec<FindingGroup> {
    let mut grouped: BTreeMap<(Category, String), FindingGroup> = BTreeMap::new();

    for &index in indices {
        let Some(event) = events.get(index) else {
            continue;
        };

        let key = (event.category, event.title.clone());
        let advice = advice_for_event(event);
        let group = grouped.entry(key.clone()).or_insert_with(|| FindingGroup {
            key: format!("{:?}-{}", key.0, sanitize_key(&key.1)),
            category: event.category,
            severity: event.severity,
            status: event.status,
            title: event.title.clone(),
            summary: event.summary.clone(),
            first_seen: event.ts,
            last_seen: event.ts,
            events: Vec::new(),
            likely_causes: advice.likely_causes.clone(),
            suggested_actions: advice.suggested_actions.clone(),
            evidence: Vec::new(),
            doc_links: Vec::new(),
            rule_ids: Vec::new(),
        });

        group.events.push(index);
        group.severity = group.severity.max(event.severity);
        group.status = max_status(group.status, event.status);
        group.first_seen = min_optional(group.first_seen, event.ts);
        group.last_seen = max_optional(group.last_seen, event.ts);

        for evidence in &event.evidence {
            if !group.evidence.iter().any(|item| item == evidence) && group.evidence.len() < 8 {
                group.evidence.push(EvidenceRef {
                    file_id: evidence.file_id.clone(),
                    line_range: evidence.line_range.clone(),
                    preview: evidence.preview.clone(),
                });
            }
        }

        if let Some(rule_id) = &event.rule_id {
            if !group.rule_ids.contains(rule_id) {
                group.rule_ids.push(rule_id.clone());
            }
        }

        if let Some(doc_link) = &event.doc_link {
            if !group.doc_links.contains(doc_link) {
                group.doc_links.push(doc_link.clone());
            }
        }
    }

    let mut groups: Vec<_> = grouped.into_values().collect();
    groups.sort_by(|left, right| {
        right
            .severity
            .cmp(&left.severity)
            .then_with(|| right.last_seen.cmp(&left.last_seen))
            .then_with(|| left.title.cmp(&right.title))
    });
    groups
}

fn sanitize_key(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn max_status(left: Status, right: Status) -> Status {
    match (left, right) {
        (Status::Fail, _) | (_, Status::Fail) => Status::Fail,
        (Status::Warn, _) | (_, Status::Warn) => Status::Warn,
        (Status::Pass, Status::Pass) => Status::Pass,
        _ => Status::Unknown,
    }
}

fn min_optional(
    left: Option<chrono::DateTime<chrono::Utc>>,
    right: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

fn max_optional(
    left: Option<chrono::DateTime<chrono::Utc>>,
    right: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

pub fn advice_for_event(event: &DiagnosticEvent) -> Advice {
    match event.category {
        Category::Connectivity => Advice {
            likely_causes: vec![
                "AMA could not reach a required endpoint from the host.".to_string(),
                "Proxy, DNS, firewall, or TLS inspection may be interfering.".to_string(),
            ],
            suggested_actions: vec![
                "Validate endpoint reachability from the machine.".to_string(),
                "Check proxy and firewall configuration for AMA endpoints.".to_string(),
            ],
        },
        Category::IdentityAuth => Advice {
            likely_causes: vec![
                "Managed identity token acquisition is failing.".to_string(),
                "IMDS access may be unavailable or blocked.".to_string(),
            ],
            suggested_actions: vec![
                "Verify managed identity assignment and IMDS connectivity.".to_string()
            ],
        },
        Category::Service | Category::Install => Advice {
            likely_causes: vec!["AMA service or extension installation is unhealthy.".to_string()],
            suggested_actions: vec![
                "Review extension/service logs and restart or reinstall if necessary.".to_string(),
            ],
        },
        Category::CpuMemory => Advice {
            likely_causes: vec![
                "The host is experiencing resource pressure that impacts AMA.".to_string(),
            ],
            suggested_actions: vec!["Inspect disk, CPU, and memory usage on the host.".to_string()],
        },
        _ => Advice {
            likely_causes: vec!["Repeated AMA-related log signals were observed.".to_string()],
            suggested_actions: vec![
                "Inspect the supporting evidence lines for context.".to_string()
            ],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::diagnostic::{DiagnosticEvent, EvidenceRef, OsKind, Severity};

    #[test]
    fn duplicate_events_are_grouped() {
        let event = DiagnosticEvent {
            ts: None,
            os: OsKind::Linux,
            category: Category::Connectivity,
            severity: Severity::High,
            status: Status::Fail,
            title: "Connectivity failure".to_string(),
            summary: "Endpoint issue".to_string(),
            details: None,
            evidence: vec![EvidenceRef {
                file_id: "a.log".to_string(),
                line_range: 1..2,
                preview: "failed".to_string(),
            }],
            tags: Vec::new(),
            rule_id: None,
            remediation: None,
            doc_link: None,
        };

        let events = vec![event.clone(), event];
        let groups = group_events(&events, &[0, 1]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].event_count(), 2);
    }
}
