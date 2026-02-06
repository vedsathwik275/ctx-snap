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
