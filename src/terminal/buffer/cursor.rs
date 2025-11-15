//! Terminal cursor state and movement.
//!
//! This module provides the [`Cursor`] type which tracks the position and
//! visual state of the terminal cursor.

use serde::{Deserialize, Serialize};

/// Cursor position and state in the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// Current column position (0-indexed).
    pub col: usize,
    /// Current row position (0-indexed).
    pub row: usize,
    /// Whether the cursor is visible.
    pub visible: bool,
    /// Cursor shape style.
    pub shape: CursorShape,
}

/// Cursor shape styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorShape {
    /// Block cursor (█).
    Block,
    /// Underline cursor (_).
    Underline,
    /// Vertical bar cursor (|).
    Bar,
}

impl Default for CursorShape {
    fn default() -> Self {
        CursorShape::Block
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            col: 0,
            row: 0,
            visible: true,
            shape: CursorShape::default(),
        }
    }
}

impl Cursor {
    /// Creates a new cursor at position (0, 0).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new cursor at the specified position.
    pub fn at(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            visible: true,
            shape: CursorShape::default(),
        }
    }

    /// Moves the cursor to the specified position.
    pub fn goto(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    /// Moves the cursor to the specified column on the current row.
    pub fn goto_col(&mut self, col: usize) {
        self.col = col;
    }

    /// Moves the cursor to the specified row, keeping the current column.
    pub fn goto_row(&mut self, row: usize) {
        self.row = row;
    }

    /// Moves the cursor up by the specified number of rows.
    ///
    /// The cursor will not move above row 0.
    pub fn move_up(&mut self, n: usize) {
        self.row = self.row.saturating_sub(n);
    }

    /// Moves the cursor down by the specified number of rows.
    pub fn move_down(&mut self, n: usize) {
        self.row = self.row.saturating_add(n);
    }

    /// Moves the cursor left by the specified number of columns.
    ///
    /// The cursor will not move before column 0.
    pub fn move_left(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }

    /// Moves the cursor right by the specified number of columns.
    pub fn move_right(&mut self, n: usize) {
        self.col = self.col.saturating_add(n);
    }

    /// Moves the cursor forward (right) by n positions, clamped to max_col.
    pub fn forward(&mut self, n: usize, max_col: usize) {
        self.col = (self.col + n).min(max_col);
    }

    /// Moves the cursor backward (left) by n positions.
    pub fn backward(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }

    /// Moves the cursor to the specified position.
    pub fn move_to(&mut self, row: usize, col: usize) {
        self.goto(row, col);
    }

    /// Moves the cursor up by n rows.
    pub fn up(&mut self, n: usize) {
        self.move_up(n);
    }

    /// Moves the cursor down by n rows.
    pub fn down(&mut self, n: usize) {
        self.move_down(n);
    }

    /// Moves the cursor to the beginning of the current line.
    pub fn carriage_return(&mut self) {
        self.col = 0;
    }

    /// Moves the cursor to the next line (down one row and to column 0).
    pub fn newline(&mut self) {
        self.row = self.row.saturating_add(1);
        self.col = 0;
    }

    /// Moves the cursor to the beginning of the line (column 0).
    pub fn line_start(&mut self) {
        self.col = 0;
    }

    /// Clamps the cursor position to stay within the given bounds.
    ///
    /// # Arguments
    /// * `rows` - Maximum number of rows (height)
    /// * `cols` - Maximum number of columns (width)
    pub fn clamp(&mut self, rows: usize, cols: usize) {
        if rows > 0 {
            self.row = self.row.min(rows - 1);
        }
        if cols > 0 {
            self.col = self.col.min(cols - 1);
        }
    }

    /// Shows the cursor.
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hides the cursor.
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Sets the cursor shape.
    pub fn set_shape(&mut self, shape: CursorShape) {
        self.shape = shape;
    }

    /// Resets the cursor to the default state (position 0,0, visible).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_default() {
        let cursor = Cursor::default();
        assert_eq!(cursor.row, 0);
        assert_eq!(cursor.col, 0);
        assert!(cursor.visible);
        assert_eq!(cursor.shape, CursorShape::Block);
    }

    #[test]
    fn test_cursor_at() {
        let cursor = Cursor::at(5, 10);
        assert_eq!(cursor.row, 5);
        assert_eq!(cursor.col, 10);
    }

    #[test]
    fn test_cursor_goto() {
        let mut cursor = Cursor::new();
        cursor.goto(3, 7);
        assert_eq!(cursor.row, 3);
        assert_eq!(cursor.col, 7);
    }

    #[test]
    fn test_cursor_movement() {
        let mut cursor = Cursor::at(5, 5);

        cursor.move_up(2);
        assert_eq!(cursor.row, 3);

        cursor.move_down(4);
        assert_eq!(cursor.row, 7);

        cursor.move_left(3);
        assert_eq!(cursor.col, 2);

        cursor.move_right(6);
        assert_eq!(cursor.col, 8);
    }

    #[test]
    fn test_cursor_saturating() {
        let mut cursor = Cursor::at(1, 1);

        // Moving up past 0 should saturate at 0
        cursor.move_up(5);
        assert_eq!(cursor.row, 0);

        // Moving left past 0 should saturate at 0
        cursor.move_left(5);
        assert_eq!(cursor.col, 0);
    }

    #[test]
    fn test_cursor_newline() {
        let mut cursor = Cursor::at(2, 5);
        cursor.newline();
        assert_eq!(cursor.row, 3);
        assert_eq!(cursor.col, 0);
    }

    #[test]
    fn test_cursor_carriage_return() {
        let mut cursor = Cursor::at(2, 5);
        cursor.carriage_return();
        assert_eq!(cursor.row, 2);
        assert_eq!(cursor.col, 0);
    }

    #[test]
    fn test_cursor_clamp() {
        let mut cursor = Cursor::at(100, 100);
        cursor.clamp(24, 80);
        assert_eq!(cursor.row, 23);
        assert_eq!(cursor.col, 79);
    }

    #[test]
    fn test_cursor_visibility() {
        let mut cursor = Cursor::new();
        assert!(cursor.visible);

        cursor.hide();
        assert!(!cursor.visible);

        cursor.show();
        assert!(cursor.visible);
    }

    #[test]
    fn test_cursor_reset() {
        let mut cursor = Cursor::at(10, 20);
        cursor.hide();
        cursor.reset();

        assert_eq!(cursor.row, 0);
        assert_eq!(cursor.col, 0);
        assert!(cursor.visible);
    }
}
