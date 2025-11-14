# Lite XL Editor - Rust Edition

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-planning-yellow.svg)](https://github.com)

> A blazingly fast, lightweight, and extensible text editor built with Rust and Floem. Inspired by Lite XL, designed for developers who value performance, simplicity, and deep customization.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Building from Source](#building-from-source)
- [Usage Guide](#usage-guide)
- [Configuration](#configuration)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [Technology Stack](#technology-stack)
- [Performance](#performance)
- [Comparison with Lite XL](#comparison-with-lite-xl)
- [License](#license)

---

## Overview

Lite XL Editor (Rust Edition) is a next-generation text editor that combines the minimalist philosophy of Lite XL with the performance and safety guarantees of Rust. Built on the modern Floem UI framework, it delivers a smooth 60 FPS editing experience with instant startup times.

### Design Philosophy

1. **Performance First**: Target 60 FPS rendering and instant startup (<100ms)
2. **Type Safety**: Leverage Rust's type system for correctness and reliability
3. **Modularity**: Clear separation of concerns with testable components
4. **Extensibility**: Plugin-ready architecture from day one
5. **Ergonomics**: Pleasant editing experience with minimal friction

### Target Users

- Software developers seeking a lightweight alternative to heavyweight IDEs
- Vim/Emacs users wanting modern UI with familiar workflows
- System administrators needing a fast, reliable text editor
- Anyone who values performance and simplicity over feature bloat

---

## Features

### Phase 1 MVP (Current Focus)

#### Core Editing
- **Single-File Editing**: Robust text buffer management with undo/redo
- **Multi-Cursor Support**: Edit multiple locations simultaneously
- **Smart Selection**: Character, word, and line-based selection modes
- **Clipboard Integration**: Full cut, copy, paste support

#### Text Operations
- **Insert/Delete**: Fast character, word, and line operations
- **Undo/Redo**: Unlimited undo history with intelligent grouping
- **Line Manipulation**: Duplicate, move, join, and split lines
- **Smart Indentation**: Auto-detect tabs vs. spaces with configurable defaults

#### Syntax Highlighting
- **50+ Languages**: Built-in support for common programming languages
- **Real-Time Highlighting**: Incremental parsing as you type
- **Theme Support**: Multiple color schemes with easy customization
- **Performance Optimized**: Maintains 60 FPS even on large files

#### User Interface
- **Clean Design**: Distraction-free editing environment
- **Line Numbers**: Configurable gutter with current line highlighting
- **Status Bar**: File info, cursor position, encoding, and line ending display
- **Smooth Animations**: 60 FPS rendering with configurable transitions

#### File Operations
- **Open/Save**: Fast file I/O with encoding auto-detection
- **Auto-Detection**: Line endings (CRLF/LF) and character encoding
- **File Monitoring**: Detects external changes and prompts to reload
- **Large File Support**: Handles files up to 100MB efficiently

### Future Features (Phase 2+)

- Multi-file editing with tabs and splits
- Fuzzy file finder and project-wide search
- Language Server Protocol (LSP) support
- Integrated terminal
- Git integration
- Plugin system with Lua/WebAssembly support
- Workspace management
- Regex find and replace

---

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────┐
│                     Application                          │
│                    (main.rs, app.rs)                     │
└────────────────────┬────────────────────────────────────┘
                     │
          ┌──────────┼──────────┐
          │          │          │
┌─────────▼─────┐ ┌──▼──────┐ ┌▼─────────┐
│   UI Layer    │ │  Core   │ │  Plugin  │
│   (Floem)     │ │ Engine  │ │  System  │
└───────┬───────┘ └──┬──────┘ └──────────┘
        │            │
        │    ┌───────┴─────────┬───────────┬──────────┐
        │    │                 │           │          │
    ┌───▼────▼───┐  ┌─────────▼──┐  ┌────▼────┐  ┌──▼─────┐
    │  Document  │  │   Buffer   │  │  Syntax │  │  File  │
    │  Manager   │  │  (ropey)   │  │(syntect)│  │   I/O  │
    └────────────┘  └────────────┘  └─────────┘  └────────┘
```

### Core Modules

- **buffer**: Text buffer management using ropey (rope data structure)
- **document**: Document abstraction with buffer + metadata
- **editor**: Editor state and high-level operations
- **syntax**: Syntax highlighting powered by syntect
- **ui**: Floem-based reactive UI components
- **commands**: Command system and keybinding management
- **io**: Async file operations with tokio
- **config**: Configuration management with TOML
- **undo**: Intelligent undo/redo system with grouping
- **selection**: Multi-cursor and selection management

For detailed architecture documentation, see [RUST_EDITOR_MVP_ARCHITECTURE.md](RUST_EDITOR_MVP_ARCHITECTURE.md).

---

## Installation

### Pre-built Binaries (Coming Soon)

Download the latest release for your platform:

- **Linux**: `lite-xl-editor-linux-x86_64.tar.gz`
- **macOS**: `lite-xl-editor-macos.dmg`
- **Windows**: `lite-xl-editor-windows-x86_64.zip`

### Package Managers (Planned)

```bash
# Cargo (Rust package manager)
cargo install lite-xl-editor

# Homebrew (macOS/Linux)
brew install lite-xl-editor

# Chocolatey (Windows)
choco install lite-xl-editor

# AUR (Arch Linux)
yay -S lite-xl-editor
```

---

## Building from Source

### Prerequisites

- **Rust**: 1.75 or later ([Install Rust](https://rustup.rs/))
- **Git**: For cloning the repository
- **C Compiler**: GCC, Clang, or MSVC (for native dependencies)

#### Platform-Specific Dependencies

**Linux:**
```bash
# Debian/Ubuntu
sudo apt install build-essential libfontconfig1-dev libfreetype6-dev

# Fedora
sudo dnf install gcc fontconfig-devel freetype-devel

# Arch Linux
sudo pacman -S base-devel fontconfig freetype2
```

**macOS:**
```bash
# Xcode Command Line Tools
xcode-select --install
```

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
- No additional dependencies needed

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/lite-xl/lite-xl-editor.git
cd lite-xl-editor

# Build in release mode (optimized)
cargo build --release

# Run the editor
cargo run --release

# Install to system (optional)
cargo install --path .
```

### Development Build

```bash
# Build in debug mode (faster compilation, includes debug symbols)
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Build Profiles

The project includes optimized build profiles:

- **dev**: Fast compilation, minimal optimization (for development)
- **release**: Maximum optimization, LTO enabled, stripped binaries
- **Binary size**: ~10-15 MB (release build)

---

## Usage Guide

### Getting Started

1. **Launch the editor**:
   ```bash
   lite-xl-editor [file_or_directory]
   ```

2. **Open a file**: `Ctrl+O` or drag-and-drop onto the window

3. **Start editing**: Just type! The editor auto-saves your session.

4. **Save your work**: `Ctrl+S`

### Basic Workflow

#### Creating a New File
- `Ctrl+N`: Create new empty document
- Start typing immediately
- `Ctrl+S`: Save with a name

#### Editing Text
- Use standard text editing shortcuts
- `Ctrl+Z` / `Ctrl+Y`: Undo/Redo
- `Ctrl+X` / `Ctrl+C` / `Ctrl+V`: Cut/Copy/Paste
- `Ctrl+A`: Select all

#### Multi-Cursor Editing
- `Ctrl+Click`: Add cursor at position
- `Ctrl+D`: Select next occurrence of word
- `Ctrl+Shift+L`: Select all occurrences
- `Ctrl+Shift+Up/Down`: Add cursor above/below

#### Navigation
- Arrow keys: Move cursor
- `Ctrl+Left/Right`: Move by word
- `Home` / `End`: Line start/end
- `Ctrl+Home` / `Ctrl+End`: Document start/end
- `Ctrl+G`: Go to line

#### View Management
- `F11`: Toggle fullscreen
- `Ctrl+ScrollWheel`: Zoom in/out
- `Ctrl+0`: Reset zoom

### Configuration Location

The editor stores configuration in:
- **Linux/macOS**: `~/.config/lite-xl-editor/config.toml`
- **Windows**: `%APPDATA%\lite-xl-editor\config.toml`

---

## Configuration

### Configuration File Format

The editor uses TOML for configuration. Here's a comprehensive example:

```toml
[editor]
tab_width = 4
use_spaces = true
auto_detect_indentation = true
line_ending = "lf"  # or "crlf"
auto_save_interval = 0  # seconds, 0 = disabled
max_file_size_mb = 100
max_undo_history = 10000
undo_group_timeout_ms = 300

[ui]
font_family = "JetBrains Mono"
font_size = 14.0
line_height = 1.4
show_line_numbers = true
highlight_current_line = true
line_length_guide = 80
theme = "monokai"
cursor_blink_rate_ms = 500
scroll_speed = 3.0

[keymap]
preset = "default"  # or "vim", "emacs"

[keymap.custom]
# Add custom keybindings
"ctrl+shift+d" = "duplicate_line"
"ctrl+/" = "toggle_comment"

[languages.rust]
extensions = ["rs"]
tab_width = 4
use_spaces = true

[languages.python]
extensions = ["py"]
tab_width = 4
use_spaces = true

[languages.javascript]
extensions = ["js", "jsx", "mjs"]
tab_width = 2
use_spaces = true
```

### Themes

Built-in themes:
- **Monokai** (default): Popular dark theme with vibrant colors
- **Solarized Dark**: Low-contrast professional theme
- **Solarized Light**: Easy on the eyes for daylight coding
- **Dracula**: Beautiful dark theme with purple accents
- **Nord**: Arctic-inspired color palette

Create custom themes by adding a `.toml` file to `~/.config/lite-xl-editor/themes/`.

### Syntax Highlighting

Customize syntax highlighting colors in your theme file:

```toml
[colors]
background = "#2d2a2e"
foreground = "#fcfcfa"
keyword = "#ff6188"
string = "#ffd866"
comment = "#727072"
function = "#a9dc76"
operator = "#fc9867"
number = "#ab9df2"
```

---

## Keyboard Shortcuts

### File Operations
| Shortcut | Action |
|----------|--------|
| `Ctrl+N` | New file |
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save file |
| `Ctrl+Shift+S` | Save as |
| `Ctrl+W` | Close file |
| `Ctrl+Q` | Quit editor |

### Editing
| Shortcut | Action |
|----------|--------|
| `Ctrl+Z` | Undo |
| `Ctrl+Y` / `Ctrl+Shift+Z` | Redo |
| `Ctrl+X` | Cut |
| `Ctrl+C` | Copy |
| `Ctrl+V` | Paste |
| `Ctrl+A` | Select all |
| `Ctrl+D` | Duplicate line |
| `Ctrl+Shift+K` | Delete line |
| `Alt+Up/Down` | Move line up/down |

### Multi-Cursor
| Shortcut | Action |
|----------|--------|
| `Ctrl+Click` | Add cursor at position |
| `Ctrl+D` | Select next occurrence |
| `Ctrl+Shift+L` | Select all occurrences |
| `Ctrl+Shift+Up` | Add cursor above |
| `Ctrl+Shift+Down` | Add cursor below |
| `Escape` | Cancel multi-cursor |

### Navigation
| Shortcut | Action |
|----------|--------|
| `Ctrl+G` | Go to line |
| `Ctrl+P` | Quick file open (Phase 2) |
| `Ctrl+F` | Find in file |
| `Ctrl+H` | Replace in file |
| `Ctrl+Shift+F` | Find in project (Phase 2) |
| `F3` / `Ctrl+G` | Find next |
| `Shift+F3` | Find previous |

### View
| Shortcut | Action |
|----------|--------|
| `F11` | Toggle fullscreen |
| `Ctrl+ScrollWheel` | Zoom in/out |
| `Ctrl+0` | Reset zoom |
| `Ctrl+B` | Toggle sidebar (Phase 2) |

### Command Palette (Phase 2)
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+P` | Open command palette |

**Note**: Keybindings are fully customizable via configuration file.

---

## Roadmap

### Phase 1: Core MVP (Weeks 1-4) - **In Progress**

**Week 1: Foundation**
- [x] Project setup and dependencies
- [x] Buffer implementation with ropey
- [x] Position/Range types
- [ ] Basic unit tests

**Week 2: Document & Editing**
- [ ] Document abstraction
- [ ] Selection and cursor management
- [ ] Basic editing operations (insert, delete, select)
- [ ] Undo/redo system with intelligent grouping

**Week 3: UI Foundation**
- [ ] Floem integration
- [ ] Basic editor view with text rendering
- [ ] Gutter with line numbers
- [ ] Event handling (keyboard, mouse)

**Week 4: Polish & Integration**
- [ ] Syntax highlighting (syntect integration)
- [ ] Async file I/O (open, save, reload)
- [ ] Keybinding system
- [ ] Status bar
- [ ] Integration tests

**MVP Success Criteria:**
- Edit text files smoothly at 60 FPS
- Undo/redo works correctly for all operations
- Save and load files without data loss
- Syntax highlighting for 50+ languages
- Startup time < 100ms
- Handles files up to 10MB efficiently

### Phase 2: Enhancement (Weeks 5-8)

**Multi-File Support**
- Tab system for multiple open files
- Split views (horizontal/vertical)
- Tab navigation and management
- Drag-and-drop tab reordering

**Advanced Editing**
- Multi-cursor support (unlimited cursors)
- Find and replace with regex
- Column/rectangular selection
- Smart indentation and auto-formatting

**Project Management**
- File tree/sidebar browser
- Fuzzy file finder (`Ctrl+P`)
- Project-wide search (`Ctrl+Shift+F`)
- .gitignore-aware file filtering

**Configuration & Themes**
- TOML-based configuration system
- Multiple color themes
- Per-language settings
- Persistent session management

**Performance Optimization**
- Render caching for improved FPS
- Lazy loading for large files
- Incremental syntax highlighting
- Memory optimization

### Phase 3: Advanced Features (Weeks 9-12)

**Plugin System**
- Plugin architecture foundation
- Lua scripting support
- Plugin API for extending editor
- Plugin manager (install, update, remove)

**Language Server Protocol (LSP)**
- LSP client implementation
- Auto-completion
- Go to definition
- Hover documentation
- Error/warning diagnostics
- Code actions and refactoring

**Advanced Features**
- Integrated terminal
- Git integration (diff, blame, status)
- Snippet system
- Macro recording and playback
- Workspace management
- Code folding
- Minimap overview

**Documentation & Polish**
- User documentation
- API documentation
- Tutorial videos
- Performance benchmarks
- Stability improvements

---

## Contributing

We welcome contributions from the community! Here's how you can help:

### Ways to Contribute

1. **Code Contributions**
   - Implement features from the roadmap
   - Fix bugs and improve performance
   - Add tests and documentation
   - Review pull requests

2. **Bug Reports**
   - Report issues on GitHub
   - Provide detailed reproduction steps
   - Include system information

3. **Feature Requests**
   - Suggest new features or improvements
   - Discuss design decisions
   - Share use cases

4. **Documentation**
   - Improve README and guides
   - Write tutorials and examples
   - Translate documentation

5. **Testing**
   - Test pre-release builds
   - Report compatibility issues
   - Benchmark performance

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**: Write code, add tests, update docs
4. **Run tests**: `cargo test`
5. **Format code**: `cargo fmt`
6. **Lint code**: `cargo clippy`
7. **Commit changes**: `git commit -m 'Add amazing feature'`
8. **Push to branch**: `git push origin feature/amazing-feature`
9. **Open a Pull Request**

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Address all Clippy warnings (`cargo clippy`)
- Write descriptive commit messages
- Add tests for new features
- Document public APIs

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench

# Check code coverage (requires tarpaulin)
cargo tarpaulin
```

### Community Guidelines

- Be respectful and inclusive
- Help others learn and grow
- Give constructive feedback
- Follow the [Code of Conduct](CODE_OF_CONDUCT.md)

---

## Technology Stack

### Core Technologies

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Language** | Rust 2021 | Memory safety, performance, concurrency |
| **UI Framework** | Floem 0.1 | Reactive UI with GPU acceleration |
| **Text Buffer** | ropey 1.6 | Efficient rope data structure for text |
| **Syntax Highlighting** | syntect 5.1 | TextMate grammar-based highlighting |
| **Async Runtime** | tokio 1.x | Non-blocking I/O operations |
| **Configuration** | toml 0.8 | Human-friendly config format |
| **Error Handling** | anyhow/thiserror | Ergonomic error management |

### Key Dependencies

#### UI & Rendering
- **floem**: Modern reactive UI framework with Wgpu backend
- **kurbo**: 2D curve and shape library
- **peniko**: Rendering primitives and styles

#### Text Processing
- **ropey**: Fast, robust rope data structure
- **unicode-segmentation**: Unicode text segmentation
- **encoding_rs**: Character encoding detection/conversion

#### Syntax & Languages
- **syntect**: Syntax highlighting engine
- **tree-sitter**: Incremental parsing (planned for Phase 2)
- **regex**: Regular expression support

#### File & System
- **tokio**: Async file I/O and runtime
- **notify**: Cross-platform file system watcher
- **memmap2**: Memory-mapped file I/O for large files
- **clipboard**: System clipboard integration

#### Utilities
- **serde**: Serialization/deserialization
- **tracing**: Structured logging and diagnostics
- **crossbeam**: Lock-free concurrent data structures

### Why Rust?

**Performance Benefits:**
- Zero-cost abstractions
- No garbage collection pauses
- Predictable memory usage
- Efficient CPU cache utilization
- SIMD optimization opportunities

**Safety Benefits:**
- Memory safety without garbage collection
- Thread safety enforced at compile time
- No null pointer exceptions
- No data races
- Strong type system catches bugs early

**Developer Experience:**
- Modern tooling (cargo, rustfmt, clippy)
- Excellent documentation
- Active community
- Great IDE support
- First-class testing framework

### Why Floem?

- **Modern Architecture**: Built on reactive principles with fine-grained updates
- **GPU Accelerated**: Uses Wgpu for smooth 60 FPS rendering
- **Lightweight**: Small runtime overhead
- **Flexible**: Easy to create custom components
- **Rust Native**: No FFI overhead, type-safe APIs
- **Cross-Platform**: Windows, macOS, Linux support

---

## Performance

### Design Targets

The editor is designed with strict performance goals:

| Metric | Target | Status |
|--------|--------|--------|
| **Startup Time (cold)** | < 100ms | Planned |
| **Startup Time (warm)** | < 50ms | Planned |
| **Frame Rate** | 60 FPS | In Progress |
| **Keystroke Latency** | < 5ms | Planned |
| **Binary Size** | < 20MB | On Track |
| **Memory (idle)** | < 30MB | Planned |
| **Memory (10 files)** | < 100MB | Planned |

### Performance Characteristics

#### Startup Performance
- **Fast Binary Loading**: Optimized release builds with LTO
- **Lazy Initialization**: Only load what's needed
- **Minimal Dependencies**: Reduce initialization overhead
- **Incremental Loading**: Async plugin and theme loading

#### Runtime Performance
- **60 FPS Rendering**: Smooth animations and scrolling
- **Incremental Rendering**: Only redraw changed regions
- **Syntax Caching**: Cache tokenization results
- **Viewport Culling**: Only render visible lines
- **GPU Acceleration**: Hardware-accelerated rendering via Wgpu

#### Text Operations
- **O(log n) Edits**: Rope structure for efficient insertions/deletions
- **O(1) Line Access**: Fast line indexing
- **Lazy Line Breaks**: Compute line breaks on demand
- **Efficient Undo**: Delta-based undo storage

#### File Operations
- **Async I/O**: Non-blocking file operations
- **Memory Mapping**: Efficient large file handling
- **Streaming**: Process large files in chunks
- **Background Loading**: Load files without blocking UI

### Optimization Strategies

1. **Data Structures**: Rope for text, Vec for small sequences
2. **Caching**: Line layouts, syntax highlighting, font metrics
3. **Lazy Evaluation**: Compute only what's visible
4. **Memory Pooling**: Reuse allocations where possible
5. **SIMD**: Vectorized operations for text processing
6. **Profiling**: Continuous performance monitoring

### Benchmarks (Planned)

```bash
# Run performance benchmarks
cargo bench

# Profile performance
cargo flamegraph
```

We maintain a [performance dashboard](docs/benchmarks.md) tracking key metrics over time.

---

## Comparison with Lite XL

### Overview

This project is inspired by Lite XL but takes a different approach using Rust and modern technologies. Here's a detailed comparison:

### Feature Comparison

| Feature | Lite XL (Lua/C) | Rust Edition | Notes |
|---------|----------------|--------------|-------|
| **Language** | Lua + C | Rust | Rust offers memory safety |
| **Startup Time** | ~50-150ms | ~50-100ms (target) | Comparable |
| **Runtime Performance** | 60 FPS | 60 FPS | Similar target |
| **Memory Usage** | ~20-50MB | ~30-50MB (target) | Slightly higher due to Rust runtime |
| **Binary Size** | ~10MB | ~15-20MB | Rust binaries larger but still small |
| **Plugin System** | Lua (mature) | Planned (Lua/WASM) | Lite XL has advantage here |
| **Type Safety** | Dynamic | Static | Rust provides compile-time guarantees |
| **Thread Safety** | Manual | Compiler-enforced | Rust prevents data races |
| **Cross-Platform** | Yes | Yes | Both support major platforms |
| **LSP Support** | Via plugins | Planned (native) | Future feature |
| **Multi-File Editing** | Yes | Planned (Phase 2) | Lite XL is more mature |
| **Customization** | Excellent | Good (improving) | Lite XL has 100+ plugins |

### Architecture Differences

**Lite XL:**
- C backend for rendering and system operations
- Lua for application logic and plugins
- SDL2 for windowing and input
- Custom renderer with FreeType

**Rust Edition:**
- Pure Rust with minimal C dependencies
- Floem for UI (Wgpu backend)
- Modern reactive architecture
- Type-safe plugin system (planned)

### Strengths of Each Approach

**Lite XL Strengths:**
- Mature plugin ecosystem (100+ plugins)
- Battle-tested stability
- Lower resource usage
- Faster development via Lua
- Simple, proven architecture
- Active community

**Rust Edition Strengths:**
- Memory safety without garbage collection
- Thread safety guarantees
- Modern UI framework (Floem)
- Better error handling
- Stronger type system
- Future-proof architecture
- Potential for better performance

### When to Choose Each

**Choose Lite XL if you:**
- Need a mature, stable editor today
- Want extensive plugin ecosystem
- Prefer Lua for customization
- Need minimal resource usage
- Want proven reliability

**Choose Rust Edition if you:**
- Want cutting-edge technology
- Value type safety and memory safety
- Are willing to contribute to development
- Want to learn Rust
- Need thread-safe architecture
- Want to be part of early community

### Compatibility

**Goal**: Maintain conceptual compatibility with Lite XL where practical:
- Similar keybindings (customizable)
- Compatible configuration concepts
- Similar plugin architecture (future)
- Shared philosophy (lightweight, fast, extensible)

**Not Goals**:
- Binary compatibility
- Direct Lua plugin compatibility (may support via bridge)
- Identical UI/UX

### Migration Path

For Lite XL users wanting to try Rust Edition:
1. Similar keyboard shortcuts work out of the box
2. Configuration can be ported to TOML format
3. Plugins will need to be rewritten (Lua/WASM support planned)
4. Themes can be adapted with minimal changes

---

## License

This project is licensed under the **MIT License** - see below for details.

```
MIT License

Copyright (c) 2025 Lite XL Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Third-Party Licenses

This project uses several open-source libraries with various licenses:

- **Floem**: MIT License
- **ropey**: MIT License
- **syntect**: MIT License
- **tokio**: MIT License
- **serde**: MIT or Apache-2.0

See [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md) for complete license information.

---

## Acknowledgments

### Inspiration
- **Lite XL**: The original lightweight editor that inspired this project
- **rxi's lite**: The precursor to Lite XL
- **Xi Editor**: Pioneering work in Rust text editors
- **Helix**: Modern terminal-based editor with great architecture

### Contributors

This project is made possible by contributors who dedicate their time and expertise. See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the full list.

### Special Thanks
- The Rust community for excellent tools and libraries
- Floem developers for the modern UI framework
- All beta testers and early adopters

---

## Community & Support

### Get Help

- **Documentation**: [docs/](docs/)
- **GitHub Issues**: [Report bugs or request features](https://github.com/lite-xl/lite-xl-editor/issues)
- **GitHub Discussions**: [Ask questions, share ideas](https://github.com/lite-xl/lite-xl-editor/discussions)
- **Discord**: [Join our community](https://discord.gg/lite-xl) (Coming soon)

### Stay Updated

- **GitHub**: Star and watch the repository
- **Twitter**: Follow [@litexleditor](https://twitter.com/litexleditor) (Coming soon)
- **Blog**: Read development updates at [blog.lite-xl-editor.org](https://blog.lite-xl-editor.org) (Coming soon)

### Related Projects

- **Lite XL**: Original Lua-based editor - [github.com/lite-xl/lite-xl](https://github.com/lite-xl/lite-xl)
- **Floem**: UI framework powering this editor - [github.com/lapce/floem](https://github.com/lapce/floem)
- **Lapce**: Fast Rust-based code editor - [github.com/lapce/lapce](https://github.com/lapce/lapce)
- **Helix**: Terminal-based modal editor - [github.com/helix-editor/helix](https://github.com/helix-editor/helix)

---

## Project Status

**Current Phase**: Phase 1 MVP (Foundation) - **In Active Development**

This is an early-stage project under active development. The editor is not yet ready for daily use but welcomes contributors and early testers.

### Quick Links
- [Architecture Documentation](RUST_EDITOR_MVP_ARCHITECTURE.md)
- [Lite XL Specification](LITE_XL_COMPREHENSIVE_SPECIFICATION.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Development Roadmap](ROADMAP.md)

---

**Built with ❤️ and Rust**
