//! Position and Range types for text buffer operations.
//!
//! This module provides fundamental types for representing locations and regions
//! in a text buffer. All positions are 0-indexed.

use std::cmp::{Ordering, max, min};
use serde::{Deserialize, Serialize};

/// A position in a text buffer (0-indexed).
///
/// Positions are represented as (line, column) pairs, where:
/// - `line`: The line number (0-indexed)
/// - `column`: The character offset within the line (0-indexed, NOT byte offset)
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::Position;
///
/// let pos = Position::new(5, 10);
/// assert_eq!(pos.line, 5);
/// assert_eq!(pos.column, 10);
///
/// let origin = Position::zero();
/// assert_eq!(origin.line, 0);
/// assert_eq!(origin.column, 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed)
    pub line: usize,
    /// Character offset within line (0-indexed, character count not byte offset)
    pub column: usize,
}

impl Position {
    /// Create a new position.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Position;
    ///
    /// let pos = Position::new(0, 0);
    /// ```
    #[inline]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Create a position at the origin (0, 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Position;
    ///
    /// let origin = Position::zero();
    /// assert_eq!(origin, Position::new(0, 0));
    /// ```
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Create a position at the start of a line.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Position;
    ///
    /// let pos = Position::line_start(5);
    /// assert_eq!(pos, Position::new(5, 0));
    /// ```
    #[inline]
    pub const fn line_start(line: usize) -> Self {
        Self::new(line, 0)
    }

    /// Move position to the start of its line.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Position;
    ///
    /// let pos = Position::new(5, 10);
    /// let start = pos.to_line_start();
    /// assert_eq!(start, Position::new(5, 0));
    /// ```
    #[inline]
    pub const fn to_line_start(self) -> Self {
        Self::new(self.line, 0)
    }

    /// Check if this position is at the start of a line.
    #[inline]
    pub const fn is_line_start(self) -> bool {
        self.column == 0
    }

    /// Offset the column by a given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Position;
    ///
    /// let pos = Position::new(5, 10);
    /// let moved = pos.offset_column(5);
    /// assert_eq!(moved, Position::new(5, 15));
    /// ```
    #[inline]
    pub const fn offset_column(self, offset: isize) -> Self {
        let new_col = if offset >= 0 {
            self.column.saturating_add(offset as usize)
        } else {
            self.column.saturating_sub((-offset) as usize)
        };
        Self::new(self.line, new_col)
    }

    /// Offset the line by a given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::Position;
    ///
    /// let pos = Position::new(5, 10);
    /// let moved = pos.offset_line(3);
    /// assert_eq!(moved, Position::new(8, 10));
    /// ```
    #[inline]
    pub const fn offset_line(self, offset: isize) -> Self {
        let new_line = if offset >= 0 {
            self.line.saturating_add(offset as usize)
        } else {
            self.line.saturating_sub((-offset) as usize)
        };
        Self::new(new_line, self.column)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => self.column.cmp(&other.column),
            other => other,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A range in a text buffer.
///
/// Represents a region of text from `start` to `end` (exclusive).
/// The range is guaranteed to be well-formed (start <= end).
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::{Position, Range};
///
/// let range = Range::new(Position::new(0, 0), Position::new(0, 5));
/// assert!(!range.is_empty());
///
/// let cursor = Range::cursor(Position::new(5, 10));
/// assert!(cursor.is_empty());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Start position (inclusive)
    pub start: Position,
    /// End position (exclusive)
    pub end: Position,
}

impl Range {
    /// Create a new range.
    ///
    /// The range will be normalized so that start <= end.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let range = Range::new(Position::new(0, 5), Position::new(0, 0));
    /// assert_eq!(range.start, Position::new(0, 0));
    /// assert_eq!(range.end, Position::new(0, 5));
    /// ```
    pub fn new(start: Position, end: Position) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self { start: end, end: start }
        }
    }

    /// Create a zero-width range at a cursor position.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let cursor = Range::cursor(Position::new(5, 10));
    /// assert_eq!(cursor.start, cursor.end);
    /// assert!(cursor.is_empty());
    /// ```
    #[inline]
    pub const fn cursor(pos: Position) -> Self {
        Self { start: pos, end: pos }
    }

    /// Check if the range is empty (zero-width).
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let cursor = Range::cursor(Position::new(5, 10));
    /// assert!(cursor.is_empty());
    ///
    /// let selection = Range::new(Position::new(0, 0), Position::new(0, 5));
    /// assert!(!selection.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.start.line == self.end.line && self.start.column == self.end.column
    }

    /// Check if a position is contained within this range.
    ///
    /// The start is inclusive, the end is exclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let range = Range::new(Position::new(0, 0), Position::new(0, 10));
    /// assert!(range.contains(Position::new(0, 5)));
    /// assert!(range.contains(Position::new(0, 0)));
    /// assert!(!range.contains(Position::new(0, 10)));
    /// ```
    pub fn contains(self, pos: Position) -> bool {
        self.start <= pos && pos < self.end
    }

    /// Check if this range overlaps with another range.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let r1 = Range::new(Position::new(0, 0), Position::new(0, 10));
    /// let r2 = Range::new(Position::new(0, 5), Position::new(0, 15));
    /// assert!(r1.overlaps(r2));
    ///
    /// let r3 = Range::new(Position::new(0, 10), Position::new(0, 20));
    /// assert!(!r1.overlaps(r3));
    /// ```
    pub fn overlaps(self, other: Range) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Merge this range with another, creating a range that encompasses both.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let r1 = Range::new(Position::new(0, 0), Position::new(0, 5));
    /// let r2 = Range::new(Position::new(0, 10), Position::new(0, 15));
    /// let merged = r1.union(r2);
    /// assert_eq!(merged.start, Position::new(0, 0));
    /// assert_eq!(merged.end, Position::new(0, 15));
    /// ```
    pub fn union(self, other: Range) -> Range {
        Range {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }

    /// Extend the range to include a position.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    /// let extended = range.extend_to(Position::new(0, 10));
    /// assert_eq!(extended.end, Position::new(0, 10));
    /// ```
    pub fn extend_to(self, pos: Position) -> Range {
        Range::new(self.start, pos)
    }

    /// Check if this range is on a single line.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let single_line = Range::new(Position::new(5, 0), Position::new(5, 10));
    /// assert!(single_line.is_single_line());
    ///
    /// let multi_line = Range::new(Position::new(5, 0), Position::new(6, 0));
    /// assert!(!multi_line.is_single_line());
    /// ```
    #[inline]
    pub const fn is_single_line(self) -> bool {
        self.start.line == self.end.line
    }

    /// Get the line span of this range.
    ///
    /// Returns (start_line, end_line) inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::{Position, Range};
    ///
    /// let range = Range::new(Position::new(5, 0), Position::new(8, 10));
    /// assert_eq!(range.line_span(), (5, 8));
    /// ```
    #[inline]
    pub const fn line_span(self) -> (usize, usize) {
        (self.start.line, self.end.line)
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.column, 10);

        let origin = Position::zero();
        assert_eq!(origin.line, 0);
        assert_eq!(origin.column, 0);
    }

    #[test]
    fn test_position_ordering() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 5);
        let p3 = Position::new(1, 0);

        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p1 < p3);
    }

    #[test]
    fn test_position_offset() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.offset_column(5), Position::new(5, 15));
        assert_eq!(pos.offset_column(-5), Position::new(5, 5));
        assert_eq!(pos.offset_line(2), Position::new(7, 10));
        assert_eq!(pos.offset_line(-2), Position::new(3, 10));
    }

    #[test]
    fn test_range_creation() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        assert_eq!(range.start, Position::new(0, 0));
        assert_eq!(range.end, Position::new(0, 5));
    }

    #[test]
    fn test_range_normalization() {
        let range = Range::new(Position::new(0, 10), Position::new(0, 5));
        assert_eq!(range.start, Position::new(0, 5));
        assert_eq!(range.end, Position::new(0, 10));
    }

    #[test]
    fn test_range_is_empty() {
        let cursor = Range::cursor(Position::new(5, 10));
        assert!(cursor.is_empty());

        let selection = Range::new(Position::new(0, 0), Position::new(0, 5));
        assert!(!selection.is_empty());
    }

    #[test]
    fn test_range_contains() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 10));
        assert!(range.contains(Position::new(0, 0)));
        assert!(range.contains(Position::new(0, 5)));
        assert!(!range.contains(Position::new(0, 10)));
        assert!(!range.contains(Position::new(1, 0)));
    }

    #[test]
    fn test_range_overlaps() {
        let r1 = Range::new(Position::new(0, 0), Position::new(0, 10));
        let r2 = Range::new(Position::new(0, 5), Position::new(0, 15));
        let r3 = Range::new(Position::new(0, 10), Position::new(0, 20));
        let r4 = Range::new(Position::new(1, 0), Position::new(1, 5));

        assert!(r1.overlaps(r2));
        assert!(!r1.overlaps(r3));
        assert!(!r1.overlaps(r4));
    }

    #[test]
    fn test_range_union() {
        let r1 = Range::new(Position::new(0, 0), Position::new(0, 5));
        let r2 = Range::new(Position::new(0, 10), Position::new(0, 15));
        let merged = r1.union(r2);
        assert_eq!(merged.start, Position::new(0, 0));
        assert_eq!(merged.end, Position::new(0, 15));
    }

    #[test]
    fn test_range_single_line() {
        let single = Range::new(Position::new(5, 0), Position::new(5, 10));
        assert!(single.is_single_line());

        let multi = Range::new(Position::new(5, 0), Position::new(6, 0));
        assert!(!multi.is_single_line());
    }
}
