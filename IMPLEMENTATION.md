# Implementation: File I/O, Syntax Highlighting, and Configuration

This document describes the implementation of the core modules for the Rust text editor.

## Overview

The following modules have been implemented:

1. **`src/io/mod.rs`** - File I/O operations with async support
2. **`src/io/file_watcher.rs`** - File system watching
3. **`src/syntax/mod.rs`** - Syntax highlighting engine
4. **`src/syntax/languages.rs`** - Language detection and definitions
5. **`src/config/mod.rs`** - Configuration management

## Module Details

### 1. File I/O Module (`src/io/mod.rs`)

Provides comprehensive async file operations with robust error handling.

#### Features:
- **Async file reading and writing** using Tokio
- **Encoding detection** (UTF-8, UTF-8 BOM, UTF-16 LE/BE, Latin-1, with fallback)
- **Line ending detection** (LF, CRLF, CR) and normalization
- **File size limits** (100 MB default) with configurable thresholds
- **BOM handling** for UTF-8/UTF-16 files
- **Utility functions** for file operations

#### Key Types:
- `FileReader` - Async file reader with encoding detection
- `FileWriter` - Async file writer with encoding/line ending control
- `FileUtils` - Utility functions (backup, text detection, etc.)
- `DetectedEncoding` - Enum for supported encodings
- `LineEnding` - Enum for line ending styles
- `FileContent` - Struct containing file text and metadata

#### Example Usage:
```rust
use lite_xl::{FileReader, FileWriter, DetectedEncoding, LineEnding};

// Read a file with automatic encoding detection
let content = FileReader::read_file("file.txt").await?;
println!("Detected: {:?}, {}", content.encoding, content.line_ending);

// Write with specific settings
FileWriter::write_file(
    "output.txt",
    "Hello, World!",
    DetectedEncoding::Utf8,
    LineEnding::Lf,
    false,
).await?;
```

#### Error Handling:
- `IoError::FileNotFound` - File doesn't exist
- `IoError::PermissionDenied` - No read/write permission
- `IoError::FileTooLarge` - File exceeds size limit
- `IoError::EncodingError` - Invalid encoding
- `IoError::Io` - Standard IO error

### 2. File Watcher Module (`src/io/file_watcher.rs`)

Monitors file system changes using the `notify` crate with debouncing.

#### Features:
- **Debounced notifications** (500ms default) to reduce event spam
- **Multiple file watching** with individual tracking
- **Event filtering** by extension and hidden files
- **Async event delivery** via Tokio channels
- **Recursive directory watching** (optional)

#### Key Types:
- `FileWatcher` - Main watcher with event queue
- `FileSystemEvent` - Enum for file events (Modified, Created, Deleted, Renamed)
- `WatcherConfig` - Configuration for watcher behavior
- `FileWatcherBuilder` - Fluent API for creating watchers

#### Example Usage:
```rust
use lite_xl::{FileWatcher, FileSystemEvent};

let mut watcher = FileWatcher::new_default()?;
watcher.watch("file.txt")?;

while let Some(event) = watcher.next_event().await {
    match event {
        FileSystemEvent::Modified(path) => {
            println!("File changed: {}", path.display());
        }
        _ => {}
    }
}
```

#### Builder Pattern:
```rust
let watcher = FileWatcherBuilder::new()
    .debounce_duration(Duration::from_millis(300))
    .extensions(vec!["rs".to_string(), "toml".to_string()])
    .ignore_hidden(true)
    .build()?;
```

### 3. Syntax Highlighting Module (`src/syntax/mod.rs`)

Provides syntax highlighting using `syntect` with incremental caching.

#### Features:
- **Multiple language support** via syntect's default syntax set
- **Theme support** with all syntect default themes
- **Incremental highlighting** with per-line caching
- **Background color support** from theme
- **Fallback to plain text** for unknown languages

#### Key Types:
- `SyntaxHighlighter` - Main highlighter with state caching
- `PlainTextHighlighter` - Fallback for plain text
- `HighlightedSpan` - Colored text segment with style
- `HighlightStyle` - Font style flags (bold, italic, underline)
- `HighlighterFactory` - Factory for creating highlighters

#### Example Usage:
```rust
use lite_xl::SyntaxHighlighter;

let highlighter = SyntaxHighlighter::new("Rust")?;

let code = "fn main() { println!(\"Hello\"); }";
let spans = highlighter.highlight_line(0, code);

for span in spans {
    println!("Text: '{}', Color: {:?}", span.text, span.fg_color);
}
```

#### Cache Management:
```rust
// Invalidate cache from line 10 onwards
highlighter.invalidate_from_line(10);

// Clear all cached highlights
highlighter.clear_cache();
```

#### Theme Management:
```rust
// List available themes
let themes = SyntaxHighlighter::available_themes();

// Change theme
highlighter.set_theme("Solarized (dark)")?;

// Get theme colors
let bg = highlighter.background_color();
let fg = highlighter.foreground_color();
```

### 4. Language Detection Module (`src/syntax/languages.rs`)

Detects programming languages from file paths and content.

#### Features:
- **Extension-based detection** (e.g., `.rs` → Rust)
- **Filename-based detection** (e.g., `Cargo.toml` → Rust)
- **Shebang detection** from file content (e.g., `#!/usr/bin/env python`)
- **Pattern-based detection** using language-specific patterns
- **Language configuration** with default settings per language

#### Supported Languages:
- Rust
- Python
- JavaScript / TypeScript
- Markdown
- JSON
- TOML
- YAML
- HTML
- CSS
- Plain Text

#### Key Types:
- `Language` - Enum for supported languages
- `LanguageDetector` - Static methods for detection
- `LanguageConfig` - Per-language settings (tab width, indentation, etc.)
- `CommentStyle` - Comment syntax for each language

#### Example Usage:
```rust
use lite_xl::{LanguageDetector, Language};
use std::path::PathBuf;

// Detect from path
let path = PathBuf::from("main.rs");
let lang = LanguageDetector::detect_from_path(&path);
assert_eq!(lang, Some(Language::Rust));

// Detect from content
let code = "fn main() { println!(\"Hello\"); }";
let lang = LanguageDetector::detect_from_content(code);
assert_eq!(lang, Some(Language::Rust));

// Get language by name
let lang = LanguageDetector::get_by_name("rust");
```

#### Language Information:
```rust
let lang = Language::Rust;
println!("Name: {}", lang.name());
println!("Extensions: {:?}", lang.extensions());
println!("Comment style: {:?}", lang.comment_style());
```

### 5. Configuration Module (`src/config/mod.rs`)

TOML-based configuration with defaults and validation.

#### Features:
- **TOML format** for human-friendly configuration
- **Default values** for all settings
- **Async loading/saving** with Tokio
- **Configuration merging** for user overrides
- **Validation** to catch invalid values
- **Per-language settings** override

#### Configuration Sections:
1. **Editor** - Tab width, line endings, auto-save, undo settings
2. **UI** - Font, theme, line numbers, scroll speed
3. **Keymap** - Keybinding preset and custom bindings
4. **Languages** - Per-language overrides

#### Key Types:
- `Config` - Root configuration struct
- `EditorConfig` - Editor-specific settings
- `UiConfig` - UI-specific settings
- `KeymapConfig` - Keybinding configuration
- `LanguageConfig` - Language-specific overrides

#### Example Usage:
```rust
use lite_xl::Config;

// Load from default location (~/.config/rust-editor/config.toml)
let config = Config::load_default().await?;

// Or load from specific path
let config = Config::load("custom.toml").await?;

// Access settings
println!("Tab width: {}", config.editor.tab_width);
println!("Theme: {}", config.ui.theme);

// Modify and save
let mut config = Config::default();
config.editor.tab_width = 2;
config.save_default().await?;
```

#### Default Configuration Locations:
- Linux: `~/.config/rust-editor/config.toml`
- macOS: `~/Library/Application Support/rust-editor/config.toml`
- Windows: `%APPDATA%\rust-editor\config.toml`

#### Validation:
```rust
// Automatic validation on load
let config = Config::load("config.toml").await?; // Returns error if invalid

// Manual validation
config.editor.validate()?;
config.ui.validate()?;
```

## Dependencies

The implementation uses the following crates:

```toml
[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"

# File I/O
encoding_rs = "0.8"      # Character encoding detection
memmap2 = "0.9"          # Memory-mapped file I/O

# File watching
notify = "6.1"                      # File system notifications
notify-debouncer-full = "0.3"      # Debounced notifications

# Syntax highlighting
syntect = "5.1"          # Syntax highlighting engine

# Configuration
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"             # TOML parsing
dirs = "5.0"             # Standard directories

# Error handling
thiserror = "1.0"        # Custom error types
anyhow = "1.0"           # Error context

# Utilities
once_cell = "1.19"       # Lazy static initialization
```

## Testing

Each module includes comprehensive unit tests:

### Run All Tests:
```bash
cargo test
```

### Run Specific Module Tests:
```bash
cargo test io::tests
cargo test syntax::tests
cargo test config::tests
```

### Test Coverage:
- File I/O: Line ending detection, encoding detection, read/write operations
- File Watcher: Event detection, filtering, debouncing
- Syntax Highlighting: Language detection, highlighting, caching
- Language Detection: Path-based, content-based, shebang detection
- Configuration: Loading, saving, validation, merging

## Examples

The `examples/` directory contains demonstrations of each module:

### Run Examples:
```bash
# File I/O
cargo run --example file_io_demo

# Syntax highlighting
cargo run --example syntax_demo

# Configuration
cargo run --example config_demo

# File watching (runs for 10 seconds)
cargo run --example file_watcher_demo
```

## Error Handling

All modules use `thiserror` for custom error types with descriptive messages:

- `IoError` - File I/O errors
- `WatchError` - File watching errors
- `SyntaxError` - Syntax highlighting errors
- `ConfigError` - Configuration errors

All errors implement `std::error::Error` and can be used with `?` operator and `Result` types.

## Performance Considerations

### File I/O:
- **Streaming**: Large files can use memory-mapped I/O (not yet implemented)
- **Buffer size**: 8KB sample for encoding detection
- **Max file size**: 100 MB default limit (configurable)

### File Watching:
- **Debouncing**: 500ms default to reduce event spam
- **Filtering**: Early filtering by extension/hidden files

### Syntax Highlighting:
- **Caching**: Per-line highlight cache for fast re-renders
- **Incremental**: Only highlight visible lines
- **Lazy loading**: Syntaxes loaded on-demand

### Configuration:
- **Async I/O**: Non-blocking load/save
- **Validation**: Early validation prevents runtime errors

## Future Enhancements

Potential improvements for future versions:

1. **File I/O**:
   - Memory-mapped files for very large files
   - Streaming read/write for files > 100 MB
   - More encoding support (e.g., Big5, Shift-JIS)
   - Atomic file writes with temp files

2. **File Watching**:
   - Pattern-based watching (glob patterns)
   - Directory tree watching
   - Event batching for multiple rapid changes

3. **Syntax Highlighting**:
   - Tree-sitter integration for incremental parsing
   - Custom theme loading from files
   - Semantic highlighting via LSP
   - Background highlighting worker thread

4. **Language Detection**:
   - More language support
   - Content-based scoring (weighted heuristics)
   - Modeline detection (vim/emacs)
   - First-line magic comments

5. **Configuration**:
   - Hot reloading on file change
   - JSON schema for validation
   - Environment variable overrides
   - Project-specific configurations (.editorconfig)

## Architecture Integration

These modules integrate with the overall editor architecture:

```
┌─────────────────────────────────────┐
│        Editor Core                   │
│   (document, buffer, undo)          │
└────────────┬────────────────────────┘
             │
     ┌───────┼────────┬────────────┐
     │       │        │            │
┌────▼────┐ │  ┌─────▼─────┐ ┌───▼────┐
│  File   │ │  │  Syntax   │ │ Config │
│   I/O   │ │  │Highlight  │ │        │
└─────────┘ │  └───────────┘ └────────┘
     │      │
┌────▼──────▼───┐
│ File Watcher  │
└───────────────┘
```

## Conclusion

The implemented modules provide a solid foundation for:
- Robust file operations with encoding detection
- Real-time file change monitoring
- Fast syntax highlighting with multiple languages
- Flexible configuration management

All modules are well-tested, documented, and follow Rust best practices with comprehensive error handling.
