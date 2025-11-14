//! Example demonstrating syntax highlighting
//!
//! This example shows:
//! - Creating a syntax highlighter for Rust code
//! - Highlighting lines of code
//! - Available languages and themes

use lite_xl::SyntaxHighlighter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Syntax Highlighting Demo ===\n");

    // Create a highlighter for Rust
    let highlighter = SyntaxHighlighter::new("Rust")?;

    println!("Highlighting Rust code:\n");
    
    let code = r#"fn main() {
    println!("Hello, World!");
    let x = 42;
    let name = "Rust";
}"#;

    for (i, line) in code.lines().enumerate() {
        println!("Line {}: {}", i + 1, line);
        let spans = highlighter.highlight_line(i, line);
        println!("  Spans: {} highlighted segments", spans.len());
        for span in spans {
            println!("    - '{}' (RGB: {:?})", span.text, span.fg_color);
        }
    }

    println!("\n=== Available Languages ===");
    let languages = SyntaxHighlighter::available_languages();
    println!("Total languages: {}", languages.len());
    for (i, lang) in languages.iter().take(10).enumerate() {
        println!("{}. {}", i + 1, lang);
    }

    println!("\n=== Available Themes ===");
    let themes = SyntaxHighlighter::available_themes();
    println!("Total themes: {}", themes.len());
    for (i, theme) in themes.iter().take(5).enumerate() {
        println!("{}. {}", i + 1, theme);
    }

    Ok(())
}
