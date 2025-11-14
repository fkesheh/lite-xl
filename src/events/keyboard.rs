//! Keyboard Event Mapping and Handling
//!
//! This module provides utilities for working with keyboard events:
//! - Parsing keybinding strings (e.g., "Ctrl+S", "Alt+Shift+F")
//! - Keyboard layout handling
//! - IME (Input Method Editor) support
//! - Special key handling
//! - Keybinding conflict detection
//!
//! # Example
//!
//! ```
//! use events::keyboard::parse_keybinding;
//!
//! let binding = parse_keybinding("Ctrl+Shift+S").unwrap();
//! println!("{:?}", binding);
//! ```

use crate::commands::{Key, KeyBinding, KeyMap, Modifiers};
use std::collections::HashMap;

/// Error type for keybinding parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeybindingError {
    /// Invalid key name
    InvalidKey(String),

    /// Empty keybinding string
    EmptyBinding,

    /// Invalid format
    InvalidFormat(String),

    /// Conflicting keybindings
    ConflictingBinding {
        binding: String,
        existing_command: String,
        new_command: String,
    },
}

impl std::fmt::Display for KeybindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeybindingError::InvalidKey(key) => write!(f, "Invalid key: {}", key),
            KeybindingError::EmptyBinding => write!(f, "Empty keybinding"),
            KeybindingError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            KeybindingError::ConflictingBinding {
                binding,
                existing_command,
                new_command,
            } => write!(
                f,
                "Keybinding '{}' conflicts: '{}' vs '{}'",
                binding, existing_command, new_command
            ),
        }
    }
}

impl std::error::Error for KeybindingError {}

/// Parse a keybinding string into a KeyBinding
///
/// # Format
///
/// Keybindings are specified as modifier+key combinations:
/// - "Ctrl+S" - Control + S
/// - "Alt+Shift+F" - Alt + Shift + F
/// - "Cmd+C" - Command + C (macOS)
/// - "F5" - F5 key (no modifiers)
/// - "Backspace" - Backspace key
///
/// # Example
///
/// ```
/// let binding = parse_keybinding("Ctrl+S")?;
/// ```
pub fn parse_keybinding(s: &str) -> Result<KeyBinding, KeybindingError> {
    if s.trim().is_empty() {
        return Err(KeybindingError::EmptyBinding);
    }

    let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();

    if parts.is_empty() {
        return Err(KeybindingError::EmptyBinding);
    }

    let mut modifiers = Modifiers::none();
    let key_str = parts.last().unwrap();

    // Parse modifiers
    for &part in parts.iter().take(parts.len() - 1) {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers.ctrl = true,
            "shift" => modifiers.shift = true,
            "alt" | "option" => modifiers.alt = true,
            "meta" | "cmd" | "command" | "super" | "win" => modifiers.meta = true,
            _ => {
                return Err(KeybindingError::InvalidFormat(format!(
                    "Unknown modifier: {}",
                    part
                )))
            }
        }
    }

    // Parse key
    let key = parse_key(key_str)?;

    Ok(KeyBinding::new(key, modifiers))
}

/// Parse a key string into a Key
fn parse_key(s: &str) -> Result<Key, KeybindingError> {
    let s_lower = s.to_lowercase();

    match s_lower.as_str() {
        "backspace" => Ok(Key::Backspace),
        "delete" | "del" => Ok(Key::Delete),
        "enter" | "return" => Ok(Key::Enter),
        "tab" => Ok(Key::Tab),
        "escape" | "esc" => Ok(Key::Escape),
        "space" => Ok(Key::Space),
        "left" => Ok(Key::Left),
        "right" => Ok(Key::Right),
        "up" => Ok(Key::Up),
        "down" => Ok(Key::Down),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" | "pgup" => Ok(Key::PageUp),
        "pagedown" | "pgdn" => Ok(Key::PageDown),
        "insert" | "ins" => Ok(Key::Insert),
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),
        _ => {
            // Check if it's a single character
            if s.len() == 1 {
                let c = s.chars().next().unwrap();
                Ok(Key::Char(c))
            } else {
                Err(KeybindingError::InvalidKey(s.to_string()))
            }
        }
    }
}

/// Format a keybinding as a string
///
/// # Example
///
/// ```
/// let binding = KeyBinding::new(Key::Char('s'), Modifiers::ctrl());
/// assert_eq!(format_keybinding(&binding), "Ctrl+S");
/// ```
pub fn format_keybinding(binding: &KeyBinding) -> String {
    let mut parts = Vec::new();

    if binding.modifiers.ctrl {
        parts.push("Ctrl");
    }
    if binding.modifiers.shift {
        parts.push("Shift");
    }
    if binding.modifiers.alt {
        parts.push("Alt");
    }
    if binding.modifiers.meta {
        parts.push("Cmd");
    }

    let key_str = format_key(&binding.key);
    parts.push(&key_str);

    parts.join("+")
}

/// Format a key as a string
fn format_key(key: &Key) -> String {
    match key {
        Key::Char(c) => c.to_uppercase().to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Escape => "Escape".to_string(),
        Key::Space => "Space".to_string(),
        Key::Left => "Left".to_string(),
        Key::Right => "Right".to_string(),
        Key::Up => "Up".to_string(),
        Key::Down => "Down".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
    }
}

/// Keyboard layout abstraction
///
/// Different keyboard layouts may produce different characters for the same
/// physical key. This trait allows handling different keyboard layouts.
pub trait KeyboardLayout {
    /// Get the character produced by a key with the given modifiers
    fn get_char(&self, key: Key, modifiers: Modifiers) -> Option<char>;

    /// Get the layout name
    fn name(&self) -> &str;
}

/// US QWERTY keyboard layout
pub struct QwertyLayout;

impl KeyboardLayout for QwertyLayout {
    fn get_char(&self, key: Key, modifiers: Modifiers) -> Option<char> {
        if let Key::Char(c) = key {
            if modifiers.shift {
                // Apply shift transformation
                Some(shift_char(c))
            } else {
                Some(c)
            }
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "US QWERTY"
    }
}

/// Apply shift transformation to a character
fn shift_char(c: char) -> char {
    match c {
        'a'..='z' => c.to_ascii_uppercase(),
        '1' => '!',
        '2' => '@',
        '3' => '#',
        '4' => '$',
        '5' => '%',
        '6' => '^',
        '7' => '&',
        '8' => '*',
        '9' => '(',
        '0' => ')',
        '-' => '_',
        '=' => '+',
        '[' => '{',
        ']' => '}',
        '\\' => '|',
        ';' => ':',
        '\'' => '"',
        ',' => '<',
        '.' => '>',
        '/' => '?',
        '`' => '~',
        _ => c,
    }
}

/// Keybinding conflict detector
///
/// Helps identify conflicting keybindings in the keymap.
pub struct ConflictDetector {
    bindings: HashMap<String, String>,
}

impl ConflictDetector {
    /// Create a new conflict detector
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Check a keymap for conflicts
    pub fn check_keymap(&mut self, keymap: &KeyMap) -> Vec<KeybindingError> {
        let mut conflicts = Vec::new();
        self.bindings.clear();

        for (binding, command) in keymap.all_bindings() {
            let binding_str = format_keybinding(binding);
            let command_str = command.description().to_string();

            if let Some(existing_command) = self.bindings.get(&binding_str) {
                if existing_command != &command_str {
                    conflicts.push(KeybindingError::ConflictingBinding {
                        binding: binding_str.clone(),
                        existing_command: existing_command.clone(),
                        new_command: command_str.clone(),
                    });
                }
            } else {
                self.bindings.insert(binding_str, command_str);
            }
        }

        conflicts
    }

    /// Add a binding to check for conflicts
    pub fn add_binding(
        &mut self,
        binding: &str,
        command: &str,
    ) -> Result<(), KeybindingError> {
        if let Some(existing_command) = self.bindings.get(binding) {
            if existing_command != command {
                return Err(KeybindingError::ConflictingBinding {
                    binding: binding.to_string(),
                    existing_command: existing_command.clone(),
                    new_command: command.to_string(),
                });
            }
        }
        self.bindings.insert(binding.to_string(), command.to_string());
        Ok(())
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// IME (Input Method Editor) state
///
/// Handles composition for complex input methods (e.g., Chinese, Japanese, Korean)
#[derive(Debug, Clone, Default)]
pub struct ImeState {
    /// Whether IME composition is active
    pub active: bool,

    /// Current composition string
    pub composition: String,

    /// Cursor position in composition (byte offset)
    pub cursor_pos: usize,

    /// Selection range in composition
    pub selection_range: Option<(usize, usize)>,
}

impl ImeState {
    /// Create a new IME state
    pub fn new() -> Self {
        Self::default()
    }

    /// Start composition
    pub fn start_composition(&mut self) {
        self.active = true;
        self.composition.clear();
        self.cursor_pos = 0;
        self.selection_range = None;
    }

    /// Update composition
    pub fn update_composition(&mut self, text: String, cursor_pos: usize) {
        self.composition = text;
        self.cursor_pos = cursor_pos;
    }

    /// End composition and return the final text
    pub fn end_composition(&mut self) -> String {
        self.active = false;
        let result = self.composition.clone();
        self.composition.clear();
        self.cursor_pos = 0;
        self.selection_range = None;
        result
    }

    /// Cancel composition
    pub fn cancel_composition(&mut self) {
        self.active = false;
        self.composition.clear();
        self.cursor_pos = 0;
        self.selection_range = None;
    }
}

/// Keyboard event recorder for macros
///
/// Records keyboard events for playback in keyboard macros.
#[derive(Debug, Clone)]
pub struct KeyboardRecorder {
    /// Whether recording is active
    recording: bool,

    /// Recorded events
    events: Vec<(Key, Modifiers)>,

    /// Maximum number of events to record
    max_events: usize,
}

impl KeyboardRecorder {
    /// Create a new keyboard recorder
    pub fn new() -> Self {
        Self {
            recording: false,
            events: Vec::new(),
            max_events: 10000,
        }
    }

    /// Start recording
    pub fn start_recording(&mut self) {
        self.recording = true;
        self.events.clear();
    }

    /// Stop recording and return recorded events
    pub fn stop_recording(&mut self) -> Vec<(Key, Modifiers)> {
        self.recording = false;
        self.events.clone()
    }

    /// Record a key event
    pub fn record_key(&mut self, key: Key, modifiers: Modifiers) {
        if self.recording && self.events.len() < self.max_events {
            self.events.push((key, modifiers));
        }
    }

    /// Check if recording is active
    pub fn is_recording(&self) -> bool {
        self.recording
    }

    /// Get recorded event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Clear recorded events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for KeyboardRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keybinding_simple() {
        let binding = parse_keybinding("Ctrl+S").unwrap();
        assert_eq!(binding.key, Key::Char('S'));
        assert!(binding.modifiers.ctrl);
        assert!(!binding.modifiers.shift);
    }

    #[test]
    fn test_parse_keybinding_multiple_modifiers() {
        let binding = parse_keybinding("Ctrl+Shift+F").unwrap();
        assert_eq!(binding.key, Key::Char('F'));
        assert!(binding.modifiers.ctrl);
        assert!(binding.modifiers.shift);
    }

    #[test]
    fn test_parse_keybinding_special_key() {
        let binding = parse_keybinding("Alt+Delete").unwrap();
        assert_eq!(binding.key, Key::Delete);
        assert!(binding.modifiers.alt);
    }

    #[test]
    fn test_parse_keybinding_function_key() {
        let binding = parse_keybinding("F5").unwrap();
        assert_eq!(binding.key, Key::F5);
        assert!(!binding.modifiers.ctrl);
    }

    #[test]
    fn test_parse_keybinding_invalid() {
        assert!(parse_keybinding("").is_err());
        assert!(parse_keybinding("Ctrl+InvalidKey").is_err());
    }

    #[test]
    fn test_format_keybinding() {
        let binding = KeyBinding::new(Key::Char('s'), Modifiers::ctrl());
        assert_eq!(format_keybinding(&binding), "Ctrl+S");

        let binding = KeyBinding::new(Key::F5, Modifiers::none());
        assert_eq!(format_keybinding(&binding), "F5");
    }

    #[test]
    fn test_qwerty_layout() {
        let layout = QwertyLayout;
        assert_eq!(
            layout.get_char(Key::Char('a'), Modifiers::none()),
            Some('a')
        );
        assert_eq!(
            layout.get_char(Key::Char('a'), Modifiers::shift()),
            Some('A')
        );
    }

    #[test]
    fn test_ime_state() {
        let mut ime = ImeState::new();
        assert!(!ime.active);

        ime.start_composition();
        assert!(ime.active);

        ime.update_composition("你好".to_string(), 3);
        assert_eq!(ime.composition, "你好");

        let result = ime.end_composition();
        assert_eq!(result, "你好");
        assert!(!ime.active);
    }

    #[test]
    fn test_keyboard_recorder() {
        let mut recorder = KeyboardRecorder::new();
        assert!(!recorder.is_recording());

        recorder.start_recording();
        assert!(recorder.is_recording());

        recorder.record_key(Key::Char('a'), Modifiers::none());
        recorder.record_key(Key::Char('b'), Modifiers::ctrl());

        assert_eq!(recorder.event_count(), 2);

        let events = recorder.stop_recording();
        assert_eq!(events.len(), 2);
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_conflict_detector() {
        let mut detector = ConflictDetector::new();

        assert!(detector.add_binding("Ctrl+S", "Save").is_ok());
        assert!(detector.add_binding("Ctrl+S", "Save").is_ok()); // Same command, OK

        let result = detector.add_binding("Ctrl+S", "Different Command");
        assert!(result.is_err());
    }
}
