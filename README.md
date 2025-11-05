# pj - Lightning-fast Project Directory Jumper

A blazing-fast fuzzy project directory jumper for developers, written in Rust. Jump to your projects and their subdirectories with minimal keystrokes.

## Features

- **Lightning Fast**: Scans only project folders limited by depth
- **Fuzzy Matching**: Match projects with abbreviations (`pj dec` → `decree-ng`)
- **Interactive Picker**: Fuzzy-find through projects when multiple matches exist
- **Configurable**: Customize scan paths, project markers, and scan depth
- **Project-Focused**: Only jumps to project roots (directories with .git, .jj, etc.)
- **Smart Detection**: Auto-detects TTY for seamless script integration

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/jcowgar/pj.git
cd pj

# Build release binary
cargo build --release

# Install binary
sudo cp target/release/pj /usr/local/bin/

# Install shell integration
echo 'source /path/to/pj/pj.sh' >> ~/.bashrc  # or ~/.zshrc
```

### Shell Setup

**Bash/Zsh:**
```bash
# Add to ~/.bashrc or ~/.zshrc
source /path/to/pj/pj.sh
```

**Fish:**
```bash
# Copy to fish functions directory
cp pj.fish ~/.config/fish/functions/

# Or manually copy the file to:
# ~/.config/fish/functions/pj.fish
```

## Usage

### Basic Examples

```bash
# Jump to a project by name
pj decree           # Jumps to ~/Projects/ai/decree-ng

# Fuzzy match with abbreviations
pj dec             # Same as above
pj ai/dec          # Matches path segments in project path

# No arguments - interactive picker
pj                 # Shows fuzzy finder with all projects

# List all projects
pj --list          # Lists all projects without picker
pj | grep decree   # Pipe-friendly output
```

### Multiple Matches

When a pattern matches multiple directories, `pj` will:
- Show an interactive picker in TTY mode (use arrow keys, type to filter)
- List all matches in non-interactive mode (pipes, scripts)

### Special Flags

```bash
pj --init-config   # Create default config at ~/.config/pj/config.toml
pj --list          # Force list mode (disable picker)
pj -               # Jump to previous project directory
pj --help          # Show help
```

## Configuration

Configuration file: `~/.config/pj/config.toml`

Generate default config:
```bash
pj --init-config
```

### Default Configuration

```toml
scan_paths = ["~/Projects"]

project_markers = [".git", ".jj", ".hg", ".project"]

max_depth = 5
```

### Configuration Options

- **scan_paths**: Directories to scan for projects
- **project_markers**: Files/folders that identify a project root
- **max_depth**: How deep to scan for project roots

### Example Custom Configuration

```toml
scan_paths = [
    "~/Projects",
    "~/Code",
    "~/work"
]

project_markers = [
    ".git",
    ".jj",
    ".hg",
    "package.json",
    "Cargo.toml",
    ".project"
]

max_depth = 6
```

## How It Works

1. **Scanning**: Walks configured directories to find project roots (identified by markers like `.git`)
2. **Matching**: Uses the [nucleo](https://github.com/helix-editor/nucleo) fuzzy matching algorithm (same as Helix editor)
3. **Selection**:
   - Single match → instant jump
   - Multiple matches → interactive picker (TTY) or list (non-TTY)
4. **Navigation**: Shell wrapper captures output and executes `cd`

## Use Cases

### Git Worktrees

```bash
# If you have separate worktrees as separate .git directories:
# ~/Projects/myproject/main/.git
# ~/Projects/myproject/feature/.git

pj mp/m        # Jump to main worktree
pj mp/f        # Jump to feature worktree
```

### Multiple Projects

```bash
# Quickly jump between related projects
pj api             # Jump to api project
pj frontend        # Jump to frontend project
pj backend         # Jump to backend project
```

### Quick Navigation

```bash
pj d<TAB>          # Even faster with shell completion
pj de              # Minimal typing
```

### Jump to Previous Directory

```bash
pj projecta        # Jump to project A
pj os              # Jump to project B
pj -               # Jump back to project A
pj -               # Jump back to project B
```

Just like `cd -`, you can bounce between your last two project directories.

## Why

**pj** is specialized for developers who organize projects in a consistent directory structure and want instant access without building up history first.

## Troubleshooting

### "No projects found"

- Run `pj --init-config` to create config
- Check that `scan_paths` includes your projects directory
- Verify your projects have one of the `project_markers` (`.git`, etc.)

### Picker not showing

- Ensure you're running in an interactive terminal
- Use `--list` to disable picker explicitly
- Check that stdout is a TTY: `[ -t 1 ] && echo "TTY" || echo "Not TTY"`

### Shell integration not working

- Make sure you **sourced** the file: `source pj.sh`, not executed it
- Restart your shell or run `source ~/.bashrc`
- Verify function is loaded: `type pj` should show "pj is a function"

## Building from Source

### Prerequisites

- Rust 1.70+ (`rustup install stable`)

### Build

```bash
git clone https://github.com/yourusername/pj.git
cd pj
cargo build --release

# Binary will be at: target/release/pj
```

### Development

```bash
# Run in debug mode
cargo run -- dec

# Run tests
cargo test

# Run with specific config
cargo run -- --init-config
```

## License

GPL 3.0 License - See LICENSE file for details

## Contributing

Contributions welcome! Please open an issue or PR.

## Credits

Built with:
- [nucleo](https://github.com/helix-editor/nucleo) - Fuzzy matching
- [clap](https://github.com/clap-rs/clap) - CLI parsing
- [walkdir](https://github.com/BurntSushi/walkdir) - Directory traversal

Inspired by:
- [zoxide](https://github.com/ajeetdsouza/zoxide)
- [autojump](https://github.com/wting/autojump)
- [fzf](https://github.com/junegunn/fzf)
