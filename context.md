# Project Context Snapshot

**Generated:** 2026-02-05 20:37:44
**Root:** `.`
**Total files:** 13
**Estimated tokens:** ~11901

## Directory Structure

```
.git/FETCH_HEAD (0 lines)
.git/HEAD (1 lines)
.git/config (7 lines)
.git/description (1 lines)
.git/hooks/README.sample (5 lines)
.git/info/exclude (2 lines)
.gitignore (1 lines)
CLAUDE.md (849 lines)
Cargo.lock (610 lines)
Cargo.toml (18 lines)
README.md (115 lines)
src/main.rs (288 lines)
test-dir/test.rs (1 lines)
```

## File Contents

### File: `.git/FETCH_HEAD` (0 lines)

```

```

### File: `.git/HEAD` (1 lines)

```
ref: refs/heads/master

```

### File: `.git/config` (7 lines)

```
[core]
	bare = false
	repositoryformatversion = 0
	filemode = true
	ignorecase = true
	precomposeunicode = true
	logallrefupdates = true

```

### File: `.git/description` (1 lines)

```
Unnamed repository; edit this file 'description' to name the repository.

```

### File: `.git/hooks/README.sample` (5 lines)

```
#!/bin/sh
#
# Place appropriately named executable hook scripts into this directory
# to intercept various actions that git takes.  See `git help hooks` for
# more information.

```

### File: `.git/info/exclude` (2 lines)

```
# File patterns to ignore; see `git help ignore` for more information.
# Lines that start with '#' are comments.

```

### File: `.gitignore` (1 lines)

```
/target

```

### File: `CLAUDE.md` (849 lines)

```markdown
# ctx-snap: Project Specification & Implementation Guide

**Version:** 0.1.0  
**Target:** Rust CLI tool for generating LLM-friendly context snapshots  
**Time to Implement:** 2-3 hours  
**Difficulty:** Beginner-Intermediate

---

## Project Overview

### Purpose
`ctx-snap` is a command-line tool that captures your entire codebase into a single, well-formatted markdown file optimized for LLM context windows. It solves the tedious problem of manually copying and pasting files when working with AI coding assistants.

### Key Features
- ✅ Respects `.gitignore` automatically
- ✅ Filters binary files and large files
- ✅ Generates directory tree visualization
- ✅ Includes full file contents with syntax highlighting
- ✅ Estimates token counts
- ✅ Custom ignore patterns
- ✅ Configurable file size limits

### Use Cases
- Sharing full project context with Claude/ChatGPT
- Code reviews and documentation
- Onboarding new developers
- Debugging with AI assistance
- Creating context for agentic coding tools

---

## Project Structure

```
ctx-snap/
├── Cargo.toml
├── README.md
├── .gitignore
├── src/
│   └── main.rs
├── tests/
│   └── integration_tests.rs
└── examples/
    └── sample_output.md
```

---

## Technical Specification

### Dependencies

```toml
[package]
name = "ctx-snap"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Generate LLM-friendly context snapshots of your codebase"
license = "MIT"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
ignore = "0.4"          # Respects .gitignore
walkdir = "2.4"         # Directory traversal
anyhow = "1.0"          # Error handling
chrono = "0.4"          # Timestamps
indicatif = "0.17"      # Progress bars (optional)
```

### Command-Line Interface

```
ctx-snap [OPTIONS] [PATH]

ARGS:
    <PATH>    Directory to snapshot [default: current directory]

OPTIONS:
    -o, --output <FILE>         Output file [default: context.md]
    -m, --max-size-kb <SIZE>    Maximum file size in KB [default: 100]
    -i, --ignore <PATTERN>      Skip files matching pattern (repeatable)
    -q, --quiet                 Suppress progress output
    -j, --json                  Output as JSON instead of markdown
    -h, --help                  Print help information
    -V, --version               Print version information

EXAMPLES:
    ctx-snap                              # Snapshot current directory
    ctx-snap /path/to/project             # Snapshot specific directory
    ctx-snap -o context.md src/           # Snapshot only src/ folder
    ctx-snap --ignore "*.log" --ignore "*.tmp"  # Custom ignore patterns
    ctx-snap --max-size-kb 50             # Skip files larger than 50KB
```

### Output Format

**Markdown Structure:**
```markdown
# Project Context Snapshot
Generated: 2026-02-05 14:23:11
Root: /path/to/project
Total files: 23
Estimated tokens: ~8,450

## Directory Structure
```
src/
├── main.rs (245 lines)
├── lib.rs (189 lines)
└── utils/
    └── parser.rs (156 lines)
tests/
└── integration_tests.rs (89 lines)
```

## File Contents

### File: src/main.rs (245 lines)
```rust
use clap::Parser;
// ... full content ...
```

### File: src/lib.rs (189 lines)
```rust
// ... full content ...
```
```

**JSON Structure (with --json flag):**
```json
{
  "generated_at": "2026-02-05T14:23:11Z",
  "root": "/path/to/project",
  "total_files": 23,
  "estimated_tokens": 8450,
  "files": [
    {
      "path": "src/main.rs",
      "relative_path": "src/main.rs",
      "lines": 245,
      "size_bytes": 8192,
      "language": "rust",
      "content": "use clap::Parser;\n..."
    }
  ]
}
```

---

## Implementation Plan

### Phase 1: Core Functionality (90 minutes)

#### File: `src/main.rs`

```rust
use anyhow::{Context, Result};
use clap::Parser;
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

/// Generate LLM-friendly context snapshots of your codebase
#[derive(Parser, Debug)]
#[command(name = "ctx-snap")]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to snapshot
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output file
    #[arg(short, long, default_value = "context.md")]
    output: PathBuf,

    /// Maximum file size in KB
    #[arg(short = 'm', long, default_value = "100")]
    max_size_kb: u64,

    /// Skip files matching pattern (can be used multiple times)
    #[arg(short, long)]
    ignore: Vec<String>,

    /// Suppress progress output
    #[arg(short, long)]
    quiet: bool,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if !args.quiet {
        println!("📸 Snapping context from: {}", args.path.display());
    }
    
    // Collect all eligible files
    let files = collect_files(&args)?;
    
    if !args.quiet {
        println!("📁 Found {} files", files.len());
    }
    
    // Generate output
    if args.json {
        generate_json_output(&args, &files)?;
    } else {
        generate_markdown_output(&args, &files)?;
    }
    
    if !args.quiet {
        println!("✅ Context saved to: {}", args.output.display());
    }
    
    Ok(())
}

/// Collect all files that should be included in the snapshot
fn collect_files(args: &Args) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    let walker = WalkBuilder::new(&args.path)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();
    
    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip directories
        if !path.is_file() {
            continue;
        }
        
        // Skip binary files
        if is_likely_binary(path)? {
            continue;
        }
        
        // Apply custom ignore patterns
        if should_ignore(path, &args.ignore) {
            continue;
        }
        
        // Check file size
        let metadata = fs::metadata(path)?;
        let size_kb = metadata.len() / 1024;
        
        if size_kb > args.max_size_kb {
            if !args.quiet {
                println!("⏭️  Skipping large file: {} ({}kb)", path.display(), size_kb);
            }
            continue;
        }
        
        files.push(path.to_path_buf());
    }
    
    // Sort files for consistent output
    files.sort();
    
    Ok(files)
}

/// Check if a file is likely binary (contains null bytes in first 512 bytes)
fn is_likely_binary(path: &Path) -> Result<bool> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(false), // If we can't open it, skip it
    };
    
    let mut buffer = [0u8; 512];
    let bytes_read = std::io::Read::read(&mut file, &mut buffer)?;
    
    // Check for null bytes
    Ok(buffer[..bytes_read].contains(&0))
}

/// Check if a file should be ignored based on custom patterns
fn should_ignore(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    
    patterns.iter().any(|pattern| {
        // Simple substring matching
        // Could be enhanced with glob patterns
        path_str.contains(pattern.as_str())
    })
}

/// Count lines in a file
fn count_lines(path: &Path) -> Result<usize> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().count())
}

/// Estimate tokens using simple heuristic (1 token ≈ 4 chars)
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

/// Detect programming language from file extension
fn detect_language(path: &Path) -> &str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("go") => "go",
        Some("java") => "java",
        Some("c") => "c",
        Some("cpp") | Some("cc") | Some("cxx") => "cpp",
        Some("h") | Some("hpp") => "cpp",
        Some("sh") => "bash",
        Some("json") => "json",
        Some("yaml") | Some("yml") => "yaml",
        Some("toml") => "toml",
        Some("md") => "markdown",
        Some("html") => "html",
        Some("css") => "css",
        Some("sql") => "sql",
        _ => "",
    }
}

/// Generate markdown output
fn generate_markdown_output(args: &Args, files: &[PathBuf]) -> Result<()> {
    let mut output = fs::File::create(&args.output)
        .context("Failed to create output file")?;
    
    // Header
    writeln!(output, "# Project Context Snapshot")?;
    writeln!(output, "")?;
    writeln!(output, "**Generated:** {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(output, "**Root:** `{}`", args.path.display())?;
    writeln!(output, "**Total files:** {}", files.len())?;
    
    // Calculate total tokens
    let total_tokens: usize = files.iter()
        .filter_map(|f| fs::read_to_string(f).ok())
        .map(|content| estimate_tokens(&content))
        .sum();
    
    writeln!(output, "**Estimated tokens:** ~{}", total_tokens)?;
    writeln!(output, "")?;
    
    // Directory tree
    writeln!(output, "## Directory Structure")?;
    writeln!(output, "")?;
    writeln!(output, "```")?;
    
    for file in files {
        if let Ok(relative) = file.strip_prefix(&args.path) {
            let lines = count_lines(file).unwrap_or(0);
            writeln!(output, "{} ({} lines)", relative.display(), lines)?;
        }
    }
    
    writeln!(output, "```")?;
    writeln!(output, "")?;
    
    // File contents
    writeln!(output, "## File Contents")?;
    writeln!(output, "")?;
    
    for file in files {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Failed to read {}: {}", file.display(), e);
                continue;
            }
        };
        
        let relative = file.strip_prefix(&args.path)
            .unwrap_or(file)
            .display();
        
        let lines = content.lines().count();
        let language = detect_language(file);
        
        writeln!(output, "### File: `{}` ({} lines)", relative, lines)?;
        writeln!(output, "")?;
        writeln!(output, "```{}", language)?;
        writeln!(output, "{}", content)?;
        writeln!(output, "```")?;
        writeln!(output, "")?;
    }
    
    Ok(())
}

/// Generate JSON output
fn generate_json_output(args: &Args, files: &[PathBuf]) -> Result<()> {
    use serde_json::json;
    
    let mut file_entries = Vec::new();
    
    for file in files {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        let relative = file.strip_prefix(&args.path)
            .unwrap_or(file)
            .to_string_lossy()
            .to_string();
        
        let metadata = fs::metadata(file)?;
        
        file_entries.push(json!({
            "path": file.display().to_string(),
            "relative_path": relative,
            "lines": content.lines().count(),
            "size_bytes": metadata.len(),
            "language": detect_language(file),
            "content": content,
        }));
    }
    
    let total_tokens: usize = file_entries.iter()
        .filter_map(|e| e["content"].as_str())
        .map(|content| estimate_tokens(content))
        .sum();
    
    let output_json = json!({
        "generated_at": chrono::Local::now().to_rfc3339(),
        "root": args.path.display().to_string(),
        "total_files": files.len(),
        "estimated_tokens": total_tokens,
        "files": file_entries,
    });
    
    let mut output = fs::File::create(&args.output)?;
    writeln!(output, "{}", serde_json::to_string_pretty(&output_json)?)?;
    
    Ok(())
}
```

**Note:** For JSON output, add to `Cargo.toml`:
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

### Phase 2: Testing (30 minutes)

#### File: `tests/integration_tests.rs`

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_basic_snapshot() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.rs");
    fs::write(&test_file, "fn main() {}\n").unwrap();
    
    let output = temp.path().join("output.md");
    
    Command::cargo_bin("ctx-snap")
        .unwrap()
        .arg(temp.path())
        .arg("-o")
        .arg(&output)
        .assert()
        .success();
    
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
```

**Add to `Cargo.toml`:**
```toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"
```

---

### Phase 3: Documentation (15 minutes)

#### File: `README.md`

```markdown
# ctx-snap

Generate LLM-friendly context snapshots of your codebase.

## Installation

```bash
cargo install ctx-snap
```

Or build from source:

```bash
git clone https://github.com/yourusername/ctx-snap
cd ctx-snap
cargo install --path .
```

## Usage

Basic usage:
```bash
# Snapshot current directory
ctx-snap

# Snapshot specific directory
ctx-snap /path/to/project

# Custom output location
ctx-snap -o mycontext.md

# Skip large files
ctx-snap --max-size-kb 50

# Ignore specific patterns
ctx-snap --ignore "*.log" --ignore "node_modules"

# JSON output
ctx-snap --json -o context.json
```

## Features

- ✅ Respects `.gitignore` automatically
- ✅ Filters binary files
- ✅ Configurable file size limits
- ✅ Custom ignore patterns
- ✅ Token estimation
- ✅ JSON or Markdown output
- ✅ Syntax highlighting in output

## Example Output

```markdown
# Project Context Snapshot

**Generated:** 2026-02-05 14:23:11
**Root:** `/Users/dev/myproject`
**Total files:** 23
**Estimated tokens:** ~8,450

## Directory Structure

```
src/
├── main.rs (245 lines)
└── lib.rs (189 lines)
```

## File Contents

### File: `src/main.rs` (245 lines)

```rust
use clap::Parser;
// ... full content ...
```
```

## Use Cases

- Sharing project context with Claude, ChatGPT, or other LLMs
- Code reviews
- Documentation
- Onboarding
- Debugging with AI assistance

## License

MIT
```

---

### Phase 4: Optional Enhancements (30-60 minutes)

#### Enhancement 1: Progress Bar

```rust
// Add to dependencies
use indicatif::{ProgressBar, ProgressStyle};

fn collect_files(args: &Args) -> Result<Vec<PathBuf>> {
    // ... existing code ...
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    
    for entry in walker.filter_map(|e| e.ok()) {
        pb.set_message(format!("Scanning: {}", entry.path().display()));
        // ... process entry ...
        pb.inc(1);
    }
    
    pb.finish_with_message(format!("Found {} files", files.len()));
    
    Ok(files)
}
```

#### Enhancement 2: Watch Mode

```rust
// Add clap arg
#[arg(short, long)]
watch: bool,

// In main.rs
if args.watch {
    use notify::{Watcher, RecursiveMode, watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;
    
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2))?;
    watcher.watch(&args.path, RecursiveMode::Recursive)?;
    
    println!("👀 Watching for changes...");
    
    loop {
        match rx.recv() {
            Ok(_) => {
                println!("🔄 Regenerating snapshot...");
                let files = collect_files(&args)?;
                generate_markdown_output(&args, &files)?;
                println!("✅ Updated!");
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }
}
```

#### Enhancement 3: File Stats Summary

```rust
fn write_stats_summary(output: &mut fs::File, files: &[PathBuf]) -> Result<()> {
    use std::collections::HashMap;
    
    let mut lang_stats: HashMap<String, (usize, usize)> = HashMap::new();
    
    for file in files {
        let lang = detect_language(file).to_string();
        let content = fs::read_to_string(file)?;
        let lines = content.lines().count();
        
        lang_stats.entry(lang)
            .and_modify(|(count, total_lines)| {
                *count += 1;
                *total_lines += lines;
            })
            .or_insert((1, lines));
    }
    
    writeln!(output, "## Language Breakdown")?;
    writeln!(output, "")?;
    writeln!(output, "| Language | Files | Lines |")?;
    writeln!(output, "|----------|-------|-------|")?;
    
    let mut sorted: Vec<_> = lang_stats.iter().collect();
    sorted.sort_by_key(|(_, (_, lines))| std::cmp::Reverse(*lines));
    
    for (lang, (count, lines)) in sorted {
        let display_lang = if lang.is_empty() { "Other" } else { lang };
        writeln!(output, "| {} | {} | {} |", display_lang, count, lines)?;
    }
    
    writeln!(output, "")?;
    Ok(())
}
```

---

## Testing Checklist

Before considering the tool complete, test:

- [ ] Works in empty directory
- [ ] Works in directory with no text files
- [ ] Respects .gitignore
- [ ] Handles binary files correctly
- [ ] Handles very large files (>1MB)
- [ ] Handles files with special characters in names
- [ ] Handles deep directory nesting
- [ ] Handles symlinks appropriately
- [ ] JSON output is valid
- [ ] Works on Windows/Linux/macOS
- [ ] Handles files with no extension
- [ ] Handles permission denied errors gracefully

---

## Deployment

### Build release binary:
```bash
cargo build --release
```

### Install locally:
```bash
cargo install --path .
```

### Package for distribution:
```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

---

## Future Enhancements

1. **Smart filtering**: Use ML to detect which files are most relevant
2. **Diff mode**: Only include changed files since last commit
3. **Interactive mode**: Let user select which files to include
4. **Cloud sync**: Upload to S3/GCS for sharing
5. **Compression**: Gzip large outputs
6. **Streaming**: Generate output progressively for very large repos
7. **LSP integration**: Extract function/class signatures
8. **Dependency graph**: Show import relationships

---

## Success Criteria

You'll know the tool is working when:
1. You can run `ctx-snap` in any project directory
2. It generates a readable markdown file
3. The file size is reasonable (<10MB for most projects)
4. You can copy-paste the output into Claude
5. Claude understands the full project structure
6. The tool runs in <5 seconds for typical projects (<1000 files)

---

<claude-mem-context>

</claude-mem-context>
```

### File: `Cargo.lock` (610 lines)

```
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "aho-corasick"
version = "1.1.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ddd31a130427c27518df266943a5308ed92d4b226cc639f5a8f1002816174301"
dependencies = [
 "memchr",
]

[[package]]
name = "android_system_properties"
version = "0.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "819e7219dbd41043ac279b19830f2efc897156490d7fd6ea916720117ee66311"
dependencies = [
 "libc",
]

[[package]]
name = "anstream"
version = "0.6.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "43d5b281e737544384e969a5ccad3f1cdd24b48086a0fc1b2a5262a26b8f4f4a"
dependencies = [
 "anstyle",
 "anstyle-parse",
 "anstyle-query",
 "anstyle-wincon",
 "colorchoice",
 "is_terminal_polyfill",
 "utf8parse",
]

[[package]]
name = "anstyle"
version = "1.0.13"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5192cca8006f1fd4f7237516f40fa183bb07f8fbdfedaa0036de5ea9b0b45e78"

[[package]]
name = "anstyle-parse"
version = "0.2.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4e7644824f0aa2c7b9384579234ef10eb7efb6a0deb83f9630a49594dd9c15c2"
dependencies = [
 "utf8parse",
]

[[package]]
name = "anstyle-query"
version = "1.1.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "40c48f72fd53cd289104fc64099abca73db4166ad86ea0b4341abe65af83dadc"
dependencies = [
 "windows-sys",
]

[[package]]
name = "anstyle-wincon"
version = "3.0.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "291e6a250ff86cd4a820112fb8898808a366d8f9f58ce16d1f538353ad55747d"
dependencies = [
 "anstyle",
 "once_cell_polyfill",
 "windows-sys",
]

[[package]]
name = "anyhow"
version = "1.0.101"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5f0e0fee31ef5ed1ba1316088939cea399010ed7731dba877ed44aeb407a75ea"

[[package]]
name = "autocfg"
version = "1.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c08606f8c3cbf4ce6ec8e28fb0014a2c086708fe954eaa885384a6165172e7e8"

[[package]]
name = "bstr"
version = "1.12.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "63044e1ae8e69f3b5a92c736ca6269b8d12fa7efe39bf34ddb06d102cf0e2cab"
dependencies = [
 "memchr",
 "serde",
]

[[package]]
name = "bumpalo"
version = "3.19.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5dd9dc738b7a8311c7ade152424974d8115f2cdad61e8dab8dac9f2362298510"

[[package]]
name = "cc"
version = "1.2.55"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "47b26a0954ae34af09b50f0de26458fa95369a0d478d8236d3f93082b219bd29"
dependencies = [
 "find-msvc-tools",
 "shlex",
]

[[package]]
name = "cfg-if"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9330f8b2ff13f34540b44e946ef35111825727b38d33286ef986142615121801"

[[package]]
name = "chrono"
version = "0.4.43"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "fac4744fb15ae8337dc853fee7fb3f4e48c0fbaa23d0afe49c447b4fab126118"
dependencies = [
 "iana-time-zone",
 "js-sys",
 "num-traits",
 "wasm-bindgen",
 "windows-link",
]

[[package]]
name = "clap"
version = "4.5.57"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6899ea499e3fb9305a65d5ebf6e3d2248c5fab291f300ad0a704fbe142eae31a"
dependencies = [
 "clap_builder",
 "clap_derive",
]

[[package]]
name = "clap_builder"
version = "4.5.57"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7b12c8b680195a62a8364d16b8447b01b6c2c8f9aaf68bee653be34d4245e238"
dependencies = [
 "anstream",
 "anstyle",
 "clap_lex",
 "strsim",
]

[[package]]
name = "clap_derive"
version = "4.5.55"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a92793da1a46a5f2a02a6f4c46c6496b28c43638adea8306fcb0caa1634f24e5"
dependencies = [
 "heck",
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "clap_lex"
version = "0.7.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c3e64b0cc0439b12df2fa678eae89a1c56a529fd067a9115f7827f1fffd22b32"

[[package]]
name = "colorchoice"
version = "1.0.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b05b61dc5112cbb17e4b6cd61790d9845d13888356391624cbe7e41efeac1e75"

[[package]]
name = "core-foundation-sys"
version = "0.8.7"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "773648b94d0e5d620f64f280777445740e61fe701025087ec8b57f45c791888b"

[[package]]
name = "crossbeam-deque"
version = "0.8.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9dd111b7b7f7d55b72c0a6ae361660ee5853c9af73f70c3c2ef6858b950e2e51"
dependencies = [
 "crossbeam-epoch",
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-epoch"
version = "0.9.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5b82ac4a3c2ca9c3460964f020e1402edd5753411d7737aa39c3714ad1b5420e"
dependencies = [
 "crossbeam-utils",
]

[[package]]
name = "crossbeam-utils"
version = "0.8.21"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d0a5c400df2834b80a4c3327b3aad3a4c4cd4de0629063962b03235697506a28"

[[package]]
name = "ctx-snap"
version = "0.1.0"
dependencies = [
 "anyhow",
 "chrono",
 "clap",
 "ignore",
 "serde",
 "serde_json",
 "walkdir",
]

[[package]]
name = "find-msvc-tools"
version = "0.1.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5baebc0774151f905a1a2cc41989300b1e6fbb29aff0ceffa1064fdd3088d582"

[[package]]
name = "globset"
version = "0.4.18"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "52dfc19153a48bde0cbd630453615c8151bce3a5adfac7a0aebfbf0a1e1f57e3"
dependencies = [
 "aho-corasick",
 "bstr",
 "log",
 "regex-automata",
 "regex-syntax",
]

[[package]]
name = "heck"
version = "0.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "2304e00983f87ffb38b55b444b5e3b60a884b5d30c0fca7d82fe33449bbe55ea"

[[package]]
name = "iana-time-zone"
version = "0.1.65"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e31bc9ad994ba00e440a8aa5c9ef0ec67d5cb5e5cb0cc7f8b744a35b389cc470"
dependencies = [
 "android_system_properties",
 "core-foundation-sys",
 "iana-time-zone-haiku",
 "js-sys",
 "log",
 "wasm-bindgen",
 "windows-core",
]

[[package]]
name = "iana-time-zone-haiku"
version = "0.1.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f31827a206f56af32e590ba56d5d2d085f558508192593743f16b2306495269f"
dependencies = [
 "cc",
]

[[package]]
name = "ignore"
version = "0.4.25"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d3d782a365a015e0f5c04902246139249abf769125006fbe7649e2ee88169b4a"
dependencies = [
 "crossbeam-deque",
 "globset",
 "log",
 "memchr",
 "regex-automata",
 "same-file",
 "walkdir",
 "winapi-util",
]

[[package]]
name = "is_terminal_polyfill"
version = "1.70.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a6cb138bb79a146c1bd460005623e142ef0181e3d0219cb493e02f7d08a35695"

[[package]]
name = "itoa"
version = "1.0.17"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "92ecc6618181def0457392ccd0ee51198e065e016d1d527a7ac1b6dc7c1f09d2"

[[package]]
name = "js-sys"
version = "0.3.85"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8c942ebf8e95485ca0d52d97da7c5a2c387d0e7f0ba4c35e93bfcaee045955b3"
dependencies = [
 "once_cell",
 "wasm-bindgen",
]

[[package]]
name = "libc"
version = "0.2.180"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "bcc35a38544a891a5f7c865aca548a982ccb3b8650a5b06d0fd33a10283c56fc"

[[package]]
name = "log"
version = "0.4.29"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5e5032e24019045c762d3c0f28f5b6b8bbf38563a65908389bf7978758920897"

[[package]]
name = "memchr"
version = "2.7.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f52b00d39961fc5b2736ea853c9cc86238e165017a493d1d5c8eac6bdc4cc273"

[[package]]
name = "num-traits"
version = "0.2.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "071dfc062690e90b734c0b2273ce72ad0ffa95f0c74596bc250dcfd960262841"
dependencies = [
 "autocfg",
]

[[package]]
name = "once_cell"
version = "1.21.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "42f5e15c9953c5e4ccceeb2e7382a716482c34515315f7b03532b8b4e8393d2d"

[[package]]
name = "once_cell_polyfill"
version = "1.70.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "384b8ab6d37215f3c5301a95a4accb5d64aa607f1fcb26a11b5303878451b4fe"

[[package]]
name = "proc-macro2"
version = "1.0.106"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8fd00f0bb2e90d81d1044c2b32617f68fcb9fa3bb7640c23e9c748e53fb30934"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "quote"
version = "1.0.44"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "21b2ebcf727b7760c461f091f9f0f539b77b8e87f2fd88131e7f1b433b3cece4"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "regex-automata"
version = "0.4.14"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "6e1dd4122fc1595e8162618945476892eefca7b88c52820e74af6262213cae8f"
dependencies = [
 "aho-corasick",
 "memchr",
 "regex-syntax",
]

[[package]]
name = "regex-syntax"
version = "0.8.9"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "a96887878f22d7bad8a3b6dc5b7440e0ada9a245242924394987b21cf2210a4c"

[[package]]
name = "rustversion"
version = "1.0.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b39cdef0fa800fc44525c84ccb54a029961a8215f9619753635a9c0d2538d46d"

[[package]]
name = "same-file"
version = "1.0.6"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502"
dependencies = [
 "winapi-util",
]

[[package]]
name = "serde"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9a8e94ea7f378bd32cbbd37198a4a91436180c5bb472411e48b5ec2e2124ae9e"
dependencies = [
 "serde_core",
 "serde_derive",
]

[[package]]
name = "serde_core"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "41d385c7d4ca58e59fc732af25c3983b67ac852c1a25000afe1175de458b67ad"
dependencies = [
 "serde_derive",
]

[[package]]
name = "serde_derive"
version = "1.0.228"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d540f220d3187173da220f885ab66608367b6574e925011a9353e4badda91d79"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "serde_json"
version = "1.0.149"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "83fc039473c5595ace860d8c4fafa220ff474b3fc6bfdb4293327f1a37e94d86"
dependencies = [
 "itoa",
 "memchr",
 "serde",
 "serde_core",
 "zmij",
]

[[package]]
name = "shlex"
version = "1.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "0fda2ff0d084019ba4d7c6f371c95d8fd75ce3524c3cb8fb653a3023f6323e64"

[[package]]
name = "strsim"
version = "0.11.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7da8b5736845d9f2fcb837ea5d9e2628564b3b043a70948a3f0b778838c5fb4f"

[[package]]
name = "syn"
version = "2.0.114"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d4d107df263a3013ef9b1879b0df87d706ff80f65a86ea879bd9c31f9b307c2a"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "unicode-ident"
version = "1.0.22"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "9312f7c4f6ff9069b165498234ce8be658059c6728633667c526e27dc2cf1df5"

[[package]]
name = "utf8parse"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "06abde3611657adf66d383f00b093d7faecc7fa57071cce2578660c9f1010821"

[[package]]
name = "walkdir"
version = "2.5.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "29790946404f91d9c5d06f9874efddea1dc06c5efe94541a7d6863108e3a5e4b"
dependencies = [
 "same-file",
 "winapi-util",
]

[[package]]
name = "wasm-bindgen"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "64024a30ec1e37399cf85a7ffefebdb72205ca1c972291c51512360d90bd8566"
dependencies = [
 "cfg-if",
 "once_cell",
 "rustversion",
 "wasm-bindgen-macro",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-macro"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "008b239d9c740232e71bd39e8ef6429d27097518b6b30bdf9086833bd5b6d608"
dependencies = [
 "quote",
 "wasm-bindgen-macro-support",
]

[[package]]
name = "wasm-bindgen-macro-support"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5256bae2d58f54820e6490f9839c49780dff84c65aeab9e772f15d5f0e913a55"
dependencies = [
 "bumpalo",
 "proc-macro2",
 "quote",
 "syn",
 "wasm-bindgen-shared",
]

[[package]]
name = "wasm-bindgen-shared"
version = "0.2.108"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "1f01b580c9ac74c8d8f0c0e4afb04eeef2acf145458e52c03845ee9cd23e3d12"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "winapi-util"
version = "0.1.11"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "c2a7b1c03c876122aa43f3020e6c3c3ee5c05081c9a00739faf7503aeba10d22"
dependencies = [
 "windows-sys",
]

[[package]]
name = "windows-core"
version = "0.62.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "b8e83a14d34d0623b51dce9581199302a221863196a1dde71a7663a4c2be9deb"
dependencies = [
 "windows-implement",
 "windows-interface",
 "windows-link",
 "windows-result",
 "windows-strings",
]

[[package]]
name = "windows-implement"
version = "0.60.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "053e2e040ab57b9dc951b72c264860db7eb3b0200ba345b4e4c3b14f67855ddf"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "windows-interface"
version = "0.59.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3f316c4a2570ba26bbec722032c4099d8c8bc095efccdc15688708623367e358"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "windows-link"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f0805222e57f7521d6a62e36fa9163bc891acd422f971defe97d64e70d0a4fe5"

[[package]]
name = "windows-result"
version = "0.4.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7781fa89eaf60850ac3d2da7af8e5242a5ea78d1a11c49bf2910bb5a73853eb5"
dependencies = [
 "windows-link",
]

[[package]]
name = "windows-strings"
version = "0.5.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "7837d08f69c77cf6b07689544538e017c1bfcf57e34b4c0ff58e6c2cd3b37091"
dependencies = [
 "windows-link",
]

[[package]]
name = "windows-sys"
version = "0.61.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "ae137229bcbd6cdf0f7b80a31df61766145077ddf49416a728b02cb3921ff3fc"
dependencies = [
 "windows-link",
]

[[package]]
name = "zmij"
version = "1.0.19"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "3ff05f8caa9038894637571ae6b9e29466c1f4f829d26c9b28f869a29cbe3445"

```

### File: `Cargo.toml` (18 lines)

```toml
[package]
name = "ctx-snap"
version = "0.1.0"
edition = "2021"
authors = ["Sathwik Toduru"]
description = "Generate LLM-friendly context snapshots of your codebase"
license = "MIT"
keywords = ["cli", "llm", "context", "snapshot", "markdown"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
clap = { version = "4.5", features = ["derive"] }
ignore = "0.4"
walkdir = "2.4"
anyhow = "1.0"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

```

### File: `README.md` (115 lines)

```markdown
# ctx-snap

Generate LLM-friendly context snapshots of your codebase.

## Installation

Build from source:

```bash
git clone https://github.com/yourusername/ctx-snap
cd ctx-snap
cargo install --path .
```

Or run directly:

```bash
cargo build --release
./target/release/ctx-snap
```

## Usage

Basic usage:
```bash
# Snapshot current directory
ctx-snap

# Snapshot specific directory
ctx-snap /path/to/project

# Custom output location
ctx-snap -o mycontext.md

# Skip large files
ctx-snap --max-size-kb 50

# Ignore specific patterns
ctx-snap --ignore "*.log" --ignore "node_modules"

# JSON output
ctx-snap --json -o context.json

# Quiet mode
ctx-snap --quiet
```

## Features

- ✅ Respects `.gitignore` automatically
- ✅ Filters binary files
- ✅ Configurable file size limits
- ✅ Custom ignore patterns
- ✅ Token estimation
- ✅ JSON or Markdown output
- ✅ Syntax highlighting in output

## Example Output

```markdown
# Project Context Snapshot

**Generated:** 2026-02-05 20:28:53
**Root:** `.`
**Total files:** 11
**Estimated tokens:** ~11,385

## Directory Structure

```
src/main.rs (288 lines)
Cargo.toml (18 lines)
Cargo.lock (610 lines)
```

## File Contents

### File: `src/main.rs` (288 lines)

```rust
use anyhow::{Context, Result};
use clap::Parser;
// ... full content ...
```
```

## Use Cases

- Sharing project context with Claude, ChatGPT, or other LLMs
- Code reviews
- Documentation
- Onboarding
- Debugging with AI assistance

## Command-Line Options

```
Usage: ctx-snap [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory to snapshot [default: .]

Options:
  -o, --output <OUTPUT>            Output file [default: context.md]
  -m, --max-size-kb <MAX_SIZE_KB>  Maximum file size in KB [default: 100]
  -i, --ignore <IGNORE>            Skip files matching pattern (can be used multiple times)
  -q, --quiet                      Suppress progress output
  -j, --json                       Output as JSON
  -h, --help                       Print help
  -V, --version                    Print version
```

## License

MIT

```

### File: `src/main.rs` (288 lines)

```rust
use anyhow::{Context, Result};
use clap::Parser;
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

/// Generate LLM-friendly context snapshots of your codebase
#[derive(Parser, Debug)]
#[command(name = "ctx-snap")]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to snapshot
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output file
    #[arg(short, long, default_value = "context.md")]
    output: PathBuf,

    /// Maximum file size in KB
    #[arg(short = 'm', long, default_value = "100")]
    max_size_kb: u64,

    /// Skip files matching pattern (can be used multiple times)
    #[arg(short, long)]
    ignore: Vec<String>,

    /// Suppress progress output
    #[arg(short, long)]
    quiet: bool,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.quiet {
        println!("📸 Snapping context from: {}", args.path.display());
    }

    // Collect all eligible files
    let files = collect_files(&args)?;

    if !args.quiet {
        println!("📁 Found {} files", files.len());
    }

    // Generate output
    if args.json {
        generate_json_output(&args, &files)?;
    } else {
        generate_markdown_output(&args, &files)?;
    }

    if !args.quiet {
        println!("✅ Context saved to: {}", args.output.display());
    }

    Ok(())
}

/// Collect all files that should be included in the snapshot
fn collect_files(args: &Args) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let walker = WalkBuilder::new(&args.path)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();

        // Skip directories
        if !path.is_file() {
            continue;
        }

        // Skip binary files
        if is_likely_binary(path)? {
            continue;
        }

        // Apply custom ignore patterns
        if should_ignore(path, &args.ignore) {
            continue;
        }

        // Check file size
        let metadata = fs::metadata(path)?;
        let size_kb = metadata.len() / 1024;

        if size_kb > args.max_size_kb {
            if !args.quiet {
                println!("⏭️  Skipping large file: {} ({}kb)", path.display(), size_kb);
            }
            continue;
        }

        files.push(path.to_path_buf());
    }

    // Sort files for consistent output
    files.sort();

    Ok(files)
}

/// Check if a file is likely binary (contains null bytes in first 512 bytes)
fn is_likely_binary(path: &Path) -> Result<bool> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(false), // If we can't open it, skip it
    };

    let mut buffer = [0u8; 512];
    let bytes_read = std::io::Read::read(&mut file, &mut buffer)?;

    // Check for null bytes
    Ok(buffer[..bytes_read].contains(&0))
}

/// Check if a file should be ignored based on custom patterns
fn should_ignore(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    patterns.iter().any(|pattern| {
        // Simple substring matching
        // Could be enhanced with glob patterns
        path_str.contains(pattern.as_str())
    })
}

/// Count lines in a file
fn count_lines(path: &Path) -> Result<usize> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().count())
}

/// Estimate tokens using simple heuristic (1 token ≈ 4 chars)
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

/// Detect programming language from file extension
fn detect_language(path: &Path) -> &str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => "rust",
        Some("py") => "python",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("go") => "go",
        Some("java") => "java",
        Some("c") => "c",
        Some("cpp") | Some("cc") | Some("cxx") => "cpp",
        Some("h") | Some("hpp") => "cpp",
        Some("sh") => "bash",
        Some("json") => "json",
        Some("yaml") | Some("yml") => "yaml",
        Some("toml") => "toml",
        Some("md") => "markdown",
        Some("html") => "html",
        Some("css") => "css",
        Some("sql") => "sql",
        _ => "",
    }
}

/// Generate markdown output
fn generate_markdown_output(args: &Args, files: &[PathBuf]) -> Result<()> {
    let mut output = fs::File::create(&args.output)
        .context("Failed to create output file")?;

    // Header
    writeln!(output, "# Project Context Snapshot")?;
    writeln!(output, "")?;
    writeln!(output, "**Generated:** {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(output, "**Root:** `{}`", args.path.display())?;
    writeln!(output, "**Total files:** {}", files.len())?;

    // Calculate total tokens
    let total_tokens: usize = files.iter()
        .filter_map(|f| fs::read_to_string(f).ok())
        .map(|content| estimate_tokens(&content))
        .sum();

    writeln!(output, "**Estimated tokens:** ~{}", total_tokens)?;
    writeln!(output, "")?;

    // Directory tree
    writeln!(output, "## Directory Structure")?;
    writeln!(output, "")?;
    writeln!(output, "```")?;

    for file in files {
        if let Ok(relative) = file.strip_prefix(&args.path) {
            let lines = count_lines(file).unwrap_or(0);
            writeln!(output, "{} ({} lines)", relative.display(), lines)?;
        }
    }

    writeln!(output, "```")?;
    writeln!(output, "")?;

    // File contents
    writeln!(output, "## File Contents")?;
    writeln!(output, "")?;

    for file in files {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Failed to read {}: {}", file.display(), e);
                continue;
            }
        };

        let relative = file.strip_prefix(&args.path)
            .unwrap_or(file)
            .display();

        let lines = content.lines().count();
        let language = detect_language(file);

        writeln!(output, "### File: `{}` ({} lines)", relative, lines)?;
        writeln!(output, "")?;
        writeln!(output, "```{}", language)?;
        writeln!(output, "{}", content)?;
        writeln!(output, "```")?;
        writeln!(output, "")?;
    }

    Ok(())
}

/// Generate JSON output
fn generate_json_output(args: &Args, files: &[PathBuf]) -> Result<()> {
    use serde_json::json;

    let mut file_entries = Vec::new();

    for file in files {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let relative = file.strip_prefix(&args.path)
            .unwrap_or(file)
            .to_string_lossy()
            .to_string();

        let metadata = fs::metadata(file)?;

        file_entries.push(json!({
            "path": file.display().to_string(),
            "relative_path": relative,
            "lines": content.lines().count(),
            "size_bytes": metadata.len(),
            "language": detect_language(file),
            "content": content,
        }));
    }

    let total_tokens: usize = file_entries.iter()
        .filter_map(|e| e["content"].as_str())
        .map(|content| estimate_tokens(content))
        .sum();

    let output_json = json!({
        "generated_at": chrono::Local::now().to_rfc3339(),
        "root": args.path.display().to_string(),
        "total_files": files.len(),
        "estimated_tokens": total_tokens,
        "files": file_entries,
    });

    let mut output = fs::File::create(&args.output)?;
    writeln!(output, "{}", serde_json::to_string_pretty(&output_json)?)?;

    Ok(())
}

```

### File: `test-dir/test.rs` (1 lines)

```rust
fn main() {}

```

