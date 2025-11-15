//! Terminal buffer management.
//!
//! This module provides the components for managing the terminal's visual state,
//! including the grid of cells, cursor position, and scrollback history.

pub mod cell;
pub mod cursor;
pub mod grid;
pub mod scrollback;

pub use cell::{Attributes, Cell, Color};
pub use cursor::{Cursor, CursorShape};
pub use grid::Grid;
pub use scrollback::Scrollback;

use serde::{Deserialize, Serialize};

/// A complete terminal buffer including visible grid, cursor, and scrollback.
///
/// This combines the grid, cursor, and scrollback buffer into a single
/// structure that represents the complete state of a terminal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalBuffer {
    /// The visible terminal grid.
    pub grid: Grid,
    /// The cursor position and state.
    pub cursor: Cursor,
    /// The scrollback buffer for history.
    pub scrollback: Scrollback,
    /// Current foreground color for new characters.
    pub current_fg: Color,
    /// Current background color for new characters.
    pub current_bg: Color,
    /// Current text attributes for new characters.
    pub current_attrs: Attributes,
}

impl TerminalBuffer {
    /// Creates a new terminal buffer with the specified dimensions.
    ///
    /// # Arguments
    /// * `rows` - Number of visible rows
    /// * `cols` - Number of columns
    /// * `scrollback_lines` - Maximum number of scrollback lines (0 for none)
    pub fn new(rows: usize, cols: usize, scrollback_lines: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols),
            cursor: Cursor::new(),
            scrollback: Scrollback::new(scrollback_lines, cols),
            current_fg: Color::Default,
            current_bg: Color::Default,
            current_attrs: Attributes::default(),
        }
    }

    /// Returns the dimensions of the visible terminal as (rows, cols).
    pub fn size(&self) -> (usize, usize) {
        self.grid.size()
    }

    /// Returns the number of visible rows.
    pub fn rows(&self) -> usize {
        self.grid.rows()
    }

    /// Returns the number of columns.
    pub fn cols(&self) -> usize {
        self.grid.cols()
    }

    /// Writes a character at the current cursor position with current styling.
    ///
    /// This does not advance the cursor automatically. Use `write_char_and_advance`
    /// for automatic cursor advancement.
    pub fn write_char(&mut self, c: char) {
        let cell = Cell::with_style(c, self.current_fg, self.current_bg, self.current_attrs);
        self.grid.set(self.cursor.row, self.cursor.col, cell);
    }

    /// Writes a character at the current cursor position and advances the cursor.
    ///
    /// If the cursor reaches the end of the line, it wraps to the next line.
    /// If the cursor reaches the bottom of the screen, the screen scrolls up.
    pub fn write_char_and_advance(&mut self, c: char) {
        self.write_char(c);
        self.advance_cursor();
    }

    /// Advances the cursor by one position, handling line wrapping and scrolling.
    pub fn advance_cursor(&mut self) {
        self.cursor.col += 1;

        // Handle line wrap
        if self.cursor.col >= self.cols() {
            self.cursor.col = 0;
            self.cursor.row += 1;

            // Handle scrolling at bottom of screen
            if self.cursor.row >= self.rows() {
                self.scroll_up();
                self.cursor.row = self.rows() - 1;
            }
        }
    }

    /// Scrolls the visible grid up by one line.
    ///
    /// The top line is moved to the scrollback buffer, and a new blank line
    /// is added at the bottom.
    pub fn scroll_up(&mut self) {
        let removed_line = self.grid.scroll_up();
        self.scrollback.push(removed_line);
    }

    /// Scrolls the visible grid down by one line.
    pub fn scroll_down(&mut self) {
        self.grid.scroll_down();
    }

    /// Moves the cursor to the specified position.
    pub fn goto(&mut self, row: usize, col: usize) {
        self.cursor.goto(row, col);
        self.cursor.clamp(self.rows(), self.cols());
    }

    /// Clears the visible screen.
    pub fn clear_screen(&mut self) {
        self.grid.clear();
    }

    /// Clears the current line.
    pub fn clear_line(&mut self) {
        self.grid.clear_row(self.cursor.row);
    }

    /// Clears from the cursor to the end of the line.
    pub fn clear_to_end_of_line(&mut self) {
        let row = self.cursor.row;
        let start_col = self.cursor.col;
        if let Some(cells) = self.grid.row_mut(row) {
            for col in start_col..cells.len() {
                cells[col].clear();
            }
        }
    }

    /// Clears from the cursor to the end of the screen.
    pub fn clear_to_end_of_screen(&mut self) {
        self.clear_to_end_of_line();
        for row in (self.cursor.row + 1)..self.rows() {
            self.grid.clear_row(row);
        }
    }

    /// Resizes the terminal buffer to new dimensions.
    ///
    /// This resizes both the visible grid and the scrollback buffer.
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        self.grid.resize(new_rows, new_cols);
        self.scrollback.resize_cols(new_cols);
        self.cursor.clamp(new_rows, new_cols);
    }

    /// Sets the current foreground color for subsequent characters.
    pub fn set_fg(&mut self, color: Color) {
        self.current_fg = color;
    }

    /// Sets the current background color for subsequent characters.
    pub fn set_bg(&mut self, color: Color) {
        self.current_bg = color;
    }

    /// Sets the current text attributes for subsequent characters.
    pub fn set_attrs(&mut self, attrs: Attributes) {
        self.current_attrs = attrs;
    }

    /// Resets the current colors and attributes to defaults.
    pub fn reset_style(&mut self) {
        self.current_fg = Color::Default;
        self.current_bg = Color::Default;
        self.current_attrs = Attributes::default();
    }

    /// Converts the visible grid to a string.
    pub fn to_string(&self) -> String {
        self.grid.to_string()
    }

    /// Returns the total number of lines available (visible + scrollback).
    pub fn total_lines(&self) -> usize {
        self.rows() + self.scrollback.len()
    }

    /// Gets a line from the complete buffer (scrollback + visible).
    ///
    /// Lines 0..scrollback.len() are from scrollback,
    /// lines scrollback.len()..total_lines() are from the visible grid.
    pub fn get_line(&self, line: usize) -> Option<String> {
        let scrollback_len = self.scrollback.len();

        if line < scrollback_len {
            self.scrollback.line_to_string(line)
        } else if line < self.total_lines() {
            let grid_row = line - scrollback_len;
            self.grid.row(grid_row).map(|cells| cells.iter().map(|c| c.c).collect())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_buffer_new() {
        let buf = TerminalBuffer::new(24, 80, 1000);
        assert_eq!(buf.size(), (24, 80));
        assert_eq!(buf.cursor.row, 0);
        assert_eq!(buf.cursor.col, 0);
    }

    #[test]
    fn test_write_char() {
        let mut buf = TerminalBuffer::new(3, 5, 10);
        buf.write_char('A');

        assert_eq!(buf.grid.get(0, 0).unwrap().c, 'A');
        assert_eq!(buf.cursor.col, 0); // Should not advance
    }

    #[test]
    fn test_write_char_and_advance() {
        let mut buf = TerminalBuffer::new(3, 5, 10);
        buf.write_char_and_advance('A');
        buf.write_char_and_advance('B');

        assert_eq!(buf.grid.get(0, 0).unwrap().c, 'A');
        assert_eq!(buf.grid.get(0, 1).unwrap().c, 'B');
        assert_eq!(buf.cursor.col, 2);
    }

    #[test]
    fn test_cursor_wrap() {
        let mut buf = TerminalBuffer::new(3, 3, 10);
        for _ in 0..3 {
            buf.advance_cursor();
        }

        assert_eq!(buf.cursor.row, 1);
        assert_eq!(buf.cursor.col, 0);
    }

    #[test]
    fn test_scroll_up() {
        let mut buf = TerminalBuffer::new(3, 3, 10);
        buf.grid.set(0, 0, Cell::new('A'));
        buf.grid.set(1, 0, Cell::new('B'));
        buf.grid.set(2, 0, Cell::new('C'));

        buf.scroll_up();

        assert_eq!(buf.grid.get(0, 0).unwrap().c, 'B');
        assert_eq!(buf.grid.get(1, 0).unwrap().c, 'C');
        assert_eq!(buf.grid.get(2, 0).unwrap().c, ' ');
        assert_eq!(buf.scrollback.len(), 1);
    }

    #[test]
    fn test_clear_screen() {
        let mut buf = TerminalBuffer::new(2, 2, 10);
        buf.grid.set(0, 0, Cell::new('A'));
        buf.grid.set(1, 1, Cell::new('B'));

        buf.clear_screen();

        assert_eq!(buf.grid.get(0, 0).unwrap().c, ' ');
        assert_eq!(buf.grid.get(1, 1).unwrap().c, ' ');
    }

    #[test]
    fn test_resize() {
        let mut buf = TerminalBuffer::new(2, 2, 10);
        buf.grid.set(0, 0, Cell::new('A'));

        buf.resize(3, 3);

        assert_eq!(buf.size(), (3, 3));
        assert_eq!(buf.grid.get(0, 0).unwrap().c, 'A');
    }

    #[test]
    fn test_set_colors() {
        let mut buf = TerminalBuffer::new(2, 2, 10);
        buf.set_fg(Color::Red);
        buf.set_bg(Color::Blue);
        buf.write_char('X');

        let cell = buf.grid.get(0, 0).unwrap();
        assert_eq!(cell.fg, Color::Red);
        assert_eq!(cell.bg, Color::Blue);
    }

    #[test]
    fn test_total_lines() {
        let mut buf = TerminalBuffer::new(3, 3, 10);
        assert_eq!(buf.total_lines(), 3);

        buf.scroll_up();
        assert_eq!(buf.total_lines(), 4); // 3 visible + 1 scrollback
    }
}
