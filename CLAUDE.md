# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`pj` is a lightning-fast project directory jumper written in Rust. It scans configured directories for project roots (identified by markers like `.git`, `.jj`, etc.) and enables fuzzy matching to quickly jump to projects with minimal keystrokes.

## Development Commands

### Building and Running

```bash
# Build in release mode
cargo build --release

# Run in debug mode with a pattern
cargo run -- dec

# Run with specific flags
cargo run -- --list
cargo run -- --init-config
```

### Testing

```bash
# Run all tests
cargo test
```

### Installation

```bash
# Install the binary locally
cargo install --path .

# Or copy to system path
sudo cp target/release/pj /usr/local/bin/
```

## Architecture

### Core Components (5 modules)

1. **main.rs** - Entry point and orchestration
   - CLI argument parsing with clap
   - TTY detection for interactive/non-interactive mode (`is_interactive()` checks `/dev/tty`)
   - Previous directory tracking (stored in `~/.local/state/pj/prev_dir`)
   - Coordinates scanner → matcher → picker flow

2. **scanner.rs** - Project discovery
   - `scan_projects()`: Walks configured scan paths up to `max_depth`
   - `Project` struct: Stores absolute path and relative display path
   - `is_project()`: Checks if directory contains any configured markers
   - Uses `walkdir` for efficient directory traversal

3. **matcher.rs** - Fuzzy matching
   - `Matcher`: Wraps `nucleo` fuzzy matching engine (same as Helix editor)
   - `add_projects()`: Populates matcher with project display paths
   - `find_matches()`: Returns sorted matches using smart case and normalization
   - Matches against relative display paths, not absolute paths

4. **picker.rs** - Interactive selection
   - `InteractivePicker`: Shows fuzzy-find UI using `nucleo-picker`
   - `pick()`: Returns selected project or None on cancellation
   - Only invoked when TTY is detected and `--list` not specified

5. **config.rs** - Configuration management
   - TOML config at `~/.config/pj/config.toml`
   - Fields: `scan_paths`, `project_markers`, `max_depth`
   - `Config::load()`: Reads config or returns defaults
   - `create_default_config()`: Generates default config file

### Key Data Flow

```
User Input (CLI args)
    ↓
Config Load (~/.config/pj/config.toml)
    ↓
Scanner (walks scan_paths, finds project markers)
    ↓
Projects (Vec<Project> with paths + display paths)
    ↓
├─ No pattern → Picker (if TTY) or List all
└─ Pattern → Matcher (fuzzy match)
         ↓
    Match Results
         ↓
    ├─ 0 matches → Error
    ├─ 1 match → Print path
    └─ Multiple → Picker (if TTY) or List all
              ↓
         Output path to stdout
              ↓
    Shell wrapper executes 'cd'
```

### Shell Integration

The binary outputs the target directory path to stdout. Shell wrappers (`pj.sh`, `pj.fish`) capture this output and execute `cd`:

- **pj.sh**: Bash/Zsh wrapper function that sources into shell
- **pj.fish**: Fish shell function in `~/.config/fish/functions/`

The wrapper also calls `pj --set-prev` before changing directories to enable `pj -` functionality.

## Important Implementation Details

### TTY Detection
- Uses `/dev/tty` open check, not `isatty(stdout)`
- This allows piping while still detecting interactive terminals
- Interactive mode → shows picker, Non-interactive → lists all matches

### Fuzzy Matching
- Uses `nucleo` with smart case matching (case-insensitive if pattern is lowercase)
- Matches against display paths (relative to scan root), not absolute paths
- Example: pattern `ai/dec` matches project `~/Projects/ai/decree-ng` via display path `ai/decree-ng`

### Previous Directory Tracking
- Stored in `~/.local/state/pj/prev_dir` (XDG state directory)
- Updated by shell wrapper before each jump using `--set-prev` flag
- Enables `pj -` to toggle between last two project directories

### Project Detection
- Only directories containing at least one marker file/folder are considered projects
- Default markers: `.git`, `.jj`, `.hg`, `.project`
- Scanner stops at project roots (doesn't descend into nested projects)

## Configuration

Default config location: `~/.config/pj/config.toml`

Generate with: `pj --init-config`

Example:
```toml
scan_paths = ["~/Projects", "~/Code"]
project_markers = [".git", ".jj", ".hg", "package.json", "Cargo.toml"]
max_depth = 6
```

## Dependencies

Key external crates:
- **nucleo** (0.5): Fuzzy matching engine
- **nucleo-picker** (0.9): Interactive TUI picker
- **walkdir** (2.5): Directory traversal
- **clap** (4.5): CLI argument parsing
- **toml** (0.8): Config file parsing
- **shellexpand** (3.1): Tilde expansion in paths
