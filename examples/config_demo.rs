//! Example demonstrating configuration management
//!
//! This example shows:
//! - Loading and saving configuration
//! - Default configuration values
//! - Configuration merging

use lite_xl::Config;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Configuration Demo ===\n");

    // Create default configuration
    let config = Config::default();

    println!("Default configuration:");
    println!("  Editor:");
    println!("    Tab width: {}", config.editor.tab_width);
    println!("    Use spaces: {}", config.editor.use_spaces);
    println!("    Line ending: {}", config.editor.line_ending);
    println!("    Max file size: {} MB", config.editor.max_file_size_mb);
    println!("\n  UI:");
    println!("    Font family: {}", config.ui.font_family);
    println!("    Font size: {}", config.ui.font_size);
    println!("    Theme: {}", config.ui.theme);
    println!("    Show line numbers: {}", config.ui.show_line_numbers);

    // Save to a temporary file
    let temp_path = PathBuf::from("/tmp/editor_config.toml");
    println!("\nSaving configuration to: {}", temp_path.display());
    config.save(&temp_path).await?;

    // Load it back
    println!("Loading configuration...");
    let loaded_config = Config::load(&temp_path).await?;

    println!("\nLoaded configuration matches: {}", 
        loaded_config.editor.tab_width == config.editor.tab_width);

    // Display example TOML
    println!("\n=== Example TOML Configuration ===");
    println!("{}", lite_xl::config::EXAMPLE_CONFIG);

    // Clean up
    std::fs::remove_file(&temp_path)?;

    Ok(())
}
