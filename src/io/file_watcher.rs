//! File watching module using the notify crate
//!
//! This module provides file system watching capabilities to detect external
//! changes to files being edited. Features include:
//! - Debounced file change notifications
//! - Multiple file watching
//! - Event filtering
//! - Async event delivery

use notify::{
    Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use notify_debouncer_full::{
    new_debouncer, DebounceEventResult, Debouncer, FileIdMap,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
pub enum WatchError {
    #[error("Failed to create watcher: {0}")]
    WatcherCreation(String),

    #[error("Failed to watch path: {0}")]
    WatchPath(String),

    #[error("Path not being watched: {0}")]
    PathNotWatched(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),
}

/// File system event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileSystemEvent {
    /// File was modified
    Modified(PathBuf),
    
    /// File was created
    Created(PathBuf),
    
    /// File was deleted
    Deleted(PathBuf),
    
    /// File was renamed
    Renamed {
        from: PathBuf,
        to: PathBuf,
    },
    
    /// Generic change (fallback)
    Changed(PathBuf),
}

impl FileSystemEvent {
    /// Get the primary path associated with this event
    pub fn path(&self) -> &Path {
        match self {
            Self::Modified(p)
            | Self::Created(p)
            | Self::Deleted(p)
            | Self::Changed(p) => p,
            Self::Renamed { to, .. } => to,
        }
    }

    /// Check if this event indicates file modification
    pub fn is_modification(&self) -> bool {
        matches!(self, Self::Modified(_) | Self::Changed(_))
    }

    /// Check if this event indicates file deletion
    pub fn is_deletion(&self) -> bool {
        matches!(self, Self::Deleted(_))
    }
}

/// File watcher configuration
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// Debounce duration (default: 500ms)
    pub debounce_duration: Duration,
    
    /// Whether to watch directories recursively
    pub recursive: bool,
    
    /// File extensions to watch (None = all files)
    pub extensions: Option<Vec<String>>,
    
    /// Whether to ignore hidden files
    pub ignore_hidden: bool,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            debounce_duration: Duration::from_millis(500),
            recursive: false,
            extensions: None,
            ignore_hidden: true,
        }
    }
}

/// File watcher that monitors file system changes
pub struct FileWatcher {
    /// Debounced watcher instance
    debouncer: Arc<Mutex<Debouncer<RecommendedWatcher, FileIdMap>>>,
    
    /// Event receiver
    event_rx: mpsc::UnboundedReceiver<FileSystemEvent>,
    
    /// Watched paths
    watched_paths: Arc<Mutex<HashMap<PathBuf, RecursiveMode>>>,
    
    /// Configuration
    config: WatcherConfig,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(config: WatcherConfig) -> Result<Self, WatchError> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let watched_paths = Arc::new(Mutex::new(HashMap::new()));
        
        let watched_paths_clone = Arc::clone(&watched_paths);
        let extensions = config.extensions.clone();
        let ignore_hidden = config.ignore_hidden;

        // Create debouncer
        let debouncer = new_debouncer(
            config.debounce_duration,
            None,
            move |result: DebounceEventResult| {
                match result {
                    Ok(events) => {
                        for event in events {
                            // Filter events based on configuration
                            if let Some(fs_event) = Self::convert_event(
                                event.event,
                                &extensions,
                                ignore_hidden,
                            ) {
                                let _ = event_tx.send(fs_event);
                            }
                        }
                    }
                    Err(errors) => {
                        eprintln!("File watcher errors: {:?}", errors);
                    }
                }
            },
        )
        .map_err(|e| WatchError::WatcherCreation(e.to_string()))?;

        Ok(Self {
            debouncer: Arc::new(Mutex::new(debouncer)),
            event_rx,
            watched_paths: watched_paths_clone,
            config,
        })
    }

    /// Create a new file watcher with default configuration
    pub fn new_default() -> Result<Self, WatchError> {
        Self::new(WatcherConfig::default())
    }

    /// Watch a file or directory
    pub fn watch(&mut self, path: impl AsRef<Path>) -> Result<(), WatchError> {
        let path = path.as_ref().to_path_buf();
        
        if !path.exists() {
            return Err(WatchError::WatchPath(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }

        let recursive_mode = if self.config.recursive && path.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        // Add watch
        let mut debouncer = self.debouncer.lock().unwrap();
        debouncer
            .watcher()
            .watch(&path, recursive_mode)
            .map_err(|e| WatchError::WatchPath(e.to_string()))?;

        // Track watched path
        let mut watched = self.watched_paths.lock().unwrap();
        watched.insert(path, recursive_mode);

        Ok(())
    }

    /// Stop watching a file or directory
    pub fn unwatch(&mut self, path: impl AsRef<Path>) -> Result<(), WatchError> {
        let path = path.as_ref();
        
        // Check if path is being watched
        let mut watched = self.watched_paths.lock().unwrap();
        if !watched.contains_key(path) {
            return Err(WatchError::PathNotWatched(path.to_path_buf()));
        }

        // Remove watch
        let mut debouncer = self.debouncer.lock().unwrap();
        debouncer
            .watcher()
            .unwatch(path)
            .map_err(|e| WatchError::Notify(e))?;

        watched.remove(path);

        Ok(())
    }

    /// Check if a path is being watched
    pub fn is_watching(&self, path: impl AsRef<Path>) -> bool {
        let watched = self.watched_paths.lock().unwrap();
        watched.contains_key(path.as_ref())
    }

    /// Get list of all watched paths
    pub fn watched_paths(&self) -> Vec<PathBuf> {
        let watched = self.watched_paths.lock().unwrap();
        watched.keys().cloned().collect()
    }

    /// Receive the next file system event (async)
    pub async fn next_event(&mut self) -> Option<FileSystemEvent> {
        self.event_rx.recv().await
    }

    /// Try to receive an event without blocking
    pub fn try_recv_event(&mut self) -> Option<FileSystemEvent> {
        self.event_rx.try_recv().ok()
    }

    /// Convert notify event to our FileSystemEvent
    fn convert_event(
        event: Event,
        extensions: &Option<Vec<String>>,
        ignore_hidden: bool,
    ) -> Option<FileSystemEvent> {
        // Filter by extension if configured
        if let Some(exts) = extensions {
            let has_valid_ext = event.paths.iter().any(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| exts.iter().any(|ext| ext.eq_ignore_ascii_case(e)))
                    .unwrap_or(false)
            });

            if !has_valid_ext {
                return None;
            }
        }

        // Filter hidden files if configured
        if ignore_hidden {
            let has_hidden = event.paths.iter().any(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with('.'))
                    .unwrap_or(false)
            });

            if has_hidden {
                return None;
            }
        }

        // Convert event kind
        match event.kind {
            EventKind::Create(_) => {
                event.paths.first().map(|p| FileSystemEvent::Created(p.clone()))
            }
            EventKind::Modify(_) => {
                event.paths.first().map(|p| FileSystemEvent::Modified(p.clone()))
            }
            EventKind::Remove(_) => {
                event.paths.first().map(|p| FileSystemEvent::Deleted(p.clone()))
            }
            EventKind::Any => {
                event.paths.first().map(|p| FileSystemEvent::Changed(p.clone()))
            }
            _ => event.paths.first().map(|p| FileSystemEvent::Changed(p.clone())),
        }
    }

    /// Stop watching all paths
    pub fn stop_all(&mut self) -> Result<(), WatchError> {
        let paths: Vec<PathBuf> = {
            let watched = self.watched_paths.lock().unwrap();
            watched.keys().cloned().collect()
        };

        for path in paths {
            self.unwatch(&path)?;
        }

        Ok(())
    }
}

/// Builder for FileWatcher with fluent API
pub struct FileWatcherBuilder {
    config: WatcherConfig,
}

impl FileWatcherBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: WatcherConfig::default(),
        }
    }

    /// Set debounce duration
    pub fn debounce_duration(mut self, duration: Duration) -> Self {
        self.config.debounce_duration = duration;
        self
    }

    /// Enable recursive watching
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    }

    /// Set file extensions to watch
    pub fn extensions(mut self, extensions: Vec<String>) -> Self {
        self.config.extensions = Some(extensions);
        self
    }

    /// Set whether to ignore hidden files
    pub fn ignore_hidden(mut self, ignore: bool) -> Self {
        self.config.ignore_hidden = ignore;
        self
    }

    /// Build the FileWatcher
    pub fn build(self) -> Result<FileWatcher, WatchError> {
        FileWatcher::new(self.config)
    }
}

impl Default for FileWatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use tokio::time::{sleep, timeout};

    #[tokio::test]
    async fn test_file_modification() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "initial content").unwrap();

        let mut watcher = FileWatcher::new_default().unwrap();
        watcher.watch(&file_path).unwrap();

        // Give watcher time to initialize
        sleep(Duration::from_millis(100)).await;

        // Modify file
        fs::write(&file_path, "modified content").unwrap();

        // Wait for event
        let event = timeout(Duration::from_secs(2), watcher.next_event())
            .await
            .expect("Timeout waiting for event")
            .expect("No event received");

        assert!(event.is_modification());
        assert_eq!(event.path(), file_path.as_path());
    }

    #[tokio::test]
    async fn test_file_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let mut watcher = FileWatcher::new_default().unwrap();
        watcher.watch(&file_path).unwrap();

        sleep(Duration::from_millis(100)).await;

        // Delete file
        fs::remove_file(&file_path).unwrap();

        // Wait for event
        let event = timeout(Duration::from_secs(2), watcher.next_event())
            .await
            .expect("Timeout waiting for event")
            .expect("No event received");

        assert!(event.is_deletion());
    }

    #[tokio::test]
    async fn test_extension_filter() {
        let temp_dir = TempDir::new().unwrap();
        let txt_file = temp_dir.path().join("test.txt");
        let rs_file = temp_dir.path().join("test.rs");
        
        fs::write(&txt_file, "txt content").unwrap();
        fs::write(&rs_file, "rs content").unwrap();

        let config = WatcherConfig {
            extensions: Some(vec!["rs".to_string()]),
            ..Default::default()
        };

        let mut watcher = FileWatcher::new(config).unwrap();
        watcher.watch(temp_dir.path()).unwrap();

        sleep(Duration::from_millis(100)).await;

        // Modify txt file (should be ignored)
        fs::write(&txt_file, "modified txt").unwrap();
        sleep(Duration::from_millis(600)).await;

        // Should not receive event for .txt file
        assert!(watcher.try_recv_event().is_none());

        // Modify rs file (should be detected)
        fs::write(&rs_file, "modified rs").unwrap();

        let event = timeout(Duration::from_secs(2), watcher.next_event())
            .await
            .expect("Timeout waiting for event")
            .expect("No event received");

        assert_eq!(event.path(), rs_file.as_path());
    }

    #[test]
    fn test_builder() {
        let watcher = FileWatcherBuilder::new()
            .debounce_duration(Duration::from_millis(300))
            .recursive(true)
            .extensions(vec!["rs".to_string(), "toml".to_string()])
            .ignore_hidden(false)
            .build();

        assert!(watcher.is_ok());
    }
}
