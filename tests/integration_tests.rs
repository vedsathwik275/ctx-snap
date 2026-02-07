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
        .arg("*.log")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("keep.rs"));
    assert!(!content.contains("ignore.log"));
}

#[test]
fn test_glob_ignore_patterns() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("keep.rs"), "// keep").unwrap();
    fs::write(temp.path().join("debug.log"), "// debug").unwrap();
    fs::write(temp.path().join("error.log"), "// error").unwrap();
    fs::write(temp.path().join("info.log"), "// info").unwrap();

    let output = temp.path().join("output.md");

    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--ignore")
        .arg("*.log")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("keep.rs"));
    assert!(!content.contains("debug.log"));
    assert!(!content.contains("error.log"));
    assert!(!content.contains("info.log"));
}

#[test]
fn test_glob_ignore_patterns_with_subdirectories() {
    let temp = TempDir::new().unwrap();
    // Create nested directory structure
    let src_dir = temp.path().join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    
    fs::write(temp.path().join("root.rs"), "// root").unwrap();
    fs::write(src_dir.join("main.rs"), "// main").unwrap();
    fs::write(src_dir.join("lib.rs"), "// lib").unwrap();
    fs::write(temp.path().join("config.toml"), "// config").unwrap();

    let output = temp.path().join("output.md");

    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--ignore")
        .arg("*.rs")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(!content.contains("root.rs"));
    assert!(!content.contains("main.rs"));
    assert!(!content.contains("lib.rs"));
    assert!(content.contains("config.toml"));
}

#[test]
fn test_include_patterns() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("main.rs"), "// main").unwrap();
    fs::write(temp.path().join("lib.rs"), "// lib").unwrap();
    fs::write(temp.path().join("config.toml"), "// config").unwrap();
    fs::write(temp.path().join("readme.md"), "# readme").unwrap();

    let output = temp.path().join("output.md");

    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--include")
        .arg("**/*.rs")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("main.rs"));
    assert!(content.contains("lib.rs"));
    assert!(!content.contains("config.toml"));
    assert!(!content.contains("readme.md"));
}

#[test]
fn test_no_gitignore_flag() {
    let temp = TempDir::new().unwrap();
    let git_dir = temp.path().join(".git");
    std::fs::create_dir_all(&git_dir).unwrap();
    
    // Create a .gitignore file
    fs::write(temp.path().join(".gitignore"), "*.tmp\n").unwrap();
    
    // Create files - one that should be ignored by gitignore
    fs::write(temp.path().join("test.tmp"), "temporary file").unwrap();
    fs::write(temp.path().join("test.rs"), "fn main() {}").unwrap();

    let output = temp.path().join("output.md");

    // Test with --no-gitignore flag
    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--no-gitignore")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("test.tmp")); // Should be included now
    assert!(content.contains("test.rs"));
}

#[test]
fn test_json_output() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("test.rs"), "fn main() {}\n").unwrap();

    let output = temp.path().join("output.json");

    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .arg("--json")
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert!(json["files"].is_array());
    assert_eq!(json["total_files"], 1);
}
