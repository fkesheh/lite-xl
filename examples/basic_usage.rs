//! Basic usage example for Lite XL document module.
//!
//! This demonstrates the core functionality of the document system.

use lite_xl::document::Document;
use lite_xl::buffer::Position;
use lite_xl::document::{Selection, Selections, Movement};

fn main() {
    println!("Lite XL Document Module - Basic Usage Example\n");

    // Example 1: Create and edit a document
    println!("=== Example 1: Basic Editing ===");
    let mut doc = Document::new();
    doc.insert("Hello, world!");
    println!("Buffer content: {}", doc.buffer().to_string());
    println!("Is modified: {}", doc.is_modified());
    println!();

    // Example 2: Undo/Redo
    println!("=== Example 2: Undo/Redo ===");
    doc.undo();
    println!("After undo: {}", doc.buffer().to_string());
    doc.redo();
    println!("After redo: {}", doc.buffer().to_string());
    println!();

    // Example 3: Multi-cursor editing
    println!("=== Example 3: Multi-Cursor Editing ===");
    let mut doc2 = Document::from_str("apple\nbanana\ncherry");
    println!("Original:\n{}", doc2.buffer().to_string());

    // Set three cursors at line starts
    let selections = Selections::from_vec(vec![
        Selection::cursor(Position::new(0, 0)),
        Selection::cursor(Position::new(1, 0)),
        Selection::cursor(Position::new(2, 0)),
    ]);
    doc2.set_selections(selections);

    // Insert at all cursors
    doc2.insert("- ");
    println!("\nAfter multi-cursor insert:\n{}", doc2.buffer().to_string());
    println!();

    // Example 4: Movement
    println!("=== Example 4: Cursor Movement ===");
    let mut doc3 = Document::from_str("Line 1\nLine 2\nLine 3");
    println!("Initial cursor: {:?}", doc3.selections().primary().cursor());

    doc3.move_cursor(Movement::Down);
    println!("After moving down: {:?}", doc3.selections().primary().cursor());

    doc3.move_cursor(Movement::LineEnd);
    println!("After moving to line end: {:?}", doc3.selections().primary().cursor());
    println!();

    // Example 5: Selection and deletion
    println!("=== Example 5: Selection and Deletion ===");
    let mut doc4 = Document::from_str("The quick brown fox");
    println!("Original: {}", doc4.buffer().to_string());

    doc4.set_selections(Selections::from_selection(
        Selection::new(Position::new(0, 4), Position::new(0, 10))
    ));
    println!("Selected: 'quick '");

    doc4.delete();
    println!("After deletion: {}", doc4.buffer().to_string());

    doc4.undo();
    println!("After undo: {}", doc4.buffer().to_string());
    println!();

    println!("=== All examples completed successfully! ===");
}
