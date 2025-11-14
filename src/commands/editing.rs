//! Editing Command Implementations
//!
//! This module provides implementations for text editing operations including:
//! - Insert and delete operations
//! - Cut, copy, and paste
//! - Line manipulation (duplicate, move, join, split)
//! - Text transformations (case conversion, indentation)
//! - Comment toggling
//!
//! All editing commands support:
//! - Multi-cursor editing
//! - Undo/redo integration
//! - Selection handling

use crate::commands::{Command, LineEndingStyle};

/// Context for executing editing commands
///
/// This trait defines the interface that the editor must implement
/// to support editing operations. It abstracts away the actual
/// document/buffer implementation.
pub trait EditContext {
    /// Insert text at the current cursor position(s)
    fn insert_text(&mut self, text: &str);

    /// Delete the current selection or character at cursor
    fn delete_selection(&mut self);

    /// Delete character before cursor (backspace)
    fn delete_backward(&mut self);

    /// Get the current selection text
    fn get_selection(&self) -> String;

    /// Set the selection text (replace)
    fn set_selection(&mut self, text: &str);

    /// Get the current line text
    fn get_current_line(&self) -> String;

    /// Set the current line text
    fn set_current_line(&mut self, text: &str);

    /// Get all selected lines
    fn get_selected_lines(&self) -> Vec<String>;

    /// Set all selected lines
    fn set_selected_lines(&mut self, lines: Vec<String>);

    /// Delete the current line(s)
    fn delete_line(&mut self);

    /// Delete from cursor to end of line
    fn delete_to_end_of_line(&mut self);

    /// Delete from cursor to start of line
    fn delete_to_start_of_line(&mut self);

    /// Delete word forward from cursor
    fn delete_word_forward(&mut self);

    /// Delete word backward from cursor
    fn delete_word_backward(&mut self);

    /// Check if there is a selection
    fn has_selection(&self) -> bool;

    /// Get the indentation string (tabs or spaces)
    fn get_indent_string(&self) -> String;

    /// Get the line comment string for current syntax
    fn get_line_comment_string(&self) -> Option<String>;

    /// Get the block comment strings for current syntax
    fn get_block_comment_strings(&self) -> Option<(String, String)>;
}

/// Execute an insert command
///
/// # Arguments
/// * `ctx` - The edit context
/// * `text` - The text to insert
///
/// # Example
/// ```
/// execute_insert(&mut editor, "Hello, world!");
/// ```
pub fn execute_insert(ctx: &mut impl EditContext, text: &str) {
    ctx.insert_text(text);
}

/// Execute a delete command
///
/// Deletes the current selection, or the character at the cursor if no selection.
pub fn execute_delete(ctx: &mut impl EditContext) {
    if ctx.has_selection() {
        ctx.delete_selection();
    } else {
        // Delete character at cursor (like Delete key)
        ctx.delete_selection();
    }
}

/// Execute a delete backward command
///
/// Deletes the current selection, or the character before the cursor if no selection.
pub fn execute_delete_backward(ctx: &mut impl EditContext) {
    if ctx.has_selection() {
        ctx.delete_selection();
    } else {
        ctx.delete_backward();
    }
}

/// Execute a delete line command
///
/// Deletes the entire current line(s) including the line break.
pub fn execute_delete_line(ctx: &mut impl EditContext) {
    ctx.delete_line();
}

/// Execute a delete to end of line command
///
/// Deletes from the cursor position to the end of the line.
pub fn execute_delete_to_end_of_line(ctx: &mut impl EditContext) {
    ctx.delete_to_end_of_line();
}

/// Execute a delete to start of line command
///
/// Deletes from the cursor position to the start of the line.
pub fn execute_delete_to_start_of_line(ctx: &mut impl EditContext) {
    ctx.delete_to_start_of_line();
}

/// Execute a delete word forward command
///
/// Deletes the word following the cursor.
pub fn execute_delete_word_forward(ctx: &mut impl EditContext) {
    ctx.delete_word_forward();
}

/// Execute a delete word backward command
///
/// Deletes the word before the cursor.
pub fn execute_delete_word_backward(ctx: &mut impl EditContext) {
    ctx.delete_word_backward();
}

/// Execute a duplicate line command
///
/// Duplicates the current line(s) and places the cursor on the new line(s).
pub fn execute_duplicate_line(ctx: &mut impl EditContext) {
    let lines = ctx.get_selected_lines();
    let duplicated = lines.iter()
        .flat_map(|line| vec![line.clone(), line.clone()])
        .collect();
    ctx.set_selected_lines(duplicated);
}

/// Execute a move line up command
///
/// Moves the current line(s) up by one position.
pub fn execute_move_line_up(ctx: &mut impl EditContext) {
    // Implementation would involve:
    // 1. Get current line(s) and the line above
    // 2. Swap them
    // 3. Adjust cursor position
    // This requires more context about the document structure
    todo!("Move line up requires document-level access")
}

/// Execute a move line down command
///
/// Moves the current line(s) down by one position.
pub fn execute_move_line_down(ctx: &mut impl EditContext) {
    // Similar to move_line_up
    todo!("Move line down requires document-level access")
}

/// Execute a join lines command
///
/// Joins the current line with the next line, removing the line break.
pub fn execute_join_lines(ctx: &mut impl EditContext) {
    let lines = ctx.get_selected_lines();
    if lines.len() >= 2 {
        let joined = lines.join(" ");
        ctx.set_selected_lines(vec![joined]);
    }
}

/// Execute a split line command
///
/// Splits the current line at the cursor position.
pub fn execute_split_line(ctx: &mut impl EditContext) {
    ctx.insert_text("\n");
}

/// Execute an indent command
///
/// Indents the current line(s) by one level.
pub fn execute_indent(ctx: &mut impl EditContext) {
    let indent_string = ctx.get_indent_string();
    let lines = ctx.get_selected_lines();

    let indented: Vec<String> = lines
        .iter()
        .map(|line| format!("{}{}", indent_string, line))
        .collect();

    ctx.set_selected_lines(indented);
}

/// Execute an unindent command
///
/// Unindents the current line(s) by one level.
pub fn execute_unindent(ctx: &mut impl EditContext) {
    let indent_string = ctx.get_indent_string();
    let lines = ctx.get_selected_lines();

    let unindented: Vec<String> = lines
        .iter()
        .map(|line| {
            if line.starts_with(&indent_string) {
                line[indent_string.len()..].to_string()
            } else {
                // Try to remove partial indentation
                if indent_string.starts_with('\t') {
                    // Remove single tab
                    line.strip_prefix('\t').unwrap_or(line).to_string()
                } else {
                    // Remove spaces (up to indent width)
                    let spaces_to_remove = line.chars().take_while(|c| *c == ' ').count();
                    let to_remove = spaces_to_remove.min(indent_string.len());
                    line[to_remove..].to_string()
                }
            }
        })
        .collect();

    ctx.set_selected_lines(unindented);
}

/// Execute a toggle comment command
///
/// Toggles line comments on the current line(s).
pub fn execute_toggle_comment(ctx: &mut impl EditContext) {
    if let Some(comment_str) = ctx.get_line_comment_string() {
        let lines = ctx.get_selected_lines();

        // Check if all lines are already commented
        let all_commented = lines
            .iter()
            .all(|line| line.trim_start().starts_with(&comment_str));

        let toggled: Vec<String> = if all_commented {
            // Uncomment
            lines
                .iter()
                .map(|line| {
                    if let Some(pos) = line.find(&comment_str) {
                        let mut result = String::new();
                        result.push_str(&line[..pos]);
                        result.push_str(&line[pos + comment_str.len()..].trim_start());
                        result
                    } else {
                        line.clone()
                    }
                })
                .collect()
        } else {
            // Comment
            lines
                .iter()
                .map(|line| {
                    if line.trim().is_empty() {
                        line.clone()
                    } else {
                        // Find first non-whitespace character
                        let indent = line.len() - line.trim_start().len();
                        let mut result = String::new();
                        result.push_str(&line[..indent]);
                        result.push_str(&comment_str);
                        result.push(' ');
                        result.push_str(&line[indent..]);
                        result
                    }
                })
                .collect()
        };

        ctx.set_selected_lines(toggled);
    }
}

/// Execute a toggle block comment command
///
/// Toggles block comments around the current selection.
pub fn execute_toggle_block_comment(ctx: &mut impl EditContext) {
    if let Some((start_comment, end_comment)) = ctx.get_block_comment_strings() {
        let selection = ctx.get_selection();

        let toggled = if selection.starts_with(&start_comment) && selection.ends_with(&end_comment) {
            // Uncomment
            let trimmed = &selection[start_comment.len()..selection.len() - end_comment.len()];
            trimmed.to_string()
        } else {
            // Comment
            format!("{}{}{}", start_comment, selection, end_comment)
        };

        ctx.set_selection(&toggled);
    }
}

/// Execute a to uppercase command
///
/// Converts the current selection to uppercase.
pub fn execute_to_uppercase(ctx: &mut impl EditContext) {
    let selection = ctx.get_selection();
    let uppercase = selection.to_uppercase();
    ctx.set_selection(&uppercase);
}

/// Execute a to lowercase command
///
/// Converts the current selection to lowercase.
pub fn execute_to_lowercase(ctx: &mut impl EditContext) {
    let selection = ctx.get_selection();
    let lowercase = selection.to_lowercase();
    ctx.set_selection(&lowercase);
}

/// Execute a to title case command
///
/// Converts the current selection to title case (first letter of each word capitalized).
pub fn execute_to_title_case(ctx: &mut impl EditContext) {
    let selection = ctx.get_selection();
    let title_case = to_title_case(&selection);
    ctx.set_selection(&title_case);
}

/// Convert a string to title case
///
/// Capitalizes the first letter of each word.
fn to_title_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.extend(c.to_lowercase());
        }
    }

    result
}

/// Paste text from clipboard
///
/// This is a placeholder - actual clipboard integration is handled
/// by the clipboard module.
pub fn execute_paste(ctx: &mut impl EditContext, text: &str) {
    ctx.set_selection(text);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock edit context for testing
    struct MockEditContext {
        text: String,
        selection: String,
        has_selection: bool,
    }

    impl MockEditContext {
        fn new(text: &str) -> Self {
            Self {
                text: text.to_string(),
                selection: String::new(),
                has_selection: false,
            }
        }
    }

    impl EditContext for MockEditContext {
        fn insert_text(&mut self, text: &str) {
            self.text.push_str(text);
        }

        fn delete_selection(&mut self) {
            self.text.clear();
            self.selection.clear();
            self.has_selection = false;
        }

        fn delete_backward(&mut self) {
            self.text.pop();
        }

        fn get_selection(&self) -> String {
            self.selection.clone()
        }

        fn set_selection(&mut self, text: &str) {
            self.selection = text.to_string();
        }

        fn get_current_line(&self) -> String {
            self.text.clone()
        }

        fn set_current_line(&mut self, text: &str) {
            self.text = text.to_string();
        }

        fn get_selected_lines(&self) -> Vec<String> {
            self.text.lines().map(String::from).collect()
        }

        fn set_selected_lines(&mut self, lines: Vec<String>) {
            self.text = lines.join("\n");
        }

        fn delete_line(&mut self) {
            self.text.clear();
        }

        fn delete_to_end_of_line(&mut self) {
            // Simplified implementation
            self.text.clear();
        }

        fn delete_to_start_of_line(&mut self) {
            self.text.clear();
        }

        fn delete_word_forward(&mut self) {
            self.text.clear();
        }

        fn delete_word_backward(&mut self) {
            self.text.pop();
        }

        fn has_selection(&self) -> bool {
            self.has_selection
        }

        fn get_indent_string(&self) -> String {
            "    ".to_string() // 4 spaces
        }

        fn get_line_comment_string(&self) -> Option<String> {
            Some("//".to_string())
        }

        fn get_block_comment_strings(&self) -> Option<(String, String)> {
            Some(("/*".to_string(), "*/".to_string()))
        }
    }

    #[test]
    fn test_insert() {
        let mut ctx = MockEditContext::new("");
        execute_insert(&mut ctx, "Hello");
        assert_eq!(ctx.text, "Hello");
    }

    #[test]
    fn test_indent() {
        let mut ctx = MockEditContext::new("line1\nline2");
        execute_indent(&mut ctx);
        assert_eq!(ctx.text, "    line1\n    line2");
    }

    #[test]
    fn test_unindent() {
        let mut ctx = MockEditContext::new("    line1\n    line2");
        execute_unindent(&mut ctx);
        assert_eq!(ctx.text, "line1\nline2");
    }

    #[test]
    fn test_toggle_comment() {
        let mut ctx = MockEditContext::new("line1\nline2");
        execute_toggle_comment(&mut ctx);
        assert!(ctx.text.contains("//"));

        execute_toggle_comment(&mut ctx);
        assert!(!ctx.text.contains("//"));
    }

    #[test]
    fn test_to_uppercase() {
        let mut ctx = MockEditContext::new("");
        ctx.selection = "hello".to_string();
        execute_to_uppercase(&mut ctx);
        assert_eq!(ctx.selection, "HELLO");
    }

    #[test]
    fn test_to_lowercase() {
        let mut ctx = MockEditContext::new("");
        ctx.selection = "HELLO".to_string();
        execute_to_lowercase(&mut ctx);
        assert_eq!(ctx.selection, "hello");
    }

    #[test]
    fn test_to_title_case() {
        let mut ctx = MockEditContext::new("");
        ctx.selection = "hello world".to_string();
        execute_to_title_case(&mut ctx);
        assert_eq!(ctx.selection, "Hello World");
    }

    #[test]
    fn test_join_lines() {
        let mut ctx = MockEditContext::new("line1\nline2\nline3");
        execute_join_lines(&mut ctx);
        assert_eq!(ctx.text, "line1 line2 line3");
    }
}
