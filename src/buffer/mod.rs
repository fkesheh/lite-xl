//! Text buffer implementation using the ropey rope data structure.
//!
//! This module provides the foundational text buffer abstraction that efficiently
//! handles text storage and manipulation using a rope data structure. It supports
//! common operations like insertion, deletion, and querying with O(log n) complexity.
//!
//! # Features
//!
//! - Efficient insertion and deletion operations (O(log n))
//! - Line-based and character-based access
//! - Automatic line ending detection and normalization
//! - Position and range types for text navigation
//! - Iterator support for lines and characters
//! - Comprehensive error handling

pub mod line_ending;
pub mod position;

pub use line_ending::{
    count_lines, detect_line_ending, detect_line_ending_with_stats, normalize_line_endings,
    split_lines_with_endings, LineEnding, LineEndingStats,
};
pub use position::{Position, Range};

use ropey::Rope;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;

/// Errors that can occur during buffer operations.
#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Invalid position: {0}")]
    InvalidPosition(Position),

    #[error("Invalid range: {0}")]
    InvalidRange(Range),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Result type for buffer operations.
pub type Result<T> = std::result::Result<T, BufferError>;

/// Unique identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(u64);

static NEXT_BUFFER_ID: AtomicU64 = AtomicU64::new(0);

impl BufferId {
    /// Create a new unique buffer ID.
    fn new() -> Self {
        Self(NEXT_BUFFER_ID.fetch_add(1, Ordering::Relaxed))
    }
}


/// A text buffer backed by a rope data structure.
///
/// The buffer provides efficient text editing operations with O(log n) complexity
/// for insertions, deletions, and queries. It uses the ropey crate's rope
/// implementation optimized for text editing.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::{Buffer, Position, Range};
///
/// let mut buffer = Buffer::new();
/// buffer.insert(Position::zero(), "Hello, world!");
/// assert_eq!(buffer.line_count(), 1);
///
/// let text = buffer.slice(Range::new(Position::zero(), Position::new(0, 5)));
/// assert_eq!(text, "Hello");
/// ```
pub struct Buffer {
    /// The underlying rope storing text content
    rope: Rope,

    /// Unique identifier for this buffer
    id: BufferId,

    /// File path (if associated with a file)
    path: Option<PathBuf>,

    /// Line ending style
    line_ending: LineEnding,

    /// Modification state
    modified: bool,

    /// Version counter (incremented on each change)
    version: u64,
}

impl Buffer {
    /// Create a new empty buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let buffer = Buffer::new();
    /// assert_eq!(buffer.line_count(), 1); // Empty buffer has one empty line
    /// ```
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            id: BufferId::new(),
            path: None,
            line_ending: LineEnding::default(),
            modified: false,
            version: 0,
        }
    }

    /// Create a buffer from a string.
    ///
    /// The line ending style will be auto-detected from the input text.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let buffer = Buffer::from_str("Hello\nWorld");
    /// assert_eq!(buffer.line_count(), 2);
    /// ```
    pub fn from_str(text: &str) -> Self {
        let line_ending = detect_line_ending(text);
        Self {
            rope: Rope::from_str(text),
            id: BufferId::new(),
            path: None,
            line_ending,
            modified: false,
            version: 0,
        }
    }

    /// Creates a buffer from a file.
    ///
    /// The line ending style will be auto-detected from the file content.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lite_xl::buffer::Buffer;
    /// use std::path::Path;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let buffer = Buffer::from_file(Path::new("example.txt")).await?;
    /// println!("Loaded {} lines", buffer.line_count());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let text = tokio::fs::read_to_string(path).await?;
        let line_ending = detect_line_ending(&text);

        Ok(Self {
            rope: Rope::from_str(&text),
            id: BufferId::new(),
            line_ending,
            path: Some(path.to_path_buf()),
            modified: false,
            version: 0,
        })
    }

    /// Synchronously creates a buffer from a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lite_xl::buffer::Buffer;
    /// use std::path::Path;
    ///
    /// let buffer = Buffer::from_file_sync(Path::new("example.txt"))?;
    /// # Ok::<(), lite_xl::buffer::BufferError>(())
    /// ```
    pub fn from_file_sync(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)?;
        let line_ending = detect_line_ending(&text);

        Ok(Self {
            rope: Rope::from_str(&text),
            id: BufferId::new(),
            line_ending,
            path: Some(path.to_path_buf()),
            modified: false,
            version: 0,
        })
    }

    /// Get the buffer's unique identifier.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Get the buffer's version.
    ///
    /// The version is incremented on each modification.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the total number of lines in the buffer.
    ///
    /// Note: An empty buffer has 1 line.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let buffer = Buffer::from_str("line1\nline2\nline3");
    /// assert_eq!(buffer.line_count(), 3);
    /// ```
    #[inline]
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total number of characters in the buffer.
    #[inline]
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the total number of bytes in the buffer.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Check if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a line by index (0-based).
    ///
    /// Returns None if the line index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let buffer = Buffer::from_str("line1\nline2\nline3");
    /// assert_eq!(buffer.line(0).as_deref(), Some("line1\n"));
    /// assert_eq!(buffer.line(1).as_deref(), Some("line2\n"));
    /// assert!(buffer.line(10).is_none());
    /// ```
    pub fn line(&self, line_idx: usize) -> Option<Cow<str>> {
        if line_idx >= self.line_count() {
            return None;
        }
        Some(self.rope.line(line_idx).into())
    }

    /// Get the length of a line in characters (including line ending).
    ///
    /// Returns None if the line index is out of bounds.
    pub fn line_len(&self, line_idx: usize) -> Option<usize> {
        if line_idx >= self.line_count() {
            return None;
        }
        Some(self.rope.line(line_idx).len_chars())
    }

    /// Get the character at a position.
    ///
    /// Returns None if the position is out of bounds.
    pub fn char_at(&self, pos: Position) -> Option<char> {
        let offset = self.pos_to_offset(pos).ok()?;
        self.rope.get_char(offset)
    }

    /// Insert text at a position.
    ///
    /// # Errors
    ///
    /// Returns an error if the position is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Buffer, Position};
    ///
    /// let mut buffer = Buffer::from_str("Hello");
    /// buffer.insert(Position::new(0, 5), ", world!").unwrap();
    /// assert_eq!(buffer.to_string(), "Hello, world!");
    /// ```
    pub fn insert(&mut self, pos: Position, text: &str) -> Result<()> {
        let offset = self.pos_to_offset(pos)?;
        self.rope.insert(offset, text);
        self.modified = true;
        self.version += 1;
        Ok(())
    }

    /// Delete a range of text.
    ///
    /// Returns the deleted text.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Buffer, Position, Range};
    ///
    /// let mut buffer = Buffer::from_str("Hello, world!");
    /// let deleted = buffer.delete(Range::new(
    ///     Position::new(0, 5),
    ///     Position::new(0, 12)
    /// )).unwrap();
    /// assert_eq!(deleted, ", world");
    /// assert_eq!(buffer.to_string(), "Hello!");
    /// ```
    pub fn delete(&mut self, range: Range) -> Result<String> {
        let start_offset = self.pos_to_offset(range.start)?;
        let end_offset = self.pos_to_offset(range.end)?;

        if start_offset > end_offset {
            return Err(BufferError::InvalidRange(range));
        }

        let deleted = self.rope.slice(start_offset..end_offset).to_string();
        self.rope.remove(start_offset..end_offset);
        self.modified = true;
        self.version += 1;
        Ok(deleted)
    }

    /// Replace a range with new text.
    ///
    /// This is more efficient than separate delete and insert operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is invalid.
    pub fn replace(&mut self, range: Range, text: &str) -> Result<String> {
        let deleted = self.delete(range)?;
        self.insert(range.start, text)?;
        Ok(deleted)
    }

    /// Get text in a range.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Buffer, Position, Range};
    ///
    /// let buffer = Buffer::from_str("Hello, world!");
    /// let text = buffer.slice(Range::new(
    ///     Position::new(0, 0),
    ///     Position::new(0, 5)
    /// )).unwrap();
    /// assert_eq!(text, "Hello");
    /// ```
    pub fn slice(&self, range: Range) -> Result<String> {
        let start_offset = self.pos_to_offset(range.start)?;
        let end_offset = self.pos_to_offset(range.end)?;

        if start_offset > end_offset {
            return Err(BufferError::InvalidRange(range));
        }

        Ok(self.rope.slice(start_offset..end_offset).to_string())
    }

    /// Convert a position to a byte offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the position is out of bounds.
    pub fn pos_to_offset(&self, pos: Position) -> Result<usize> {
        if pos.line >= self.line_count() {
            return Err(BufferError::InvalidPosition(pos));
        }

        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.rope.line(pos.line).len_chars();

        // Allow position at end of line (for cursor placement)
        if pos.column > line_len {
            return Err(BufferError::InvalidPosition(pos));
        }

        Ok(line_start + pos.column)
    }

    /// Convert a byte offset to a position.
    ///
    /// # Errors
    ///
    /// Returns an error if the offset is out of bounds.
    pub fn offset_to_pos(&self, offset: usize) -> Result<Position> {
        if offset > self.len_chars() {
            return Err(BufferError::InvalidPosition(Position::new(0, offset)));
        }

        let line = self.rope.char_to_line(offset);
        let line_start = self.rope.line_to_char(line);
        let column = offset - line_start;

        Ok(Position::new(line, column))
    }

    /// Check if a position is valid for this buffer.
    pub fn is_valid_position(&self, pos: Position) -> bool {
        self.pos_to_offset(pos).is_ok()
    }

    /// Clamp a position to valid bounds.
    ///
    /// If the position is beyond the end of the buffer, it will be clamped
    /// to the last valid position.
    pub fn clamp_position(&self, pos: Position) -> Position {
        if self.is_empty() {
            return Position::zero();
        }

        let line = pos.line.min(self.line_count().saturating_sub(1));
        let line_len = self.line_len(line).unwrap_or(0);
        
        // Don't include line ending in column clamping
        let line_content_len = if line_len > 0 {
            let line_text = self.line(line).unwrap();
            line_text.trim_end_matches(&['\n', '\r'][..]).len()
        } else {
            0
        };

        let column = pos.column.min(line_content_len);
        Position::new(line, column)
    }

    /// Get the buffer content as a string.
    ///
    /// This is useful for small buffers but can be expensive for large ones.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Check if the buffer has been modified.
    #[inline]
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark the buffer as unmodified.
    ///
    /// This is typically called after saving.
    pub fn clear_modified(&mut self) {
        self.modified = false;
    }

    /// Get the buffer's file path, if any.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Set the buffer's file path.
    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    /// Get the line ending style.
    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    /// Set the line ending style.
    ///
    /// This does not convert existing line endings. To convert, use
    /// [`normalize_to_line_ending`](Self::normalize_to_line_ending).
    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    /// Normalizes all line endings in the buffer to the target style.
    ///
    /// This converts all line endings (LF, CRLF, CR) to the target style.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Buffer, LineEnding};
    ///
    /// let mut buffer = Buffer::from_str("line1\r\nline2\nline3");
    /// buffer.normalize_to_line_ending(LineEnding::Lf);
    /// assert_eq!(buffer.to_string(), "line1\nline2\nline3");
    /// ```
    pub fn normalize_to_line_ending(&mut self, target: LineEnding) {
        if self.line_ending == target {
            return;
        }

        let text = self.to_string();
        let normalized = normalize_line_endings(&text, target);
        self.rope = Rope::from_str(&normalized);
        self.line_ending = target;
        self.version += 1;
        self.modified = true;
    }

    /// Returns an iterator over the lines in the buffer.
    ///
    /// Lines are returned without their line endings.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let buffer = Buffer::from_str("line1\nline2\nline3");
    /// let lines: Vec<_> = buffer.lines().collect();
    /// assert_eq!(lines.len(), 3);
    /// ```
    pub fn lines(&self) -> impl Iterator<Item = Cow<'_, str>> {
        (0..self.line_count()).filter_map(move |i| self.line(i))
    }

    /// Returns an iterator over the characters in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let buffer = Buffer::from_str("Hi!");
    /// let chars: Vec<_> = buffer.chars().collect();
    /// assert_eq!(chars, vec!['H', 'i', '!']);
    /// ```
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.rope.chars()
    }

    /// Returns an iterator over the characters in a specific range.
    ///
    /// # Errors
    ///
    /// Returns an error if the range is invalid.
    pub fn chars_in_range(&self, range: Range) -> Result<impl Iterator<Item = char> + '_> {
        let start_offset = self.pos_to_offset(range.start)?;
        let end_offset = self.pos_to_offset(range.end)?;

        Ok(self.rope.slice(start_offset..end_offset).chars())
    }

    /// Clears all content from the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Buffer;
    ///
    /// let mut buffer = Buffer::from_str("Hello, world!");
    /// buffer.clear();
    /// assert!(buffer.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.rope = Rope::new();
        self.version += 1;
        self.modified = true;
    }

    /// Saves the buffer to a file.
    ///
    /// If `path` is `None`, saves to the buffer's current file path.
    ///
    /// # Errors
    ///
    /// Returns an error if no path is provided and the buffer has no associated path,
    /// or if the file write fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lite_xl::buffer::Buffer;
    /// use std::path::Path;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut buffer = Buffer::from_str("Hello, world!");
    /// buffer.save(Some(Path::new("output.txt"))).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save(&mut self, path: Option<&std::path::Path>) -> Result<()> {
        let save_path = path
            .or(self.path.as_deref())
            .ok_or_else(|| BufferError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path provided",
            )))?;

        let content = self.to_string();
        tokio::fs::write(save_path, content).await?;

        self.path = Some(save_path.to_path_buf());
        self.modified = false;

        Ok(())
    }

    /// Synchronously saves the buffer to a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lite_xl::buffer::Buffer;
    /// use std::path::Path;
    ///
    /// let mut buffer = Buffer::from_str("Hello, world!");
    /// buffer.save_sync(Some(Path::new("output.txt")))?;
    /// # Ok::<(), lite_xl::buffer::BufferError>(())
    /// ```
    pub fn save_sync(&mut self, path: Option<&std::path::Path>) -> Result<()> {
        let save_path = path
            .or(self.path.as_deref())
            .ok_or_else(|| BufferError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file path provided",
            )))?;

        let content = self.to_string();
        std::fs::write(save_path, content)?;

        self.path = Some(save_path.to_path_buf());
        self.modified = false;

        Ok(())
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new();
        assert_eq!(buffer.line_count(), 1);
        assert!(buffer.is_empty());
        assert!(!buffer.is_modified());
    }

    #[test]
    fn test_buffer_from_string() {
        let buffer = Buffer::from_str("line1\nline2\nline3");
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.line(0).as_deref(), Some("line1\n"));
        assert_eq!(buffer.line(1).as_deref(), Some("line2\n"));
        assert_eq!(buffer.line(2).as_deref(), Some("line3"));
    }

    #[test]
    fn test_insert() {
        let mut buffer = Buffer::from_str("Hello");
        buffer.insert(Position::new(0, 5), ", world!").unwrap();
        assert_eq!(buffer.to_string(), "Hello, world!");
        assert!(buffer.is_modified());
    }

    #[test]
    fn test_delete() {
        let mut buffer = Buffer::from_str("Hello, world!");
        let deleted = buffer.delete(Range::new(
            Position::new(0, 5),
            Position::new(0, 12)
        )).unwrap();
        assert_eq!(deleted, ", world");
        assert_eq!(buffer.to_string(), "Hello!");
    }

    #[test]
    fn test_slice() {
        let buffer = Buffer::from_str("Hello, world!");
        let text = buffer.slice(Range::new(
            Position::new(0, 0),
            Position::new(0, 5)
        )).unwrap();
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_pos_to_offset() {
        let buffer = Buffer::from_str("line1\nline2\nline3");
        assert_eq!(buffer.pos_to_offset(Position::new(0, 0)).unwrap(), 0);
        assert_eq!(buffer.pos_to_offset(Position::new(1, 0)).unwrap(), 6);
        assert_eq!(buffer.pos_to_offset(Position::new(2, 0)).unwrap(), 12);
    }

    #[test]
    fn test_offset_to_pos() {
        let buffer = Buffer::from_str("line1\nline2\nline3");
        assert_eq!(buffer.offset_to_pos(0).unwrap(), Position::new(0, 0));
        assert_eq!(buffer.offset_to_pos(6).unwrap(), Position::new(1, 0));
        assert_eq!(buffer.offset_to_pos(12).unwrap(), Position::new(2, 0));
    }

    #[test]
    fn test_line_ending_detection() {
        assert_eq!(detect_line_ending("line1\nline2\n"), LineEnding::Lf);
        assert_eq!(detect_line_ending("line1\r\nline2\r\n"), LineEnding::CrLf);
        assert_eq!(detect_line_ending("line1\rline2\r"), LineEnding::Cr);
    }

    #[test]
    fn test_normalize_line_endings() {
        let mut buffer = Buffer::from_str("line1\r\nline2\nline3\rline4");
        buffer.normalize_to_line_ending(LineEnding::Lf);
        assert_eq!(buffer.to_string(), "line1\nline2\nline3\nline4");
        assert_eq!(buffer.line_ending(), LineEnding::Lf);
    }

    #[test]
    fn test_iterators() {
        let buffer = Buffer::from_str("line1\nline2\nline3");

        let lines: Vec<_> = buffer.lines().collect();
        assert_eq!(lines.len(), 3);

        let chars: Vec<_> = buffer.chars().take(5).collect();
        assert_eq!(chars, vec!['l', 'i', 'n', 'e', '1']);
    }

    #[test]
    fn test_clear() {
        let mut buffer = Buffer::from_str("Hello, world!");
        assert!(!buffer.is_empty());
        buffer.clear();
        assert!(buffer.is_empty());
        assert!(buffer.is_modified());
    }

    #[test]
    fn test_multiline_operations() {
        let mut buffer = Buffer::from_str("line1\nline2\nline3");
        
        // Insert in middle
        buffer.insert(Position::new(1, 5), " inserted").unwrap();
        assert_eq!(buffer.line(1).as_deref(), Some("line2 inserted\n"));

        // Delete across lines
        let deleted = buffer.delete(Range::new(
            Position::new(0, 3),
            Position::new(1, 3)
        )).unwrap();
        assert_eq!(deleted, "e1\nlin");
    }

    #[test]
    fn test_clamp_position() {
        let buffer = Buffer::from_str("short\nlonger line\nx");
        
        // Clamp to end of short line
        let clamped = buffer.clamp_position(Position::new(0, 100));
        assert_eq!(clamped, Position::new(0, 5));
        
        // Clamp to last line
        let clamped = buffer.clamp_position(Position::new(100, 0));
        assert_eq!(clamped, Position::new(2, 0));
    }
}
