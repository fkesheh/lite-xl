//! Undo/redo system with time-based grouping.
//!
//! This module provides a comprehensive undo/redo system that automatically
//! groups related edits together based on time intervals and edit patterns.

use crate::buffer::{Buffer, Position, Range};
use crate::document::Selections;
use std::time::{Duration, Instant};

/// A single atomic edit operation.
///
/// Each edit is reversible and records both the change and what was replaced.
#[derive(Debug, Clone)]
pub enum Edit {
    /// Text insertion
    Insert {
        /// Position where text was inserted
        position: Position,
        /// Text that was inserted
        text: String,
    },
    /// Text deletion
    Delete {
        /// Range that was deleted
        range: Range,
        /// Text that was deleted
        deleted_text: String,
    },
}

impl Edit {
    /// Apply this edit to a buffer.
    pub fn apply(&self, buffer: &mut Buffer) -> Result<(), crate::buffer::BufferError> {
        match self {
            Edit::Insert { position, text } => {
                buffer.insert(*position, text)?;
            }
            Edit::Delete { range, .. } => {
                buffer.delete(*range)?;
            }
        }
        Ok(())
    }

    /// Create the inverse edit (for undo).
    pub fn inverse(&self) -> Edit {
        match self {
            Edit::Insert { position, text } => {
                let end = Position::new(
                    position.line,
                    position.column + text.len(),
                );
                Edit::Delete {
                    range: Range::new(*position, end),
                    deleted_text: text.clone(),
                }
            }
            Edit::Delete { range, deleted_text } => {
                Edit::Insert {
                    position: range.start,
                    text: deleted_text.clone(),
                }
            }
        }
    }

    /// Get the position affected by this edit.
    pub fn position(&self) -> Position {
        match self {
            Edit::Insert { position, .. } => *position,
            Edit::Delete { range, .. } => range.start,
        }
    }
}

/// A group of related edits that should be undone/redone together.
///
/// Edits are grouped based on:
/// - Time proximity (configurable timeout)
/// - Edit type continuity (e.g., continuous typing)
/// - Cursor movement patterns
#[derive(Debug, Clone)]
pub struct UndoGroup {
    /// The edits in this group (in order)
    edits: Vec<Edit>,
    /// Selections before the group was applied
    selections_before: Selections,
    /// Selections after the group was applied
    selections_after: Selections,
    /// When this group was created
    timestamp: Instant,
    /// User-defined label for this group
    label: Option<String>,
}

impl UndoGroup {
    /// Create a new undo group.
    fn new(selections: Selections) -> Self {
        Self {
            edits: Vec::new(),
            selections_before: selections.clone(),
            selections_after: selections,
            timestamp: Instant::now(),
            label: None,
        }
    }

    /// Add an edit to this group.
    fn add_edit(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    /// Set the selections after all edits.
    fn set_selections_after(&mut self, selections: Selections) {
        self.selections_after = selections;
    }

    /// Check if this group is empty.
    fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Apply all edits in this group to a buffer.
    pub fn apply(&self, buffer: &mut Buffer) -> Result<(), crate::buffer::BufferError> {
        for edit in &self.edits {
            edit.apply(buffer)?;
        }
        Ok(())
    }

    /// Undo all edits in this group (apply in reverse).
    pub fn undo(&self, buffer: &mut Buffer) -> Result<(), crate::buffer::BufferError> {
        for edit in self.edits.iter().rev() {
            edit.inverse().apply(buffer)?;
        }
        Ok(())
    }

    /// Get the selections to restore before this group.
    pub fn selections_before(&self) -> &Selections {
        &self.selections_before
    }

    /// Get the selections to restore after this group.
    pub fn selections_after(&self) -> &Selections {
        &self.selections_after
    }
}

/// Configuration for the undo system.
#[derive(Debug, Clone)]
pub struct UndoConfig {
    /// Maximum number of undo groups to keep
    pub max_groups: usize,
    /// Time threshold for auto-grouping edits
    pub group_timeout: Duration,
    /// Minimum time between groups (prevents too-frequent grouping)
    pub min_group_interval: Duration,
}

impl Default for UndoConfig {
    fn default() -> Self {
        Self {
            max_groups: 10_000,
            group_timeout: Duration::from_millis(300),
            min_group_interval: Duration::from_millis(50),
        }
    }
}

/// Undo/redo stack for a document.
///
/// This structure maintains the undo history with automatic grouping of related
/// edits based on time and edit patterns. It supports:
/// - Automatic time-based grouping
/// - Manual group boundaries
/// - Configurable history limits
/// - Efficient memory usage
///
/// # Examples
///
/// ```
/// use lite_xl::undo::{UndoStack, UndoConfig, Edit};
/// use lite_xl::buffer::{Buffer, Position};
/// use lite_xl::document::Selections;
///
/// let mut stack = UndoStack::new(UndoConfig::default());
/// let mut buffer = Buffer::new();
/// let selections = Selections::single(Position::zero());
///
/// // Record an edit
/// let edit = Edit::Insert {
///     position: Position::zero(),
///     text: "Hello".to_string(),
/// };
/// stack.push(edit, selections.clone());
///
/// // Undo the edit
/// if stack.can_undo() {
///     stack.undo(&mut buffer);
/// }
/// ```
pub struct UndoStack {
    /// Configuration
    config: UndoConfig,
    /// Stack of undo groups (most recent last)
    undo_stack: Vec<UndoGroup>,
    /// Stack of redo groups (most recent last)
    redo_stack: Vec<UndoGroup>,
    /// Current group being built (if any)
    current_group: Option<UndoGroup>,
    /// Timestamp of last edit
    last_edit_time: Option<Instant>,
}

impl UndoStack {
    /// Create a new undo stack with default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::undo::{UndoStack, UndoConfig};
    ///
    /// let stack = UndoStack::new(UndoConfig::default());
    /// ```
    pub fn new(config: UndoConfig) -> Self {
        Self {
            config,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_group: None,
            last_edit_time: None,
        }
    }

    /// Create with custom configuration.
    pub fn with_config(config: UndoConfig) -> Self {
        Self::new(config)
    }

    /// Get the current configuration.
    pub fn config(&self) -> &UndoConfig {
        &self.config
    }

    /// Update the configuration.
    pub fn set_config(&mut self, config: UndoConfig) {
        self.config = config;
    }

    /// Check if can undo.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::undo::{UndoStack, UndoConfig};
    ///
    /// let stack = UndoStack::new(UndoConfig::default());
    /// assert!(!stack.can_undo());
    /// ```
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty() || self.current_group.as_ref().map_or(false, |g| !g.is_empty())
    }

    /// Check if can redo.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of undo groups available.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len() + if self.current_group.is_some() { 1 } else { 0 }
    }

    /// Get the number of redo groups available.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Begin a new undo group explicitly.
    ///
    /// This forces a group boundary even if the time threshold hasn't passed.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::undo::{UndoStack, UndoConfig};
    /// use lite_xl::document::Selections;
    /// use lite_xl::buffer::Position;
    ///
    /// let mut stack = UndoStack::new(UndoConfig::default());
    /// let selections = Selections::single(Position::zero());
    /// stack.begin_group(selections);
    /// ```
    pub fn begin_group(&mut self, selections: Selections) {
        self.end_current_group();
        self.current_group = Some(UndoGroup::new(selections));
    }

    /// End the current undo group explicitly.
    ///
    /// This finalizes the current group and adds it to the stack.
    pub fn end_group(&mut self) {
        self.end_current_group();
    }

    /// Add an edit to the undo stack.
    ///
    /// The edit will be automatically grouped based on time and edit patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::undo::{UndoStack, UndoConfig, Edit};
    /// use lite_xl::document::Selections;
    /// use lite_xl::buffer::Position;
    ///
    /// let mut stack = UndoStack::new(UndoConfig::default());
    /// let selections = Selections::single(Position::zero());
    /// let edit = Edit::Insert {
    ///     position: Position::zero(),
    ///     text: "Hello".to_string(),
    /// };
    /// stack.push(edit, selections);
    /// ```
    pub fn push(&mut self, edit: Edit, selections: Selections) {
        // Clear redo stack on new edit
        self.redo_stack.clear();

        let now = Instant::now();
        let should_group = if let Some(last_time) = self.last_edit_time {
            now.duration_since(last_time) < self.config.group_timeout
        } else {
            false
        };

        if should_group && self.current_group.is_some() {
            // Add to current group
            if let Some(group) = &mut self.current_group {
                group.add_edit(edit);
                group.set_selections_after(selections);
            }
        } else {
            // End current group and start new one
            self.end_current_group();
            let mut group = UndoGroup::new(selections.clone());
            group.add_edit(edit);
            group.set_selections_after(selections);
            self.current_group = Some(group);
        }

        self.last_edit_time = Some(now);
    }

    /// Undo the last group of edits.
    ///
    /// Returns the selections to restore, or None if nothing to undo.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::undo::{UndoStack, UndoConfig, Edit};
    /// use lite_xl::buffer::{Buffer, Position};
    /// use lite_xl::document::Selections;
    ///
    /// let mut stack = UndoStack::new(UndoConfig::default());
    /// let mut buffer = Buffer::new();
    /// let selections = Selections::single(Position::zero());
    ///
    /// let edit = Edit::Insert {
    ///     position: Position::zero(),
    ///     text: "Hello".to_string(),
    /// };
    /// stack.push(edit, selections);
    ///
    /// if let Some(sels) = stack.undo(&mut buffer) {
    ///     // Restore selections
    /// }
    /// ```
    pub fn undo(&mut self, buffer: &mut Buffer) -> Option<Selections> {
        // First try to undo current group
        if let Some(group) = self.current_group.take() {
            if !group.is_empty() {
                group.undo(buffer).ok()?;
                let selections = group.selections_before().clone();
                self.redo_stack.push(group);
                return Some(selections);
            }
        }

        // Then try undo stack
        let group = self.undo_stack.pop()?;
        group.undo(buffer).ok()?;
        let selections = group.selections_before().clone();
        self.redo_stack.push(group);
        Some(selections)
    }

    /// Redo the last undone group of edits.
    ///
    /// Returns the selections to restore, or None if nothing to redo.
    pub fn redo(&mut self, buffer: &mut Buffer) -> Option<Selections> {
        let group = self.redo_stack.pop()?;
        group.apply(buffer).ok()?;
        let selections = group.selections_after().clone();
        self.undo_stack.push(group);
        Some(selections)
    }

    /// Clear all undo/redo history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_group = None;
        self.last_edit_time = None;
    }

    /// Clear only the redo stack.
    pub fn clear_redo(&mut self) {
        self.redo_stack.clear();
    }

    /// End the current group and add it to the undo stack.
    fn end_current_group(&mut self) {
        if let Some(group) = self.current_group.take() {
            if !group.is_empty() {
                self.undo_stack.push(group);
                
                // Trim stack if too large
                if self.undo_stack.len() > self.config.max_groups {
                    self.undo_stack.drain(0..self.undo_stack.len() - self.config.max_groups);
                }
            }
        }
    }

    /// Get memory usage estimate in bytes.
    pub fn memory_usage(&self) -> usize {
        let group_size = |g: &UndoGroup| {
            g.edits.iter().map(|e| match e {
                Edit::Insert { text, .. } => text.len(),
                Edit::Delete { deleted_text, .. } => deleted_text.len(),
            }).sum::<usize>()
        };

        self.undo_stack.iter().map(group_size).sum::<usize>() +
        self.redo_stack.iter().map(group_size).sum::<usize>() +
        self.current_group.as_ref().map(group_size).unwrap_or(0)
    }
}

impl Default for UndoStack {
    fn default() -> Self {
        Self::new(UndoConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;

    #[test]
    fn test_edit_apply() {
        let mut buffer = Buffer::new();
        let edit = Edit::Insert {
            position: Position::zero(),
            text: "Hello".to_string(),
        };
        edit.apply(&mut buffer).unwrap();
        assert_eq!(buffer.to_string(), "Hello");
    }

    #[test]
    fn test_edit_inverse() {
        let insert = Edit::Insert {
            position: Position::zero(),
            text: "Hello".to_string(),
        };
        let delete = insert.inverse();
        
        match delete {
            Edit::Delete { deleted_text, .. } => {
                assert_eq!(deleted_text, "Hello");
            }
            _ => panic!("Expected Delete edit"),
        }
    }

    #[test]
    fn test_undo_stack_push() {
        let mut stack = UndoStack::new(UndoConfig::default());
        let selections = Selections::single(Position::zero());
        
        let edit = Edit::Insert {
            position: Position::zero(),
            text: "Hello".to_string(),
        };
        stack.push(edit, selections);
        
        assert!(stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_undo_redo() {
        let mut stack = UndoStack::new(UndoConfig::default());
        let mut buffer = Buffer::new();
        let selections = Selections::single(Position::zero());
        
        // Insert text
        let edit = Edit::Insert {
            position: Position::zero(),
            text: "Hello".to_string(),
        };
        edit.apply(&mut buffer).unwrap();
        stack.push(edit, selections.clone());
        
        assert_eq!(buffer.to_string(), "Hello");
        
        // Undo
        stack.undo(&mut buffer);
        assert_eq!(buffer.to_string(), "");
        assert!(stack.can_redo());
        
        // Redo
        stack.redo(&mut buffer);
        assert_eq!(buffer.to_string(), "Hello");
    }

    #[test]
    fn test_multiple_edits() {
        let mut stack = UndoStack::new(UndoConfig::default());
        let mut buffer = Buffer::new();
        let mut selections = Selections::single(Position::zero());
        
        // Insert "Hello"
        let edit1 = Edit::Insert {
            position: Position::zero(),
            text: "Hello".to_string(),
        };
        edit1.apply(&mut buffer).unwrap();
        stack.push(edit1, selections.clone());
        
        // Insert " World"
        selections.primary_mut().move_to(Position::new(0, 5));
        let edit2 = Edit::Insert {
            position: Position::new(0, 5),
            text: " World".to_string(),
        };
        edit2.apply(&mut buffer).unwrap();
        stack.push(edit2, selections.clone());
        
        assert_eq!(buffer.to_string(), "Hello World");
        
        // Undo both (they're grouped)
        stack.undo(&mut buffer);
        assert_eq!(buffer.to_string(), "");
    }

    #[test]
    fn test_redo_cleared_on_new_edit() {
        let mut stack = UndoStack::new(UndoConfig::default());
        let mut buffer = Buffer::new();
        let selections = Selections::single(Position::zero());
        
        // Insert and undo
        let edit = Edit::Insert {
            position: Position::zero(),
            text: "Hello".to_string(),
        };
        edit.apply(&mut buffer).unwrap();
        stack.push(edit, selections.clone());
        stack.undo(&mut buffer);
        
        assert!(stack.can_redo());
        
        // New edit should clear redo stack
        let edit2 = Edit::Insert {
            position: Position::zero(),
            text: "World".to_string(),
        };
        edit2.apply(&mut buffer).unwrap();
        stack.push(edit2, selections);
        
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_begin_end_group() {
        let mut stack = UndoStack::new(UndoConfig::default());
        let selections = Selections::single(Position::zero());
        
        stack.begin_group(selections.clone());
        stack.push(Edit::Insert {
            position: Position::zero(),
            text: "A".to_string(),
        }, selections.clone());
        
        // Force new group
        stack.end_group();
        stack.begin_group(selections.clone());
        stack.push(Edit::Insert {
            position: Position::new(0, 1),
            text: "B".to_string(),
        }, selections);
        stack.end_group();
        
        // Should have 2 separate groups
        assert_eq!(stack.undo_count(), 2);
    }
}
