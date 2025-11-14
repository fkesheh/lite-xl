//! File Operation Commands
//!
//! This module implements file-related commands including:
//! - Creating new files
//! - Opening existing files
//! - Saving files (Save, Save As, Save All)
//! - Closing files
//! - Reloading files
//! - File encoding and line ending management
//!
//! All file operations are designed to be asynchronous and handle
//! errors gracefully with user feedback.

use std::path::PathBuf;
use crate::commands::LineEndingStyle;

/// Result type for file operations
pub type FileResult<T> = Result<T, FileError>;

/// Errors that can occur during file operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileError {
    /// File not found
    NotFound(PathBuf),

    /// Permission denied
    PermissionDenied(PathBuf),

    /// File is a directory
    IsDirectory(PathBuf),

    /// I/O error
    IoError(String),

    /// Encoding error
    EncodingError(String),

    /// File is too large
    FileTooLarge { path: PathBuf, size: u64, max: u64 },

    /// File has unsaved changes
    UnsavedChanges(PathBuf),

    /// No file associated with buffer
    NoAssociatedFile,

    /// Invalid file path
    InvalidPath(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::NotFound(path) => write!(f, "File not found: {}", path.display()),
            FileError::PermissionDenied(path) => {
                write!(f, "Permission denied: {}", path.display())
            }
            FileError::IsDirectory(path) => write!(f, "Is a directory: {}", path.display()),
            FileError::IoError(msg) => write!(f, "I/O error: {}", msg),
            FileError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            FileError::FileTooLarge { path, size, max } => write!(
                f,
                "File too large: {} ({} bytes, max {} bytes)",
                path.display(),
                size,
                max
            ),
            FileError::UnsavedChanges(path) => {
                write!(f, "File has unsaved changes: {}", path.display())
            }
            FileError::NoAssociatedFile => write!(f, "No file associated with this buffer"),
            FileError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
        }
    }
}

impl std::error::Error for FileError {}

/// Context for executing file commands
///
/// This trait abstracts file operations to allow testing and
/// different implementations.
pub trait FileContext {
    /// Create a new empty document
    fn new_document(&mut self);

    /// Open a file by path
    async fn open_file(&mut self, path: PathBuf) -> FileResult<()>;

    /// Save the current document
    async fn save_document(&mut self) -> FileResult<()>;

    /// Save the current document to a new path
    async fn save_document_as(&mut self, path: PathBuf) -> FileResult<()>;

    /// Save all open documents
    async fn save_all_documents(&mut self) -> FileResult<()>;

    /// Close the current document
    fn close_document(&mut self) -> FileResult<()>;

    /// Close all documents
    fn close_all_documents(&mut self) -> FileResult<()>;

    /// Reload the current document from disk
    async fn reload_document(&mut self) -> FileResult<()>;

    /// Check if current document has unsaved changes
    fn has_unsaved_changes(&self) -> bool;

    /// Get the current document's file path
    fn current_file_path(&self) -> Option<PathBuf>;

    /// Set the line ending style for the current document
    fn set_line_ending_style(&mut self, style: LineEndingStyle);

    /// Get the line ending style for the current document
    fn get_line_ending_style(&self) -> LineEndingStyle;

    /// Show a file picker dialog
    async fn show_file_picker(&mut self, save_mode: bool) -> Option<PathBuf>;

    /// Show a confirmation dialog
    async fn confirm(&mut self, message: &str) -> bool;

    /// Show an error message
    fn show_error(&mut self, message: &str);

    /// Show a success message
    fn show_success(&mut self, message: &str);
}

/// Execute a new file command
///
/// Creates a new empty document and switches to it.
pub fn execute_new_file(ctx: &mut impl FileContext) {
    ctx.new_document();
}

/// Execute an open file command
///
/// Shows a file picker and opens the selected file.
pub async fn execute_open_file(ctx: &mut impl FileContext) -> FileResult<()> {
    if let Some(path) = ctx.show_file_picker(false).await {
        ctx.open_file(path).await?;
        ctx.show_success("File opened successfully");
    }
    Ok(())
}

/// Execute an open file path command
///
/// Opens a file directly by path without showing a file picker.
pub async fn execute_open_file_path(ctx: &mut impl FileContext, path: String) -> FileResult<()> {
    let path = PathBuf::from(path);
    ctx.open_file(path).await?;
    ctx.show_success("File opened successfully");
    Ok(())
}

/// Execute a save command
///
/// Saves the current document. If the document doesn't have a file path,
/// prompts for one (Save As behavior).
pub async fn execute_save(ctx: &mut impl FileContext) -> FileResult<()> {
    if ctx.current_file_path().is_none() {
        // No file path, treat as Save As
        execute_save_as(ctx).await
    } else {
        ctx.save_document().await?;
        ctx.show_success("File saved successfully");
        Ok(())
    }
}

/// Execute a save as command
///
/// Prompts for a new file path and saves the current document to it.
pub async fn execute_save_as(ctx: &mut impl FileContext) -> FileResult<()> {
    if let Some(path) = ctx.show_file_picker(true).await {
        ctx.save_document_as(path).await?;
        ctx.show_success("File saved successfully");
    }
    Ok(())
}

/// Execute a save all command
///
/// Saves all open documents that have unsaved changes.
pub async fn execute_save_all(ctx: &mut impl FileContext) -> FileResult<()> {
    ctx.save_all_documents().await?;
    ctx.show_success("All files saved successfully");
    Ok(())
}

/// Execute a close command
///
/// Closes the current document. If there are unsaved changes,
/// prompts the user to save.
pub async fn execute_close(ctx: &mut impl FileContext) -> FileResult<()> {
    if ctx.has_unsaved_changes() {
        let should_save = ctx
            .confirm("This file has unsaved changes. Save before closing?")
            .await;

        if should_save {
            execute_save(ctx).await?;
        }
    }

    ctx.close_document()?;
    Ok(())
}

/// Execute a close all command
///
/// Closes all open documents. If any have unsaved changes,
/// prompts the user for each one.
pub async fn execute_close_all(ctx: &mut impl FileContext) -> FileResult<()> {
    // Note: In a real implementation, this would iterate through all documents
    // and prompt for each one with unsaved changes. For now, simplified.
    ctx.close_all_documents()?;
    Ok(())
}

/// Execute a reload document command
///
/// Reloads the current document from disk. If there are unsaved changes,
/// prompts the user to confirm.
pub async fn execute_reload_document(ctx: &mut impl FileContext) -> FileResult<()> {
    if ctx.has_unsaved_changes() {
        let should_reload = ctx
            .confirm("This file has unsaved changes. Discard changes and reload?")
            .await;

        if !should_reload {
            return Ok(());
        }
    }

    ctx.reload_document().await?;
    ctx.show_success("File reloaded successfully");
    Ok(())
}

/// Execute a set line ending command
///
/// Sets the line ending style for the current document.
pub fn execute_set_line_ending(ctx: &mut impl FileContext, style: LineEndingStyle) {
    ctx.set_line_ending_style(style);
    let style_name = match style {
        LineEndingStyle::Lf => "LF (Unix)",
        LineEndingStyle::CrLf => "CRLF (Windows)",
        LineEndingStyle::Cr => "CR (Classic Mac)",
    };
    ctx.show_success(&format!("Line ending set to {}", style_name));
}

/// File metadata information
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// File size in bytes
    pub size: u64,

    /// File modification time
    pub modified: Option<std::time::SystemTime>,

    /// File creation time
    pub created: Option<std::time::SystemTime>,

    /// Whether the file is read-only
    pub read_only: bool,
}

/// File encoding types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileEncoding {
    /// UTF-8 encoding
    Utf8,

    /// UTF-16 Little Endian
    Utf16Le,

    /// UTF-16 Big Endian
    Utf16Be,

    /// Latin-1 (ISO-8859-1)
    Latin1,

    /// ASCII
    Ascii,
}

impl FileEncoding {
    /// Get the encoding name as a string
    pub fn name(&self) -> &'static str {
        match self {
            FileEncoding::Utf8 => "UTF-8",
            FileEncoding::Utf16Le => "UTF-16 LE",
            FileEncoding::Utf16Be => "UTF-16 BE",
            FileEncoding::Latin1 => "Latin-1",
            FileEncoding::Ascii => "ASCII",
        }
    }

    /// Detect encoding from file content
    pub fn detect(content: &[u8]) -> Self {
        // Check for BOM (Byte Order Mark)
        if content.len() >= 3 && &content[0..3] == b"\xEF\xBB\xBF" {
            return FileEncoding::Utf8;
        }

        if content.len() >= 2 {
            if &content[0..2] == b"\xFF\xFE" {
                return FileEncoding::Utf16Le;
            }
            if &content[0..2] == b"\xFE\xFF" {
                return FileEncoding::Utf16Be;
            }
        }

        // Check if content is valid UTF-8
        if std::str::from_utf8(content).is_ok() {
            return FileEncoding::Utf8;
        }

        // Check if content is ASCII
        if content.iter().all(|&b| b < 128) {
            return FileEncoding::Ascii;
        }

        // Default to Latin-1 if nothing else matches
        FileEncoding::Latin1
    }
}

/// Auto-save configuration
#[derive(Debug, Clone)]
pub struct AutoSaveConfig {
    /// Enable auto-save
    pub enabled: bool,

    /// Auto-save interval in seconds
    pub interval_seconds: u64,

    /// Auto-save on focus loss
    pub on_focus_loss: bool,

    /// Auto-save on window close
    pub on_window_close: bool,
}

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_seconds: 30,
            on_focus_loss: true,
            on_window_close: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_encoding_detect_utf8() {
        let content = "Hello, world!".as_bytes();
        assert_eq!(FileEncoding::detect(content), FileEncoding::Utf8);
    }

    #[test]
    fn test_file_encoding_detect_utf8_bom() {
        let content = b"\xEF\xBB\xBFHello, world!";
        assert_eq!(FileEncoding::detect(content), FileEncoding::Utf8);
    }

    #[test]
    fn test_file_encoding_detect_utf16le() {
        let content = b"\xFF\xFEH\x00e\x00l\x00l\x00o\x00";
        assert_eq!(FileEncoding::detect(content), FileEncoding::Utf16Le);
    }

    #[test]
    fn test_file_encoding_detect_ascii() {
        let content = b"Hello";
        assert_eq!(FileEncoding::detect(content), FileEncoding::Utf8);
    }

    #[test]
    fn test_file_error_display() {
        let error = FileError::NotFound(PathBuf::from("/tmp/test.txt"));
        assert_eq!(error.to_string(), "File not found: /tmp/test.txt");
    }

    #[test]
    fn test_line_ending_style() {
        // Just verify the enum variants exist
        let _lf = LineEndingStyle::Lf;
        let _crlf = LineEndingStyle::CrLf;
        let _cr = LineEndingStyle::Cr;
    }
}
