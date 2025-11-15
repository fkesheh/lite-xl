//! Terminal clipboard operations
//!
//! This module provides clipboard functionality for terminal selections:
//! - Text selection tracking
//! - Clipboard copy/paste operations
//! - Selection rectangle calculation
//! - Integration with system clipboard

use crate::terminal::buffer::Grid;
use std::ops::Range;

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Normal selection (character-based)
    Normal,
    /// Block selection (rectangular)
    Block,
    /// Line selection (whole lines)
    Line,
}

/// Terminal selection
#[derive(Debug, Clone)]
pub struct Selection {
    /// Selection mode
    pub mode: SelectionMode,
    /// Start position (row, col)
    pub start: (usize, usize),
    /// End position (row, col)
    pub end: (usize, usize),
    /// Whether selection is active
    pub active: bool,
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self {
            mode: SelectionMode::Normal,
            start: (0, 0),
            end: (0, 0),
            active: false,
        }
    }

    /// Start a new selection
    pub fn start(&mut self, row: usize, col: usize, mode: SelectionMode) {
        self.mode = mode;
        self.start = (row, col);
        self.end = (row, col);
        self.active = true;
    }

    /// Update selection end point
    pub fn update(&mut self, row: usize, col: usize) {
        if self.active {
            self.end = (row, col);
        }
    }

    /// Finalize selection
    pub fn finalize(&mut self) {
        // Selection remains active but is finalized
    }

    /// Clear selection
    pub fn clear(&mut self) {
        self.active = false;
        self.start = (0, 0);
        self.end = (0, 0);
    }

    /// Get normalized selection bounds (start always before end)
    pub fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        let (start, end) = if self.start <= self.end {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        };

        (start, end)
    }

    /// Check if a cell is within the selection
    pub fn contains(&self, row: usize, col: usize) -> bool {
        if !self.active {
            return false;
        }

        let ((start_row, start_col), (end_row, end_col)) = self.bounds();

        match self.mode {
            SelectionMode::Normal => {
                if row < start_row || row > end_row {
                    return false;
                }
                if row == start_row && row == end_row {
                    col >= start_col && col <= end_col
                } else if row == start_row {
                    col >= start_col
                } else if row == end_row {
                    col <= end_col
                } else {
                    true
                }
            }
            SelectionMode::Block => {
                row >= start_row && row <= end_row && col >= start_col && col <= end_col
            }
            SelectionMode::Line => row >= start_row && row <= end_row,
        }
    }

    /// Get selected rows range
    pub fn row_range(&self) -> Range<usize> {
        if !self.active {
            return 0..0;
        }

        let ((start_row, _), (end_row, _)) = self.bounds();
        start_row..(end_row + 1)
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

/// Terminal clipboard manager
pub struct TerminalClipboard {
    /// Current selection
    selection: Selection,
    /// Internal clipboard buffer
    buffer: String,
}

impl TerminalClipboard {
    /// Create a new clipboard manager
    pub fn new() -> Self {
        Self {
            selection: Selection::new(),
            buffer: String::new(),
        }
    }

    /// Start a selection
    pub fn start_selection(&mut self, row: usize, col: usize, mode: SelectionMode) {
        self.selection.start(row, col, mode);
    }

    /// Update selection end point
    pub fn update_selection(&mut self, row: usize, col: usize) {
        self.selection.update(row, col);
    }

    /// Finalize selection
    pub fn finalize_selection(&mut self) {
        self.selection.finalize();
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    /// Get current selection
    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.selection.active
    }

    /// Copy selected text from grid to clipboard
    pub fn copy_selection(&mut self, grid: &Grid) -> String {
        if !self.selection.active {
            return String::new();
        }

        let mut text = String::new();
        let ((start_row, start_col), (end_row, end_col)) = self.selection.bounds();

        match self.selection.mode {
            SelectionMode::Normal => {
                for row in start_row..=end_row {
                    if let Some(cells) = grid.row(row) {
                        let start = if row == start_row { start_col } else { 0 };
                        let end = if row == end_row {
                            end_col.min(cells.len().saturating_sub(1))
                        } else {
                            cells.len().saturating_sub(1)
                        };

                        for col in start..=end {
                            if let Some(cell) = cells.get(col) {
                                text.push(cell.c);
                            }
                        }

                        if row < end_row {
                            text.push('\n');
                        }
                    }
                }
            }
            SelectionMode::Block => {
                for row in start_row..=end_row {
                    if let Some(cells) = grid.row(row) {
                        for col in start_col..=end_col {
                            if let Some(cell) = cells.get(col) {
                                text.push(cell.c);
                            }
                        }
                        if row < end_row {
                            text.push('\n');
                        }
                    }
                }
            }
            SelectionMode::Line => {
                for row in start_row..=end_row {
                    if let Some(cells) = grid.row(row) {
                        for cell in cells {
                            text.push(cell.c);
                        }
                        if row < end_row {
                            text.push('\n');
                        }
                    }
                }
            }
        }

        // Trim trailing spaces from each line
        text = text
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");

        self.buffer = text.clone();
        text
    }

    /// Get clipboard text
    pub fn get_text(&self) -> &str {
        &self.buffer
    }

    /// Set clipboard text
    pub fn set_text(&mut self, text: String) {
        self.buffer = text;
    }

    /// Paste text (returns text to be inserted)
    pub fn paste(&self) -> &str {
        &self.buffer
    }
}

impl Default for TerminalClipboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_new() {
        let sel = Selection::new();
        assert!(!sel.active);
        assert_eq!(sel.mode, SelectionMode::Normal);
    }

    #[test]
    fn test_selection_start() {
        let mut sel = Selection::new();
        sel.start(1, 2, SelectionMode::Normal);
        assert!(sel.active);
        assert_eq!(sel.start, (1, 2));
        assert_eq!(sel.end, (1, 2));
    }

    #[test]
    fn test_selection_update() {
        let mut sel = Selection::new();
        sel.start(0, 0, SelectionMode::Normal);
        sel.update(2, 5);
        assert_eq!(sel.end, (2, 5));
    }

    #[test]
    fn test_selection_bounds() {
        let mut sel = Selection::new();
        sel.start(3, 5, SelectionMode::Normal);
        sel.update(1, 2);

        let ((start_row, start_col), (end_row, end_col)) = sel.bounds();
        assert_eq!(start_row, 1);
        assert_eq!(start_col, 2);
        assert_eq!(end_row, 3);
        assert_eq!(end_col, 5);
    }

    #[test]
    fn test_selection_contains_normal() {
        let mut sel = Selection::new();
        sel.start(1, 2, SelectionMode::Normal);
        sel.update(3, 4);

        assert!(!sel.contains(0, 5));
        assert!(sel.contains(1, 2));
        assert!(sel.contains(2, 0));
        assert!(sel.contains(3, 4));
        assert!(!sel.contains(3, 5));
        assert!(!sel.contains(4, 0));
    }

    #[test]
    fn test_selection_contains_block() {
        let mut sel = Selection::new();
        sel.start(1, 2, SelectionMode::Block);
        sel.update(3, 4);

        assert!(sel.contains(1, 2));
        assert!(sel.contains(2, 3));
        assert!(!sel.contains(2, 1));
        assert!(!sel.contains(2, 5));
        assert!(!sel.contains(0, 3));
        assert!(!sel.contains(4, 3));
    }

    #[test]
    fn test_clipboard_new() {
        let clipboard = TerminalClipboard::new();
        assert!(!clipboard.has_selection());
        assert_eq!(clipboard.get_text(), "");
    }

    #[test]
    fn test_clipboard_selection() {
        let mut clipboard = TerminalClipboard::new();
        clipboard.start_selection(0, 0, SelectionMode::Normal);
        assert!(clipboard.has_selection());

        clipboard.clear_selection();
        assert!(!clipboard.has_selection());
    }

    #[test]
    fn test_clipboard_copy_normal() {
        let mut clipboard = TerminalClipboard::new();
        let grid = Grid::new(10, 5);

        clipboard.start_selection(0, 0, SelectionMode::Normal);
        clipboard.update_selection(0, 5);

        let text = clipboard.copy_selection(&grid);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_clipboard_text_operations() {
        let mut clipboard = TerminalClipboard::new();

        clipboard.set_text("Hello, World!".to_string());
        assert_eq!(clipboard.get_text(), "Hello, World!");
        assert_eq!(clipboard.paste(), "Hello, World!");
    }
}
