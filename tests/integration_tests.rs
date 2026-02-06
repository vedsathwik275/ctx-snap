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

#[test]
fn test_respects_max_size() {
    let temp = TempDir::new().unwrap();
    let large_file = temp.path().join("large.txt");

    // Create 200KB file
    let large_content = "x".repeat(200 * 1024);
    fs::write(&large_file, large_content).unwrap();

    let output = temp.path().join("output.md");

    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--max-size-kb")
        .arg("100")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(!content.contains("large.txt"));
}

#[test]
fn test_custom_ignore_pattern() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("keep.rs"), "// keep").unwrap();
    fs::write(temp.path().join("ignore.log"), "// ignore").unwrap();

    let output = temp.path().join("output.md");

    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--ignore")
        .arg(".log")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("keep.rs"));
    assert!(!content.contains("ignore.log"));
}
