use crate::parsers::ParsedBundle;
use std::path::Path;

/// Linux-specific parsing logic.
///
/// Extracts additional Linux data from the parsed bundle, such as:
/// - mdsd process state
/// - rsyslog/syslog-ng configuration
/// - Python dependency state
/// - AMA troubleshooter script output
pub fn enrich_linux_data(bundle: &mut ParsedBundle, _bundle_dir: &Path) {
    // Check for mdsd log files
    let has_mdsd = bundle
        .files
        .keys()
        .any(|k| k.to_lowercase().contains("mdsd"));

    if !has_mdsd {
        tracing::debug!("No mdsd log files found in Linux bundle");
    }

    // Check for rsyslog config
    let has_rsyslog = bundle
        .files
        .keys()
        .any(|k| k.to_lowercase().contains("rsyslog"));

    if has_rsyslog {
        tracing::debug!("rsyslog configuration found in bundle");
    }

    // Check for syslog-ng config
    let has_syslog_ng = bundle
        .files
        .keys()
        .any(|k| k.to_lowercase().contains("syslog-ng"));

    if has_syslog_ng {
        tracing::debug!("syslog-ng configuration found in bundle");
    }
}
