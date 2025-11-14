# Buffer Module

The buffer module provides a production-ready text buffer implementation using the [ropey](https://crates.io/crates/ropey) crate for efficient text storage and manipulation.

## Architecture

The module is organized into three main files:

### 1. `position.rs` - Position and Range Types

Defines the fundamental types for representing locations and regions in a text buffer:

- **`Position`**: A (line, column) coordinate in the buffer (0-indexed)
  - Both line and column are 0-indexed
  - Column represents character offset, not byte offset
  - Supports ordering, comparison, and offset operations

- **`Range`**: A half-open range from start to end position
  - Start position is inclusive, end position is exclusive
  - Automatically normalized so start ≤ end
  - Supports containment checks, overlaps, union, and intersection

#### Examples

```rust
use lite_xl::buffer::{Position, Range};

// Create positions
let pos1 = Position::new(5, 10);
let pos2 = Position::new(8, 20);

// Position operations
assert!(pos1 < pos2);
let line_start = pos1.line_start();
let next = pos1.next_line();

// Create ranges
let range = Range::new(pos1, pos2);
assert!(!range.is_empty());
assert!(range.contains(Position::new(6, 0)));

// Range operations
let range2 = Range::new(Position::new(7, 0), Position::new(10, 0));
assert!(range.overlaps(range2));
let union = range.union(range2);
```

### 2. `line_ending.rs` - Line Ending Detection and Conversion

Handles different line ending styles across operating systems:

- **`LineEnding`** enum: `Lf` (Unix), `CrLf` (Windows), `Cr` (Classic Mac)
- **Line ending detection**: Automatically detects the most common line ending in text
- **Normalization**: Converts all line endings to a target style
- **Statistics**: Provides detailed statistics about line ending usage
- **Utilities**: Line counting, splitting with endings preserved

#### Examples

```rust
use lite_xl::buffer::{LineEnding, detect_line_ending, normalize_line_endings};

// Detect line endings
let unix_text = "line1\nline2\nline3";
assert_eq!(detect_line_ending(unix_text), LineEnding::Lf);

let windows_text = "line1\r\nline2\r\nline3";
assert_eq!(detect_line_ending(windows_text), LineEnding::CrLf);

// Normalize line endings
let mixed = "line1\nline2\r\nline3\rline4";
let normalized = normalize_line_endings(mixed, LineEnding::Lf);
assert_eq!(normalized, "line1\nline2\nline3\nline4");

// Get detailed statistics
let (ending, stats) = detect_line_ending_with_stats("line1\nline2\r\n");
assert_eq!(stats.lf, 1);
assert_eq!(stats.crlf, 1);
assert!(!stats.is_consistent());
```

### 3. `mod.rs` - Main Buffer Implementation

The core `Buffer` struct wraps `ropey::Rope` and provides:

#### Features

- **Efficient Operations**: O(log n) complexity for insert, delete, and query operations
- **Line-based Access**: Get individual lines with or without line endings
- **Character-based Access**: Get characters at specific positions
- **Position/Offset Conversion**: Convert between (line, column) positions and byte offsets
- **Text Manipulation**: Insert, delete, replace, and slice operations
- **File I/O**: Load from and save to files (both sync and async)
- **Iterators**: Iterate over lines or characters
- **Version Tracking**: Each modification increments a version counter
- **Modification State**: Track whether buffer has been modified since last save
- **Line Ending Support**: Auto-detect and normalize line endings

#### Buffer Operations

```rust
use lite_xl::buffer::{Buffer, Position, Range};

// Create buffers
let mut buffer = Buffer::new();
let buffer2 = Buffer::from_str("Hello\nWorld");

// Insert text
buffer.insert(Position::zero(), "Hello, world!").unwrap();

// Query buffer
assert_eq!(buffer.line_count(), 1);
assert_eq!(buffer.len_chars(), 13);
assert_eq!(buffer.char_at(Position::new(0, 0)), Some('H'));

// Delete text
let range = Range::new(Position::new(0, 5), Position::new(0, 7));
let deleted = buffer.delete(range).unwrap();
assert_eq!(deleted, ", ");

// Replace text
let range = Range::new(Position::new(0, 7), Position::new(0, 12));
buffer.replace(range, "Rust").unwrap();

// Slice text
let range = Range::new(Position::new(0, 0), Position::new(0, 5));
let slice = buffer.slice(range).unwrap();
```

#### File Operations

```rust
use lite_xl::buffer::Buffer;
use std::path::Path;

// Load from file (async)
let buffer = Buffer::from_file(Path::new("example.txt")).await?;

// Load from file (sync)
let buffer = Buffer::from_file_sync(Path::new("example.txt"))?;

// Save to file (async)
buffer.save(Some(Path::new("output.txt"))).await?;

// Save to file (sync)
buffer.save_sync(Some(Path::new("output.txt")))?;
```

#### Iterators

```rust
use lite_xl::buffer::Buffer;

let buffer = Buffer::from_str("line1\nline2\nline3");

// Iterate over lines (without line endings)
for line in buffer.lines() {
    println!("{}", line);
}

// Iterate over characters
for ch in buffer.chars() {
    print!("{}", ch);
}

// Iterate over characters in a range
let range = Range::new(Position::new(0, 0), Position::new(0, 5));
for ch in buffer.chars_in_range(range)? {
    print!("{}", ch);
}
```

## Error Handling

The module defines `BufferError` for comprehensive error handling:

```rust
pub enum BufferError {
    OutOfBounds(Position),    // Position is out of bounds
    InvalidRange(Range),      // Range is invalid
    Io(std::io::Error),       // IO error
    NoFilePath,               // No file path provided
    Utf8(FromUtf8Error),      // UTF-8 encoding error
    Rope(String),             // Rope operation error
}
```

All fallible operations return `Result<T, BufferError>`.

## Design Decisions

### 1. Position: Character Offsets vs Byte Offsets

The `Position` type uses character offsets, not byte offsets. This is more intuitive for text editing operations but requires conversion when working with the underlying rope's byte-based indices.

### 2. Range: Half-Open Intervals

Ranges are half-open `[start, end)` - the start is inclusive, the end is exclusive. This is consistent with Rust's standard range types and makes range operations more intuitive.

### 3. Line Endings: Separate Line Handling

Lines are stored with their line endings in the rope, but the `line()` method returns lines without endings for convenience. Use `line_with_ending()` when you need the raw line with its ending.

### 4. Automatic Version Tracking

Every modification increments the version counter, allowing efficient change detection without comparing entire buffer contents.

### 5. Ropey Integration

The buffer is a thin wrapper around `ropey::Rope`, exposing a more ergonomic API while preserving ropey's performance characteristics:

- O(log n) insert/delete
- O(log n) position/offset conversion
- Memory-efficient for large files
- Efficient iterators

## Performance

- **Insert**: O(log n) where n is the buffer size
- **Delete**: O(log n)
- **Query (line, char_at)**: O(log n)
- **Position ↔ Offset conversion**: O(log n)
- **Iteration**: O(1) per element
- **Memory**: Proportional to content size with rope overhead

## Thread Safety

The `Buffer` type is **not** thread-safe. It's designed to be owned by a single thread (typically the main UI thread). If you need to share buffers across threads, wrap them in appropriate synchronization primitives (e.g., `Arc<Mutex<Buffer>>`).

## Testing

The module includes comprehensive unit tests covering:

- Basic buffer operations (create, insert, delete, replace)
- Position and range operations
- Line ending detection and conversion
- Multiline operations
- Position/offset conversions
- Edge cases (empty buffers, out-of-bounds positions)
- Iterator functionality

Run tests with:

```bash
cargo test buffer
```

## Examples

See `/examples/buffer_demo.rs` for a comprehensive demonstration of all buffer functionality.

Run the demo with:

```bash
cargo run --example buffer_demo
```

## Future Enhancements

Potential areas for future improvement:

1. **Incremental Parsing**: Add hooks for incremental syntax highlighting
2. **Rope Chunks**: Expose chunk iteration for efficient processing
3. **Transaction API**: Batch multiple edits into atomic transactions
4. **UTF-16 Indexing**: Add UTF-16 code unit offsets for LSP compatibility
5. **Grapheme Clusters**: Support grapheme cluster boundaries for complex Unicode
6. **Memory Mapping**: Support memory-mapped files for very large documents

## API Stability

The buffer API is designed to be stable. Breaking changes will only be introduced with major version bumps. The public API includes:

- `Buffer` struct and its methods
- `Position` and `Range` types
- `LineEnding` enum and detection functions
- `BufferError` and `Result` types
- `BufferId` type

Internal implementation details (rope structure, caching strategies) may change between minor versions without breaking the public API.
