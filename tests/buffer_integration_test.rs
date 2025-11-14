//! Integration tests for the buffer module.
//!
//! These tests verify that the buffer module works correctly in isolation.

use lite_xl::buffer::{Buffer, Position, Range, LineEnding, detect_line_ending, normalize_line_endings};

#[test]
fn test_buffer_basic_operations() {
    let mut buffer = Buffer::new();
    assert!(buffer.is_empty());
    assert_eq!(buffer.line_count(), 1);

    // Insert text
    buffer.insert(Position::zero(), "Hello, world!").unwrap();
    assert!(!buffer.is_empty());
    assert_eq!(buffer.to_string(), "Hello, world!");
    assert!(buffer.is_modified());

    // Delete text
    let range = Range::new(Position::new(0, 5), Position::new(0, 7));
    let deleted = buffer.delete(range).unwrap();
    assert_eq!(deleted, ", ");
    assert_eq!(buffer.to_string(), "Helloworld!");
}

#[test]
fn test_buffer_multiline() {
    let buffer = Buffer::from_str("line1\nline2\nline3");
    assert_eq!(buffer.line_count(), 3);
    assert_eq!(buffer.line(0).as_deref(), Some("line1"));
    assert_eq!(buffer.line(1).as_deref(), Some("line2"));
    assert_eq!(buffer.line(2).as_deref(), Some("line3"));
}

#[test]
fn test_position_ordering() {
    let pos1 = Position::new(0, 0);
    let pos2 = Position::new(0, 5);
    let pos3 = Position::new(1, 0);

    assert!(pos1 < pos2);
    assert!(pos2 < pos3);
    assert!(pos1 < pos3);
}

#[test]
fn test_range_operations() {
    let range1 = Range::new(Position::new(0, 0), Position::new(0, 10));
    let range2 = Range::new(Position::new(0, 5), Position::new(0, 15));

    assert!(range1.overlaps(range2));
    assert!(range1.contains(Position::new(0, 5)));
    assert!(!range1.contains(Position::new(0, 10)));

    let union = range1.union(range2);
    assert_eq!(union.start, Position::new(0, 0));
    assert_eq!(union.end, Position::new(0, 15));
}

#[test]
fn test_line_ending_detection() {
    assert_eq!(detect_line_ending("line1\nline2"), LineEnding::Lf);
    assert_eq!(detect_line_ending("line1\r\nline2"), LineEnding::CrLf);
    assert_eq!(detect_line_ending("line1\rline2"), LineEnding::Cr);
}

#[test]
fn test_line_ending_normalization() {
    let mixed = "line1\nline2\r\nline3\rline4";
    let normalized = normalize_line_endings(mixed, LineEnding::Lf);
    assert_eq!(normalized, "line1\nline2\nline3\nline4");

    let mut buffer = Buffer::from_str(mixed);
    buffer.normalize_to_line_ending(LineEnding::CrLf);
    assert_eq!(buffer.to_string(), "line1\r\nline2\r\nline3\r\nline4");
}

#[test]
fn test_buffer_replace() {
    let mut buffer = Buffer::from_str("Hello, world!");
    let range = Range::new(Position::new(0, 7), Position::new(0, 12));
    let deleted = buffer.replace(range, "Rust").unwrap();
    assert_eq!(deleted, "world");
    assert_eq!(buffer.to_string(), "Hello, Rust!");
}

#[test]
fn test_buffer_iterators() {
    let buffer = Buffer::from_str("A\nB\nC");

    let lines: Vec<_> = buffer.lines().collect();
    assert_eq!(lines.len(), 3);

    let chars: Vec<_> = buffer.chars().take(3).collect();
    assert_eq!(chars, vec!['A', '\n', 'B']);
}

#[test]
fn test_position_offset_conversion() {
    let buffer = Buffer::from_str("Hello\nWorld");

    let pos = Position::new(1, 0);
    let offset = buffer.pos_to_offset(pos).unwrap();
    assert_eq!(offset, 6);

    let back = buffer.offset_to_pos(offset).unwrap();
    assert_eq!(back, pos);
}

#[test]
fn test_buffer_version_tracking() {
    let mut buffer = Buffer::new();
    let v1 = buffer.version();

    buffer.insert(Position::zero(), "Hello").unwrap();
    let v2 = buffer.version();
    assert!(v2 > v1);

    buffer.delete(Range::new(Position::zero(), Position::new(0, 5))).unwrap();
    let v3 = buffer.version();
    assert!(v3 > v2);
}

#[test]
fn test_buffer_clear() {
    let mut buffer = Buffer::from_str("Some text");
    assert!(!buffer.is_empty());

    buffer.clear();
    assert!(buffer.is_empty());
    assert!(buffer.is_modified());
}

#[test]
fn test_position_clamping() {
    let buffer = Buffer::from_str("short\nlonger line\nx");

    let pos = Position::new(100, 100);
    let clamped = buffer.clamp_position(pos);
    assert!(clamped.line < buffer.line_count());
}

#[test]
fn test_range_cursor() {
    let pos = Position::new(5, 10);
    let cursor = Range::cursor(pos);

    assert!(cursor.is_empty());
    assert_eq!(cursor.start, pos);
    assert_eq!(cursor.end, pos);
}

#[test]
fn test_range_normalization() {
    // Range should auto-normalize so start < end
    let range = Range::new(Position::new(10, 0), Position::new(5, 0));
    assert_eq!(range.start, Position::new(5, 0));
    assert_eq!(range.end, Position::new(10, 0));
}

#[test]
fn test_line_ending_conversion() {
    let text = "line1\nline2\nline3";
    let converted = LineEnding::Lf.convert_to(text, LineEnding::CrLf);
    assert_eq!(converted, "line1\r\nline2\r\nline3");
}

#[test]
fn test_buffer_char_at() {
    let buffer = Buffer::from_str("Hello");
    assert_eq!(buffer.char_at(Position::new(0, 0)), Some('H'));
    assert_eq!(buffer.char_at(Position::new(0, 4)), Some('o'));
    assert_eq!(buffer.char_at(Position::new(0, 100)), None);
}

#[test]
fn test_buffer_slice() {
    let buffer = Buffer::from_str("Hello, world!");
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    let slice = buffer.slice(range).unwrap();
    assert_eq!(slice, "Hello");
}

#[test]
fn test_empty_buffer() {
    let buffer = Buffer::new();
    assert!(buffer.is_empty());
    assert_eq!(buffer.len_chars(), 0);
    assert_eq!(buffer.line_count(), 1); // Empty buffer has 1 empty line
}

#[test]
fn test_buffer_from_str() {
    let buffer = Buffer::from_str("Test content");
    assert_eq!(buffer.to_string(), "Test content");
    assert!(!buffer.is_modified()); // Not modified initially
}
