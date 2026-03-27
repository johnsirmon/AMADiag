use crate::analyzers::finding::{Category, Finding, Severity};
use crate::parsers::ParsedBundle;
use comfy_table::{Cell, Table};
use serde::Deserialize;

/// A YAML-defined detection rule.
#[derive(Debug, Clone, Deserialize)]
pub struct RuleDefinition {
    pub id: String,
    pub name: String,
    pub severity: RuleSeverity,
    pub category: RuleCategory,
    pub platforms: Vec<String>,
    pub description: String,
    pub detection: Detection,
    pub remediation: String,
    pub doc_link: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    Critical,
    Warning,
    Info,
}

impl From<RuleSeverity> for Severity {
    fn from(s: RuleSeverity) -> Self {
        match s {
            RuleSeverity::Critical => Severity::Critical,
            RuleSeverity::Warning => Severity::Warning,
            RuleSeverity::Info => Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleCategory {
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

impl From<RuleCategory> for Category {
    fn from(c: RuleCategory) -> Self {
        match c {
            RuleCategory::Installation => Category::Installation,
            RuleCategory::AgentNotRunning => Category::AgentNotRunning,
            RuleCategory::Identity => Category::Identity,
            RuleCategory::Connectivity => Category::Connectivity,
            RuleCategory::Dcr => Category::Dcr,
            RuleCategory::PerformanceCounters => Category::PerformanceCounters,
            RuleCategory::EventLog => Category::EventLog,
            RuleCategory::Syslog => Category::Syslog,
            RuleCategory::MetricsExtension => Category::MetricsExtension,
            RuleCategory::Heartbeat => Category::Heartbeat,
            RuleCategory::ArcAgent => Category::ArcAgent,
            RuleCategory::VersionMismatch => Category::VersionMismatch,
            RuleCategory::Sizing => Category::Sizing,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Detection {
    /// Check if a file matching this pattern exists
    pub file_pattern: Option<String>,
    /// Condition: "file_missing", "file_present", "content_match", "xml_element_missing"
    pub condition: String,
    /// Regex to match against file content (for content_match)
    pub content_regex: Option<String>,
    /// XML element name to check (for xml_element_missing)
    pub xml_element: Option<String>,
}

// Embed YAML rules at compile time
const INSTALLATION_RULES: &str = include_str!("../rules/installation.yaml");
const CONNECTIVITY_RULES: &str = include_str!("../rules/connectivity.yaml");
const DCR_RULES: &str = include_str!("../rules/dcr.yaml");
const IDENTITY_RULES: &str = include_str!("../rules/identity.yaml");
const PERFORMANCE_RULES: &str = include_str!("../rules/performance.yaml");
const SYSLOG_RULES: &str = include_str!("../rules/syslog.yaml");
const SIZING_RULES: &str = include_str!("../rules/sizing.yaml");
const LINUX_COLLECTION_RULES: &str = include_str!("../rules/linux_collection.yaml");

/// Load all built-in detection rules from embedded YAML.
pub fn load_builtin_rules() -> anyhow::Result<Vec<RuleDefinition>> {
    let mut all_rules = Vec::new();

    let yaml_sources = [
        INSTALLATION_RULES,
        CONNECTIVITY_RULES,
        DCR_RULES,
        IDENTITY_RULES,
        PERFORMANCE_RULES,
        SYSLOG_RULES,
        SIZING_RULES,
        LINUX_COLLECTION_RULES,
    ];

    for yaml in &yaml_sources {
        let rules: Vec<RuleDefinition> = serde_yaml::from_str(yaml)?;
        all_rules.extend(rules);
    }

    Ok(all_rules)
}

/// Evaluate YAML-defined rules against a parsed bundle.
pub fn evaluate_rules(rules: &[RuleDefinition], bundle: &ParsedBundle) -> Vec<Finding> {
    let mut findings = Vec::new();

    for rule in rules {
        // Check platform applicability
        if let Some(platform) = &bundle.platform {
            let platform_str = platform.as_str();
            if !rule
                .platforms
                .iter()
                .any(|p| p.eq_ignore_ascii_case(platform_str) || p == "all")
            {
                continue;
            }
        }

        if let Some(finding) = evaluate_single_rule(rule, bundle) {
            findings.push(finding);
        }
    }

    // Also run pattern-based log scanning
    findings.extend(crate::analyzers::scan_log_patterns(bundle));

    findings
}

fn evaluate_single_rule(rule: &RuleDefinition, bundle: &ParsedBundle) -> Option<Finding> {
    let matched = match rule.detection.condition.as_str() {
        "file_missing" => {
            if let Some(pattern) = &rule.detection.file_pattern {
                let pat_lower = pattern.to_lowercase();
                let file_missing = !bundle
                    .files
                    .keys()
                    .any(|k| k.to_lowercase().contains(&pat_lower));

                // For identity checks: if the auth token file is missing but
                // IMDS metadata response is present, the VM has working identity
                // via IMDS. Don't flag as missing identity.
                if file_missing && pat_lower.contains("authtoken") {
                    let has_imds = bundle
                        .files
                        .keys()
                        .any(|k| k.to_lowercase().contains("imdsmetadata"));
                    if has_imds {
                        return None;
                    }
                }

                file_missing
            } else {
                false
            }
        }
        "file_present" => {
            if let Some(pattern) = &rule.detection.file_pattern {
                let pat_lower = pattern.to_lowercase();
                bundle
                    .files
                    .keys()
                    .any(|k| k.to_lowercase().contains(&pat_lower))
            } else {
                false
            }
        }
        "content_match" => {
            if let (Some(file_pat), Some(regex_str)) =
                (&rule.detection.file_pattern, &rule.detection.content_regex)
            {
                let file_pat_lower = file_pat.to_lowercase();
                let re = match regex::Regex::new(regex_str) {
                    Ok(r) => r,
                    Err(_) => return None,
                };
                bundle
                    .files
                    .iter()
                    .any(|(k, v)| k.to_lowercase().contains(&file_pat_lower) && re.is_match(v))
            } else {
                false
            }
        }
        "xml_element_missing" => {
            if let Some(element) = &rule.detection.xml_element {
                let elem_lower = element.to_lowercase();
                // If we have XML configs, check if the element is missing
                if bundle.xml_configs.is_empty() {
                    false // Can't evaluate without XML
                } else {
                    match elem_lower.as_str() {
                        "counterset" => {
                            // If there are JSON-based DCR configs (modern AMA),
                            // CounterSets in XML may legitimately be absent.
                            let has_json_config = bundle.files.keys().any(|k| {
                                let lk = k.to_lowercase();
                                lk.contains("mcsconfig") && lk.ends_with(".json")
                                    || lk.contains("configchunks")
                            });
                            if has_json_config {
                                false
                            } else {
                                bundle.xml_configs.iter().all(|c| c.counter_sets.is_empty())
                            }
                        }
                        "subscription" => bundle
                            .xml_configs
                            .iter()
                            .all(|c| c.subscriptions.is_empty()),
                        _ => false,
                    }
                }
            } else {
                false
            }
        }
        _ => false,
    };

    if matched {
        let mut evidence = Vec::new();

        // Collect evidence based on detection type
        if let Some(file_pat) = &rule.detection.file_pattern {
            let pat_lower = file_pat.to_lowercase();
            for key in bundle.files.keys() {
                if key.to_lowercase().contains(&pat_lower) {
                    evidence.push(format!("File: {key}"));
                }
            }
        }

        if evidence.is_empty() {
            evidence.push(format!("Detected by rule: {}", rule.id));
        }

        Some(Finding {
            rule_id: rule.id.clone(),
            name: rule.name.clone(),
            severity: rule.severity.clone().into(),
            category: rule.category.clone().into(),
            description: rule.description.clone(),
            evidence,
            remediation: rule.remediation.clone(),
            doc_link: rule.doc_link.clone(),
        })
    } else {
        None
    }
}

/// Print rules as a formatted table to stdout.
pub fn print_rules_table(rules: &[RuleDefinition]) {
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID"),
        Cell::new("Name"),
        Cell::new("Severity"),
        Cell::new("Category"),
        Cell::new("Platforms"),
    ]);

    for rule in rules {
        table.add_row(vec![
            Cell::new(&rule.id),
            Cell::new(&rule.name),
            Cell::new(format!("{:?}", rule.severity)),
            Cell::new(format!("{:?}", rule.category)),
            Cell::new(rule.platforms.join(", ")),
        ]);
    }

    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::ParsedBundle;
    use std::collections::HashMap;

    #[test]
    fn load_all_builtin_rules() {
        let rules = load_builtin_rules().unwrap();
        assert!(!rules.is_empty(), "Should load at least one rule");
        // Verify each rule has required fields
        for rule in &rules {
            assert!(!rule.id.is_empty(), "Rule ID must not be empty");
            assert!(!rule.name.is_empty(), "Rule name must not be empty");
            assert!(
                !rule.platforms.is_empty(),
                "Rule {} must have platforms",
                rule.id
            );
            assert!(
                !rule.description.is_empty(),
                "Rule {} must have description",
                rule.id
            );
            assert!(
                !rule.remediation.is_empty(),
                "Rule {} must have remediation",
                rule.id
            );
        }
    }

    #[test]
    fn rule_ids_are_unique() {
        let rules = load_builtin_rules().unwrap();
        let mut ids: Vec<&str> = rules.iter().map(|r| r.id.as_str()).collect();
        ids.sort();
        let before_dedup = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), before_dedup, "Rule IDs must be unique");
    }

    #[test]
    fn content_match_rule_fires_on_matching_content() {
        let rule = RuleDefinition {
            id: "TEST-001".to_string(),
            name: "Test Rule".to_string(),
            severity: RuleSeverity::Warning,
            category: RuleCategory::Connectivity,
            platforms: vec!["windows".to_string()],
            description: "Test".to_string(),
            detection: Detection {
                file_pattern: Some(".log".to_string()),
                condition: "content_match".to_string(),
                content_regex: Some(r"(?i)AMCS.*refused".to_string()),
                xml_element: None,
            },
            remediation: "Fix".to_string(),
            doc_link: None,
        };

        let mut files = HashMap::new();
        files.insert(
            "agent.log".to_string(),
            "AMCS endpoint connection refused".to_string(),
        );
        let bundle = ParsedBundle {
            platform: Some(crate::analyzers::finding::Platform::Windows),
            files,
            ..Default::default()
        };

        let finding = evaluate_single_rule(&rule, &bundle);
        assert!(finding.is_some(), "Rule should fire on matching content");
        assert_eq!(finding.unwrap().rule_id, "TEST-001");
    }

    #[test]
    fn content_match_rule_does_not_fire_on_clean_content() {
        let rule = RuleDefinition {
            id: "TEST-002".to_string(),
            name: "Test Rule".to_string(),
            severity: RuleSeverity::Critical,
            category: RuleCategory::Connectivity,
            platforms: vec!["windows".to_string()],
            description: "Test".to_string(),
            detection: Detection {
                file_pattern: Some(".log".to_string()),
                condition: "content_match".to_string(),
                content_regex: Some(r"(?i)AMCS.*refused".to_string()),
                xml_element: None,
            },
            remediation: "Fix".to_string(),
            doc_link: None,
        };

        let mut files = HashMap::new();
        files.insert(
            "agent.log".to_string(),
            "AMCS endpoint connected successfully".to_string(),
        );
        let bundle = ParsedBundle {
            platform: Some(crate::analyzers::finding::Platform::Windows),
            files,
            ..Default::default()
        };

        assert!(
            evaluate_single_rule(&rule, &bundle).is_none(),
            "Rule should not fire on clean content"
        );
    }

    #[test]
    fn file_missing_rule_fires_when_absent() {
        let rule = RuleDefinition {
            id: "TEST-003".to_string(),
            name: "Missing Config".to_string(),
            severity: RuleSeverity::Critical,
            category: RuleCategory::Dcr,
            platforms: vec!["windows".to_string()],
            description: "Test".to_string(),
            detection: Detection {
                file_pattern: Some("mcsconfig".to_string()),
                condition: "file_missing".to_string(),
                content_regex: None,
                xml_element: None,
            },
            remediation: "Fix".to_string(),
            doc_link: None,
        };

        let bundle = ParsedBundle {
            platform: Some(crate::analyzers::finding::Platform::Windows),
            files: HashMap::new(), // no files at all
            ..Default::default()
        };

        assert!(
            evaluate_single_rule(&rule, &bundle).is_some(),
            "file_missing rule should fire when file is absent"
        );
    }

    #[test]
    fn file_missing_rule_silent_when_present() {
        let rule = RuleDefinition {
            id: "TEST-004".to_string(),
            name: "Missing Config".to_string(),
            severity: RuleSeverity::Critical,
            category: RuleCategory::Dcr,
            platforms: vec!["windows".to_string()],
            description: "Test".to_string(),
            detection: Detection {
                file_pattern: Some("mcsconfig".to_string()),
                condition: "file_missing".to_string(),
                content_regex: None,
                xml_element: None,
            },
            remediation: "Fix".to_string(),
            doc_link: None,
        };

        let mut files = HashMap::new();
        files.insert("mcsconfig.lkg.xml".to_string(), "<xml/>".to_string());
        let bundle = ParsedBundle {
            platform: Some(crate::analyzers::finding::Platform::Windows),
            files,
            ..Default::default()
        };

        assert!(
            evaluate_single_rule(&rule, &bundle).is_none(),
            "file_missing rule should not fire when file is present"
        );
    }

    #[test]
    fn platform_filter_skips_non_matching_rules() {
        let rules = vec![RuleDefinition {
            id: "LINUX-001".to_string(),
            name: "Linux Only".to_string(),
            severity: RuleSeverity::Warning,
            category: RuleCategory::Syslog,
            platforms: vec!["linux".to_string()],
            description: "Test".to_string(),
            detection: Detection {
                file_pattern: Some(".log".to_string()),
                condition: "content_match".to_string(),
                content_regex: Some(r"always-match".to_string()),
                xml_element: None,
            },
            remediation: "Fix".to_string(),
            doc_link: None,
        }];

        let mut files = HashMap::new();
        files.insert("test.log".to_string(), "always-match".to_string());
        let bundle = ParsedBundle {
            platform: Some(crate::analyzers::finding::Platform::Windows),
            files,
            ..Default::default()
        };

        let findings = evaluate_rules(&rules, &bundle);
        // The rule targets linux only, bundle is windows — rule should be skipped
        // (but log pattern scanning may still produce findings)
        assert!(
            !findings.iter().any(|f| f.rule_id == "LINUX-001"),
            "Linux rule should not fire on Windows bundle"
        );
    }
}
