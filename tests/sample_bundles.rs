use amadiag::{analyzers::finding::Platform, detect, input};
use std::path::{Path, PathBuf};

fn sample_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("samples")
        .join(name)
}

#[test]
fn all_sample_bundles_validate_and_analyze() {
    let sample_names = [
        "AgentTroubleshooterOutput-2026-03-26-00-23-17Z.zip",
        "AgentTroubleshooterOutput-amawin-2026-03-26.zip",
        "amalogs-ama-rhel8.tgz",
        "amalogs-ama-ubuntu22.tgz",
    ];

    for sample_name in sample_names {
        let sample = sample_path(sample_name);
        assert!(
            sample.exists(),
            "Missing sample bundle: {}",
            sample.display()
        );

        let validation = input::validate_bundle(&sample)
            .unwrap_or_else(|error| panic!("validate failed for {}: {error:#}", sample.display()));
        assert!(
            validation.contains("Status:     OK"),
            "validation did not report OK for {}:\n{validation}",
            sample.display()
        );

        let report = detect::analyze_bundle(&sample)
            .unwrap_or_else(|error| panic!("analyze failed for {}: {error:#}", sample.display()));
        assert!(
            report.files_analyzed > 0,
            "expected analyzed files for {}",
            sample.display()
        );
    }
}

#[test]
fn windows_sample_does_not_emit_linux_only_findings() {
    let report = detect::analyze_bundle(&sample_path(
        "AgentTroubleshooterOutput-amawin-2026-03-26.zip",
    ))
    .unwrap();

    assert_eq!(report.environment.platform, Some(Platform::Windows));
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.rule_id == "SERVICE-001"));
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.rule_id == "SYSLOG-001"));
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.rule_id == "GUEST-001"));
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.rule_id == "QOS-001"));
}
