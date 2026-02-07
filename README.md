# ctx-snap

Generate LLM-friendly context snapshots of your codebase.

## Installation

Build from source:

```bash
git clone https://github.com/vedsathwik275/ctx-snap
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

# Ignore specific patterns (now supports glob patterns)
ctx-snap --ignore "*.log" --ignore "**/node_modules/**"

# Include only specific file types
ctx-snap --include "**/*.rs" --include "**/*.toml"

# Disable gitignore respect
ctx-snap --no-gitignore

# JSON output
ctx-snap --json -o context.json

# Quiet mode
ctx-snap --quiet
```

## Features

- ✅ Respects `.gitignore` automatically (with option to disable)
- ✅ Filters binary files
- ✅ Configurable file size limits
- ✅ Custom ignore patterns with glob support (`*.log`, `**/node_modules/**`)
- ✅ Include patterns for positive filtering
- ✅ Token estimation
- ✅ JSON or Markdown output
- ✅ Syntax highlighting in output

## Example Usage

```bash
# Use glob patterns for ignoring
ctx-snap --ignore "*.log" --ignore "**/node_modules/**"

# Include only specific file types
ctx-snap --include "**/*.rs" --include "**/*.toml"

# Disable gitignore respect
ctx-snap --no-gitignore

# Combine include and exclude patterns
ctx-snap --include "**/*.rs" --ignore "target/**"
```

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
  -i, --ignore <IGNORE>            Skip files matching glob pattern (can be used multiple times)
      --include <INCLUDE>          Only include files matching glob pattern (can be used multiple times)
      --no-gitignore               Disable automatic .gitignore respect
  -q, --quiet                      Suppress progress output
  -j, --json                       Output as JSON
  -h, --help                       Print help
  -V, --version                    Print version
```

## Testing

Run the test suite:

```bash
cargo test
```

Integration tests cover:
- Basic snapshot generation
- Large file filtering (--max-size-kb)
- Custom ignore patterns (--ignore)
- JSON output format (--json)

## License

MIT
