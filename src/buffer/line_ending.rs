//! Line ending detection and conversion utilities.
//!
//! This module provides functionality for detecting and converting between different
//! line ending styles (LF, CRLF, CR).
//!
//! # Examples
//!
//! ```
//! use lite_xl::buffer::line_ending::{LineEnding, detect_line_ending};
//!
//! let text_unix = "line1\nline2\nline3";
//! assert_eq!(detect_line_ending(text_unix), LineEnding::Lf);
//!
//! let text_windows = "line1\r\nline2\r\nline3";
//! assert_eq!(detect_line_ending(text_windows), LineEnding::CrLf);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Line ending style.
///
/// Represents the different line ending conventions used across operating systems.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::line_ending::LineEnding;
///
/// let unix = LineEnding::Lf;
/// assert_eq!(unix.as_str(), "\n");
///
/// let windows = LineEnding::CrLf;
/// assert_eq!(windows.as_str(), "\r\n");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineEnding {
    /// Unix/Linux style: Line Feed (`\n`)
    Lf,
    /// Windows style: Carriage Return + Line Feed (`\r\n`)
    CrLf,
    /// Classic Mac style: Carriage Return (`\r`)
    Cr,
}

impl LineEnding {
    /// Returns the string representation of this line ending.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::line_ending::LineEnding;
    ///
    /// assert_eq!(LineEnding::Lf.as_str(), "\n");
    /// assert_eq!(LineEnding::CrLf.as_str(), "\r\n");
    /// assert_eq!(LineEnding::Cr.as_str(), "\r");
    /// ```
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
            LineEnding::Cr => "\r",
        }
    }

    /// Returns the byte length of this line ending.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::line_ending::LineEnding;
    ///
    /// assert_eq!(LineEnding::Lf.len(), 1);
    /// assert_eq!(LineEnding::CrLf.len(), 2);
    /// assert_eq!(LineEnding::Cr.len(), 1);
    /// ```
    #[inline]
    pub const fn len(self) -> usize {
        match self {
            LineEnding::Lf => 1,
            LineEnding::CrLf => 2,
            LineEnding::Cr => 1,
        }
    }

    /// Returns the name of this line ending style.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::line_ending::LineEnding;
    ///
    /// assert_eq!(LineEnding::Lf.name(), "LF");
    /// assert_eq!(LineEnding::CrLf.name(), "CRLF");
    /// assert_eq!(LineEnding::Cr.name(), "CR");
    /// ```
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            LineEnding::Lf => "LF",
            LineEnding::CrLf => "CRLF",
            LineEnding::Cr => "CR",
        }
    }

    /// Returns the platform's native line ending.
    ///
    /// On Unix-like systems (Linux, macOS), this returns `LineEnding::Lf`.
    /// On Windows, this returns `LineEnding::CrLf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::line_ending::LineEnding;
    ///
    /// let native = LineEnding::native();
    /// #[cfg(unix)]
    /// assert_eq!(native, LineEnding::Lf);
    /// #[cfg(windows)]
    /// assert_eq!(native, LineEnding::CrLf);
    /// ```
    #[inline]
    pub const fn native() -> Self {
        #[cfg(windows)]
        {
            LineEnding::CrLf
        }
        #[cfg(not(windows))]
        {
            LineEnding::Lf
        }
    }

    /// Converts text from this line ending to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::buffer::line_ending::LineEnding;
    ///
    /// let text = "line1\nline2\nline3";
    /// let converted = LineEnding::Lf.convert_to(text, LineEnding::CrLf);
    /// assert_eq!(converted, "line1\r\nline2\r\nline3");
    /// ```
    pub fn convert_to(self, text: &str, target: LineEnding) -> String {
        if self == target {
            return text.to_string();
        }

        normalize_line_endings(text, target)
    }
}

impl Default for LineEnding {
    /// Returns the platform's native line ending as the default.
    fn default() -> Self {
        Self::native()
    }
}

impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<&str> for LineEnding {
    /// Attempts to parse a line ending from a string.
    ///
    /// Falls back to `LineEnding::Lf` if the string doesn't match a known line ending.
    fn from(s: &str) -> Self {
        match s {
            "\n" => LineEnding::Lf,
            "\r\n" => LineEnding::CrLf,
            "\r" => LineEnding::Cr,
            _ => LineEnding::Lf,
        }
    }
}

/// Detects the line ending style used in the given text.
///
/// The function counts occurrences of each line ending type and returns
/// the most common one. If no line endings are found, returns the platform's
/// native line ending.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::line_ending::{LineEnding, detect_line_ending};
///
/// let unix_text = "line1\nline2\nline3";
/// assert_eq!(detect_line_ending(unix_text), LineEnding::Lf);
///
/// let windows_text = "line1\r\nline2\r\nline3";
/// assert_eq!(detect_line_ending(windows_text), LineEnding::CrLf);
///
/// let mac_text = "line1\rline2\rline3";
/// assert_eq!(detect_line_ending(mac_text), LineEnding::Cr);
/// ```
pub fn detect_line_ending(text: &str) -> LineEnding {
    let mut crlf_count = 0;
    let mut lf_count = 0;
    let mut cr_count = 0;

    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    crlf_count += 1;
                    i += 2;
                } else {
                    cr_count += 1;
                    i += 1;
                }
            }
            b'\n' => {
                lf_count += 1;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Return the most common line ending
    if crlf_count >= lf_count && crlf_count >= cr_count {
        if crlf_count > 0 {
            LineEnding::CrLf
        } else {
            LineEnding::native()
        }
    } else if lf_count >= cr_count {
        if lf_count > 0 {
            LineEnding::Lf
        } else {
            LineEnding::native()
        }
    } else {
        if cr_count > 0 {
            LineEnding::Cr
        } else {
            LineEnding::native()
        }
    }
}

/// Detects the line ending with detailed statistics.
///
/// Returns the detected line ending along with counts of each type found.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::line_ending::{LineEnding, detect_line_ending_with_stats};
///
/// let text = "line1\nline2\r\nline3\n";
/// let (ending, stats) = detect_line_ending_with_stats(text);
/// assert_eq!(ending, LineEnding::Lf);
/// assert_eq!(stats.lf, 2);
/// assert_eq!(stats.crlf, 1);
/// assert_eq!(stats.cr, 0);
/// ```
pub fn detect_line_ending_with_stats(text: &str) -> (LineEnding, LineEndingStats) {
    let mut stats = LineEndingStats::default();

    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    stats.crlf += 1;
                    i += 2;
                } else {
                    stats.cr += 1;
                    i += 1;
                }
            }
            b'\n' => {
                stats.lf += 1;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    let ending = if stats.crlf >= stats.lf && stats.crlf >= stats.cr {
        if stats.crlf > 0 {
            LineEnding::CrLf
        } else {
            LineEnding::native()
        }
    } else if stats.lf >= stats.cr {
        if stats.lf > 0 {
            LineEnding::Lf
        } else {
            LineEnding::native()
        }
    } else {
        if stats.cr > 0 {
            LineEnding::Cr
        } else {
            LineEnding::native()
        }
    };

    (ending, stats)
}

/// Statistics about line endings in text.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LineEndingStats {
    /// Number of LF line endings
    pub lf: usize,
    /// Number of CRLF line endings
    pub crlf: usize,
    /// Number of CR line endings
    pub cr: usize,
}

impl LineEndingStats {
    /// Returns the total number of line endings.
    #[inline]
    pub fn total(&self) -> usize {
        self.lf + self.crlf + self.cr
    }

    /// Checks if the text uses consistent line endings.
    #[inline]
    pub fn is_consistent(&self) -> bool {
        let has_lf = self.lf > 0;
        let has_crlf = self.crlf > 0;
        let has_cr = self.cr > 0;

        // Only one type should be present
        (has_lf as u8 + has_crlf as u8 + has_cr as u8) <= 1
    }
}

/// Normalizes all line endings in text to the specified style.
///
/// This function converts all line endings (LF, CRLF, CR) to the target style.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::line_ending::{LineEnding, normalize_line_endings};
///
/// let mixed = "line1\nline2\r\nline3\rline4";
/// let normalized = normalize_line_endings(mixed, LineEnding::Lf);
/// assert_eq!(normalized, "line1\nline2\nline3\nline4");
/// ```
pub fn normalize_line_endings(text: &str, target: LineEnding) -> String {
    let target_str = target.as_str();

    // Estimate capacity to reduce allocations
    let estimated_capacity = text.len() + (text.len() / 80) * target.len();
    let mut result = String::with_capacity(estimated_capacity);

    let bytes = text.as_bytes();
    let mut i = 0;
    let mut last_copied = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                // Copy everything before this line ending
                result.push_str(&text[last_copied..i]);

                // Add the target line ending
                result.push_str(target_str);

                // Skip the line ending (handle both CR and CRLF)
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    i += 2;
                } else {
                    i += 1;
                }
                last_copied = i;
            }
            b'\n' => {
                // Copy everything before this line ending
                result.push_str(&text[last_copied..i]);

                // Add the target line ending
                result.push_str(target_str);

                i += 1;
                last_copied = i;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Copy any remaining text
    if last_copied < text.len() {
        result.push_str(&text[last_copied..]);
    }

    result
}

/// Counts the number of lines in text, respecting different line endings.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::line_ending::count_lines;
///
/// assert_eq!(count_lines("line1\nline2\nline3"), 3);
/// assert_eq!(count_lines("line1\r\nline2\r\nline3"), 3);
/// assert_eq!(count_lines("single line"), 1);
/// assert_eq!(count_lines(""), 0);
/// ```
pub fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    let mut count = 1; // Start with 1 for the first line
    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                count += 1;
                // Skip LF if this is CRLF
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            b'\n' => {
                count += 1;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    count
}

/// Splits text into lines, preserving line endings.
///
/// # Examples
///
/// ```
/// use lite_xl::buffer::line_ending::split_lines_with_endings;
///
/// let text = "line1\nline2\nline3";
/// let lines: Vec<_> = split_lines_with_endings(text).collect();
/// assert_eq!(lines, vec!["line1\n", "line2\n", "line3"]);
/// ```
pub fn split_lines_with_endings(text: &str) -> impl Iterator<Item = &str> {
    let mut last = 0;
    let bytes = text.as_bytes();

    std::iter::from_fn(move || {
        if last >= text.len() {
            return None;
        }

        let mut i = last;
        while i < bytes.len() {
            match bytes[i] {
                b'\r' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                        let line = &text[last..i + 2];
                        last = i + 2;
                        return Some(line);
                    } else {
                        let line = &text[last..i + 1];
                        last = i + 1;
                        return Some(line);
                    }
                }
                b'\n' => {
                    let line = &text[last..i + 1];
                    last = i + 1;
                    return Some(line);
                }
                _ => {
                    i += 1;
                }
            }
        }

        // Last line without ending
        if last < text.len() {
            let line = &text[last..];
            last = text.len();
            Some(line)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_ending_as_str() {
        assert_eq!(LineEnding::Lf.as_str(), "\n");
        assert_eq!(LineEnding::CrLf.as_str(), "\r\n");
        assert_eq!(LineEnding::Cr.as_str(), "\r");
    }

    #[test]
    fn test_line_ending_len() {
        assert_eq!(LineEnding::Lf.len(), 1);
        assert_eq!(LineEnding::CrLf.len(), 2);
        assert_eq!(LineEnding::Cr.len(), 1);
    }

    #[test]
    fn test_detect_line_ending() {
        assert_eq!(detect_line_ending("line1\nline2\nline3"), LineEnding::Lf);
        assert_eq!(
            detect_line_ending("line1\r\nline2\r\nline3"),
            LineEnding::CrLf
        );
        assert_eq!(detect_line_ending("line1\rline2\rline3"), LineEnding::Cr);

        // Mixed - should return most common
        assert_eq!(detect_line_ending("line1\nline2\r\n"), LineEnding::Lf);

        // No line endings - should return native
        assert_eq!(detect_line_ending("single line"), LineEnding::native());
    }

    #[test]
    fn test_detect_with_stats() {
        let (ending, stats) = detect_line_ending_with_stats("line1\nline2\r\nline3\n");
        assert_eq!(ending, LineEnding::Lf);
        assert_eq!(stats.lf, 2);
        assert_eq!(stats.crlf, 1);
        assert_eq!(stats.cr, 0);
        assert_eq!(stats.total(), 3);
        assert!(!stats.is_consistent());

        let (ending2, stats2) = detect_line_ending_with_stats("line1\nline2\nline3\n");
        assert_eq!(ending2, LineEnding::Lf);
        assert!(stats2.is_consistent());
    }

    #[test]
    fn test_normalize_line_endings() {
        let mixed = "line1\nline2\r\nline3\rline4";

        let unix = normalize_line_endings(mixed, LineEnding::Lf);
        assert_eq!(unix, "line1\nline2\nline3\nline4");

        let windows = normalize_line_endings(mixed, LineEnding::CrLf);
        assert_eq!(windows, "line1\r\nline2\r\nline3\r\nline4");

        let mac = normalize_line_endings(mixed, LineEnding::Cr);
        assert_eq!(mac, "line1\rline2\rline3\rline4");
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("line1\nline2\nline3"), 3);
        assert_eq!(count_lines("line1\r\nline2\r\nline3"), 3);
        assert_eq!(count_lines("line1\rline2\rline3"), 3);
        assert_eq!(count_lines("single line"), 1);
        assert_eq!(count_lines(""), 0);
        assert_eq!(count_lines("line1\n"), 2); // One line plus newline creates second empty line
    }

    #[test]
    fn test_split_lines_with_endings() {
        let text = "line1\nline2\nline3";
        let lines: Vec<_> = split_lines_with_endings(text).collect();
        assert_eq!(lines, vec!["line1\n", "line2\n", "line3"]);

        let text2 = "line1\r\nline2\r\nline3";
        let lines2: Vec<_> = split_lines_with_endings(text2).collect();
        assert_eq!(lines2, vec!["line1\r\n", "line2\r\n", "line3"]);
    }

    #[test]
    fn test_convert_to() {
        let unix_text = "line1\nline2\nline3";
        let converted = LineEnding::Lf.convert_to(unix_text, LineEnding::CrLf);
        assert_eq!(converted, "line1\r\nline2\r\nline3");
    }

    #[test]
    fn test_native() {
        let native = LineEnding::native();
        #[cfg(windows)]
        assert_eq!(native, LineEnding::CrLf);
        #[cfg(not(windows))]
        assert_eq!(native, LineEnding::Lf);
    }
}
