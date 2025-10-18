# Evan's Day Field Guide - Rust + Ratatui TUI

A terminal user interface (TUI) implementation of your personal operational patterns and glitch sanctuary protocols. Converts the React browser app to a standalone Rust CLI tool using `ratatui` for rendering and `tokio` for async operations.

## Features

### FieldGuide View
- **7 sections** of operational patterns for daily rhythm and focus
- Color-coded sections (Cyan, Purple, Green, Amber, Blue, Teal, Rose)
- Each section contains pattern descriptions, signal indicators, and protocols
- Keyboard navigation with vim-like controls (hjkl)
- Entry details expansion
- Color-based filtering

### GlitchSanctuary View
- **4 programs** for system maintenance and recovery
- Record-based structure with expandable steps
- Status indicators (OPERATIONAL, CAUTION, DEGRADED, CRITICAL)
- System metrics display (Focus, Energy, Stability, Creativity)
- Tab-based program switching

### Navigation System
- **Command palette** (`: ` prefix) inspired by Zelij
- Switch between views with Tab key
- Global view switching via `:view field-guide` or `:view sanctuary`

## Quick Start

### Prerequisites
- Rust 1.75+ (run `rustc --version`)
- Cargo (comes with Rust)

### Installation
```bash
# Clone the repository
git clone https://github.com/e-schultz/rusty-react-terminal.git
cd rusty-react-terminal

# Build the project
cargo build --release

# Run the application
./target/release/evan-tui

# Or run directly with Cargo
cargo run
```

### Controls

#### FieldGuide View
| Key | Action |
|-----|--------|
| `h` / `←` | Previous section |
| `l` / `→` | Next section |
| `k` / `↑` | Previous entry |
| `j` / `↓` | Next entry |
| `Enter` | View/hide entry details |
| `:` | Open command palette |
| `Tab` | Switch to GlitchSanctuary |
| `q` | Quit |

#### GlitchSanctuary View
| Key | Action |
|-----|--------|
| `h` / `←` | Previous program |
| `l` / `→` | Next program |
| `k` / `↑` | Previous record |
| `j` / `↓` | Next record |
| `Enter` | Expand/collapse record details |
| `:` | Open command palette |
| `Tab` | Switch to FieldGuide |
| `q` | Quit |

#### Command Palette
| Command | Description |
|---------|-------------|
| `:view field-guide` | Switch to Field Guide |
| `:view sanctuary` | Switch to Glitch Sanctuary |
| `:quit` | Exit application |
| `Ctrl-U` | Clear input |

## Architecture

```
src/
├── main.rs           # Event loop, terminal setup
├── models.rs         # Data structures (Entry, Section, Program, etc.)
├── state.rs          # UI state management for each view
├── input.rs          # Keyboard event handling
├── ui.rs             # Rendering functions (ratatui)
└── data.rs           # Data loading (TOML/JSON)

data/
├── field_guide.toml  # FieldGuide sections and entries
└── sanctuary.toml    # Sanctuary programs and records
```

### Core Concepts

**Models** (`models.rs`): Define the data structure mirroring the React version
- `Entry`: A single pattern with description, signals, protocol
- `Section`: Container for related entries
- `Program`: Container for related records (Sanctuary)
- `Record`: A maintenance/recovery procedure with steps

**State** (`state.rs`): Manages UI selection and navigation state
- `FieldGuideState`: Tracks selected section/entry, active filter
- `SanctuaryState`: Tracks active program/record, expansion state
- `AppState`: Global app state, current view, command input

**Input** (`input.rs`): Processes keyboard events
- Routes to view-specific handlers
- Handles command palette input
- Command processing (view switching, quit, etc.)

**UI** (`ui.rs`): Renders to terminal using ratatui
- Pure rendering functions (no state mutations)
- Layout calculations using ratatui's constraint system
- Color mapping from data to ratatui colors

## Data Format

### Field Guide (TOML)
```toml
[meta]
title = "Evan's Day Field Guide"
version = "1.2"
# ... metadata

[[sections]]
id = "morning-brain-boot"
title = "Morning Brain Boot Sequence"
color = "cyan"
icon = "scan"

[[sections.entries]]
pattern = "Context Archaeology"
description = "Load recent threads/logs..."
signals = ["Post-wake fuzz", "What was I doing?"]
protocol = "Open yesterday's log → scan last 5 anchors..."
```

### Sanctuary (TOML)
```toml
[meta]
title = "Glitch Sanctuary Protocols"
status = "UNSTABLE"

[metrics]
focus = 68
energy = 42

[[programs]]
id = "reset-sequence"
title = "Reset Sequence"
color = "cyan"

[[programs.records]]
id = "rs-001"
name = "Power Cycle Protocol"
description = "Complete system shutdown and restart"
steps = ["Terminate all processes", "Save state", ...]
status = "OPERATIONAL"
```

## Development

### Project Structure
- **Pure functional rendering**: No hidden state in UI layer
- **Separation of concerns**: State, input, and rendering are separate
- **Async-ready**: Uses tokio for future extensibility
- **Idiomatic Rust**: Follows Rust conventions and ownership patterns

### Building
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Adding New Features

1. **New data**: Add to `data/field_guide.toml` or `data/sanctuary.toml`
2. **New view**: Add state struct to `state.rs`, rendering to `ui.rs`, input handling to `input.rs`
3. **New command**: Add to `input.rs::process_command`
4. **New fields**: Update models in `models.rs`, data loading in `data.rs`

## GitHub Issues Workflow

This project uses GitHub Issues to track work with these statuses:

- `planning`: Not started, requires discussion
- `in-progress`: Actively developing
- `self-verify`: Self-verification checklist passed
- `review`: Awaiting code review
- `done`: Completed and verified

See the [Issues](https://github.com/e-schultz/rusty-react-terminal/issues) page for current work.

## Lessons Learned

### Initial Implementation (Commit e3bc610)
- Ratatui 0.28 API is simpler than earlier versions (no generic backend)
- Frame methods: `f.area()` replaces deprecated `f.size()`
- Buffer cell manipulation: `cell_mut()` for accessing cells
- Vim-like keybindings (hjkl) feel natural for TUI navigation
- Simple string-based cursor (`_`) is sufficient for command palette

### Future Improvements
- Add scrollbar support for large lists
- Implement persistent window state
- Add mouse support for clicking
- Cache rendered frames for performance
- Add configuration file support
- Export data to JSON/other formats

## Comparison with React Version

| Feature | React | Rust TUI |
|---------|-------|----------|
| FieldGuide display | ✓ | ✓ |
| Sanctuary display | ✓ | ✓ |
| Navigation | ✓ | ✓ (keyboard) |
| Color coding | ✓ | ✓ |
| Entry details | ✓ | In progress |
| CRT effects | ✓ | Not included (kept simple) |
| Data format | Embedded | TOML files |
| Installation | Browser | Standalone binary |

## Contributing

1. Check existing [issues](https://github.com/e-schultz/rusty-react-terminal/issues)
2. Fork and create a feature branch
3. Implement changes following the architecture
4. Add tests for new functionality
5. Run `cargo fmt && cargo clippy`
6. Open a pull request with detailed description

## References

- [Ratatui Documentation](https://docs.rs/ratatui/)
- [Crossterm](https://docs.rs/crossterm/) - Terminal manipulation
- [Tokio](https://tokio.rs/) - Async runtime
- [Zelij](https://zellij.dev/) - Inspiration for command palette
- [Vim Keybindings](https://www.vim.org/) - Navigation scheme

## License

Personal project. Feel free to use as inspiration for your own TUI tools.

---

**Status**: MVP working. See [Issues](https://github.com/e-schultz/rusty-react-terminal/issues) for planned features and current progress.
