//! Command System
//!
//! This module defines the command system for the editor, including:
//! - Command types for all editor operations
//! - Keybinding mapping system
//! - Command execution framework
//!
//! # Architecture
//!
//! The command system provides a unified interface for all editor operations,
//! whether triggered by keyboard shortcuts, menu items, or programmatic calls.
//!
//! # Example
//!
//! ```
//! use commands::{Command, KeyMap, Movement};
//!
//! let mut keymap = KeyMap::default();
//! let command = Command::MoveCursor(Movement::Down);
//! // Execute command on editor...
//! ```

pub mod editing;
pub mod file;
pub mod navigation;

use std::collections::HashMap;
use std::fmt;

/// Main command enum representing all possible editor operations
///
/// Commands are the primary way to interact with the editor. Each command
/// represents a discrete, undoable action that modifies editor state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    // ===== File Operations =====
    /// Create a new empty document
    NewFile,

    /// Open file dialog
    OpenFile,

    /// Open a specific file by path
    OpenFilePath(String),

    /// Save current document
    Save,

    /// Save current document to a new path
    SaveAs,

    /// Save all open documents
    SaveAll,

    /// Close current document
    Close,

    /// Close all documents
    CloseAll,

    /// Quit the editor
    Quit,

    /// Force quit without saving
    ForceQuit,

    // ===== Editing Operations =====
    /// Insert text at cursor(s)
    Insert(String),

    /// Undo last change
    Undo,

    /// Redo last undone change
    Redo,

    /// Cut selected text to clipboard
    Cut,

    /// Copy selected text to clipboard
    Copy,

    /// Paste from clipboard
    Paste,

    /// Select all text in document
    SelectAll,

    /// Delete selected text or character at cursor
    Delete,

    /// Delete character before cursor (backspace)
    DeleteBackward,

    /// Delete entire line
    DeleteLine,

    /// Delete from cursor to end of line
    DeleteToEndOfLine,

    /// Delete from cursor to start of line
    DeleteToStartOfLine,

    /// Delete word forward
    DeleteWordForward,

    /// Delete word backward
    DeleteWordBackward,

    /// Duplicate current line(s)
    DuplicateLine,

    /// Move line(s) up
    MoveLineUp,

    /// Move line(s) down
    MoveLineDown,

    /// Join current line with next line
    JoinLines,

    /// Split line at cursor
    SplitLine,

    /// Indent selection or current line
    Indent,

    /// Unindent selection or current line
    Unindent,

    /// Toggle line comment
    ToggleComment,

    /// Toggle block comment
    ToggleBlockComment,

    /// Convert selection to uppercase
    ToUpperCase,

    /// Convert selection to lowercase
    ToLowerCase,

    /// Convert selection to title case
    ToTitleCase,

    // ===== Navigation =====
    /// Move cursor by specified movement
    MoveCursor(Movement),

    /// Extend selection by specified movement
    Select(Movement),

    /// Go to specific line number
    GoToLine(usize),

    /// Go to specific character position
    GoToPosition { line: usize, column: usize },

    /// Jump to matching bracket
    GoToMatchingBracket,

    /// Scroll up by one page
    PageUp,

    /// Scroll down by one page
    PageDown,

    /// Center cursor in viewport
    CenterCursor,

    // ===== Search and Replace =====
    /// Open find dialog
    Find,

    /// Find next occurrence
    FindNext,

    /// Find previous occurrence
    FindPrevious,

    /// Open replace dialog
    Replace,

    /// Replace next occurrence
    ReplaceNext,

    /// Replace all occurrences
    ReplaceAll,

    /// Use selection as find query
    UseSelectionForFind,

    /// Toggle case sensitivity in search
    ToggleCaseSensitive,

    /// Toggle regex mode in search
    ToggleRegex,

    /// Toggle whole word matching in search
    ToggleWholeWord,

    // ===== View Operations =====
    /// Toggle line numbers visibility
    ToggleLineNumbers,

    /// Toggle current line highlighting
    ToggleHighlightCurrentLine,

    /// Toggle whitespace visibility
    ToggleShowWhitespace,

    /// Toggle word wrap
    ToggleWordWrap,

    /// Increase font size
    ZoomIn,

    /// Decrease font size
    ZoomOut,

    /// Reset font size to default
    ZoomReset,

    /// Toggle fullscreen mode
    ToggleFullscreen,

    /// Toggle status bar
    ToggleStatusBar,

    /// Split editor horizontally
    SplitHorizontal,

    /// Split editor vertically
    SplitVertical,

    /// Close current split
    CloseSplit,

    /// Focus next split
    FocusNextSplit,

    /// Focus previous split
    FocusPreviousSplit,

    // ===== Multi-Cursor Operations =====
    /// Add cursor above current cursor
    AddCursorAbove,

    /// Add cursor below current cursor
    AddCursorBelow,

    /// Add cursor at next occurrence of selection
    AddCursorAtNextOccurrence,

    /// Add cursor at all occurrences of selection
    AddCursorAtAllOccurrences,

    /// Remove last added cursor
    RemoveLastCursor,

    /// Remove all cursors except primary
    RemoveAllCursors,

    /// Split selection into lines
    SplitSelectionIntoLines,

    // ===== Document Management =====
    /// Switch to next document
    NextDocument,

    /// Switch to previous document
    PreviousDocument,

    /// Switch to document by index
    SwitchToDocument(usize),

    /// Reload current document from disk
    ReloadDocument,

    // ===== Syntax and Language =====
    /// Set syntax highlighting language
    SetSyntax(String),

    /// Auto-detect syntax from file extension
    AutoDetectSyntax,

    // ===== Configuration =====
    /// Open settings/preferences
    OpenSettings,

    /// Open keymap configuration
    OpenKeymap,

    /// Reload configuration
    ReloadConfig,

    /// Set tab width
    SetTabWidth(usize),

    /// Toggle between spaces and tabs
    ToggleUseSpaces,

    /// Set line ending style
    SetLineEnding(LineEndingStyle),

    // ===== System Integration =====
    /// Show command palette
    ShowCommandPalette,

    /// Show file picker
    ShowFilePicker,

    /// Open terminal (deprecated - use Terminal commands instead)
    OpenTerminal,

    // ===== Terminal Operations =====
    /// Create new terminal
    TerminalNew,

    /// Close current terminal
    TerminalClose,

    /// Close all terminals
    TerminalCloseAll,

    /// Switch to next terminal
    TerminalNext,

    /// Switch to previous terminal
    TerminalPrevious,

    /// Switch to terminal by index (1-based)
    TerminalSwitchTo(usize),

    /// Toggle terminal panel visibility
    TerminalToggle,

    /// Show terminal panel
    TerminalShow,

    /// Hide terminal panel
    TerminalHide,

    /// Clear current terminal
    TerminalClear,

    /// Kill current terminal process
    TerminalKill,

    /// Send input to terminal
    TerminalInput(String),

    // ===== Custom Commands =====
    /// Custom command for plugins/extensions
    Custom {
        id: String,
        args: Vec<String>,
    },

    /// No operation (null command)
    Noop,
}

/// Cursor movement types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Movement {
    /// Move left by one character
    Left,

    /// Move right by one character
    Right,

    /// Move up by one line
    Up,

    /// Move down by one line
    Down,

    /// Move to start of line
    LineStart,

    /// Move to end of line
    LineEnd,

    /// Move to first non-whitespace character of line
    LineStartNonWhitespace,

    /// Move left by one word
    WordLeft,

    /// Move right by one word
    WordRight,

    /// Move to start of document
    DocumentStart,

    /// Move to end of document
    DocumentEnd,

    /// Move up by one page
    PageUp,

    /// Move down by one page
    PageDown,

    /// Move to start of next paragraph
    ParagraphNext,

    /// Move to start of previous paragraph
    ParagraphPrevious,

    /// Move to matching bracket
    MatchingBracket,
}

/// Line ending styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineEndingStyle {
    /// Unix-style line endings (\n)
    Lf,

    /// Windows-style line endings (\r\n)
    CrLf,

    /// Classic Mac line endings (\r)
    Cr,
}

impl Command {
    /// Returns true if this command modifies the document
    pub fn modifies_document(&self) -> bool {
        matches!(
            self,
            Command::Insert(_)
                | Command::Delete
                | Command::DeleteBackward
                | Command::DeleteLine
                | Command::DeleteToEndOfLine
                | Command::DeleteToStartOfLine
                | Command::DeleteWordForward
                | Command::DeleteWordBackward
                | Command::Paste
                | Command::Cut
                | Command::DuplicateLine
                | Command::MoveLineUp
                | Command::MoveLineDown
                | Command::JoinLines
                | Command::SplitLine
                | Command::Indent
                | Command::Unindent
                | Command::ToggleComment
                | Command::ToggleBlockComment
                | Command::ToUpperCase
                | Command::ToLowerCase
                | Command::ToTitleCase
                | Command::ReplaceNext
                | Command::ReplaceAll
        )
    }

    /// Returns true if this command should be saved in undo history
    pub fn is_undoable(&self) -> bool {
        self.modifies_document()
    }

    /// Returns a human-readable description of the command
    pub fn description(&self) -> &'static str {
        match self {
            Command::NewFile => "New File",
            Command::OpenFile => "Open File",
            Command::OpenFilePath(_) => "Open File Path",
            Command::Save => "Save",
            Command::SaveAs => "Save As",
            Command::SaveAll => "Save All",
            Command::Close => "Close",
            Command::CloseAll => "Close All",
            Command::Quit => "Quit",
            Command::ForceQuit => "Force Quit",
            Command::Insert(_) => "Insert Text",
            Command::Undo => "Undo",
            Command::Redo => "Redo",
            Command::Cut => "Cut",
            Command::Copy => "Copy",
            Command::Paste => "Paste",
            Command::SelectAll => "Select All",
            Command::Delete => "Delete",
            Command::DeleteBackward => "Delete Backward",
            Command::DeleteLine => "Delete Line",
            Command::DeleteToEndOfLine => "Delete to End of Line",
            Command::DeleteToStartOfLine => "Delete to Start of Line",
            Command::DeleteWordForward => "Delete Word Forward",
            Command::DeleteWordBackward => "Delete Word Backward",
            Command::DuplicateLine => "Duplicate Line",
            Command::MoveLineUp => "Move Line Up",
            Command::MoveLineDown => "Move Line Down",
            Command::JoinLines => "Join Lines",
            Command::SplitLine => "Split Line",
            Command::Indent => "Indent",
            Command::Unindent => "Unindent",
            Command::ToggleComment => "Toggle Comment",
            Command::ToggleBlockComment => "Toggle Block Comment",
            Command::ToUpperCase => "To Uppercase",
            Command::ToLowerCase => "To Lowercase",
            Command::ToTitleCase => "To Title Case",
            Command::MoveCursor(_) => "Move Cursor",
            Command::Select(_) => "Select",
            Command::GoToLine(_) => "Go to Line",
            Command::GoToPosition { .. } => "Go to Position",
            Command::GoToMatchingBracket => "Go to Matching Bracket",
            Command::PageUp => "Page Up",
            Command::PageDown => "Page Down",
            Command::CenterCursor => "Center Cursor",
            Command::Find => "Find",
            Command::FindNext => "Find Next",
            Command::FindPrevious => "Find Previous",
            Command::Replace => "Replace",
            Command::ReplaceNext => "Replace Next",
            Command::ReplaceAll => "Replace All",
            Command::UseSelectionForFind => "Use Selection for Find",
            Command::ToggleCaseSensitive => "Toggle Case Sensitive",
            Command::ToggleRegex => "Toggle Regex",
            Command::ToggleWholeWord => "Toggle Whole Word",
            Command::ToggleLineNumbers => "Toggle Line Numbers",
            Command::ToggleHighlightCurrentLine => "Toggle Highlight Current Line",
            Command::ToggleShowWhitespace => "Toggle Show Whitespace",
            Command::ToggleWordWrap => "Toggle Word Wrap",
            Command::ZoomIn => "Zoom In",
            Command::ZoomOut => "Zoom Out",
            Command::ZoomReset => "Reset Zoom",
            Command::ToggleFullscreen => "Toggle Fullscreen",
            Command::ToggleStatusBar => "Toggle Status Bar",
            Command::SplitHorizontal => "Split Horizontal",
            Command::SplitVertical => "Split Vertical",
            Command::CloseSplit => "Close Split",
            Command::FocusNextSplit => "Focus Next Split",
            Command::FocusPreviousSplit => "Focus Previous Split",
            Command::AddCursorAbove => "Add Cursor Above",
            Command::AddCursorBelow => "Add Cursor Below",
            Command::AddCursorAtNextOccurrence => "Add Cursor at Next Occurrence",
            Command::AddCursorAtAllOccurrences => "Add Cursor at All Occurrences",
            Command::RemoveLastCursor => "Remove Last Cursor",
            Command::RemoveAllCursors => "Remove All Cursors",
            Command::SplitSelectionIntoLines => "Split Selection into Lines",
            Command::NextDocument => "Next Document",
            Command::PreviousDocument => "Previous Document",
            Command::SwitchToDocument(_) => "Switch to Document",
            Command::ReloadDocument => "Reload Document",
            Command::SetSyntax(_) => "Set Syntax",
            Command::AutoDetectSyntax => "Auto Detect Syntax",
            Command::OpenSettings => "Open Settings",
            Command::OpenKeymap => "Open Keymap",
            Command::ReloadConfig => "Reload Config",
            Command::SetTabWidth(_) => "Set Tab Width",
            Command::ToggleUseSpaces => "Toggle Use Spaces",
            Command::SetLineEnding(_) => "Set Line Ending",
            Command::ShowCommandPalette => "Show Command Palette",
            Command::ShowFilePicker => "Show File Picker",
            Command::OpenTerminal => "Open Terminal",
            Command::TerminalNew => "New Terminal",
            Command::TerminalClose => "Close Terminal",
            Command::TerminalCloseAll => "Close All Terminals",
            Command::TerminalNext => "Next Terminal",
            Command::TerminalPrevious => "Previous Terminal",
            Command::TerminalSwitchTo(_) => "Switch to Terminal",
            Command::TerminalToggle => "Toggle Terminal Panel",
            Command::TerminalShow => "Show Terminal Panel",
            Command::TerminalHide => "Hide Terminal Panel",
            Command::TerminalClear => "Clear Terminal",
            Command::TerminalKill => "Kill Terminal Process",
            Command::TerminalInput(_) => "Send Input to Terminal",
            Command::Custom { .. } => "Custom Command",
            Command::Noop => "No Operation",
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Keybinding modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    /// Control key (Cmd on macOS)
    pub ctrl: bool,

    /// Shift key
    pub shift: bool,

    /// Alt/Option key
    pub alt: bool,

    /// Meta/Super/Windows key (Cmd on macOS in some contexts)
    pub meta: bool,
}

impl Modifiers {
    /// Create new modifiers with all keys released
    pub const fn none() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }

    /// Create modifiers with only Ctrl pressed
    pub const fn ctrl() -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: false,
            meta: false,
        }
    }

    /// Create modifiers with only Shift pressed
    pub const fn shift() -> Self {
        Self {
            ctrl: false,
            shift: true,
            alt: false,
            meta: false,
        }
    }

    /// Create modifiers with only Alt pressed
    pub const fn alt() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: true,
            meta: false,
        }
    }

    /// Create modifiers with Ctrl and Shift pressed
    pub const fn ctrl_shift() -> Self {
        Self {
            ctrl: true,
            shift: true,
            alt: false,
            meta: false,
        }
    }

    /// Create modifiers with Ctrl and Alt pressed
    pub const fn ctrl_alt() -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: true,
            meta: false,
        }
    }
}

/// Key codes for keyboard input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Character key
    Char(char),

    /// Backspace
    Backspace,

    /// Delete
    Delete,

    /// Enter/Return
    Enter,

    /// Tab
    Tab,

    /// Escape
    Escape,

    /// Space
    Space,

    /// Arrow keys
    Left,
    Right,
    Up,
    Down,

    /// Home
    Home,

    /// End
    End,

    /// Page Up
    PageUp,

    /// Page Down
    PageDown,

    /// Insert
    Insert,

    /// Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

/// A keybinding combining a key and modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl KeyBinding {
    /// Create a new keybinding
    pub const fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    /// Create a keybinding with no modifiers
    pub const fn simple(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers::none(),
        }
    }
}

/// Maps keybindings to commands
///
/// The keymap is responsible for translating keyboard input into editor commands.
/// It supports multiple keybinding schemes (default, vim, emacs, etc.) and
/// allows custom user keybindings.
pub struct KeyMap {
    /// Keybinding to command mappings
    bindings: HashMap<KeyBinding, Command>,

    /// Reverse mapping for looking up keybindings for a command
    reverse_bindings: HashMap<Command, Vec<KeyBinding>>,
}

impl KeyMap {
    /// Create a new empty keymap
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            reverse_bindings: HashMap::new(),
        }
    }

    /// Create the default keymap
    pub fn default_keymap() -> Self {
        let mut keymap = Self::new();
        keymap.register_default_bindings();
        keymap
    }

    /// Bind a key to a command
    pub fn bind(&mut self, key: Key, modifiers: Modifiers, command: Command) {
        let binding = KeyBinding::new(key, modifiers);

        // Remove old binding if it exists
        if let Some(old_command) = self.bindings.remove(&binding) {
            if let Some(bindings) = self.reverse_bindings.get_mut(&old_command) {
                bindings.retain(|b| b != &binding);
            }
        }

        // Add new binding
        self.bindings.insert(binding, command.clone());
        self.reverse_bindings
            .entry(command)
            .or_insert_with(Vec::new)
            .push(binding);
    }

    /// Remove a keybinding
    pub fn unbind(&mut self, key: Key, modifiers: Modifiers) -> Option<Command> {
        let binding = KeyBinding::new(key, modifiers);
        if let Some(command) = self.bindings.remove(&binding) {
            if let Some(bindings) = self.reverse_bindings.get_mut(&command) {
                bindings.retain(|b| b != &binding);
            }
            Some(command)
        } else {
            None
        }
    }

    /// Look up command for a keybinding
    pub fn lookup(&self, key: Key, modifiers: Modifiers) -> Option<&Command> {
        let binding = KeyBinding::new(key, modifiers);
        self.bindings.get(&binding)
    }

    /// Get all keybindings for a command
    pub fn bindings_for_command(&self, command: &Command) -> &[KeyBinding] {
        self.reverse_bindings
            .get(command)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all bindings
    pub fn all_bindings(&self) -> impl Iterator<Item = (&KeyBinding, &Command)> {
        self.bindings.iter()
    }

    /// Register default keybindings
    fn register_default_bindings(&mut self) {
        // File operations
        self.bind(Key::Char('n'), Modifiers::ctrl(), Command::NewFile);
        self.bind(Key::Char('o'), Modifiers::ctrl(), Command::OpenFile);
        self.bind(Key::Char('s'), Modifiers::ctrl(), Command::Save);
        self.bind(Key::Char('s'), Modifiers::ctrl_shift(), Command::SaveAs);
        self.bind(Key::Char('w'), Modifiers::ctrl(), Command::Close);
        self.bind(Key::Char('q'), Modifiers::ctrl(), Command::Quit);

        // Editing
        self.bind(Key::Char('z'), Modifiers::ctrl(), Command::Undo);
        self.bind(Key::Char('y'), Modifiers::ctrl(), Command::Redo);
        self.bind(Key::Char('z'), Modifiers::ctrl_shift(), Command::Redo);
        self.bind(Key::Char('x'), Modifiers::ctrl(), Command::Cut);
        self.bind(Key::Char('c'), Modifiers::ctrl(), Command::Copy);
        self.bind(Key::Char('v'), Modifiers::ctrl(), Command::Paste);
        self.bind(Key::Char('a'), Modifiers::ctrl(), Command::SelectAll);
        self.bind(Key::Delete, Modifiers::none(), Command::Delete);
        self.bind(Key::Backspace, Modifiers::none(), Command::DeleteBackward);
        self.bind(Key::Char('d'), Modifiers::ctrl(), Command::DeleteLine);
        self.bind(Key::Char('k'), Modifiers::ctrl(), Command::DeleteToEndOfLine);
        self.bind(Key::Delete, Modifiers::ctrl(), Command::DeleteWordForward);
        self.bind(Key::Backspace, Modifiers::ctrl(), Command::DeleteWordBackward);
        self.bind(Key::Char('d'), Modifiers::ctrl_shift(), Command::DuplicateLine);
        self.bind(Key::Char('/'), Modifiers::ctrl(), Command::ToggleComment);

        // Navigation
        self.bind(Key::Left, Modifiers::none(), Command::MoveCursor(Movement::Left));
        self.bind(Key::Right, Modifiers::none(), Command::MoveCursor(Movement::Right));
        self.bind(Key::Up, Modifiers::none(), Command::MoveCursor(Movement::Up));
        self.bind(Key::Down, Modifiers::none(), Command::MoveCursor(Movement::Down));
        self.bind(Key::Home, Modifiers::none(), Command::MoveCursor(Movement::LineStart));
        self.bind(Key::End, Modifiers::none(), Command::MoveCursor(Movement::LineEnd));
        self.bind(Key::Left, Modifiers::ctrl(), Command::MoveCursor(Movement::WordLeft));
        self.bind(Key::Right, Modifiers::ctrl(), Command::MoveCursor(Movement::WordRight));
        self.bind(Key::Home, Modifiers::ctrl(), Command::MoveCursor(Movement::DocumentStart));
        self.bind(Key::End, Modifiers::ctrl(), Command::MoveCursor(Movement::DocumentEnd));
        self.bind(Key::PageUp, Modifiers::none(), Command::PageUp);
        self.bind(Key::PageDown, Modifiers::none(), Command::PageDown);
        self.bind(Key::Char('g'), Modifiers::ctrl(), Command::Find);
        self.bind(Key::Char('l'), Modifiers::ctrl(), Command::Find);

        // Selection
        self.bind(Key::Left, Modifiers::shift(), Command::Select(Movement::Left));
        self.bind(Key::Right, Modifiers::shift(), Command::Select(Movement::Right));
        self.bind(Key::Up, Modifiers::shift(), Command::Select(Movement::Up));
        self.bind(Key::Down, Modifiers::shift(), Command::Select(Movement::Down));
        self.bind(Key::Home, Modifiers::shift(), Command::Select(Movement::LineStart));
        self.bind(Key::End, Modifiers::shift(), Command::Select(Movement::LineEnd));
        self.bind(Key::Left, Modifiers::ctrl_shift(), Command::Select(Movement::WordLeft));
        self.bind(Key::Right, Modifiers::ctrl_shift(), Command::Select(Movement::WordRight));
        self.bind(Key::Home, Modifiers::ctrl_shift(), Command::Select(Movement::DocumentStart));
        self.bind(Key::End, Modifiers::ctrl_shift(), Command::Select(Movement::DocumentEnd));

        // Search
        self.bind(Key::Char('f'), Modifiers::ctrl(), Command::Find);
        self.bind(Key::F3, Modifiers::none(), Command::FindNext);
        self.bind(Key::F3, Modifiers::shift(), Command::FindPrevious);
        self.bind(Key::Char('h'), Modifiers::ctrl(), Command::Replace);

        // View
        self.bind(Key::Char('='), Modifiers::ctrl(), Command::ZoomIn);
        self.bind(Key::Char('-'), Modifiers::ctrl(), Command::ZoomOut);
        self.bind(Key::Char('0'), Modifiers::ctrl(), Command::ZoomReset);
        self.bind(Key::F11, Modifiers::none(), Command::ToggleFullscreen);

        // Multi-cursor
        self.bind(Key::Up, Modifiers::ctrl_alt(), Command::AddCursorAbove);
        self.bind(Key::Down, Modifiers::ctrl_alt(), Command::AddCursorBelow);
        self.bind(Key::Char('d'), Modifiers::alt(), Command::AddCursorAtNextOccurrence);

        // Document management
        self.bind(Key::Tab, Modifiers::ctrl(), Command::NextDocument);
        self.bind(Key::Tab, Modifiers::ctrl_shift(), Command::PreviousDocument);

        // System
        self.bind(Key::Char('p'), Modifiers::ctrl_shift(), Command::ShowCommandPalette);
        self.bind(Key::Char('p'), Modifiers::ctrl(), Command::ShowFilePicker);

        // Terminal (backtick/grave accent is represented as '`')
        self.bind(Key::Char('`'), Modifiers::ctrl(), Command::TerminalToggle);
        self.bind(Key::Char('t'), Modifiers::ctrl_shift(), Command::TerminalNew);
        self.bind(Key::Char('w'), Modifiers::ctrl_shift(), Command::TerminalClose);
        self.bind(Key::Char('1'), Modifiers::alt(), Command::TerminalSwitchTo(1));
        self.bind(Key::Char('2'), Modifiers::alt(), Command::TerminalSwitchTo(2));
        self.bind(Key::Char('3'), Modifiers::alt(), Command::TerminalSwitchTo(3));
        self.bind(Key::Char('4'), Modifiers::alt(), Command::TerminalSwitchTo(4));
        self.bind(Key::Char('5'), Modifiers::alt(), Command::TerminalSwitchTo(5));
        self.bind(Key::Char('k'), Modifiers::ctrl_shift(), Command::TerminalClear);
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        Self::default_keymap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_properties() {
        assert!(Command::Insert("test".to_string()).modifies_document());
        assert!(Command::Delete.modifies_document());
        assert!(!Command::MoveCursor(Movement::Left).modifies_document());
        assert!(!Command::Copy.modifies_document());
    }

    #[test]
    fn test_keymap_bind_unbind() {
        let mut keymap = KeyMap::new();

        keymap.bind(Key::Char('s'), Modifiers::ctrl(), Command::Save);
        assert_eq!(
            keymap.lookup(Key::Char('s'), Modifiers::ctrl()),
            Some(&Command::Save)
        );

        keymap.unbind(Key::Char('s'), Modifiers::ctrl());
        assert_eq!(keymap.lookup(Key::Char('s'), Modifiers::ctrl()), None);
    }

    #[test]
    fn test_keymap_reverse_lookup() {
        let mut keymap = KeyMap::new();

        keymap.bind(Key::Char('s'), Modifiers::ctrl(), Command::Save);
        keymap.bind(Key::F2, Modifiers::none(), Command::Save);

        let bindings = keymap.bindings_for_command(&Command::Save);
        assert_eq!(bindings.len(), 2);
    }

    #[test]
    fn test_default_keymap() {
        let keymap = KeyMap::default();

        assert_eq!(
            keymap.lookup(Key::Char('s'), Modifiers::ctrl()),
            Some(&Command::Save)
        );
        assert_eq!(
            keymap.lookup(Key::Char('c'), Modifiers::ctrl()),
            Some(&Command::Copy)
        );
        assert_eq!(
            keymap.lookup(Key::Char('v'), Modifiers::ctrl()),
            Some(&Command::Paste)
        );
    }
}
