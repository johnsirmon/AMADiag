use crate::parsers::fluentbit_config;
use crate::parsers::mdsd_qos;
use crate::parsers::{LinuxData, ParsedBundle};
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

/// Parsed /etc/os-release fields.
#[derive(Debug, Clone, Default)]
pub struct OsRelease {
    pub id: Option<String>,
    pub version_id: Option<String>,
    pub name: Option<String>,
    pub pretty_name: Option<String>,
}

/// A parsed rsyslog forwarding rule for AMA.
#[derive(Debug, Clone)]
pub struct RsyslogForwardRule {
    pub facility: Option<String>,
    pub target_port: Option<u16>,
    pub target_host: Option<String>,
    pub raw_line: String,
}

/// Linux-specific parsing and enrichment.
///
/// Populates `bundle.linux_data` by scanning bundle files for:
/// - mdsd.qos upload metrics
/// - Fluentbit td-agent.conf configuration
/// - rsyslog forwarding rules
/// - /etc/os-release distro info
/// - proxy.conf settings
pub fn enrich_linux_data(bundle: &mut ParsedBundle, _bundle_dir: &Path) {
    let mut linux = LinuxData::default();

    for (name, content) in &bundle.files {
        let lower = name.to_lowercase();

        // Parse mdsd.qos files
        if lower.contains("mdsd.qos") || lower.contains("mdsd_qos") {
            let entries = mdsd_qos::parse_mdsd_qos(content);
            if !entries.is_empty() {
                tracing::debug!("Parsed {} mdsd.qos entries from {}", entries.len(), name);
                linux.mdsd_qos_entries.extend(entries);
            }
        }

        // Parse Fluentbit td-agent.conf
        if lower.contains("td-agent.conf") || lower.contains("td_agent.conf") {
            let config = fluentbit_config::parse_fluentbit_config(content);
            tracing::debug!(
                "Parsed Fluentbit config from {}: {} inputs, {} outputs",
                name,
                config.inputs.len(),
                config.outputs.len()
            );
            linux.fluentbit_config = Some(config);
        }

        // Parse rsyslog config files
        if lower.contains("rsyslog") && !lower.ends_with(".log") {
            let rules = parse_rsyslog_rules(content);
            if !rules.is_empty() {
                tracing::debug!("Parsed {} rsyslog rules from {}", rules.len(), name);
                linux.rsyslog_rules.extend(rules);
            }
        }

        // Parse /etc/os-release
        if lower.contains("os-release") || lower.contains("os_release") {
            if let Some(release) = parse_os_release(content) {
                tracing::debug!(
                    "Detected OS: {} {}",
                    release.id.as_deref().unwrap_or("unknown"),
                    release.version_id.as_deref().unwrap_or("")
                );
                linux.os_release = Some(release);
            }
        }

        // Capture proxy.conf content
        if lower.contains("proxy.conf") || lower.contains("proxy_conf") {
            linux.proxy_config = Some(content.clone());
        }
    }

    // Log summary
    let has_mdsd = bundle
        .files
        .keys()
        .any(|k| k.to_lowercase().contains("mdsd"));
    if !has_mdsd {
        tracing::debug!("No mdsd log files found in Linux bundle");
    }

    bundle.linux_data = Some(linux);
}

// Rsyslog forwarding rules look like:
//   *.* @@127.0.0.1:28330
//   local4.* @@127.0.0.1:28330
//   if $rawmsg contains "CEF:" then @@127.0.0.1:28330
static RSYSLOG_FWD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(\S+(?:\.\S+)?)\s+@@?(\S+):(\d+)").unwrap()
});

/// Parse rsyslog config content for forwarding rules.
pub fn parse_rsyslog_rules(content: &str) -> Vec<RsyslogForwardRule> {
    RSYSLOG_FWD_RE
        .captures_iter(content)
        .map(|caps| {
            let facility = caps.get(1).map(|m| m.as_str().to_string());
            let host = caps.get(2).map(|m| m.as_str().to_string());
            let port = caps.get(3).and_then(|m| m.as_str().parse().ok());

            RsyslogForwardRule {
                facility,
                target_host: host,
                target_port: port,
                raw_line: caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default(),
            }
        })
        .collect()
}

/// Parse /etc/os-release content into structured fields.
pub fn parse_os_release(content: &str) -> Option<OsRelease> {
    let mut release = OsRelease::default();
    let mut found_any = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some((key, value)) = trimmed.split_once('=') {
            let val = value.trim_matches('"').trim().to_string();
            match key.trim() {
                "ID" => {
                    release.id = Some(val);
                    found_any = true;
                }
                "VERSION_ID" => {
                    release.version_id = Some(val);
                    found_any = true;
                }
                "NAME" => {
                    release.name = Some(val);
                    found_any = true;
                }
                "PRETTY_NAME" => {
                    release.pretty_name = Some(val);
                    found_any = true;
                }
                _ => {}
            }
        }
    }

    if found_any {
        Some(release)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_os_release_ubuntu() {
        let content = r#"NAME="Ubuntu"
VERSION_ID="22.04"
ID=ubuntu
PRETTY_NAME="Ubuntu 22.04.3 LTS"
HOME_URL="https://www.ubuntu.com/"
"#;
        let release = parse_os_release(content).unwrap();
        assert_eq!(release.id.as_deref(), Some("ubuntu"));
        assert_eq!(release.version_id.as_deref(), Some("22.04"));
        assert_eq!(release.name.as_deref(), Some("Ubuntu"));
        assert_eq!(
            release.pretty_name.as_deref(),
            Some("Ubuntu 22.04.3 LTS")
        );
    }

    #[test]
    fn parse_os_release_sles() {
        let content = r#"NAME="SLES"
VERSION_ID="15.4"
ID="sles"
PRETTY_NAME="SUSE Linux Enterprise Server 15 SP4"
"#;
        let release = parse_os_release(content).unwrap();
        assert_eq!(release.id.as_deref(), Some("sles"));
        assert_eq!(release.version_id.as_deref(), Some("15.4"));
    }

    #[test]
    fn parse_os_release_empty() {
        assert!(parse_os_release("").is_none());
        assert!(parse_os_release("some random text\n").is_none());
    }

    #[test]
    fn parse_rsyslog_tcp_forward() {
        let content = "*.* @@127.0.0.1:28330\n";
        let rules = parse_rsyslog_rules(content);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].facility.as_deref(), Some("*.*"));
        assert_eq!(rules[0].target_host.as_deref(), Some("127.0.0.1"));
        assert_eq!(rules[0].target_port, Some(28330));
    }

    #[test]
    fn parse_rsyslog_facility_filter() {
        let content = "local4.* @@127.0.0.1:28330\n";
        let rules = parse_rsyslog_rules(content);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].facility.as_deref(), Some("local4.*"));
    }

    #[test]
    fn parse_rsyslog_udp_forward() {
        let content = "*.* @10.0.0.1:514\n";
        let rules = parse_rsyslog_rules(content);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].target_port, Some(514));
    }

    #[test]
    fn parse_rsyslog_multiple_rules() {
        let content = "\
# AMA forwarding
*.* @@127.0.0.1:28330
local4.* @@127.0.0.1:28330
";
        let rules = parse_rsyslog_rules(content);
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn parse_rsyslog_no_rules() {
        let content = "# just comments\n$ModLoad imtcp\n";
        let rules = parse_rsyslog_rules(content);
        assert!(rules.is_empty());
    }
}
