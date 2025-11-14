/// Editor state management module
///
/// This module manages the core editor state including:
/// - Text buffer (lines of text)
/// - Cursor position
/// - Selection
/// - Scroll position
/// - Editing operations

use std::cmp::{max, min};

/// Represents a position in the editor (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    pub fn zero() -> Self {
        Self { line: 0, col: 0 }
    }
}

/// Editor state
#[derive(Debug, Clone)]
pub struct EditorState {
    /// Lines of text in the buffer
    lines: Vec<String>,

    /// Current cursor position
    cursor: Position,

    /// Selection range (if any)
    selection: Option<(Position, Position)>,

    /// Vertical scroll offset (in lines)
    scroll_offset: f64,

    /// Horizontal scroll offset (in pixels)
    scroll_x: f64,

    /// Modified flag
    modified: bool,

    /// File path (if any)
    file_path: Option<String>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    /// Create a new empty editor state
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Position::zero(),
            selection: None,
            scroll_offset: 0.0,
            scroll_x: 0.0,
            modified: false,
            file_path: None,
        }
    }

    /// Create editor state with initial content
    pub fn with_text(text: &str) -> Self {
        let lines: Vec<String> = if text.is_empty() {
            vec![String::new()]
        } else {
            text.lines().map(|s| s.to_string()).collect()
        };

        Self {
            lines,
            cursor: Position::zero(),
            selection: None,
            scroll_offset: 0.0,
            scroll_x: 0.0,
            modified: false,
            file_path: None,
        }
    }

    /// Get all lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Get total number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get a specific line
    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    /// Get cursor position
    pub fn cursor(&self) -> Position {
        self.cursor
    }

    /// Get selection range
    pub fn selection(&self) -> Option<(Position, Position)> {
        self.selection
    }

    /// Get scroll offset
    pub fn scroll_offset(&self) -> f64 {
        self.scroll_offset
    }

    /// Set scroll offset
    pub fn set_scroll_offset(&mut self, offset: f64) {
        self.scroll_offset = offset.max(0.0);
    }

    /// Get horizontal scroll
    pub fn scroll_x(&self) -> f64 {
        self.scroll_x
    }

    /// Check if modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get file path
    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        self.delete_selection();

        let line_idx = self.cursor.line;
        if line_idx >= self.lines.len() {
            return;
        }

        let line = &mut self.lines[line_idx];
        let col = self.cursor.col.min(line.len());

        line.insert(col, c);
        self.cursor.col += 1;
        self.modified = true;
    }

    /// Insert string at cursor
    pub fn insert_string(&mut self, s: &str) {
        self.delete_selection();

        let line_idx = self.cursor.line;
        if line_idx >= self.lines.len() {
            return;
        }

        if s.contains('\n') {
            // Handle multi-line insert
            let parts: Vec<&str> = s.split('\n').collect();
            let line = &mut self.lines[line_idx];
            let col = self.cursor.col.min(line.len());

            let after = line[col..].to_string();
            line.truncate(col);
            line.push_str(parts[0]);

            for (i, part) in parts[1..].iter().enumerate() {
                if i == parts.len() - 2 {
                    // Last part
                    self.lines.insert(line_idx + i + 1, format!("{}{}", part, after));
                    self.cursor.line = line_idx + i + 1;
                    self.cursor.col = part.len();
                } else {
                    self.lines.insert(line_idx + i + 1, part.to_string());
                }
            }
        } else {
            // Single line insert
            let line = &mut self.lines[line_idx];
            let col = self.cursor.col.min(line.len());
            line.insert_str(col, s);
            self.cursor.col += s.len();
        }

        self.modified = true;
    }

    /// Delete character before cursor (backspace)
    pub fn delete_backward(&mut self) {
        if self.delete_selection() {
            return;
        }

        if self.cursor.col > 0 {
            // Delete within line
            let line = &mut self.lines[self.cursor.line];
            let col = self.cursor.col.min(line.len());
            if col > 0 {
                line.remove(col - 1);
                self.cursor.col -= 1;
                self.modified = true;
            }
        } else if self.cursor.line > 0 {
            // Join with previous line
            let current_line = self.lines.remove(self.cursor.line);
            self.cursor.line -= 1;
            self.cursor.col = self.lines[self.cursor.line].len();
            self.lines[self.cursor.line].push_str(&current_line);
            self.modified = true;
        }
    }

    /// Delete character at cursor (delete key)
    pub fn delete_forward(&mut self) {
        if self.delete_selection() {
            return;
        }

        let line_idx = self.cursor.line;
        if line_idx >= self.lines.len() {
            return;
        }

        let line = &mut self.lines[line_idx];
        let col = self.cursor.col.min(line.len());

        if col < line.len() {
            // Delete within line
            line.remove(col);
            self.modified = true;
        } else if line_idx < self.lines.len() - 1 {
            // Join with next line
            let next_line = self.lines.remove(line_idx + 1);
            self.lines[line_idx].push_str(&next_line);
            self.modified = true;
        }
    }

    /// Delete current selection, returns true if something was deleted
    fn delete_selection(&mut self) -> bool {
        if let Some((start, end)) = self.selection {
            let (start, end) = self.normalize_selection(start, end);

            if start.line == end.line {
                // Single line selection
                let line = &mut self.lines[start.line];
                let start_col = start.col.min(line.len());
                let end_col = end.col.min(line.len());
                line.drain(start_col..end_col);
                self.cursor = start;
            } else {
                // Multi-line selection
                let first_line = &mut self.lines[start.line];
                let start_col = start.col.min(first_line.len());
                let after = first_line[start_col..].to_string();
                first_line.truncate(start_col);

                let last_line = &self.lines[end.line];
                let end_col = end.col.min(last_line.len());
                let remaining = last_line[end_col..].to_string();

                // Remove lines in between
                self.lines.drain(start.line + 1..=end.line);

                // Append remaining text to first line
                self.lines[start.line].push_str(&remaining);
                self.cursor = start;
            }

            self.selection = None;
            self.modified = true;
            true
        } else {
            false
        }
    }

    /// Normalize selection so start is always before end
    fn normalize_selection(&self, pos1: Position, pos2: Position) -> (Position, Position) {
        if pos1.line < pos2.line || (pos1.line == pos2.line && pos1.col < pos2.col) {
            (pos1, pos2)
        } else {
            (pos2, pos1)
        }
    }

    /// Insert newline at cursor
    pub fn insert_newline(&mut self) {
        self.delete_selection();

        let line_idx = self.cursor.line;
        if line_idx >= self.lines.len() {
            return;
        }

        let line = &mut self.lines[line_idx];
        let col = self.cursor.col.min(line.len());

        let after = line[col..].to_string();
        line.truncate(col);

        self.lines.insert(line_idx + 1, after);
        self.cursor.line += 1;
        self.cursor.col = 0;
        self.modified = true;
    }

    /// Move cursor up
    pub fn move_up(&mut self, extend_selection: bool) {
        if extend_selection && self.selection.is_none() {
            self.selection = Some((self.cursor, self.cursor));
        }

        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            let line_len = self.lines[self.cursor.line].len();
            self.cursor.col = self.cursor.col.min(line_len);
        }

        if extend_selection {
            if let Some((start, _)) = self.selection {
                self.selection = Some((start, self.cursor));
            }
        } else {
            self.selection = None;
        }
    }

    /// Move cursor down
    pub fn move_down(&mut self, extend_selection: bool) {
        if extend_selection && self.selection.is_none() {
            self.selection = Some((self.cursor, self.cursor));
        }

        if self.cursor.line < self.lines.len() - 1 {
            self.cursor.line += 1;
            let line_len = self.lines[self.cursor.line].len();
            self.cursor.col = self.cursor.col.min(line_len);
        }

        if extend_selection {
            if let Some((start, _)) = self.selection {
                self.selection = Some((start, self.cursor));
            }
        } else {
            self.selection = None;
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self, extend_selection: bool) {
        if extend_selection && self.selection.is_none() {
            self.selection = Some((self.cursor, self.cursor));
        }

        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.col = self.lines[self.cursor.line].len();
        }

        if extend_selection {
            if let Some((start, _)) = self.selection {
                self.selection = Some((start, self.cursor));
            }
        } else {
            self.selection = None;
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self, extend_selection: bool) {
        if extend_selection && self.selection.is_none() {
            self.selection = Some((self.cursor, self.cursor));
        }

        let line_len = self.lines[self.cursor.line].len();
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.line < self.lines.len() - 1 {
            self.cursor.line += 1;
            self.cursor.col = 0;
        }

        if extend_selection {
            if let Some((start, _)) = self.selection {
                self.selection = Some((start, self.cursor));
            }
        } else {
            self.selection = None;
        }
    }

    /// Move cursor to start of line
    pub fn move_line_start(&mut self, extend_selection: bool) {
        if extend_selection && self.selection.is_none() {
            self.selection = Some((self.cursor, self.cursor));
        }

        self.cursor.col = 0;

        if extend_selection {
            if let Some((start, _)) = self.selection {
                self.selection = Some((start, self.cursor));
            }
        } else {
            self.selection = None;
        }
    }

    /// Move cursor to end of line
    pub fn move_line_end(&mut self, extend_selection: bool) {
        if extend_selection && self.selection.is_none() {
            self.selection = Some((self.cursor, self.cursor));
        }

        self.cursor.col = self.lines[self.cursor.line].len();

        if extend_selection {
            if let Some((start, _)) = self.selection {
                self.selection = Some((start, self.cursor));
            }
        } else {
            self.selection = None;
        }
    }

    /// Select all text
    pub fn select_all(&mut self) {
        let start = Position::zero();
        let end_line = self.lines.len() - 1;
        let end_col = self.lines[end_line].len();
        let end = Position::new(end_line, end_col);

        self.selection = Some((start, end));
        self.cursor = end;
    }

    /// Get selected text
    pub fn get_selected_text(&self) -> Option<String> {
        self.selection.map(|(start, end)| {
            let (start, end) = self.normalize_selection(start, end);

            if start.line == end.line {
                let line = &self.lines[start.line];
                let start_col = start.col.min(line.len());
                let end_col = end.col.min(line.len());
                line[start_col..end_col].to_string()
            } else {
                let mut result = String::new();

                // First line
                let first_line = &self.lines[start.line];
                let start_col = start.col.min(first_line.len());
                result.push_str(&first_line[start_col..]);
                result.push('\n');

                // Middle lines
                for i in start.line + 1..end.line {
                    result.push_str(&self.lines[i]);
                    result.push('\n');
                }

                // Last line
                let last_line = &self.lines[end.line];
                let end_col = end.col.min(last_line.len());
                result.push_str(&last_line[..end_col]);

                result
            }
        })
    }

    /// Get all text as a single string
    pub fn get_text(&self) -> String {
        self.lines.join("\n")
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, pos: Position) {
        self.cursor.line = pos.line.min(self.lines.len() - 1);
        self.cursor.col = pos.col.min(self.lines[self.cursor.line].len());
    }
}
