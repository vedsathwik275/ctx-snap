use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_basic_snapshot() {
    // Test setup
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.rs");
    fs::write(&test_file, "fn main() {}\n").unwrap();

    let output = temp.path().join("output.md");

    // Run ctx-snap
    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .assert()
        .success();

    // Verify output
    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("Project Context Snapshot"));
    assert!(content.contains("test.rs"));
    assert!(content.contains("fn main()"));
}
