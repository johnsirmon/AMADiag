use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_help_succeeds() {
    Command::cargo_bin("amadiag")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("AMA"));
}

#[test]
fn cli_version_succeeds() {
    Command::cargo_bin("amadiag")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("amadiag"));
}

#[test]
fn cli_rules_list_succeeds() {
    Command::cargo_bin("amadiag")
        .unwrap()
        .args(["rules", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"));
}

#[test]
fn cli_validate_missing_path_fails() {
    Command::cargo_bin("amadiag")
        .unwrap()
        .args(["validate", "/nonexistent/path/bundle.tgz"])
        .assert()
        .failure();
}

#[test]
fn cli_analyze_missing_path_fails() {
    Command::cargo_bin("amadiag")
        .unwrap()
        .args(["analyze", "/nonexistent/path/bundle.tgz"])
        .assert()
        .failure();
}

#[test]
fn cli_analyze_directory_bundle() {
    // Create a minimal directory bundle
    let dir = std::env::temp_dir().join("amadiag-test-bundle");
    let _ = std::fs::create_dir_all(&dir);
    std::fs::write(dir.join("test.log"), "error: test failure\n").unwrap();

    let result = Command::cargo_bin("amadiag")
        .unwrap()
        .args(["analyze", &dir.to_string_lossy()])
        .assert()
        // Exit code 1 is expected when critical findings are detected
        .stdout(predicate::str::contains("AMADiag"));

    // Clean up
    let _ = std::fs::remove_dir_all(&dir);

    let _ = result;
}
