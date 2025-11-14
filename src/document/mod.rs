//! Document abstraction combining buffer, selections, and undo system.
//!
//! A document represents an editable text file with its associated editing state,
//! including cursor positions, selections, and undo history.

pub mod selection;

pub use selection::{Selection, Selections};

use crate::buffer::{Buffer, BufferError, BufferId, LineEnding, Position, Range};
use crate::undo::{Edit, UndoConfig, UndoStack};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur during document operations.
#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("Buffer error: {0}")]
    Buffer(#[from] BufferError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No file path associated with document")]
    NoPath,

    #[error("Document has unsaved changes")]
    UnsavedChanges,
}

/// Result type for document operations.
pub type Result<T> = std::result::Result<T, DocumentError>;

/// Document-specific settings.
///
/// These settings can override global editor settings on a per-document basis.
#[derive(Debug, Clone)]
pub struct DocumentSettings {
    /// Number of spaces per tab
    pub tab_width: usize,

    /// Use spaces instead of tabs
    pub use_spaces: bool,

    /// Auto-detect indentation from content
    pub auto_indent: bool,

    /// Show line numbers in the gutter
    pub show_line_numbers: bool,

    /// Highlight the current line
    pub highlight_current_line: bool,

    /// Column position for line length guide (None = disabled)
    pub line_length_guide: Option<usize>,

    /// Trim trailing whitespace on save
    pub trim_trailing_whitespace: bool,

    /// Ensure newline at end of file
    pub ensure_final_newline: bool,
}

impl Default for DocumentSettings {
    fn default() -> Self {
        Self {
            tab_width: 4,
            use_spaces: true,
            auto_indent: true,
            show_line_numbers: true,
            highlight_current_line: true,
            line_length_guide: Some(80),
            trim_trailing_whitespace: false,
            ensure_final_newline: true,
        }
    }
}

/// Cursor movement operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    /// Move left by one character
    Left,
    /// Move right by one character
    Right,
    /// Move up by one line
    Up,
    /// Move down by one line
    Down,
    /// Move to start of line
    LineStart,
    /// Move to end of line
    LineEnd,
    /// Move left by one word
    WordLeft,
    /// Move right by one word
    WordRight,
    /// Move to start of document
    DocumentStart,
    /// Move to end of document
    DocumentEnd,
    /// Move up one page
    PageUp,
    /// Move down one page
    PageDown,
}

/// A document represents an editable text buffer with associated state.
///
/// The document combines:
/// - A text buffer (using rope data structure)
/// - Cursor positions and selections (with multi-cursor support)
/// - Undo/redo history
/// - Document-specific settings
/// - File association and modification tracking
///
/// # Examples
///
/// ```
/// use lite_xl::document::Document;
/// use lite_xl::buffer::Position;
///
/// let mut doc = Document::new();
/// doc.insert("Hello, world!");
/// assert_eq!(doc.buffer().to_string(), "Hello, world!");
///
/// doc.undo();
/// assert_eq!(doc.buffer().to_string(), "");
/// ```
pub struct Document {
    /// The underlying text buffer
    buffer: Buffer,

    /// Current selections/cursors
    selections: Selections,

    /// Undo/redo stack
    undo_stack: UndoStack,

    /// Scroll position (in lines)
    scroll_offset: f32,

    /// Document-specific settings
    settings: DocumentSettings,

    /// Saved version (for tracking modifications)
    saved_version: u64,
}

impl Document {
    /// Create a new empty document.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Document;
    ///
    /// let doc = Document::new();
    /// assert!(!doc.is_modified());
    /// ```
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            selections: Selections::single(Position::zero()),
            undo_stack: UndoStack::default(),
            scroll_offset: 0.0,
            settings: DocumentSettings::default(),
            saved_version: 0,
        }
    }

    /// Create a document from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Document;
    ///
    /// let doc = Document::from_str("Hello\nWorld");
    /// assert_eq!(doc.buffer().line_count(), 2);
    /// ```
    pub fn from_str(text: &str) -> Self {
        Self {
            buffer: Buffer::from_str(text),
            selections: Selections::single(Position::zero()),
            undo_stack: UndoStack::default(),
            scroll_offset: 0.0,
            settings: DocumentSettings::default(),
            saved_version: 0,
        }
    }

    /// Create a document from a file.
    ///
    /// This reads the file contents and detects line endings automatically.
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        
        let mut buffer = Buffer::from_str(&content);
        buffer.set_path(Some(path));
        
        Ok(Self {
            buffer,
            selections: Selections::single(Position::zero()),
            undo_stack: UndoStack::default(),
            scroll_offset: 0.0,
            settings: DocumentSettings::default(),
            saved_version: 0,
        })
    }

    /// Get the buffer ID.
    pub fn id(&self) -> BufferId {
        self.buffer.id()
    }

    /// Get a reference to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Document;
    ///
    /// let doc = Document::from_str("Hello");
    /// assert_eq!(doc.buffer().to_string(), "Hello");
    /// ```
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get a mutable reference to the buffer.
    ///
    /// Warning: Direct buffer modifications bypass undo tracking.
    /// Use the document's editing methods instead when possible.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Get the current selections.
    pub fn selections(&self) -> &Selections {
        &self.selections
    }

    /// Get mutable selections.
    pub fn selections_mut(&mut self) -> &mut Selections {
        &mut self.selections
    }

    /// Set the selections.
    pub fn set_selections(&mut self, selections: Selections) {
        self.selections = selections;
    }

    /// Get the document settings.
    pub fn settings(&self) -> &DocumentSettings {
        &self.settings
    }

    /// Get mutable document settings.
    pub fn settings_mut(&mut self) -> &mut DocumentSettings {
        &mut self.settings
    }

    /// Get the file path, if any.
    pub fn path(&self) -> Option<&Path> {
        self.buffer.path().map(|p| p.as_path())
    }

    /// Set the file path.
    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.buffer.set_path(path);
    }

    /// Get the scroll offset.
    pub fn scroll_offset(&self) -> f32 {
        self.scroll_offset
    }

    /// Set the scroll offset.
    pub fn set_scroll_offset(&mut self, offset: f32) {
        self.scroll_offset = offset.max(0.0);
    }

    /// Check if the document has been modified since last save.
    pub fn is_modified(&self) -> bool {
        self.buffer.version() != self.saved_version
    }

    /// Insert text at all cursor positions.
    ///
    /// This inserts the text at each cursor, supporting multi-cursor editing.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Document;
    ///
    /// let mut doc = Document::new();
    /// doc.insert("Hello, world!");
    /// assert_eq!(doc.buffer().to_string(), "Hello, world!");
    /// ```
    pub fn insert(&mut self, text: &str) {
        let mut offset_adjustment = 0isize;
        let mut new_selections = Vec::new();

        // Sort selections by position
        let mut sorted_selections: Vec<_> = self.selections.iter().collect();
        sorted_selections.sort_by_key(|s| s.range().start);

        for selection in sorted_selections {
            let pos = if offset_adjustment >= 0 {
                Position::new(
                    selection.head().line,
                    (selection.head().column as isize + offset_adjustment) as usize,
                )
            } else {
                Position::new(
                    selection.head().line,
                    selection.head().column.saturating_sub((-offset_adjustment) as usize),
                )
            };

            // Record edit for undo
            let edit = Edit::Insert {
                position: pos,
                text: text.to_string(),
            };
            self.undo_stack.push(edit, self.selections.clone());

            // Apply to buffer
            if let Ok(()) = self.buffer.insert(pos, text) {
                // Calculate new cursor position
                let new_cursor = Position::new(pos.line, pos.column + text.len());
                new_selections.push(Selection::cursor(new_cursor));
                
                // Track offset for next cursor
                offset_adjustment += text.len() as isize;
            }
        }

        if !new_selections.is_empty() {
            self.selections = Selections::from_vec(new_selections);
        }
    }

    /// Delete selected text or character at cursor.
    ///
    /// If there's a selection, deletes the selected text.
    /// Otherwise, deletes the character after the cursor (like Delete key).
    pub fn delete(&mut self) {
        let mut new_selections = Vec::new();

        for selection in self.selections.iter() {
            if selection.is_cursor() {
                // Delete character after cursor
                let pos = selection.head();
                let char_range = Range::new(pos, pos.offset_column(1));
                
                if let Ok(deleted) = self.buffer.delete(char_range) {
                    let edit = Edit::Delete {
                        range: char_range,
                        deleted_text: deleted,
                    };
                    self.undo_stack.push(edit, self.selections.clone());
                    new_selections.push(Selection::cursor(pos));
                }
            } else {
                // Delete selection
                let range = selection.range();
                if let Ok(deleted) = self.buffer.delete(range) {
                    let edit = Edit::Delete {
                        range,
                        deleted_text: deleted,
                    };
                    self.undo_stack.push(edit, self.selections.clone());
                    new_selections.push(Selection::cursor(range.start));
                }
            }
        }

        if !new_selections.is_empty() {
            self.selections = Selections::from_vec(new_selections);
        }
    }

    /// Delete backward (backspace).
    ///
    /// If there's a selection, deletes the selected text.
    /// Otherwise, deletes the character before the cursor.
    pub fn delete_backward(&mut self) {
        let mut new_selections = Vec::new();

        for selection in self.selections.iter() {
            if selection.is_cursor() {
                // Delete character before cursor
                let pos = selection.head();
                if pos.column > 0 {
                    let char_range = Range::new(pos.offset_column(-1), pos);
                    
                    if let Ok(deleted) = self.buffer.delete(char_range) {
                        let edit = Edit::Delete {
                            range: char_range,
                            deleted_text: deleted,
                        };
                        self.undo_stack.push(edit, self.selections.clone());
                        new_selections.push(Selection::cursor(char_range.start));
                    }
                } else if pos.line > 0 {
                    // Delete newline at end of previous line
                    let prev_line_end = Position::new(pos.line - 1, 
                        self.buffer.line_len(pos.line - 1).unwrap_or(0).saturating_sub(1));
                    let char_range = Range::new(prev_line_end, pos);
                    
                    if let Ok(deleted) = self.buffer.delete(char_range) {
                        let edit = Edit::Delete {
                            range: char_range,
                            deleted_text: deleted,
                        };
                        self.undo_stack.push(edit, self.selections.clone());
                        new_selections.push(Selection::cursor(char_range.start));
                    }
                }
            } else {
                // Delete selection
                let range = selection.range();
                if let Ok(deleted) = self.buffer.delete(range) {
                    let edit = Edit::Delete {
                        range,
                        deleted_text: deleted,
                    };
                    self.undo_stack.push(edit, self.selections.clone());
                    new_selections.push(Selection::cursor(range.start));
                }
            }
        }

        if !new_selections.is_empty() {
            self.selections = Selections::from_vec(new_selections);
        }
    }

    /// Move cursor(s) without extending selection.
    pub fn move_cursor(&mut self, movement: Movement) {
        let buffer = &self.buffer;
        self.selections.transform(|sel| {
            let new_pos = calculate_movement_helper(buffer, sel.head(), movement);
            Selection::cursor(new_pos)
        });
    }

    /// Extend selection(s) in the given direction.
    pub fn select(&mut self, movement: Movement) {
        let buffer = &self.buffer;
        self.selections.transform(|sel| {
            let mut new_sel = sel.clone();
            let new_cursor = calculate_movement_helper(buffer, sel.head(), movement);
            new_sel.extend_to(new_cursor);
            new_sel
        });
    }

    /// Calculate new position for a movement.
    fn calculate_movement(&self, pos: Position, movement: Movement) -> Position {
        calculate_movement_helper(&self.buffer, pos, movement)
    }
}

/// Helper function for calculating movement without borrowing self
fn calculate_movement_helper(buffer: &Buffer, pos: Position, movement: Movement) -> Position {
    match movement {
        Movement::Left => {
            if pos.column > 0 {
                pos.offset_column(-1)
            } else if pos.line > 0 {
                let prev_line_len = buffer.line_len(pos.line - 1).unwrap_or(0);
                Position::new(pos.line - 1, prev_line_len.saturating_sub(1))
            } else {
                pos
            }
        }
        Movement::Right => {
            let line_len = buffer.line_len(pos.line).unwrap_or(0);
            if pos.column < line_len.saturating_sub(1) {
                pos.offset_column(1)
            } else if pos.line < buffer.line_count().saturating_sub(1) {
                Position::new(pos.line + 1, 0)
            } else {
                pos
            }
        }
        Movement::Up => {
            if pos.line > 0 {
                pos.offset_line(-1)
            } else {
                pos
            }
        }
        Movement::Down => {
            if pos.line < buffer.line_count().saturating_sub(1) {
                pos.offset_line(1)
            } else {
                pos
            }
        }
        Movement::LineStart => pos.to_line_start(),
        Movement::LineEnd => {
            let line_len = buffer.line_len(pos.line).unwrap_or(0);
            Position::new(pos.line, line_len.saturating_sub(1).max(0))
        }
        Movement::DocumentStart => Position::zero(),
        Movement::DocumentEnd => {
            let last_line = buffer.line_count().saturating_sub(1);
            let last_line_len = buffer.line_len(last_line).unwrap_or(0);
            Position::new(last_line, last_line_len.saturating_sub(1).max(0))
        }
        Movement::WordLeft | Movement::WordRight => {
            // Simplified word movement (can be enhanced)
            if movement == Movement::WordLeft {
                pos.offset_column(-1)
            } else {
                pos.offset_column(1)
            }
        }
        Movement::PageUp => pos.offset_line(-20),
        Movement::PageDown => pos.offset_line(20),
    }
}

impl Document {
    /// Undo the last change.
    ///
    /// Returns true if there was something to undo.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Document;
    ///
    /// let mut doc = Document::new();
    /// doc.insert("Hello");
    /// assert!(doc.undo());
    /// assert_eq!(doc.buffer().to_string(), "");
    /// ```
    pub fn undo(&mut self) -> bool {
        if let Some(selections) = self.undo_stack.undo(&mut self.buffer) {
            self.selections = selections;
            true
        } else {
            false
        }
    }

    /// Redo the last undone change.
    ///
    /// Returns true if there was something to redo.
    pub fn redo(&mut self) -> bool {
        if let Some(selections) = self.undo_stack.redo(&mut self.buffer) {
            self.selections = selections;
            true
        } else {
            false
        }
    }

    /// Check if can undo.
    pub fn can_undo(&self) -> bool {
        self.undo_stack.can_undo()
    }

    /// Check if can redo.
    pub fn can_redo(&self) -> bool {
        self.undo_stack.can_redo()
    }

    /// Save the document to its associated file.
    ///
    /// # Errors
    ///
    /// Returns an error if there's no associated file path.
    pub fn save(&mut self) -> Result<()> {
        let path = self.path().ok_or(DocumentError::NoPath)?.to_path_buf();
        self.save_to_file(&path)
    }

    /// Save the document to a specific file.
    pub fn save_as(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        self.save_to_file(&path)?;
        self.buffer.set_path(Some(path));
        Ok(())
    }

    /// Internal save implementation.
    fn save_to_file(&mut self, path: &Path) -> Result<()> {
        // Apply settings
        let mut content = self.buffer.to_string();

        if self.settings.trim_trailing_whitespace {
            content = content
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join(self.buffer.line_ending().as_str());
        }

        if self.settings.ensure_final_newline && !content.ends_with('\n') {
            content.push_str(self.buffer.line_ending().as_str());
        }

        std::fs::write(path, content)?;
        
        self.saved_version = self.buffer.version();
        self.buffer.clear_modified();
        
        Ok(())
    }

    /// Get line ending style.
    pub fn line_ending(&self) -> LineEnding {
        self.buffer.line_ending()
    }

    /// Set line ending style.
    pub fn set_line_ending(&mut self, ending: LineEnding) {
        self.buffer.set_line_ending(ending);
    }

    /// Select all text.
    pub fn select_all(&mut self) {
        let start = Position::zero();
        let end = if self.buffer.is_empty() {
            Position::zero()
        } else {
            let last_line = self.buffer.line_count().saturating_sub(1);
            let last_col = self.buffer.line_len(last_line).unwrap_or(0);
            Position::new(last_line, last_col)
        };
        
        self.selections = Selections::from_selection(Selection::new(start, end));
    }

    /// Clear undo history.
    pub fn clear_undo_history(&mut self) {
        self.undo_stack.clear();
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert!(!doc.is_modified());
        assert_eq!(doc.buffer().line_count(), 1);
    }

    #[test]
    fn test_document_from_string() {
        let doc = Document::from_str("Hello\nWorld");
        assert_eq!(doc.buffer().line_count(), 2);
        assert!(!doc.is_modified());
    }

    #[test]
    fn test_insert() {
        let mut doc = Document::new();
        doc.insert("Hello");
        assert_eq!(doc.buffer().to_string(), "Hello");
        assert!(doc.can_undo());
    }

    #[test]
    fn test_undo_redo() {
        let mut doc = Document::new();
        doc.insert("Hello");
        assert_eq!(doc.buffer().to_string(), "Hello");
        
        doc.undo();
        assert_eq!(doc.buffer().to_string(), "");
        
        doc.redo();
        assert_eq!(doc.buffer().to_string(), "Hello");
    }

    #[test]
    fn test_delete_selection() {
        let mut doc = Document::from_str("Hello, world!");
        doc.set_selections(Selections::from_selection(
            Selection::new(Position::new(0, 0), Position::new(0, 5))
        ));
        
        doc.delete();
        assert_eq!(doc.buffer().to_string(), ", world!");
    }

    #[test]
    fn test_movement() {
        let mut doc = Document::from_str("Hello\nWorld");

        doc.move_cursor(Movement::Right);
        assert_eq!(doc.selections().primary().head(), Position::new(0, 1));

        doc.move_cursor(Movement::Down);
        assert_eq!(doc.selections().primary().head(), Position::new(1, 1));

        doc.move_cursor(Movement::LineStart);
        assert_eq!(doc.selections().primary().head(), Position::new(1, 0));
    }

    #[test]
    fn test_select_all() {
        let mut doc = Document::from_str("Hello\nWorld");
        doc.select_all();
        
        let sel = doc.selections().primary();
        assert_eq!(sel.range().start, Position::zero());
        assert!(!sel.is_cursor());
    }

    #[test]
    fn test_modification_tracking() {
        let mut doc = Document::new();
        assert!(!doc.is_modified());
        
        doc.insert("Hello");
        assert!(doc.is_modified());
    }
}
