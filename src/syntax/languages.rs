//! Language detection and definitions
//!
//! This module provides language detection based on file extensions,
//! file names, and content analysis. Supports Rust, Python, JavaScript,
//! Markdown, and plain text.

use std::collections::HashMap;
use std::path::Path;
use once_cell::sync::Lazy;

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Markdown,
    Json,
    Toml,
    Yaml,
    Html,
    Css,
    PlainText,
}

impl Language {
    /// Get the language name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Markdown => "Markdown",
            Self::Json => "JSON",
            Self::Toml => "TOML",
            Self::Yaml => "YAML",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::PlainText => "Plain Text",
        }
    }

    /// Get the syntect syntax name (may differ from display name)
    pub fn syntect_name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Markdown => "Markdown",
            Self::Json => "JSON",
            Self::Toml => "TOML",
            Self::Yaml => "YAML",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::PlainText => "Plain Text",
        }
    }

    /// Get typical file extensions for this language
    pub fn extensions(&self) -> &[&'static str] {
        match self {
            Self::Rust => &["rs"],
            Self::Python => &["py", "pyw", "pyi"],
            Self::JavaScript => &["js", "jsx", "mjs", "cjs"],
            Self::TypeScript => &["ts", "tsx"],
            Self::Markdown => &["md", "markdown", "mdown", "mkd"],
            Self::Json => &["json"],
            Self::Toml => &["toml"],
            Self::Yaml => &["yaml", "yml"],
            Self::Html => &["html", "htm"],
            Self::Css => &["css"],
            Self::PlainText => &["txt", "text"],
        }
    }

    /// Get special file names associated with this language
    pub fn special_filenames(&self) -> &[&'static str] {
        match self {
            Self::Rust => &["Cargo.toml"],
            Self::Python => &["requirements.txt", "setup.py", "pyproject.toml"],
            Self::JavaScript => &["package.json"],
            Self::TypeScript => &["tsconfig.json"],
            Self::Markdown => &["README.md", "CHANGELOG.md"],
            Self::Toml => &["Cargo.toml", "pyproject.toml"],
            _ => &[],
        }
    }

    /// Get shebang patterns for this language
    pub fn shebang_patterns(&self) -> &[&'static str] {
        match self {
            Self::Python => &["python", "python3", "python2"],
            Self::JavaScript => &["node", "nodejs"],
            _ => &[],
        }
    }

    /// Check if this language supports comments
    pub fn comment_style(&self) -> Option<CommentStyle> {
        match self {
            Self::Rust | Self::JavaScript | Self::TypeScript | Self::Css => {
                Some(CommentStyle::CStyle)
            }
            Self::Python | Self::Toml | Self::Yaml => Some(CommentStyle::Hash),
            Self::Html => Some(CommentStyle::Html),
            _ => None,
        }
    }
}

/// Comment style for a language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStyle {
    /// C-style comments: // line, /* block */
    CStyle,
    /// Hash comments: # comment
    Hash,
    /// HTML comments: <!-- comment -->
    Html,
}

impl CommentStyle {
    /// Get the line comment prefix
    pub fn line_prefix(&self) -> &'static str {
        match self {
            Self::CStyle => "//",
            Self::Hash => "#",
            Self::Html => "<!--",
        }
    }

    /// Get the block comment delimiters
    pub fn block_delimiters(&self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::CStyle => Some(("/*", "*/")),
            Self::Html => Some(("<!--", "-->")),
            Self::Hash => None,
        }
    }
}

/// Extension to language mapping
static EXTENSION_MAP: Lazy<HashMap<&'static str, Language>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for lang in ALL_LANGUAGES.iter() {
        for ext in lang.extensions() {
            map.insert(*ext, *lang);
        }
    }
    
    map
});

/// Filename to language mapping
static FILENAME_MAP: Lazy<HashMap<&'static str, Language>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    for lang in ALL_LANGUAGES.iter() {
        for name in lang.special_filenames() {
            map.insert(*name, *lang);
        }
    }
    
    map
});

/// All supported languages
static ALL_LANGUAGES: &[Language] = &[
    Language::Rust,
    Language::Python,
    Language::JavaScript,
    Language::TypeScript,
    Language::Markdown,
    Language::Json,
    Language::Toml,
    Language::Yaml,
    Language::Html,
    Language::Css,
    Language::PlainText,
];

/// Language detector
pub struct LanguageDetector;

impl LanguageDetector {
    /// Detect language from file path
    pub fn detect_from_path(path: impl AsRef<Path>) -> Option<Language> {
        let path = path.as_ref();
        
        // Check filename first (e.g., Cargo.toml, package.json)
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(lang) = FILENAME_MAP.get(filename) {
                return Some(*lang);
            }
        }
        
        // Check extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            if let Some(lang) = EXTENSION_MAP.get(ext_lower.as_str()) {
                return Some(*lang);
            }
        }
        
        None
    }

    /// Detect language from file content (using heuristics)
    pub fn detect_from_content(content: &str) -> Option<Language> {
        // Check shebang
        if let Some(lang) = Self::detect_from_shebang(content) {
            return Some(lang);
        }
        
        // Check content patterns
        Self::detect_from_patterns(content)
    }

    /// Detect language from shebang line
    fn detect_from_shebang(content: &str) -> Option<Language> {
        let first_line = content.lines().next()?;
        
        if !first_line.starts_with("#!") {
            return None;
        }
        
        let shebang = first_line.to_lowercase();
        
        for lang in ALL_LANGUAGES.iter() {
            for pattern in lang.shebang_patterns() {
                if shebang.contains(pattern) {
                    return Some(*lang);
                }
            }
        }
        
        None
    }

    /// Detect language from content patterns
    fn detect_from_patterns(content: &str) -> Option<Language> {
        // Simple heuristics based on common patterns
        
        // Rust patterns
        if content.contains("fn main()") || content.contains("impl ") || content.contains("pub struct ") {
            return Some(Language::Rust);
        }
        
        // Python patterns
        if content.contains("def ") && content.contains(":") || content.contains("import ") {
            return Some(Language::Python);
        }
        
        // JavaScript patterns
        if content.contains("function ") || content.contains("const ") || content.contains("=>") {
            return Some(Language::JavaScript);
        }
        
        // TypeScript patterns
        if content.contains("interface ") || content.contains(": string") || content.contains(": number") {
            return Some(Language::TypeScript);
        }
        
        // Markdown patterns
        if content.contains("# ") && content.contains("\n## ") {
            return Some(Language::Markdown);
        }
        
        // JSON patterns
        if content.trim_start().starts_with('{') && content.contains("\":") {
            return Some(Language::Json);
        }
        
        // TOML patterns
        if content.contains("[") && content.contains("]") && content.contains(" = ") {
            return Some(Language::Toml);
        }
        
        // HTML patterns
        if content.contains("<!DOCTYPE") || content.contains("<html") {
            return Some(Language::Html);
        }
        
        None
    }

    /// Get all supported languages
    pub fn all_languages() -> &'static [Language] {
        ALL_LANGUAGES
    }

    /// Get language by name (case-insensitive)
    pub fn get_by_name(name: &str) -> Option<Language> {
        let name_lower = name.to_lowercase();
        ALL_LANGUAGES.iter().find(|lang| {
            lang.name().to_lowercase() == name_lower
        }).copied()
    }
}

/// Language configuration
#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub language: Language,
    pub tab_width: usize,
    pub use_spaces: bool,
    pub auto_indent: bool,
}

impl LanguageConfig {
    /// Create default configuration for a language
    pub fn default_for(language: Language) -> Self {
        match language {
            Language::Rust => Self {
                language,
                tab_width: 4,
                use_spaces: true,
                auto_indent: true,
            },
            Language::Python => Self {
                language,
                tab_width: 4,
                use_spaces: true,
                auto_indent: true,
            },
            Language::JavaScript | Language::TypeScript => Self {
                language,
                tab_width: 2,
                use_spaces: true,
                auto_indent: true,
            },
            Language::Json | Language::Html | Language::Css => Self {
                language,
                tab_width: 2,
                use_spaces: true,
                auto_indent: true,
            },
            Language::Markdown | Language::PlainText => Self {
                language,
                tab_width: 4,
                use_spaces: true,
                auto_indent: false,
            },
            Language::Toml | Language::Yaml => Self {
                language,
                tab_width: 2,
                use_spaces: true,
                auto_indent: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_detect_rust() {
        let path = PathBuf::from("main.rs");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::Rust));
        
        let path = PathBuf::from("src/lib.rs");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::Rust));
    }

    #[test]
    fn test_detect_python() {
        let path = PathBuf::from("script.py");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::Python));
    }

    #[test]
    fn test_detect_javascript() {
        let path = PathBuf::from("app.js");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::JavaScript));
        
        let path = PathBuf::from("component.jsx");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::JavaScript));
    }

    #[test]
    fn test_detect_markdown() {
        let path = PathBuf::from("README.md");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::Markdown));
    }

    #[test]
    fn test_special_filenames() {
        let path = PathBuf::from("Cargo.toml");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::Rust));
        
        let path = PathBuf::from("package.json");
        assert_eq!(LanguageDetector::detect_from_path(&path), Some(Language::JavaScript));
    }

    #[test]
    fn test_shebang_detection() {
        let python_script = "#!/usr/bin/env python3\nprint('hello')";
        assert_eq!(
            LanguageDetector::detect_from_content(python_script),
            Some(Language::Python)
        );
        
        let node_script = "#!/usr/bin/env node\nconsole.log('hello')";
        assert_eq!(
            LanguageDetector::detect_from_content(node_script),
            Some(Language::JavaScript)
        );
    }

    #[test]
    fn test_content_patterns() {
        let rust_code = "fn main() { println!(\"Hello\"); }";
        assert_eq!(
            LanguageDetector::detect_from_content(rust_code),
            Some(Language::Rust)
        );
        
        let python_code = "def hello():\n    print('world')";
        assert_eq!(
            LanguageDetector::detect_from_content(python_code),
            Some(Language::Python)
        );
    }

    #[test]
    fn test_comment_styles() {
        assert_eq!(
            Language::Rust.comment_style(),
            Some(CommentStyle::CStyle)
        );
        assert_eq!(
            Language::Python.comment_style(),
            Some(CommentStyle::Hash)
        );
    }

    #[test]
    fn test_language_by_name() {
        assert_eq!(LanguageDetector::get_by_name("rust"), Some(Language::Rust));
        assert_eq!(LanguageDetector::get_by_name("PYTHON"), Some(Language::Python));
        assert_eq!(LanguageDetector::get_by_name("javascript"), Some(Language::JavaScript));
    }

    #[test]
    fn test_all_languages() {
        let langs = LanguageDetector::all_languages();
        assert!(langs.len() >= 10);
        assert!(langs.contains(&Language::Rust));
        assert!(langs.contains(&Language::Python));
    }

    #[test]
    fn test_language_config() {
        let rust_config = LanguageConfig::default_for(Language::Rust);
        assert_eq!(rust_config.tab_width, 4);
        assert!(rust_config.use_spaces);
        
        let js_config = LanguageConfig::default_for(Language::JavaScript);
        assert_eq!(js_config.tab_width, 2);
        assert!(js_config.use_spaces);
    }
}
