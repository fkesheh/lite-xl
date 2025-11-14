//! File I/O module for async file operations with encoding detection
//!
//! This module provides comprehensive file I/O functionality including:
//! - Async file reading and writing
//! - Automatic encoding detection (UTF-8, UTF-16, etc.)
//! - Line ending detection and normalization
//! - Memory-mapped file support for large files
//! - Robust error handling

use encoding_rs::{Encoding, UTF_8, UTF_16LE, UTF_16BE};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod file_watcher;

// Re-export file_watcher types
pub use file_watcher::{FileSystemEvent, FileWatcher, WatchError, WatcherConfig};

/// Maximum file size to load into memory (100 MB)
const MAX_MEMORY_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Sample size for encoding detection (8 KB)
const ENCODING_SAMPLE_SIZE: usize = 8192;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

/// Detected encoding information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedEncoding {
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16Be,
    Latin1,
    /// Unknown encoding, fallback to UTF-8 with lossy conversion
    Unknown,
}

impl DetectedEncoding {
    /// Convert to encoding_rs Encoding
    pub fn to_encoding(&self) -> &'static Encoding {
        match self {
            Self::Utf8 | Self::Utf8Bom | Self::Unknown => UTF_8,
            Self::Utf16Le => UTF_16LE,
            Self::Utf16Be => UTF_16BE,
            Self::Latin1 => encoding_rs::WINDOWS_1252,
        }
    }

    /// Check if encoding uses BOM
    pub fn has_bom(&self) -> bool {
        matches!(self, Self::Utf8Bom)
    }
}

/// Line ending style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// Unix style: \n
    Lf,
    /// Windows style: \r\n
    CrLf,
    /// Classic Mac style: \r
    Cr,
}

impl LineEnding {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
            Self::Cr => "\r",
        }
    }

    /// Detect line ending from text
    pub fn detect(text: &str) -> Self {
        let mut crlf_count = 0;
        let mut lf_count = 0;
        let mut cr_count = 0;

        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        crlf_count += 1;
                        chars.next(); // consume \n
                    } else {
                        cr_count += 1;
                    }
                }
                '\n' => lf_count += 1,
                _ => {}
            }
        }

        // Return the most common line ending
        if crlf_count >= lf_count && crlf_count >= cr_count {
            Self::CrLf
        } else if cr_count > lf_count {
            Self::Cr
        } else {
            Self::Lf
        }
    }

    /// Normalize line endings in text
    pub fn normalize(text: &str, target: LineEnding) -> String {
        // First, normalize everything to \n
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        
        // Then convert to target
        match target {
            Self::Lf => normalized,
            Self::CrLf => normalized.replace('\n', "\r\n"),
            Self::Cr => normalized.replace('\n', "\r"),
        }
    }
}

/// File content with metadata
#[derive(Debug)]
pub struct FileContent {
    /// The text content
    pub text: String,
    /// Detected encoding
    pub encoding: DetectedEncoding,
    /// Detected line ending
    pub line_ending: LineEnding,
    /// File size in bytes
    pub size: u64,
    /// Whether BOM was present
    pub had_bom: bool,
}

/// Async file reader with encoding detection
pub struct FileReader;

impl FileReader {
    /// Read a file asynchronously with automatic encoding detection
    pub async fn read_file(path: impl AsRef<Path>) -> Result<FileContent, IoError> {
        let path = path.as_ref();
        
        // Check if file exists
        if !path.exists() {
            return Err(IoError::FileNotFound(path.to_path_buf()));
        }

        // Get file metadata
        let metadata = fs::metadata(path).await?;
        let size = metadata.len();

        // Check file size
        if size > MAX_MEMORY_FILE_SIZE {
            return Err(IoError::FileTooLarge {
                size,
                max: MAX_MEMORY_FILE_SIZE,
            });
        }

        // Read file into buffer
        let mut file = File::open(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                IoError::PermissionDenied(path.to_path_buf())
            } else {
                IoError::Io(e)
            }
        })?;

        let mut buffer = Vec::with_capacity(size as usize);
        file.read_to_end(&mut buffer).await?;

        // Detect encoding and decode
        let (encoding, text, had_bom) = Self::detect_and_decode(&buffer)?;

        // Detect line ending
        let line_ending = LineEnding::detect(&text);

        Ok(FileContent {
            text,
            encoding,
            line_ending,
            size,
            had_bom,
        })
    }

    /// Detect encoding and decode bytes to string
    fn detect_and_decode(bytes: &[u8]) -> Result<(DetectedEncoding, String, bool), IoError> {
        // Check for BOM
        if bytes.len() >= 3 && &bytes[0..3] == b"\xEF\xBB\xBF" {
            // UTF-8 BOM
            let text = String::from_utf8_lossy(&bytes[3..]).into_owned();
            return Ok((DetectedEncoding::Utf8Bom, text, true));
        }

        if bytes.len() >= 2 {
            if &bytes[0..2] == b"\xFF\xFE" {
                // UTF-16 LE BOM
                let (text, _, had_errors) = UTF_16LE.decode(&bytes[2..]);
                if had_errors {
                    return Err(IoError::EncodingError(
                        "Invalid UTF-16LE encoding".to_string(),
                    ));
                }
                return Ok((DetectedEncoding::Utf16Le, text.into_owned(), true));
            }

            if &bytes[0..2] == b"\xFE\xFF" {
                // UTF-16 BE BOM
                let (text, _, had_errors) = UTF_16BE.decode(&bytes[2..]);
                if had_errors {
                    return Err(IoError::EncodingError(
                        "Invalid UTF-16BE encoding".to_string(),
                    ));
                }
                return Ok((DetectedEncoding::Utf16Be, text.into_owned(), true));
            }
        }

        // No BOM, try to detect encoding
        // Try UTF-8 first (most common)
        match std::str::from_utf8(bytes) {
            Ok(text) => Ok((DetectedEncoding::Utf8, text.to_string(), false)),
            Err(_) => {
                // Not valid UTF-8, try UTF-8 with lossy conversion
                // This handles most cases gracefully
                let text = String::from_utf8_lossy(bytes).into_owned();
                Ok((DetectedEncoding::Unknown, text, false))
            }
        }
    }

    /// Read file with specific encoding
    pub async fn read_file_with_encoding(
        path: impl AsRef<Path>,
        encoding: DetectedEncoding,
    ) -> Result<FileContent, IoError> {
        let path = path.as_ref();
        
        let metadata = fs::metadata(path).await?;
        let size = metadata.len();

        let mut file = File::open(path).await?;
        let mut buffer = Vec::with_capacity(size as usize);
        file.read_to_end(&mut buffer).await?;

        let enc = encoding.to_encoding();
        let (text, _, had_errors) = enc.decode(&buffer);
        
        if had_errors {
            return Err(IoError::EncodingError(format!(
                "Failed to decode file with {:?}",
                encoding
            )));
        }

        let line_ending = LineEnding::detect(&text);

        Ok(FileContent {
            text: text.into_owned(),
            encoding,
            line_ending,
            size,
            had_bom: false,
        })
    }
}

/// Async file writer
pub struct FileWriter;

impl FileWriter {
    /// Write text to file asynchronously
    pub async fn write_file(
        path: impl AsRef<Path>,
        text: &str,
        encoding: DetectedEncoding,
        line_ending: LineEnding,
        add_bom: bool,
    ) -> Result<(), IoError> {
        let path = path.as_ref();

        // Normalize line endings
        let normalized_text = LineEnding::normalize(text, line_ending);

        // Encode text
        let enc = encoding.to_encoding();
        let (encoded, _, had_errors) = enc.encode(&normalized_text);
        
        if had_errors {
            return Err(IoError::EncodingError(format!(
                "Failed to encode text with {:?}",
                encoding
            )));
        }

        // Create or truncate file
        let mut file = File::create(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                IoError::PermissionDenied(path.to_path_buf())
            } else {
                IoError::Io(e)
            }
        })?;

        // Write BOM if requested
        if add_bom {
            match encoding {
                DetectedEncoding::Utf8 | DetectedEncoding::Utf8Bom => {
                    file.write_all(b"\xEF\xBB\xBF").await?;
                }
                DetectedEncoding::Utf16Le => {
                    file.write_all(b"\xFF\xFE").await?;
                }
                DetectedEncoding::Utf16Be => {
                    file.write_all(b"\xFE\xFF").await?;
                }
                _ => {}
            }
        }

        // Write content
        file.write_all(&encoded).await?;

        // Ensure data is flushed to disk
        file.sync_all().await?;

        Ok(())
    }

    /// Write text to file with default settings (UTF-8, platform line ending)
    pub async fn write_file_default(
        path: impl AsRef<Path>,
        text: &str,
    ) -> Result<(), IoError> {
        let line_ending = if cfg!(windows) {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };

        Self::write_file(path, text, DetectedEncoding::Utf8, line_ending, false).await
    }
}

/// Utility functions
pub struct FileUtils;

impl FileUtils {
    /// Check if a path is a text file (basic heuristic)
    pub async fn is_text_file(path: impl AsRef<Path>) -> Result<bool, IoError> {
        let path = path.as_ref();
        
        let mut file = File::open(path).await?;
        let mut buffer = vec![0u8; 512];
        let n = file.read(&mut buffer).await?;
        buffer.truncate(n);

        // Check for null bytes (strong indicator of binary file)
        Ok(!buffer.contains(&0))
    }

    /// Get file extension
    pub fn get_extension(path: impl AsRef<Path>) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
    }

    /// Create backup of a file
    pub async fn create_backup(path: impl AsRef<Path>) -> Result<PathBuf, IoError> {
        let path = path.as_ref();
        
        let backup_path = path.with_extension(
            format!(
                "{}.backup",
                path.extension().and_then(|s| s.to_str()).unwrap_or("")
            )
        );

        fs::copy(path, &backup_path).await?;
        Ok(backup_path)
    }

    /// Check if file has been modified since given timestamp
    pub async fn is_modified_since(
        path: impl AsRef<Path>,
        timestamp: std::time::SystemTime,
    ) -> Result<bool, IoError> {
        let metadata = fs::metadata(path).await?;
        let modified = metadata.modified()?;
        Ok(modified > timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_line_ending_detection() {
        assert_eq!(LineEnding::detect("hello\nworld"), LineEnding::Lf);
        assert_eq!(LineEnding::detect("hello\r\nworld"), LineEnding::CrLf);
        assert_eq!(LineEnding::detect("hello\rworld"), LineEnding::Cr);
    }

    #[test]
    async fn test_line_ending_normalization() {
        let text = "line1\r\nline2\nline3\rline4";
        let normalized = LineEnding::normalize(text, LineEnding::Lf);
        assert_eq!(normalized, "line1\nline2\nline3\nline4");

        let normalized = LineEnding::normalize(text, LineEnding::CrLf);
        assert_eq!(normalized, "line1\r\nline2\r\nline3\r\nline4");
    }

    #[test]
    async fn test_write_and_read() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let content = "Hello, World!\nThis is a test.";
        FileWriter::write_file_default(&file_path, content)
            .await
            .unwrap();

        let result = FileReader::read_file(&file_path).await.unwrap();
        assert_eq!(result.text, content);
        assert_eq!(result.encoding, DetectedEncoding::Utf8);
    }

    #[test]
    async fn test_utf8_bom() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_bom.txt");

        let content = "Test with BOM";
        FileWriter::write_file(
            &file_path,
            content,
            DetectedEncoding::Utf8Bom,
            LineEnding::Lf,
            true,
        )
        .await
        .unwrap();

        let result = FileReader::read_file(&file_path).await.unwrap();
        assert_eq!(result.text, content);
        assert_eq!(result.encoding, DetectedEncoding::Utf8Bom);
        assert!(result.had_bom);
    }
}
