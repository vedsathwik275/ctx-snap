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