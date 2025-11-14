//! Example demonstrating file watching
//!
//! This example shows:
//! - Watching a file for changes
//! - Receiving file system events
//! - Handling different event types

use lite_xl::{FileWatcher, FileSystemEvent};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== File Watcher Demo ===\n");

    // Create a temporary file to watch
    let temp_path = PathBuf::from("/tmp/watched_file.txt");
    std::fs::write(&temp_path, "Initial content")?;

    // Create file watcher
    let mut watcher = FileWatcher::new_default()?;

    println!("Watching file: {}", temp_path.display());
    watcher.watch(&temp_path)?;

    // Spawn a task to modify the file after a short delay
    let watch_path = temp_path.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        println!("\n[Background] Modifying file...");
        std::fs::write(&watch_path, "Modified content").unwrap();
        
        sleep(Duration::from_secs(2)).await;
        println!("[Background] Modifying again...");
        std::fs::write(&watch_path, "Modified content again").unwrap();
        
        sleep(Duration::from_secs(2)).await;
        println!("[Background] Deleting file...");
        std::fs::remove_file(&watch_path).unwrap();
    });

    // Listen for events
    println!("Listening for events (will wait up to 10 seconds)...\n");
    
    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();
    
    while start.elapsed() < timeout {
        if let Ok(event) = tokio::time::timeout(
            Duration::from_millis(100),
            watcher.next_event()
        ).await {
            if let Some(event) = event {
                match event {
                    FileSystemEvent::Modified(path) => {
                        println!("✓ File modified: {}", path.display());
                    }
                    FileSystemEvent::Created(path) => {
                        println!("✓ File created: {}", path.display());
                    }
                    FileSystemEvent::Deleted(path) => {
                        println!("✓ File deleted: {}", path.display());
                        break; // Exit after deletion
                    }
                    FileSystemEvent::Renamed { from, to } => {
                        println!("✓ File renamed: {} -> {}", from.display(), to.display());
                    }
                    FileSystemEvent::Changed(path) => {
                        println!("✓ File changed: {}", path.display());
                    }
                }
            }
        }
    }

    println!("\nDemo completed!");

    // Clean up if file still exists
    if temp_path.exists() {
        std::fs::remove_file(&temp_path)?;
    }

    Ok(())
}
