//! Configuration management module
//!
//! This module provides TOML-based configuration with:
//! - Default configuration values
//! - User configuration overrides
//! - Per-language settings
//! - Runtime configuration updates
//! - Configuration validation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(String),

    #[error("Failed to parse config: {0}")]
    ParseError(String),

    #[error("Failed to write config: {0}")]
    WriteError(String),

    #[error("Invalid configuration: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Global editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Editor settings
    #[serde(default)]
    pub editor: EditorConfig,

    /// UI settings
    #[serde(default)]
    pub ui: UiConfig,

    /// Keymap settings
    #[serde(default)]
    pub keymap: KeymapConfig,

    /// Language-specific settings
    #[serde(default)]
    pub languages: HashMap<String, LanguageConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorConfig::default(),
            ui: UiConfig::default(),
            keymap: KeymapConfig::default(),
            languages: Self::default_languages(),
        }
    }
}

impl Config {
    /// Load configuration from file
    pub async fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }

        let mut file = fs::File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let config: Config = toml::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        config.validate()?;

        Ok(config)
    }

    /// Load configuration from default location
    pub async fn load_default() -> Result<Self, ConfigError> {
        let config_path = Self::default_config_path()?;
        Self::load(config_path).await
    }

    /// Save configuration to file
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::WriteError(e.to_string()))?;

        fs::write(path, contents).await?;

        Ok(())
    }

    /// Save configuration to default location
    pub async fn save_default(&self) -> Result<(), ConfigError> {
        let config_path = Self::default_config_path()?;
        self.save(config_path).await
    }

    /// Get default config file path
    pub fn default_config_path() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ConfigError::ReadError("Could not find config directory".to_string()))?;

        Ok(config_dir.join("rust-editor").join("config.toml"))
    }

    /// Merge with another configuration (other takes precedence)
    pub fn merge(&mut self, other: Config) {
        self.editor.merge(other.editor);
        self.ui.merge(other.ui);
        self.keymap.merge(other.keymap);

        // Merge language configs
        for (lang, config) in other.languages {
            self.languages.insert(lang, config);
        }
    }

    /// Validate configuration
    fn validate(&self) -> Result<(), ConfigError> {
        self.editor.validate()?;
        self.ui.validate()?;
        Ok(())
    }

    /// Get default language configurations
    fn default_languages() -> HashMap<String, LanguageConfig> {
        let mut languages = HashMap::new();

        languages.insert(
            "rust".to_string(),
            LanguageConfig {
                tab_width: Some(4),
                use_spaces: Some(true),
                auto_indent: Some(true),
                line_length_guide: Some(100),
            },
        );

        languages.insert(
            "python".to_string(),
            LanguageConfig {
                tab_width: Some(4),
                use_spaces: Some(true),
                auto_indent: Some(true),
                line_length_guide: Some(88),
            },
        );

        languages.insert(
            "javascript".to_string(),
            LanguageConfig {
                tab_width: Some(2),
                use_spaces: Some(true),
                auto_indent: Some(true),
                line_length_guide: Some(80),
            },
        );

        languages.insert(
            "markdown".to_string(),
            LanguageConfig {
                tab_width: Some(2),
                use_spaces: Some(true),
                auto_indent: Some(false),
                line_length_guide: None,
            },
        );

        languages
    }
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Tab width (default: 4)
    #[serde(default = "default_tab_width")]
    pub tab_width: usize,

    /// Use spaces instead of tabs (default: true)
    #[serde(default = "default_true")]
    pub use_spaces: bool,

    /// Auto-detect indentation (default: true)
    #[serde(default = "default_true")]
    pub auto_detect_indentation: bool,

    /// Line ending style (default: "lf")
    #[serde(default = "default_line_ending")]
    pub line_ending: String,

    /// Auto-save interval in seconds (0 = disabled, default: 0)
    #[serde(default)]
    pub auto_save_interval: u64,

    /// Maximum file size to open in MB (default: 100)
    #[serde(default = "default_max_file_size")]
    pub max_file_size_mb: usize,

    /// Maximum undo history (default: 10000)
    #[serde(default = "default_undo_history")]
    pub max_undo_history: usize,

    /// Undo grouping timeout in milliseconds (default: 300)
    #[serde(default = "default_undo_timeout")]
    pub undo_group_timeout_ms: u64,

    /// Trim trailing whitespace on save (default: false)
    #[serde(default)]
    pub trim_trailing_whitespace: bool,

    /// Insert final newline on save (default: true)
    #[serde(default = "default_true")]
    pub insert_final_newline: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_width: 4,
            use_spaces: true,
            auto_detect_indentation: true,
            line_ending: "lf".to_string(),
            auto_save_interval: 0,
            max_file_size_mb: 100,
            max_undo_history: 10000,
            undo_group_timeout_ms: 300,
            trim_trailing_whitespace: false,
            insert_final_newline: true,
        }
    }
}

impl EditorConfig {
    fn merge(&mut self, other: EditorConfig) {
        *self = other;
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.tab_width == 0 || self.tab_width > 16 {
            return Err(ConfigError::ValidationError(
                "tab_width must be between 1 and 16".to_string(),
            ));
        }

        if !["lf", "crlf", "cr"].contains(&self.line_ending.as_str()) {
            return Err(ConfigError::ValidationError(
                "line_ending must be 'lf', 'crlf', or 'cr'".to_string(),
            ));
        }

        if self.max_file_size_mb == 0 || self.max_file_size_mb > 1000 {
            return Err(ConfigError::ValidationError(
                "max_file_size_mb must be between 1 and 1000".to_string(),
            ));
        }

        Ok(())
    }
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Font family (default: "monospace")
    #[serde(default = "default_font_family")]
    pub font_family: String,

    /// Font size (default: 14.0)
    #[serde(default = "default_font_size")]
    pub font_size: f32,

    /// Line height multiplier (default: 1.4)
    #[serde(default = "default_line_height")]
    pub line_height: f32,

    /// Show line numbers (default: true)
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,

    /// Highlight current line (default: true)
    #[serde(default = "default_true")]
    pub highlight_current_line: bool,

    /// Line length guide column (default: 80)
    #[serde(default = "default_line_length_guide")]
    pub line_length_guide: Option<usize>,

    /// Color theme (default: "base16-ocean.dark")
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Cursor blink rate in milliseconds (0 = no blink, default: 500)
    #[serde(default = "default_cursor_blink")]
    pub cursor_blink_rate_ms: u64,

    /// Scroll speed multiplier (default: 3.0)
    #[serde(default = "default_scroll_speed")]
    pub scroll_speed: f32,

    /// Show whitespace characters (default: false)
    #[serde(default)]
    pub show_whitespace: bool,

    /// Word wrap (default: false)
    #[serde(default)]
    pub word_wrap: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            font_family: "monospace".to_string(),
            font_size: 14.0,
            line_height: 1.4,
            show_line_numbers: true,
            highlight_current_line: true,
            line_length_guide: Some(80),
            theme: "base16-ocean.dark".to_string(),
            cursor_blink_rate_ms: 500,
            scroll_speed: 3.0,
            show_whitespace: false,
            word_wrap: false,
        }
    }
}

impl UiConfig {
    fn merge(&mut self, other: UiConfig) {
        *self = other;
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.font_size <= 0.0 || self.font_size > 72.0 {
            return Err(ConfigError::ValidationError(
                "font_size must be between 0 and 72".to_string(),
            ));
        }

        if self.line_height < 1.0 || self.line_height > 3.0 {
            return Err(ConfigError::ValidationError(
                "line_height must be between 1.0 and 3.0".to_string(),
            ));
        }

        if self.scroll_speed <= 0.0 || self.scroll_speed > 10.0 {
            return Err(ConfigError::ValidationError(
                "scroll_speed must be between 0 and 10".to_string(),
            ));
        }

        Ok(())
    }
}

/// Keymap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeymapConfig {
    /// Keybinding preset (default: "default")
    #[serde(default = "default_keymap_preset")]
    pub preset: String,

    /// Custom keybindings
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

impl Default for KeymapConfig {
    fn default() -> Self {
        Self {
            preset: "default".to_string(),
            custom: HashMap::new(),
        }
    }
}

impl KeymapConfig {
    fn merge(&mut self, other: KeymapConfig) {
        self.preset = other.preset;
        for (key, value) in other.custom {
            self.custom.insert(key, value);
        }
    }
}

/// Language-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Tab width override
    pub tab_width: Option<usize>,

    /// Use spaces override
    pub use_spaces: Option<bool>,

    /// Auto-indent override
    pub auto_indent: Option<bool>,

    /// Line length guide override
    pub line_length_guide: Option<usize>,
}

// Default value functions
fn default_tab_width() -> usize {
    4
}

fn default_true() -> bool {
    true
}

fn default_line_ending() -> String {
    "lf".to_string()
}

fn default_max_file_size() -> usize {
    100
}

fn default_undo_history() -> usize {
    10000
}

fn default_undo_timeout() -> u64 {
    300
}

fn default_font_family() -> String {
    "monospace".to_string()
}

fn default_font_size() -> f32 {
    14.0
}

fn default_line_height() -> f32 {
    1.4
}

fn default_line_length_guide() -> Option<usize> {
    Some(80)
}

fn default_theme() -> String {
    "base16-ocean.dark".to_string()
}

fn default_cursor_blink() -> u64 {
    500
}

fn default_scroll_speed() -> f32 {
    3.0
}

fn default_keymap_preset() -> String {
    "default".to_string()
}

/// Example TOML configuration
pub const EXAMPLE_CONFIG: &str = r#"
[editor]
tab_width = 4
use_spaces = true
auto_detect_indentation = true
line_ending = "lf"
auto_save_interval = 0
max_file_size_mb = 100
max_undo_history = 10000
undo_group_timeout_ms = 300
trim_trailing_whitespace = false
insert_final_newline = true

[ui]
font_family = "JetBrains Mono"
font_size = 14.0
line_height = 1.4
show_line_numbers = true
highlight_current_line = true
line_length_guide = 80
theme = "base16-ocean.dark"
cursor_blink_rate_ms = 500
scroll_speed = 3.0
show_whitespace = false
word_wrap = false

[keymap]
preset = "default"

[keymap.custom]
"ctrl+shift+d" = "duplicate_line"
"ctrl+/" = "toggle_comment"

[languages.rust]
tab_width = 4
use_spaces = true
auto_indent = true
line_length_guide = 100

[languages.python]
tab_width = 4
use_spaces = true
auto_indent = true
line_length_guide = 88

[languages.javascript]
tab_width = 2
use_spaces = true
auto_indent = true
line_length_guide = 80

[languages.markdown]
tab_width = 2
use_spaces = true
auto_indent = false
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.editor.tab_width, 4);
        assert!(config.editor.use_spaces);
        assert_eq!(config.ui.font_size, 14.0);
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let config = Config::default();
        config.save(&config_path).await.unwrap();

        let loaded = Config::load(&config_path).await.unwrap();
        assert_eq!(loaded.editor.tab_width, config.editor.tab_width);
        assert_eq!(loaded.ui.font_size, config.ui.font_size);
    }

    #[tokio::test]
    async fn test_parse_example_config() {
        let config: Config = toml::from_str(EXAMPLE_CONFIG).unwrap();
        assert_eq!(config.editor.tab_width, 4);
        assert_eq!(config.ui.font_family, "JetBrains Mono");
        assert!(config.languages.contains_key("rust"));
    }

    #[tokio::test]
    async fn test_merge_configs() {
        let mut config1 = Config::default();
        config1.editor.tab_width = 4;

        let mut config2 = Config::default();
        config2.editor.tab_width = 2;

        config1.merge(config2);
        assert_eq!(config1.editor.tab_width, 2);
    }

    #[test]
    fn test_validation() {
        let mut config = EditorConfig::default();
        assert!(config.validate().is_ok());

        config.tab_width = 0;
        assert!(config.validate().is_err());

        config.tab_width = 4;
        config.line_ending = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_ui_validation() {
        let mut config = UiConfig::default();
        assert!(config.validate().is_ok());

        config.font_size = 0.0;
        assert!(config.validate().is_err());

        config.font_size = 14.0;
        config.line_height = 0.5;
        assert!(config.validate().is_err());
    }
}
