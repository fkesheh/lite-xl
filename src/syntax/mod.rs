//! Syntax highlighting module using syntect
//!
//! This module provides comprehensive syntax highlighting capabilities:
//! - Support for multiple languages (Rust, Python, JavaScript, Markdown, Plain text)
//! - Theme support with popular color schemes
//! - Incremental highlighting with state caching
//! - Background highlighting for performance

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use thiserror::Error;

pub mod languages;

use languages::LanguageDetector;

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("Unknown language: {0}")]
    UnknownLanguage(String),

    #[error("Theme not found: {0}")]
    ThemeNotFound(String),

    #[error("Failed to load syntax set: {0}")]
    SyntaxSetLoad(String),

    #[error("Highlighting error: {0}")]
    HighlightError(String),
}

/// Global syntax set (loaded once)
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

/// Global theme set (loaded once)
static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| ThemeSet::load_defaults());

/// A highlighted span of text
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightedSpan {
    /// Text content
    pub text: String,
    
    /// Foreground color (RGB)
    pub fg_color: (u8, u8, u8),
    
    /// Background color (RGB, optional)
    pub bg_color: Option<(u8, u8, u8)>,
    
    /// Font style flags
    pub style: HighlightStyle,
}

/// Font style flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighlightStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl From<FontStyle> for HighlightStyle {
    fn from(style: FontStyle) -> Self {
        Self {
            bold: style.contains(FontStyle::BOLD),
            italic: style.contains(FontStyle::ITALIC),
            underline: style.contains(FontStyle::UNDERLINE),
        }
    }
}

impl HighlightedSpan {
    /// Create a new highlighted span from syntect style
    pub fn from_syntect(text: String, style: Style) -> Self {
        Self {
            text,
            fg_color: (style.foreground.r, style.foreground.g, style.foreground.b),
            bg_color: None,
            style: style.font_style.into(),
        }
    }

    /// Create a plain span with default colors
    pub fn plain(text: String, fg: (u8, u8, u8)) -> Self {
        Self {
            text,
            fg_color: fg,
            bg_color: None,
            style: HighlightStyle {
                bold: false,
                italic: false,
                underline: false,
            },
        }
    }
}

/// Syntax highlighter for a document
pub struct SyntaxHighlighter {
    /// Syntax definition
    syntax: &'static SyntaxReference,
    
    /// Current theme
    theme: &'static Theme,
    
    /// Cached highlight states per line
    /// None means line hasn't been highlighted yet
    line_cache: Arc<Mutex<HashMap<usize, Vec<HighlightedSpan>>>>,
    
    /// Language name
    language_name: String,
}

impl SyntaxHighlighter {
    /// Create a new highlighter for a specific language
    pub fn new(language: &str) -> Result<Self, SyntaxError> {
        let syntax = Self::find_syntax(language)?;
        let theme = Self::get_default_theme();
        
        Ok(Self {
            syntax,
            theme,
            line_cache: Arc::new(Mutex::new(HashMap::new())),
            language_name: language.to_string(),
        })
    }

    /// Create a new highlighter by detecting language from file path
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SyntaxError> {
        let language = LanguageDetector::detect_from_path(path.as_ref())
            .ok_or_else(|| SyntaxError::UnknownLanguage("unknown".to_string()))?;
        
        Self::new(language.name())
    }

    /// Create a new highlighter with specific theme
    pub fn with_theme(language: &str, theme_name: &str) -> Result<Self, SyntaxError> {
        let syntax = Self::find_syntax(language)?;
        let theme = Self::get_theme(theme_name)?;
        
        Ok(Self {
            syntax,
            theme,
            line_cache: Arc::new(Mutex::new(HashMap::new())),
            language_name: language.to_string(),
        })
    }

    /// Highlight a single line
    pub fn highlight_line(&self, line_idx: usize, line_text: &str) -> Vec<HighlightedSpan> {
        // Check cache first
        {
            let cache = self.line_cache.lock().unwrap();
            if let Some(cached) = cache.get(&line_idx) {
                return cached.clone();
            }
        }

        // Perform highlighting
        let mut highlighter = HighlightLines::new(self.syntax, self.theme);
        let spans = match highlighter.highlight_line(line_text, &SYNTAX_SET) {
            Ok(ranges) => ranges
                .into_iter()
                .map(|(style, text)| HighlightedSpan::from_syntect(text.to_string(), style))
                .collect(),
            Err(_) => {
                // Fallback to plain text
                vec![HighlightedSpan::plain(
                    line_text.to_string(),
                    self.theme.settings.foreground.map_or(
                        (255, 255, 255),
                        |c| (c.r, c.g, c.b),
                    ),
                )]
            }
        };

        // Cache result
        {
            let mut cache = self.line_cache.lock().unwrap();
            cache.insert(line_idx, spans.clone());
        }

        spans
    }

    /// Highlight multiple lines
    pub fn highlight_lines(&self, start_line: usize, lines: &[String]) -> Vec<Vec<HighlightedSpan>> {
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| self.highlight_line(start_line + i, line))
            .collect()
    }

    /// Invalidate cache from a specific line onwards
    pub fn invalidate_from_line(&self, line_idx: usize) {
        let mut cache = self.line_cache.lock().unwrap();
        cache.retain(|&idx, _| idx < line_idx);
    }

    /// Clear all cached highlights
    pub fn clear_cache(&self) {
        let mut cache = self.line_cache.lock().unwrap();
        cache.clear();
    }

    /// Get the language name
    pub fn language(&self) -> &str {
        &self.language_name
    }

    /// Change the theme
    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), SyntaxError> {
        self.theme = Self::get_theme(theme_name)?;
        self.clear_cache(); // Invalidate cache when theme changes
        Ok(())
    }

    /// Get available theme names
    pub fn available_themes() -> Vec<String> {
        THEME_SET.themes.keys().map(|s| s.to_string()).collect()
    }

    /// Get available language names
    pub fn available_languages() -> Vec<String> {
        SYNTAX_SET
            .syntaxes()
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }

    /// Find syntax by name
    fn find_syntax(name: &str) -> Result<&'static SyntaxReference, SyntaxError> {
        SYNTAX_SET
            .find_syntax_by_name(name)
            .or_else(|| SYNTAX_SET.find_syntax_by_extension(name))
            .ok_or_else(|| SyntaxError::UnknownLanguage(name.to_string()))
    }

    /// Get theme by name
    fn get_theme(name: &str) -> Result<&'static Theme, SyntaxError> {
        THEME_SET
            .themes
            .get(name)
            .ok_or_else(|| SyntaxError::ThemeNotFound(name.to_string()))
    }

    /// Get default theme
    fn get_default_theme() -> &'static Theme {
        // Use a popular dark theme as default
        THEME_SET
            .themes
            .get("base16-ocean.dark")
            .or_else(|| THEME_SET.themes.values().next())
            .expect("No themes available")
    }

    /// Get background color from theme
    pub fn background_color(&self) -> (u8, u8, u8) {
        self.theme
            .settings
            .background
            .map_or((0, 0, 0), |c| (c.r, c.g, c.b))
    }

    /// Get foreground color from theme
    pub fn foreground_color(&self) -> (u8, u8, u8) {
        self.theme
            .settings
            .foreground
            .map_or((255, 255, 255), |c| (c.r, c.g, c.b))
    }

    /// Get selection color from theme
    pub fn selection_color(&self) -> (u8, u8, u8) {
        self.theme
            .settings
            .selection
            .map_or((64, 64, 64), |c| (c.r, c.g, c.b))
    }
}

/// Plain text highlighter (no syntax highlighting)
pub struct PlainTextHighlighter {
    fg_color: (u8, u8, u8),
}

impl PlainTextHighlighter {
    pub fn new() -> Self {
        Self {
            fg_color: (255, 255, 255),
        }
    }

    pub fn with_color(fg_color: (u8, u8, u8)) -> Self {
        Self { fg_color }
    }

    pub fn highlight_line(&self, line_text: &str) -> Vec<HighlightedSpan> {
        vec![HighlightedSpan::plain(line_text.to_string(), self.fg_color)]
    }
}

impl Default for PlainTextHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating highlighters
pub struct HighlighterFactory;

impl HighlighterFactory {
    /// Create a highlighter for a file
    pub fn create_for_file(path: impl AsRef<Path>) -> Box<dyn Highlighter> {
        match SyntaxHighlighter::from_path(path) {
            Ok(highlighter) => Box::new(highlighter),
            Err(_) => Box::new(PlainTextHighlighter::new()),
        }
    }

    /// Create a highlighter for a language
    pub fn create_for_language(language: &str) -> Box<dyn Highlighter> {
        match SyntaxHighlighter::new(language) {
            Ok(highlighter) => Box::new(highlighter),
            Err(_) => Box::new(PlainTextHighlighter::new()),
        }
    }
}

/// Trait for highlighters
pub trait Highlighter: Send + Sync {
    fn highlight_line(&self, line_idx: usize, line_text: &str) -> Vec<HighlightedSpan>;
    fn invalidate_from_line(&self, line_idx: usize);
    fn clear_cache(&self);
}

impl Highlighter for SyntaxHighlighter {
    fn highlight_line(&self, line_idx: usize, line_text: &str) -> Vec<HighlightedSpan> {
        self.highlight_line(line_idx, line_text)
    }

    fn invalidate_from_line(&self, line_idx: usize) {
        self.invalidate_from_line(line_idx);
    }

    fn clear_cache(&self) {
        self.clear_cache();
    }
}

impl Highlighter for PlainTextHighlighter {
    fn highlight_line(&self, _line_idx: usize, line_text: &str) -> Vec<HighlightedSpan> {
        self.highlight_line(line_text)
    }

    fn invalidate_from_line(&self, _line_idx: usize) {
        // No-op for plain text
    }

    fn clear_cache(&self) {
        // No-op for plain text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_highlighting() {
        let highlighter = SyntaxHighlighter::new("Rust").unwrap();
        let code = "fn main() { println!(\"Hello\"); }";
        let spans = highlighter.highlight_line(0, code);
        
        assert!(!spans.is_empty());
        // Should have multiple spans for different syntax elements
        assert!(spans.len() > 1);
    }

    #[test]
    fn test_python_highlighting() {
        let highlighter = SyntaxHighlighter::new("Python").unwrap();
        let code = "def hello(): print('world')";
        let spans = highlighter.highlight_line(0, code);
        
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_plain_text() {
        let highlighter = PlainTextHighlighter::new();
        let text = "Just plain text";
        let spans = highlighter.highlight_line(text);
        
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].text, text);
    }

    #[test]
    fn test_cache_invalidation() {
        let highlighter = SyntaxHighlighter::new("Rust").unwrap();
        
        // Highlight some lines
        highlighter.highlight_line(0, "line 0");
        highlighter.highlight_line(1, "line 1");
        highlighter.highlight_line(2, "line 2");
        
        // Invalidate from line 1
        highlighter.invalidate_from_line(1);
        
        // Line 0 should still be cached
        let cache = highlighter.line_cache.lock().unwrap();
        assert!(cache.contains_key(&0));
        assert!(!cache.contains_key(&1));
        assert!(!cache.contains_key(&2));
    }

    #[test]
    fn test_available_languages() {
        let languages = SyntaxHighlighter::available_languages();
        assert!(!languages.is_empty());
        assert!(languages.contains(&"Rust".to_string()));
        assert!(languages.contains(&"Python".to_string()));
    }

    #[test]
    fn test_available_themes() {
        let themes = SyntaxHighlighter::available_themes();
        assert!(!themes.is_empty());
    }

    #[test]
    fn test_theme_change() {
        let mut highlighter = SyntaxHighlighter::new("Rust").unwrap();
        let themes = SyntaxHighlighter::available_themes();
        
        if themes.len() > 1 {
            let theme = &themes[1];
            assert!(highlighter.set_theme(theme).is_ok());
        }
    }
}
