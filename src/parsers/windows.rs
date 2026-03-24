use crate::parsers::ParsedBundle;
use std::path::Path;

/// Windows-specific parsing logic.
///
/// Extracts additional Windows data from the parsed bundle, such as:
/// - Extension provisioning status
/// - MonAgentCore service state
/// - AuthToken-MSI.json presence and contents
/// - AgentTroubleshooter output
pub fn enrich_windows_data(bundle: &mut ParsedBundle, _bundle_dir: &Path) {
    // Check for AuthToken-MSI.json
    let has_auth_token = bundle
        .files
        .keys()
        .any(|k| k.to_lowercase().contains("authtoken-msi"));

    if !has_auth_token {
        tracing::debug!("AuthToken-MSI.json not found in Windows bundle");
    }

    // Check for mcsconfig files
    if bundle.xml_configs.is_empty() {
        tracing::debug!("No mcsconfig XML files found in Windows bundle");
    }

    // Look for troubleshooter output
    let has_troubleshooter = bundle
        .files
        .keys()
        .any(|k| k.to_lowercase().contains("agenttroubleshooter"));

    if has_troubleshooter {
        tracing::debug!("AgentTroubleshooter output found in bundle");
    }
}
