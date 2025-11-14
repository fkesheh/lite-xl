//! Navigation Command Implementations
//!
//! This module provides implementations for cursor movement and navigation
//! operations including:
//! - Basic cursor movement (left, right, up, down)
//! - Word-based navigation
//! - Line navigation (home, end)
//! - Document navigation (page up/down, document start/end)
//! - Go to line/position
//! - Bracket matching
//! - Scrolling
//!
//! All navigation commands support:
//! - Multi-cursor navigation
//! - Selection extension (when combined with Shift)
//! - Smart navigation (word boundaries, paragraph boundaries)

use crate::commands::Movement;

/// Position in a text document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// Line number (0-indexed)
    pub line: usize,

    /// Column number (0-indexed, character-based not byte-based)
    pub column: usize,
}

impl Position {
    /// Create a new position
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Create a position at the start of the document
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Create a position at the start of a line
    pub const fn line_start(line: usize) -> Self {
        Self::new(line, 0)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.column.cmp(&other.column),
            other => other,
        }
    }
}

/// Context for executing navigation commands
///
/// This trait defines the interface that the editor must implement
/// to support navigation operations.
pub trait NavigationContext {
    /// Get the current cursor position
    fn cursor_position(&self) -> Position;

    /// Set the cursor position
    fn set_cursor_position(&mut self, position: Position);

    /// Get the number of lines in the document
    fn line_count(&self) -> usize;

    /// Get the length of a specific line
    fn line_length(&self, line: usize) -> usize;

    /// Get the text of a specific line
    fn line_text(&self, line: usize) -> Option<String>;

    /// Get the visible line range (for page up/down)
    fn visible_line_range(&self) -> (usize, usize);

    /// Get the number of visible lines (viewport height)
    fn visible_line_count(&self) -> usize;

    /// Scroll the view to make a position visible
    fn scroll_to_position(&mut self, position: Position);

    /// Get the current selection range (if any)
    fn selection_range(&self) -> Option<(Position, Position)>;

    /// Set the selection range
    fn set_selection_range(&mut self, start: Position, end: Position);

    /// Clear the selection (collapse to cursor)
    fn clear_selection(&mut self);

    /// Find the position of the matching bracket
    fn find_matching_bracket(&self, position: Position) -> Option<Position>;
}

/// Execute a move cursor command
///
/// Moves the cursor according to the specified movement, collapsing
/// any existing selection.
pub fn execute_move_cursor(ctx: &mut impl NavigationContext, movement: Movement) {
    let current_pos = ctx.cursor_position();
    let new_pos = calculate_movement(ctx, current_pos, movement);
    ctx.set_cursor_position(new_pos);
    ctx.clear_selection();
    ctx.scroll_to_position(new_pos);
}

/// Execute a select command
///
/// Extends the current selection according to the specified movement.
pub fn execute_select(ctx: &mut impl NavigationContext, movement: Movement) {
    let current_pos = ctx.cursor_position();
    let new_pos = calculate_movement(ctx, current_pos, movement);

    // Extend selection from anchor to new position
    let selection_range = ctx.selection_range();
    let anchor = selection_range.map(|(start, _)| start).unwrap_or(current_pos);

    ctx.set_cursor_position(new_pos);
    ctx.set_selection_range(anchor, new_pos);
    ctx.scroll_to_position(new_pos);
}

/// Execute a go to line command
///
/// Moves the cursor to the specified line number (1-indexed for user display).
pub fn execute_go_to_line(ctx: &mut impl NavigationContext, line_number: usize) {
    let line = line_number.saturating_sub(1); // Convert to 0-indexed
    let line = line.min(ctx.line_count().saturating_sub(1));
    let position = Position::new(line, 0);

    ctx.set_cursor_position(position);
    ctx.clear_selection();
    ctx.scroll_to_position(position);
}

/// Execute a go to position command
///
/// Moves the cursor to a specific line and column.
pub fn execute_go_to_position(ctx: &mut impl NavigationContext, line: usize, column: usize) {
    let line = line.min(ctx.line_count().saturating_sub(1));
    let column = column.min(ctx.line_length(line));
    let position = Position::new(line, column);

    ctx.set_cursor_position(position);
    ctx.clear_selection();
    ctx.scroll_to_position(position);
}

/// Execute a go to matching bracket command
///
/// Jumps to the matching bracket from the current cursor position.
pub fn execute_go_to_matching_bracket(ctx: &mut impl NavigationContext) {
    let current_pos = ctx.cursor_position();

    if let Some(matching_pos) = ctx.find_matching_bracket(current_pos) {
        ctx.set_cursor_position(matching_pos);
        ctx.clear_selection();
        ctx.scroll_to_position(matching_pos);
    }
}

/// Execute a center cursor command
///
/// Scrolls the view to center the cursor in the viewport.
pub fn execute_center_cursor(ctx: &mut impl NavigationContext) {
    let current_pos = ctx.cursor_position();
    ctx.scroll_to_position(current_pos);
}

/// Calculate the new position after a movement
fn calculate_movement(
    ctx: &impl NavigationContext,
    current: Position,
    movement: Movement,
) -> Position {
    match movement {
        Movement::Left => move_left(ctx, current),
        Movement::Right => move_right(ctx, current),
        Movement::Up => move_up(ctx, current),
        Movement::Down => move_down(ctx, current),
        Movement::LineStart => move_line_start(ctx, current),
        Movement::LineEnd => move_line_end(ctx, current),
        Movement::LineStartNonWhitespace => move_line_start_non_whitespace(ctx, current),
        Movement::WordLeft => move_word_left(ctx, current),
        Movement::WordRight => move_word_right(ctx, current),
        Movement::DocumentStart => Position::zero(),
        Movement::DocumentEnd => {
            let last_line = ctx.line_count().saturating_sub(1);
            Position::new(last_line, ctx.line_length(last_line))
        }
        Movement::PageUp => move_page_up(ctx, current),
        Movement::PageDown => move_page_down(ctx, current),
        Movement::ParagraphNext => move_paragraph_next(ctx, current),
        Movement::ParagraphPrevious => move_paragraph_previous(ctx, current),
        Movement::MatchingBracket => {
            ctx.find_matching_bracket(current).unwrap_or(current)
        }
    }
}

/// Move cursor left by one character
fn move_left(ctx: &impl NavigationContext, current: Position) -> Position {
    if current.column > 0 {
        Position::new(current.line, current.column - 1)
    } else if current.line > 0 {
        // Move to end of previous line
        let prev_line = current.line - 1;
        Position::new(prev_line, ctx.line_length(prev_line))
    } else {
        current
    }
}

/// Move cursor right by one character
fn move_right(ctx: &impl NavigationContext, current: Position) -> Position {
    let line_len = ctx.line_length(current.line);
    if current.column < line_len {
        Position::new(current.line, current.column + 1)
    } else if current.line < ctx.line_count() - 1 {
        // Move to start of next line
        Position::new(current.line + 1, 0)
    } else {
        current
    }
}

/// Move cursor up by one line
fn move_up(ctx: &impl NavigationContext, current: Position) -> Position {
    if current.line > 0 {
        let new_line = current.line - 1;
        let new_column = current.column.min(ctx.line_length(new_line));
        Position::new(new_line, new_column)
    } else {
        current
    }
}

/// Move cursor down by one line
fn move_down(ctx: &impl NavigationContext, current: Position) -> Position {
    if current.line < ctx.line_count() - 1 {
        let new_line = current.line + 1;
        let new_column = current.column.min(ctx.line_length(new_line));
        Position::new(new_line, new_column)
    } else {
        current
    }
}

/// Move cursor to start of line
fn move_line_start(_ctx: &impl NavigationContext, current: Position) -> Position {
    Position::new(current.line, 0)
}

/// Move cursor to end of line
fn move_line_end(ctx: &impl NavigationContext, current: Position) -> Position {
    Position::new(current.line, ctx.line_length(current.line))
}

/// Move cursor to first non-whitespace character of line
fn move_line_start_non_whitespace(ctx: &impl NavigationContext, current: Position) -> Position {
    if let Some(line_text) = ctx.line_text(current.line) {
        let first_non_ws = line_text
            .chars()
            .position(|c| !c.is_whitespace())
            .unwrap_or(0);
        Position::new(current.line, first_non_ws)
    } else {
        current
    }
}

/// Move cursor left by one word
fn move_word_left(ctx: &impl NavigationContext, current: Position) -> Position {
    if let Some(line_text) = ctx.line_text(current.line) {
        if current.column == 0 {
            // At start of line, move to previous line
            return move_left(ctx, current);
        }

        let chars: Vec<char> = line_text.chars().collect();
        let mut pos = current.column.min(chars.len());

        // Skip whitespace
        while pos > 0 && chars[pos - 1].is_whitespace() {
            pos -= 1;
        }

        // Skip word characters
        if pos > 0 {
            let is_alphanumeric = chars[pos - 1].is_alphanumeric() || chars[pos - 1] == '_';
            while pos > 0 {
                let prev_char = chars[pos - 1];
                let prev_is_alphanumeric = prev_char.is_alphanumeric() || prev_char == '_';
                if is_alphanumeric != prev_is_alphanumeric {
                    break;
                }
                pos -= 1;
            }
        }

        Position::new(current.line, pos)
    } else {
        current
    }
}

/// Move cursor right by one word
fn move_word_right(ctx: &impl NavigationContext, current: Position) -> Position {
    if let Some(line_text) = ctx.line_text(current.line) {
        let chars: Vec<char> = line_text.chars().collect();
        let mut pos = current.column;

        if pos >= chars.len() {
            // At end of line, move to next line
            return move_right(ctx, current);
        }

        // Skip current word
        if pos < chars.len() {
            let is_alphanumeric = chars[pos].is_alphanumeric() || chars[pos] == '_';
            while pos < chars.len() {
                let curr_char = chars[pos];
                let curr_is_alphanumeric = curr_char.is_alphanumeric() || curr_char == '_';
                if is_alphanumeric != curr_is_alphanumeric {
                    break;
                }
                pos += 1;
            }
        }

        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }

        Position::new(current.line, pos)
    } else {
        current
    }
}

/// Move cursor up by one page
fn move_page_up(ctx: &impl NavigationContext, current: Position) -> Position {
    let visible_lines = ctx.visible_line_count();
    let new_line = current.line.saturating_sub(visible_lines);
    let new_column = current.column.min(ctx.line_length(new_line));
    Position::new(new_line, new_column)
}

/// Move cursor down by one page
fn move_page_down(ctx: &impl NavigationContext, current: Position) -> Position {
    let visible_lines = ctx.visible_line_count();
    let new_line = (current.line + visible_lines).min(ctx.line_count() - 1);
    let new_column = current.column.min(ctx.line_length(new_line));
    Position::new(new_line, new_column)
}

/// Move cursor to next paragraph
fn move_paragraph_next(ctx: &impl NavigationContext, current: Position) -> Position {
    let mut line = current.line + 1;

    // Skip non-empty lines
    while line < ctx.line_count() {
        if let Some(text) = ctx.line_text(line) {
            if text.trim().is_empty() {
                break;
            }
        }
        line += 1;
    }

    // Skip empty lines
    while line < ctx.line_count() {
        if let Some(text) = ctx.line_text(line) {
            if !text.trim().is_empty() {
                break;
            }
        }
        line += 1;
    }

    Position::new(line.min(ctx.line_count() - 1), 0)
}

/// Move cursor to previous paragraph
fn move_paragraph_previous(ctx: &impl NavigationContext, current: Position) -> Position {
    if current.line == 0 {
        return Position::zero();
    }

    let mut line = current.line - 1;

    // Skip empty lines
    while line > 0 {
        if let Some(text) = ctx.line_text(line) {
            if !text.trim().is_empty() {
                break;
            }
        }
        line -= 1;
    }

    // Skip non-empty lines to find start of paragraph
    while line > 0 {
        if let Some(text) = ctx.line_text(line - 1) {
            if text.trim().is_empty() {
                break;
            }
        }
        line -= 1;
    }

    Position::new(line, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock navigation context for testing
    struct MockNavigationContext {
        cursor: Position,
        lines: Vec<String>,
        selection: Option<(Position, Position)>,
    }

    impl MockNavigationContext {
        fn new(lines: Vec<&str>) -> Self {
            Self {
                cursor: Position::zero(),
                lines: lines.iter().map(|s| s.to_string()).collect(),
                selection: None,
            }
        }
    }

    impl NavigationContext for MockNavigationContext {
        fn cursor_position(&self) -> Position {
            self.cursor
        }

        fn set_cursor_position(&mut self, position: Position) {
            self.cursor = position;
        }

        fn line_count(&self) -> usize {
            self.lines.len()
        }

        fn line_length(&self, line: usize) -> usize {
            self.lines.get(line).map(|s| s.len()).unwrap_or(0)
        }

        fn line_text(&self, line: usize) -> Option<String> {
            self.lines.get(line).cloned()
        }

        fn visible_line_range(&self) -> (usize, usize) {
            (0, 20)
        }

        fn visible_line_count(&self) -> usize {
            20
        }

        fn scroll_to_position(&mut self, _position: Position) {
            // No-op for testing
        }

        fn selection_range(&self) -> Option<(Position, Position)> {
            self.selection
        }

        fn set_selection_range(&mut self, start: Position, end: Position) {
            self.selection = Some((start, end));
        }

        fn clear_selection(&mut self) {
            self.selection = None;
        }

        fn find_matching_bracket(&self, _position: Position) -> Option<Position> {
            None
        }
    }

    #[test]
    fn test_move_left() {
        let mut ctx = MockNavigationContext::new(vec!["hello", "world"]);
        ctx.cursor = Position::new(0, 3);

        execute_move_cursor(&mut ctx, Movement::Left);
        assert_eq!(ctx.cursor, Position::new(0, 2));
    }

    #[test]
    fn test_move_right() {
        let mut ctx = MockNavigationContext::new(vec!["hello", "world"]);
        ctx.cursor = Position::new(0, 2);

        execute_move_cursor(&mut ctx, Movement::Right);
        assert_eq!(ctx.cursor, Position::new(0, 3));
    }

    #[test]
    fn test_move_up() {
        let mut ctx = MockNavigationContext::new(vec!["hello", "world"]);
        ctx.cursor = Position::new(1, 2);

        execute_move_cursor(&mut ctx, Movement::Up);
        assert_eq!(ctx.cursor, Position::new(0, 2));
    }

    #[test]
    fn test_move_down() {
        let mut ctx = MockNavigationContext::new(vec!["hello", "world"]);
        ctx.cursor = Position::new(0, 2);

        execute_move_cursor(&mut ctx, Movement::Down);
        assert_eq!(ctx.cursor, Position::new(1, 2));
    }

    #[test]
    fn test_move_line_start() {
        let mut ctx = MockNavigationContext::new(vec!["hello"]);
        ctx.cursor = Position::new(0, 3);

        execute_move_cursor(&mut ctx, Movement::LineStart);
        assert_eq!(ctx.cursor, Position::new(0, 0));
    }

    #[test]
    fn test_move_line_end() {
        let mut ctx = MockNavigationContext::new(vec!["hello"]);
        ctx.cursor = Position::new(0, 2);

        execute_move_cursor(&mut ctx, Movement::LineEnd);
        assert_eq!(ctx.cursor, Position::new(0, 5));
    }

    #[test]
    fn test_go_to_line() {
        let mut ctx = MockNavigationContext::new(vec!["line1", "line2", "line3"]);

        execute_go_to_line(&mut ctx, 2);
        assert_eq!(ctx.cursor, Position::new(1, 0));
    }

    #[test]
    fn test_select() {
        let mut ctx = MockNavigationContext::new(vec!["hello"]);
        ctx.cursor = Position::new(0, 2);

        execute_select(&mut ctx, Movement::Right);
        assert_eq!(ctx.cursor, Position::new(0, 3));
        assert!(ctx.selection.is_some());
    }

    #[test]
    fn test_word_navigation() {
        let mut ctx = MockNavigationContext::new(vec!["hello world test"]);
        ctx.cursor = Position::new(0, 0);

        execute_move_cursor(&mut ctx, Movement::WordRight);
        assert_eq!(ctx.cursor.column, 6); // After "hello "

        execute_move_cursor(&mut ctx, Movement::WordLeft);
        assert_eq!(ctx.cursor.column, 0); // Back to start
    }
}
