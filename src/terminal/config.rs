//! Terminal configuration module
//!
//! This module provides configuration for terminal emulation including:
//! - Color schemes
//! - Scrollback buffer size
//! - Shell command and arguments
//! - Font settings
//! - Cursor style

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Terminal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Shell command to run
    #[serde(default = "default_shell")]
    pub shell: String,

    /// Shell arguments
    #[serde(default)]
    pub shell_args: Vec<String>,

    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Number of scrollback lines
    #[serde(default = "default_scrollback")]
    pub scrollback_lines: usize,

    /// Font family (default: "monospace")
    #[serde(default = "default_font_family")]
    pub font_family: String,

    /// Font size for terminal
    #[serde(default = "default_font_size")]
    pub font_size: f32,

    /// Cursor style
    #[serde(default)]
    pub cursor_style: CursorStyle,

    /// Cursor blink rate in milliseconds (0 = no blink)
    #[serde(default = "default_cursor_blink")]
    pub cursor_blink_rate_ms: u64,

    /// Default terminal width in columns
    #[serde(default = "default_cols")]
    pub cols: u16,

    /// Default terminal height in rows
    #[serde(default = "default_rows")]
    pub rows: u16,

    /// Working directory (default: current directory)
    #[serde(default)]
    pub working_directory: Option<PathBuf>,

    /// Terminal color scheme
    #[serde(default)]
    pub colors: TerminalColors,

    /// Enable bell (default: true)
    #[serde(default = "default_true")]
    pub enable_bell: bool,

    /// Enable URL detection (default: true)
    #[serde(default = "default_true")]
    pub enable_url_detection: bool,

    /// Enable hyperlinks (default: true)
    #[serde(default = "default_true")]
    pub enable_hyperlinks: bool,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            shell_args: Vec::new(),
            env: HashMap::new(),
            scrollback_lines: default_scrollback(),
            font_family: default_font_family(),
            font_size: default_font_size(),
            cursor_style: CursorStyle::default(),
            cursor_blink_rate_ms: default_cursor_blink(),
            cols: default_cols(),
            rows: default_rows(),
            working_directory: None,
            colors: TerminalColors::default(),
            enable_bell: true,
            enable_url_detection: true,
            enable_hyperlinks: true,
        }
    }
}

impl TerminalConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.scrollback_lines == 0 {
            return Err("scrollback_lines must be greater than 0".to_string());
        }

        if self.scrollback_lines > 1_000_000 {
            return Err("scrollback_lines must be less than 1,000,000".to_string());
        }

        if self.font_size <= 0.0 || self.font_size > 72.0 {
            return Err("font_size must be between 0 and 72".to_string());
        }

        if self.cols == 0 || self.rows == 0 {
            return Err("cols and rows must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Merge with another configuration (other takes precedence)
    pub fn merge(&mut self, other: TerminalConfig) {
        *self = other;
    }
}

/// Cursor style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CursorStyle {
    /// Block cursor (█)
    Block,
    /// Underline cursor (_)
    Underline,
    /// Vertical bar cursor (|)
    Bar,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Block
    }
}

/// Terminal color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalColors {
    /// Foreground color (default: #e0e0e0)
    #[serde(default = "default_foreground")]
    pub foreground: String,

    /// Background color (default: #1e1e1e)
    #[serde(default = "default_background")]
    pub background: String,

    /// Cursor color (default: #ffffff)
    #[serde(default = "default_cursor")]
    pub cursor: String,

    /// Selection background color (default: #264f78)
    #[serde(default = "default_selection")]
    pub selection_background: String,

    /// Selection foreground color (default: none, use normal foreground)
    #[serde(default)]
    pub selection_foreground: Option<String>,

    /// Standard ANSI colors (0-15)
    #[serde(default = "default_ansi_colors")]
    pub ansi_colors: Vec<String>,

    /// Extended 256 color palette (optional)
    #[serde(default)]
    pub palette: Vec<String>,
}

impl Default for TerminalColors {
    fn default() -> Self {
        Self {
            foreground: default_foreground(),
            background: default_background(),
            cursor: default_cursor(),
            selection_background: default_selection(),
            selection_foreground: None,
            ansi_colors: default_ansi_colors(),
            palette: Vec::new(),
        }
    }
}

impl TerminalColors {
    /// Parse hex color to RGB
    pub fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some((r, g, b))
    }

    /// Get ANSI color by index (0-255)
    pub fn get_color(&self, index: u8) -> Option<String> {
        if (index as usize) < self.ansi_colors.len() {
            Some(self.ansi_colors[index as usize].clone())
        } else if !self.palette.is_empty() {
            self.palette.get(index as usize).cloned()
        } else {
            None
        }
    }
}

// Default value functions
#[cfg(unix)]
fn default_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

#[cfg(windows)]
fn default_shell() -> String {
    std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
}

fn default_scrollback() -> usize {
    10000
}

fn default_font_family() -> String {
    "monospace".to_string()
}

fn default_font_size() -> f32 {
    14.0
}

fn default_cols() -> u16 {
    80
}

fn default_rows() -> u16 {
    24
}

fn default_cursor_blink() -> u64 {
    500
}

fn default_true() -> bool {
    true
}

fn default_foreground() -> String {
    "#e0e0e0".to_string()
}

fn default_background() -> String {
    "#1e1e1e".to_string()
}

fn default_cursor() -> String {
    "#ffffff".to_string()
}

fn default_selection() -> String {
    "#264f78".to_string()
}

/// Default ANSI colors (16 colors: 8 normal + 8 bright)
fn default_ansi_colors() -> Vec<String> {
    vec![
        // Normal colors
        "#000000".to_string(), // Black
        "#cd3131".to_string(), // Red
        "#0dbc79".to_string(), // Green
        "#e5e510".to_string(), // Yellow
        "#2472c8".to_string(), // Blue
        "#bc3fbc".to_string(), // Magenta
        "#11a8cd".to_string(), // Cyan
        "#e5e5e5".to_string(), // White
        // Bright colors
        "#666666".to_string(), // Bright Black
        "#f14c4c".to_string(), // Bright Red
        "#23d18b".to_string(), // Bright Green
        "#f5f543".to_string(), // Bright Yellow
        "#3b8eea".to_string(), // Bright Blue
        "#d670d6".to_string(), // Bright Magenta
        "#29b8db".to_string(), // Bright Cyan
        "#ffffff".to_string(), // Bright White
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TerminalConfig::default();
        assert_eq!(config.scrollback_lines, 10000);
        assert_eq!(config.font_size, 14.0);
        assert_eq!(config.cursor_style, CursorStyle::Block);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation() {
        let mut config = TerminalConfig::default();
        assert!(config.validate().is_ok());

        config.scrollback_lines = 0;
        assert!(config.validate().is_err());

        config.scrollback_lines = 10000;
        config.font_size = 0.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(
            TerminalColors::parse_hex_color("#ff0000"),
            Some((255, 0, 0))
        );
        assert_eq!(
            TerminalColors::parse_hex_color("00ff00"),
            Some((0, 255, 0))
        );
        assert_eq!(TerminalColors::parse_hex_color("#invalid"), None);
    }

    #[test]
    fn test_ansi_colors() {
        let colors = TerminalColors::default();
        assert_eq!(colors.ansi_colors.len(), 16);

        // Test getting colors
        assert_eq!(colors.get_color(0), Some("#000000".to_string()));
        assert_eq!(colors.get_color(1), Some("#cd3131".to_string()));
    }

    #[test]
    fn test_cursor_style_serialization() {
        let config = r#"cursor_style = "block""#;
        let style: CursorStyle = toml::from_str::<HashMap<String, CursorStyle>>(config)
            .unwrap()
            .remove("cursor_style")
            .unwrap();
        assert_eq!(style, CursorStyle::Block);
    }
}
