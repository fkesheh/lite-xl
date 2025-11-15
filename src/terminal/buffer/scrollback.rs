//! Scrollback buffer for terminal history.
//!
//! This module provides the [`Scrollback`] type which implements a circular
//! buffer for storing terminal history lines that have scrolled off the top
//! of the visible terminal.

use super::cell::Cell;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A scrollback buffer for storing terminal history.
///
/// This implements a circular buffer with a maximum capacity. When the buffer
/// is full and a new line is added, the oldest line is removed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scrollback {
    /// The buffer storing historical lines.
    lines: VecDeque<Vec<Cell>>,
    /// Maximum number of lines to keep in the scrollback buffer.
    max_lines: usize,
    /// Number of columns per line.
    cols: usize,
}

impl Scrollback {
    /// Creates a new scrollback buffer with the specified capacity.
    ///
    /// # Arguments
    /// * `max_lines` - Maximum number of lines to store (0 means no scrollback)
    /// * `cols` - Number of columns per line
    pub fn new(max_lines: usize, cols: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines.min(10000)), // Cap at 10k for memory
            max_lines,
            cols,
        }
    }

    /// Returns the maximum number of lines that can be stored.
    pub fn max_lines(&self) -> usize {
        self.max_lines
    }

    /// Returns the current number of lines in the scrollback buffer.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns true if the scrollback buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Returns true if the scrollback buffer is full.
    pub fn is_full(&self) -> bool {
        self.max_lines > 0 && self.lines.len() >= self.max_lines
    }

    /// Returns the number of columns per line.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Adds a line to the scrollback buffer.
    ///
    /// If the buffer is full, the oldest line is removed first.
    /// If the line is shorter than `cols`, it's padded with spaces.
    /// If the line is longer than `cols`, it's truncated.
    pub fn push(&mut self, mut line: Vec<Cell>) {
        // Ensure line has exactly `cols` cells
        line.resize(self.cols, Cell::default());

        if self.max_lines == 0 {
            return; // No scrollback
        }

        // Remove oldest line if buffer is full
        if self.is_full() {
            self.lines.pop_front();
        }

        self.lines.push_back(line);
    }

    /// Adds a line to the scrollback buffer (alias for push).
    pub fn push_line(&mut self, line: Vec<Cell>) {
        self.push(line);
    }

    /// Gets a reference to the line at the specified index.
    ///
    /// Index 0 is the oldest line, and `len() - 1` is the newest line.
    /// Returns None if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Vec<Cell>> {
        self.lines.get(index)
    }

    /// Gets a line from the scrollback buffer, counting from the most recent line.
    ///
    /// `from_end(0)` returns the most recent line, `from_end(1)` returns the
    /// second-most recent, etc. Returns None if the index is out of bounds.
    pub fn from_end(&self, index: usize) -> Option<&Vec<Cell>> {
        if index < self.lines.len() {
            self.lines.get(self.lines.len() - 1 - index)
        } else {
            None
        }
    }

    /// Clears the scrollback buffer.
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Resizes the scrollback buffer to a new maximum capacity.
    ///
    /// If the new capacity is smaller than the current number of lines,
    /// the oldest lines are removed.
    pub fn resize(&mut self, new_max_lines: usize) {
        self.max_lines = new_max_lines;

        // Remove oldest lines if buffer is now too large
        while self.lines.len() > self.max_lines && self.max_lines > 0 {
            self.lines.pop_front();
        }

        // Clear all lines if max_lines is 0 (no scrollback)
        if self.max_lines == 0 {
            self.lines.clear();
        }
    }

    /// Resizes the number of columns per line.
    ///
    /// Existing lines are adjusted to the new width (truncated or padded).
    pub fn resize_cols(&mut self, new_cols: usize) {
        if new_cols == self.cols {
            return;
        }

        for line in &mut self.lines {
            line.resize(new_cols, Cell::default());
        }

        self.cols = new_cols;
    }

    /// Returns an iterator over all lines in the scrollback buffer.
    ///
    /// Lines are returned in order from oldest to newest.
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.lines.iter()
    }

    /// Returns a mutable iterator over all lines in the scrollback buffer.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec<Cell>> {
        self.lines.iter_mut()
    }

    /// Converts a specific line to a string.
    pub fn line_to_string(&self, index: usize) -> Option<String> {
        self.get(index).map(|line| line.iter().map(|cell| cell.c).collect())
    }

    /// Converts all scrollback lines to a string, separated by newlines.
    pub fn to_string(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.iter().map(|cell| cell.c).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns the total memory usage of the scrollback buffer in bytes (approximate).
    pub fn memory_usage(&self) -> usize {
        self.lines.len() * self.cols * std::mem::size_of::<Cell>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_line(text: &str, cols: usize) -> Vec<Cell> {
        let mut line: Vec<Cell> = text.chars().map(Cell::new).collect();
        line.resize(cols, Cell::default());
        line
    }

    #[test]
    fn test_scrollback_new() {
        let sb = Scrollback::new(100, 80);
        assert_eq!(sb.max_lines(), 100);
        assert_eq!(sb.cols(), 80);
        assert_eq!(sb.len(), 0);
        assert!(sb.is_empty());
        assert!(!sb.is_full());
    }

    #[test]
    fn test_scrollback_push() {
        let mut sb = Scrollback::new(3, 5);
        sb.push(make_line("AAA", 5));
        sb.push(make_line("BBB", 5));

        assert_eq!(sb.len(), 2);
        assert!(!sb.is_full());
    }

    #[test]
    fn test_scrollback_push_overflow() {
        let mut sb = Scrollback::new(3, 5);
        sb.push(make_line("AAA", 5));
        sb.push(make_line("BBB", 5));
        sb.push(make_line("CCC", 5));
        assert!(sb.is_full());

        sb.push(make_line("DDD", 5));

        assert_eq!(sb.len(), 3);
        assert_eq!(sb.line_to_string(0).unwrap().trim(), "BBB");
        assert_eq!(sb.line_to_string(2).unwrap().trim(), "DDD");
    }

    #[test]
    fn test_scrollback_get() {
        let mut sb = Scrollback::new(10, 5);
        sb.push(make_line("AAA", 5));
        sb.push(make_line("BBB", 5));

        assert!(sb.get(0).is_some());
        assert!(sb.get(1).is_some());
        assert!(sb.get(2).is_none());
    }

    #[test]
    fn test_scrollback_from_end() {
        let mut sb = Scrollback::new(10, 5);
        sb.push(make_line("AAA", 5));
        sb.push(make_line("BBB", 5));
        sb.push(make_line("CCC", 5));

        assert_eq!(sb.from_end(0).map(|l| &l[0].c), Some(&'C'));
        assert_eq!(sb.from_end(1).map(|l| &l[0].c), Some(&'B'));
        assert_eq!(sb.from_end(2).map(|l| &l[0].c), Some(&'A'));
        assert!(sb.from_end(3).is_none());
    }

    #[test]
    fn test_scrollback_clear() {
        let mut sb = Scrollback::new(10, 5);
        sb.push(make_line("AAA", 5));
        sb.push(make_line("BBB", 5));

        sb.clear();

        assert_eq!(sb.len(), 0);
        assert!(sb.is_empty());
    }

    #[test]
    fn test_scrollback_resize() {
        let mut sb = Scrollback::new(5, 5);
        sb.push(make_line("AAA", 5));
        sb.push(make_line("BBB", 5));
        sb.push(make_line("CCC", 5));
        sb.push(make_line("DDD", 5));

        // Shrink capacity
        sb.resize(2);

        assert_eq!(sb.max_lines(), 2);
        assert_eq!(sb.len(), 2);
        assert_eq!(sb.line_to_string(0).unwrap().trim(), "CCC");
        assert_eq!(sb.line_to_string(1).unwrap().trim(), "DDD");
    }

    #[test]
    fn test_scrollback_resize_cols() {
        let mut sb = Scrollback::new(5, 3);
        sb.push(make_line("ABC", 3));

        sb.resize_cols(5);

        assert_eq!(sb.cols(), 5);
        let line = sb.get(0).unwrap();
        assert_eq!(line.len(), 5);
        assert_eq!(line[0].c, 'A');
        assert_eq!(line[4].c, ' ');
    }

    #[test]
    fn test_scrollback_no_scrollback() {
        let mut sb = Scrollback::new(0, 5);
        sb.push(make_line("AAA", 5));

        assert_eq!(sb.len(), 0);
        assert!(sb.is_empty());
    }

    #[test]
    fn test_scrollback_to_string() {
        let mut sb = Scrollback::new(10, 3);
        sb.push(make_line("AAA", 3));
        sb.push(make_line("BBB", 3));
        sb.push(make_line("CCC", 3));

        let result = sb.to_string();
        assert_eq!(result, "AAA\nBBB\nCCC");
    }
}
