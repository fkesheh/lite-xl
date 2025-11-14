//! Demo of the buffer module functionality.
//!
//! Run with: cargo run --example buffer_demo

use lite_xl::buffer::{Buffer, Position, Range, LineEnding, detect_line_ending, normalize_line_endings};

fn main() {
    println!("=== Lite XL Buffer Module Demo ===\n");

    // 1. Basic buffer creation
    println!("1. Creating buffers:");
    let mut buffer = Buffer::new();
    println!("   - Empty buffer: {} lines, {} chars", buffer.line_count(), buffer.len_chars());

    let buffer2 = Buffer::from_str("Hello\nWorld\n!");
    println!("   - From string: {} lines, {} chars", buffer2.line_count(), buffer2.len_chars());

    // 2. Insertions
    println!("\n2. Insertions:");
    buffer.insert(Position::zero(), "Hello, ").expect("Insert failed");
    buffer.insert(Position::new(0, 7), "beautiful ").expect("Insert failed");
    buffer.insert(Position::new(0, 17), "world!").expect("Insert failed");
    println!("   Buffer: '{}'", buffer.to_string());
    println!("   Modified: {}, Version: {}", buffer.is_modified(), buffer.version());

    // 3. Queries
    println!("\n3. Queries:");
    println!("   Line count: {}", buffer.line_count());
    println!("   Line 0: {:?}", buffer.line(0));
    println!("   Char at (0, 0): {:?}", buffer.char_at(Position::new(0, 0)));
    println!("   Char at (0, 7): {:?}", buffer.char_at(Position::new(0, 7)));

    // 4. Deletions
    println!("\n4. Deletions:");
    let range = Range::new(Position::new(0, 7), Position::new(0, 17));
    let deleted = buffer.delete(range).expect("Delete failed");
    println!("   Deleted: '{}'", deleted);
    println!("   Buffer: '{}'", buffer.to_string());

    // 5. Slicing
    println!("\n5. Slicing:");
    let slice_range = Range::new(Position::new(0, 0), Position::new(0, 5));
    let slice = buffer.slice(slice_range).expect("Slice failed");
    println!("   Slice (0,0) to (0,5): '{}'", slice);

    // 6. Replace
    println!("\n6. Replace:");
    let replace_range = Range::new(Position::new(0, 7), Position::new(0, 13));
    buffer.replace(replace_range, "Rust").expect("Replace failed");
    println!("   Buffer: '{}'", buffer.to_string());

    // 7. Multiline buffer
    println!("\n7. Multiline buffer:");
    let mut multiline = Buffer::from_str("Line 1\nLine 2\nLine 3");
    println!("   Lines: {}", multiline.line_count());
    for i in 0..multiline.line_count() {
        println!("   Line {}: {:?}", i, multiline.line(i));
    }

    // 8. Position/offset conversion
    println!("\n8. Position ↔ Offset conversion:");
    let pos = Position::new(1, 3);
    let offset = multiline.pos_to_offset(pos).expect("pos_to_offset failed");
    let back_to_pos = multiline.offset_to_pos(offset).expect("offset_to_pos failed");
    println!("   Position {:?} → Offset {} → Position {:?}", pos, offset, back_to_pos);

    // 9. Line ending detection
    println!("\n9. Line ending detection:");
    let unix_text = "line1\nline2\nline3";
    let windows_text = "line1\r\nline2\r\nline3";
    let mac_text = "line1\rline2\rline3";

    println!("   Unix text: {:?}", detect_line_ending(unix_text));
    println!("   Windows text: {:?}", detect_line_ending(windows_text));
    println!("   Mac text: {:?}", detect_line_ending(mac_text));

    // 10. Line ending normalization
    println!("\n10. Line ending normalization:");
    let mixed = "line1\nline2\r\nline3\rline4";
    println!("   Mixed: {:?}", mixed);
    println!("   To LF: {:?}", normalize_line_endings(mixed, LineEnding::Lf));
    println!("   To CRLF: {:?}", normalize_line_endings(mixed, LineEnding::CrLf));

    let mut buffer_with_endings = Buffer::from_str("line1\r\nline2\nline3");
    println!("   Buffer line ending: {:?}", buffer_with_endings.line_ending());
    buffer_with_endings.normalize_to_line_ending(LineEnding::Lf);
    println!("   After normalization: {:?}", buffer_with_endings.line_ending());
    println!("   Content: {:?}", buffer_with_endings.to_string());

    // 11. Iterators
    println!("\n11. Iterators:");
    let iter_buffer = Buffer::from_str("A\nB\nC");
    print!("   Lines: ");
    for line in iter_buffer.lines() {
        print!("{:?} ", line);
    }
    println!();

    print!("   First 5 chars: ");
    for ch in iter_buffer.chars().take(5) {
        print!("'{}' ", ch);
    }
    println!();

    // 12. Range operations
    println!("\n12. Range operations:");
    let pos1 = Position::new(5, 10);
    let pos2 = Position::new(8, 20);
    let range1 = Range::new(pos1, pos2);
    let range2 = Range::new(Position::new(7, 15), Position::new(10, 5));

    println!("   Range 1: {}", range1);
    println!("   Range 2: {}", range2);
    println!("   Range 1 contains (6, 0): {}", range1.contains(Position::new(6, 0)));
    println!("   Ranges overlap: {}", range1.overlaps(range2));
    println!("   Union: {}", range1.union(range2));
    if let Some(intersection) = range1.intersection(range2) {
        println!("   Intersection: {}", intersection);
    }

    // 13. Buffer clearing
    println!("\n13. Buffer clearing:");
    let mut clear_buffer = Buffer::from_str("To be cleared");
    println!("   Before: '{}' (empty: {})", clear_buffer.to_string(), clear_buffer.is_empty());
    clear_buffer.clear();
    println!("   After: '{}' (empty: {})", clear_buffer.to_string(), clear_buffer.is_empty());

    // 14. Position clamping
    println!("\n14. Position clamping:");
    let clamp_buffer = Buffer::from_str("short\nlonger line\nx");
    let out_of_bounds = Position::new(100, 100);
    let clamped = clamp_buffer.clamp_position(out_of_bounds);
    println!("   Position {} → clamped to {}", out_of_bounds, clamped);

    println!("\n=== Demo Complete ===");
}
