//! Example demonstrating file I/O operations
//!
//! This example shows:
//! - Reading files with encoding detection
//! - Writing files with different encodings
//! - Line ending detection and normalization

use lite_xl::{FileReader, FileWriter, DetectedEncoding, LineEnding};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== File I/O Demo ===\n");

    // Create a temporary file
    let temp_path = PathBuf::from("/tmp/test_file.txt");

    // Write a file with specific settings
    let content = "Hello, World!\nThis is a test file.\nWith multiple lines.";
    println!("Writing file to: {}", temp_path.display());
    FileWriter::write_file(
        &temp_path,
        content,
        DetectedEncoding::Utf8,
        LineEnding::Lf,
        false,
    )
    .await?;

    // Read the file back
    println!("\nReading file...");
    let file_content = FileReader::read_file(&temp_path).await?;

    println!("Encoding: {:?}", file_content.encoding);
    println!("Line ending: {:?}", file_content.line_ending);
    println!("Size: {} bytes", file_content.size);
    println!("Had BOM: {}", file_content.had_bom);
    println!("\nContent:\n{}", file_content.text);

    // Clean up
    std::fs::remove_file(&temp_path)?;

    Ok(())
}
