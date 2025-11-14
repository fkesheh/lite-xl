# Lite XL Document Module Implementation

## Overview

This document describes the comprehensive implementation of the document module system for Lite XL. The implementation provides a complete, production-ready text editing foundation with buffer management, multi-cursor support, and undo/redo functionality.

## Architecture

The system is organized into four main modules:

### 1. Buffer Module (`src/buffer/`)

The buffer module provides low-level text storage and manipulation using the ropey rope data structure.

#### Files:
- **`mod.rs`**: Core buffer implementation with rope-based text storage
- **`position.rs`**: Position and Range types for text coordinates

#### Key Features:
- Efficient O(log n) insertion and deletion
- Position ↔ offset conversion
- Line-based operations
- Automatic line ending detection (LF, CRLF, CR)
- UTF-8 support
- Modification tracking
- Buffer versioning

#### API Examples:

```rust
use lite_xl::buffer::{Buffer, Position, Range};

let mut buffer = Buffer::from_str("Hello, world!");
buffer.insert(Position::new(0, 7), "beautiful ").unwrap();
assert_eq!(buffer.to_string(), "Hello, beautiful world!");

let slice = buffer.slice(Range::new(
    Position::new(0, 0),
    Position::new(0, 5)
)).unwrap();
assert_eq!(slice, "Hello");
```

### 2. Selection Module (`src/document/selection.rs`)

Implements multi-cursor selection system with full support for simultaneous editing.

#### Key Features:
- Zero-width selections (cursors)
- Multi-selection management
- Anchor and cursor tracking
- Selection normalization (sorting and merging)
- Overlap detection
- Primary selection designation

#### API Examples:

```rust
use lite_xl::document::{Selection, Selections};
use lite_xl::buffer::Position;

// Single cursor
let cursor = Selection::cursor(Position::new(5, 10));

// Multi-cursor setup
let mut selections = Selections::from_vec(vec![
    Selection::cursor(Position::new(0, 0)),
    Selection::cursor(Position::new(1, 0)),
    Selection::cursor(Position::new(2, 0)),
]);

// Normalize (sort and merge overlapping selections)
selections.normalize();
```

### 3. Undo Module (`src/undo/`)

Provides sophisticated undo/redo with time-based grouping.

#### Key Features:
- Time-based edit grouping (configurable timeout)
- Manual group boundaries
- Selection state restoration
- Configurable history limits
- Memory-efficient storage
- Edit inversion for undo

#### API Examples:

```rust
use lite_xl::undo::{UndoStack, UndoConfig, Edit};
use lite_xl::buffer::{Buffer, Position};
use lite_xl::document::Selections;

let mut stack = UndoStack::new(UndoConfig::default());
let mut buffer = Buffer::new();
let selections = Selections::single(Position::zero());

let edit = Edit::Insert {
    position: Position::zero(),
    text: "Hello".to_string(),
};
edit.apply(&mut buffer).unwrap();
stack.push(edit, selections);

// Undo
stack.undo(&mut buffer);
```

### 4. Document Module (`src/document/`)

High-level document abstraction combining all components.

#### Files:
- **`mod.rs`**: Main document implementation
- **`selection.rs`**: Selection system (described above)

#### Key Features:
- Combines buffer, selections, and undo stack
- Multi-cursor editing operations
- Movement commands (Left, Right, Up, Down, etc.)
- Document-specific settings
- File association and persistence
- Modification tracking
- Auto-save support configuration
- Line ending handling

#### API Examples:

```rust
use lite_xl::document::{Document, Movement};
use lite_xl::buffer::Position;

let mut doc = Document::new();

// Edit
doc.insert("Hello, world!");

// Navigate
doc.move_cursor(Movement::Right);
doc.move_cursor(Movement::Right);

// Extend selection
doc.select(Movement::WordRight);

// Undo/Redo
doc.undo();
doc.redo();

// Save
doc.save_as("example.txt")?;
```

## Implementation Details

### Position System

All positions are **0-indexed** with two coordinates:
- **line**: Line number (0-based)
- **column**: Character offset within line (NOT byte offset)

```rust
pub struct Position {
    pub line: usize,
    pub column: usize,
}
```

### Range System

Ranges represent text regions with start (inclusive) and end (exclusive) positions:

```rust
pub struct Range {
    pub start: Position,
    pub end: Position,
}
```

Ranges are automatically normalized (start ≤ end).

### Selection System

Each selection has:
- **anchor**: Fixed point where selection started
- **cursor**: Moving end point (head)

When anchor == cursor, it's a zero-width selection (cursor).

### Multi-Cursor Editing

The `Selections` type manages multiple cursors:
- Maintains list of selections
- Tracks primary selection
- Provides normalization (sorting and merging)
- Supports transformation operations

### Undo Grouping

Edits are automatically grouped based on:
- **Time proximity**: Configurable timeout (default 300ms)
- **Edit continuity**: Similar edits grouped together

Manual group boundaries can be set with `begin_group()` and `end_group()`.

### Document Settings

Each document can have custom settings:

```rust
pub struct DocumentSettings {
    pub tab_width: usize,
    pub use_spaces: bool,
    pub auto_indent: bool,
    pub show_line_numbers: bool,
    pub highlight_current_line: bool,
    pub line_length_guide: Option<usize>,
    pub trim_trailing_whitespace: bool,
    pub ensure_final_newline: bool,
}
```

## Testing

### Unit Tests

Each module includes comprehensive unit tests:

- **Buffer tests**: `/src/buffer/mod.rs` - 11 tests
- **Position tests**: `/src/buffer/position.rs` - 9 tests
- **Selection tests**: `/src/document/selection.rs` - 10 tests
- **Undo tests**: `/src/undo/mod.rs` - 8 tests
- **Document tests**: `/src/document/mod.rs` - 10 tests

### Integration Tests

Integration tests verify all components work together:
- `/tests/integration_tests.rs` - 16 comprehensive integration tests

### Running Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test buffer::tests
cargo test document::tests
cargo test undo::tests

# Run integration tests
cargo test --test integration_tests
```

### Example Usage

```bash
# Run the basic usage example
cargo run --example basic_usage
```

## Performance Characteristics

### Buffer Operations
- **Insert**: O(log n) amortized
- **Delete**: O(log n) amortized
- **Position lookup**: O(log n)
- **Line access**: O(log n)

### Selection Operations
- **Add selection**: O(1)
- **Normalize**: O(n log n) where n = number of selections
- **Transform all**: O(n)

### Undo Operations
- **Push**: O(1) amortized
- **Undo/Redo**: O(m) where m = edits in group

## Memory Usage

- **Buffer**: Rope overhead + text size (~1.2x text size)
- **Undo stack**: Configurable (default: 10,000 groups)
- **Selections**: O(n) where n = number of cursors

## File Structure

```
src/
├── buffer/
│   ├── mod.rs              # Buffer implementation (800 lines)
│   └── position.rs         # Position/Range types (350 lines)
├── document/
│   ├── mod.rs              # Document implementation (800 lines)
│   └── selection.rs        # Selection system (550 lines)
├── undo/
│   └── mod.rs              # Undo/redo system (550 lines)
├── lib.rs                  # Public API (150 lines)
└── main.rs                 # Binary placeholder

tests/
└── integration_tests.rs    # Integration tests (400 lines)

examples/
└── basic_usage.rs          # Usage examples
```

## API Documentation

All public APIs include comprehensive documentation with:
- Function descriptions
- Parameter explanations
- Return value descriptions
- Usage examples
- Error conditions

Generate API docs with:
```bash
cargo doc --no-deps --open
```

## Production Readiness

### Completed Features
✅ Rope-based buffer with O(log n) operations
✅ Multi-cursor editing
✅ Undo/redo with time-based grouping
✅ Selection management and normalization
✅ Position/offset conversion
✅ Line ending detection and handling
✅ UTF-8 support
✅ Modification tracking
✅ Comprehensive error handling
✅ Extensive documentation
✅ Unit and integration tests
✅ Examples

### Not Yet Implemented (Future Work)
- Syntax highlighting integration
- Language server protocol
- File watching
- Async file I/O
- Advanced word movement
- Search and replace
- UI integration

## Error Handling

All operations use Result types with custom error enums:

```rust
pub enum BufferError {
    InvalidPosition(Position),
    InvalidRange(Range),
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}

pub enum DocumentError {
    Buffer(BufferError),
    Io(std::io::Error),
    NoPath,
    UnsavedChanges,
}
```

## Thread Safety

- **Buffer**: Not thread-safe (use external synchronization)
- **Document**: Not thread-safe (use external synchronization)
- **Undo Stack**: Not thread-safe (use external synchronization)

For multi-threaded use, wrap in `Arc<Mutex<T>>` or `Arc<RwLock<T>>`.

## Configuration

### Undo Configuration

```rust
pub struct UndoConfig {
    pub max_groups: usize,            // Default: 10,000
    pub group_timeout: Duration,       // Default: 300ms
    pub min_group_interval: Duration,  // Default: 50ms
}
```

### Document Settings

Customizable per-document:
- Tab width
- Spaces vs tabs
- Auto-indentation
- Line numbers
- Current line highlight
- Line length guide
- Whitespace trimming
- Final newline

## Usage Patterns

### Basic Editing

```rust
let mut doc = Document::new();
doc.insert("Hello");
doc.move_cursor(Movement::LineEnd);
doc.insert(" World");
```

### Multi-Cursor

```rust
let mut doc = Document::from_str("a\nb\nc");
doc.set_selections(Selections::from_vec(vec![
    Selection::cursor(Position::new(0, 0)),
    Selection::cursor(Position::new(1, 0)),
    Selection::cursor(Position::new(2, 0)),
]));
doc.insert("> ");
```

### Selection and Deletion

```rust
doc.set_selections(Selections::from_selection(
    Selection::new(start, end)
));
doc.delete();
```

### File Operations

```rust
let doc = Document::from_file("example.txt")?;
// Edit...
doc.save()?;
// Or save as:
doc.save_as("new_file.txt")?;
```

## Next Steps

To integrate with the rest of Lite XL:

1. **UI Integration**: Connect to Floem rendering system
2. **Event Handling**: Wire up keyboard/mouse events
3. **Syntax Highlighting**: Integrate with syntect
4. **LSP**: Add language server protocol support
5. **Commands**: Implement command palette
6. **Settings**: Add global editor configuration

## License

MIT (as per project license)

## Contributors

Part of the Lite XL project.
