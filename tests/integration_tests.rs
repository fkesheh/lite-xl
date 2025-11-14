//! Integration tests for Lite XL.
//!
//! These tests verify that all components work together correctly.

use lite_xl::buffer::{Buffer, Position, Range};
use lite_xl::document::{Document, Movement, Selection, Selections};
use lite_xl::undo::{Edit, UndoConfig, UndoStack};

#[test]
fn test_full_editing_workflow() {
    let mut doc = Document::new();
    
    // Insert initial text
    doc.insert("Hello, world!");
    assert_eq!(doc.buffer().to_string(), "Hello, world!");
    assert!(doc.is_modified());
    
    // Move cursor
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Right);
    
    // Insert more text
    doc.insert(" beautiful");
    assert_eq!(doc.buffer().to_string(), "Hello beautiful, world!");
    
    // Undo twice
    assert!(doc.undo());
    assert!(doc.undo());
    assert_eq!(doc.buffer().to_string(), "");
    
    // Redo
    assert!(doc.redo());
    assert_eq!(doc.buffer().to_string(), "Hello, world!");
}

#[test]
fn test_multi_cursor_workflow() {
    let mut doc = Document::from_str("apple\nbanana\ncherry");
    
    // Create three cursors at line starts
    let selections = Selections::from_vec(vec![
        Selection::cursor(Position::new(0, 0)),
        Selection::cursor(Position::new(1, 0)),
        Selection::cursor(Position::new(2, 0)),
    ]);
    doc.set_selections(selections);
    
    // Insert at all cursors
    doc.insert("* ");
    assert_eq!(doc.buffer().to_string(), "* apple\n* banana\n* cherry");
    
    // Undo
    doc.undo();
    assert_eq!(doc.buffer().to_string(), "apple\nbanana\ncherry");
}

#[test]
fn test_selection_and_delete() {
    let mut doc = Document::from_str("The quick brown fox");
    
    // Select "quick "
    doc.set_selections(Selections::from_selection(
        Selection::new(Position::new(0, 4), Position::new(0, 10))
    ));
    
    // Delete selection
    doc.delete();
    assert_eq!(doc.buffer().to_string(), "The brown fox");
    
    // Undo
    doc.undo();
    assert_eq!(doc.buffer().to_string(), "The quick brown fox");
}

#[test]
fn test_line_operations() {
    let mut doc = Document::from_str("Line 1\nLine 2\nLine 3");
    
    // Move to second line
    doc.move_cursor(Movement::Down);
    doc.move_cursor(Movement::LineEnd);
    
    // Insert at end of line
    doc.insert(" extended");
    assert_eq!(doc.buffer().line(1).unwrap().trim(), "Line 2 extended");
}

#[test]
fn test_buffer_positions() {
    let buffer = Buffer::from_str("Line 1\nLine 2\nLine 3");
    
    // Test position to offset conversion
    let pos = Position::new(1, 3);
    let offset = buffer.pos_to_offset(pos).unwrap();
    let back = buffer.offset_to_pos(offset).unwrap();
    assert_eq!(back, pos);
}

#[test]
fn test_buffer_slicing() {
    let buffer = Buffer::from_str("Hello, world!");
    
    let slice = buffer.slice(Range::new(
        Position::new(0, 0),
        Position::new(0, 5)
    )).unwrap();
    assert_eq!(slice, "Hello");
    
    let slice2 = buffer.slice(Range::new(
        Position::new(0, 7),
        Position::new(0, 12)
    )).unwrap();
    assert_eq!(slice2, "world");
}

#[test]
fn test_undo_grouping() {
    let mut stack = UndoStack::new(UndoConfig::default());
    let mut buffer = Buffer::new();
    let selections = Selections::single(Position::zero());
    
    // Quick succession of edits (should be grouped)
    for ch in "Hello".chars() {
        let edit = Edit::Insert {
            position: Position::new(0, 0),
            text: ch.to_string(),
        };
        edit.apply(&mut buffer).unwrap();
        stack.push(edit, selections.clone());
    }
    
    assert_eq!(buffer.to_string(), "Hello");
    
    // All edits should undo in one group
    stack.undo(&mut buffer);
    assert_eq!(buffer.to_string(), "");
    assert_eq!(stack.undo_count(), 0);
}

#[test]
fn test_selection_merging() {
    let mut selections = Selections::from_vec(vec![
        Selection::new(Position::new(0, 0), Position::new(0, 5)),
        Selection::new(Position::new(0, 3), Position::new(0, 8)),
        Selection::new(Position::new(0, 10), Position::new(0, 15)),
    ]);
    
    selections.normalize();
    
    // First two should merge, third remains separate
    assert_eq!(selections.len(), 2);
}

#[test]
fn test_empty_document_operations() {
    let mut doc = Document::new();
    
    // Operations on empty document shouldn't panic
    doc.move_cursor(Movement::Left);
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Up);
    doc.move_cursor(Movement::Down);
    doc.delete();
    doc.delete_backward();
    
    assert_eq!(doc.buffer().to_string(), "");
}

#[test]
fn test_large_document() {
    // Create a document with many lines
    let text = (0..1000).map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    
    let mut doc = Document::from_str(&text);
    assert_eq!(doc.buffer().line_count(), 1000);
    
    // Navigate to middle
    doc.set_selections(Selections::single(Position::new(500, 0)));
    doc.insert("MIDDLE");
    
    assert!(doc.buffer().line(500).unwrap().contains("MIDDLE"));
    
    // Undo
    doc.undo();
    assert!(!doc.buffer().line(500).unwrap().contains("MIDDLE"));
}

#[test]
fn test_unicode_handling() {
    let mut doc = Document::from_str("Hello, 世界!");
    
    // Move through unicode characters
    doc.move_cursor(Movement::Right);
    doc.insert("X");
    
    // Should handle unicode correctly
    assert!(doc.buffer().to_string().contains("HXello"));
}

#[test]
fn test_document_modification_tracking() {
    let mut doc = Document::new();
    assert!(!doc.is_modified());
    
    doc.insert("Hello");
    assert!(doc.is_modified());
    
    // Simulate save (we'll just manually update the saved version)
    let version = doc.buffer().version();
    
    // Undo should still show as modified from original
    doc.undo();
    assert!(doc.is_modified());
}

#[test]
fn test_select_all() {
    let mut doc = Document::from_str("Line 1\nLine 2\nLine 3");
    doc.select_all();
    
    assert!(!doc.selections().primary().is_cursor());
    assert_eq!(doc.selections().primary().range().start, Position::zero());
}

#[test]
fn test_movement_edge_cases() {
    let mut doc = Document::from_str("ab\ncd\nef");
    
    // Move to end
    doc.move_cursor(Movement::DocumentEnd);
    let pos = doc.selections().primary().cursor();
    assert_eq!(pos.line, 2);
    
    // Try to move past end
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Right);
    doc.move_cursor(Movement::Down);
    
    // Should stay at valid position
    assert!(doc.buffer().is_valid_position(doc.selections().primary().cursor()));
}

#[test]
fn test_multiline_edit() {
    let mut doc = Document::from_str("Line 1\nLine 2\nLine 3");
    
    // Select across lines
    doc.set_selections(Selections::from_selection(
        Selection::new(Position::new(0, 3), Position::new(2, 3))
    ));
    
    doc.delete();
    
    // Should delete across lines
    assert_eq!(doc.buffer().to_string(), "Line 3");
}
