# Rust Text Editor MVP - Architecture Specification

**Version:** 1.0
**Target:** Phase 1 MVP
**Last Updated:** November 2025

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Technology Stack](#technology-stack)
3. [Module Architecture](#module-architecture)
4. [Core Data Structures](#core-data-structures)
5. [Event Handling System](#event-handling-system)
6. [Rendering Pipeline](#rendering-pipeline)
7. [State Management](#state-management)
8. [Configuration Management](#configuration-management)
9. [Async Strategy](#async-strategy)
10. [File Structure](#file-structure)
11. [API Design](#api-design)
12. [Extension Points](#extension-points)
13. [Performance Targets](#performance-targets)
14. [Testing Strategy](#testing-strategy)

---

## 1. Executive Summary

This document defines the architecture for a lightweight, fast, and extensible text editor written in Rust. The MVP focuses on core editing functionality while establishing a foundation for future enhancements.

### Design Principles

1. **Performance First**: Target 60 FPS rendering, instant startup
2. **Type Safety**: Leverage Rust's type system for correctness
3. **Modularity**: Clear separation of concerns, testable components
4. **Extensibility**: Plugin-ready architecture from day one
5. **Ergonomics**: Pleasant editing experience with minimal friction

### Phase 1 MVP Goals

- Single-file editing with undo/redo
- Basic text operations (insert, delete, select, copy, paste)
- File I/O with auto-detection of encoding and line endings
- Syntax highlighting for common languages
- 60 FPS rendering with smooth scrolling
- Line numbers and basic gutter
- Keyboard-driven workflow

---

## 2. Technology Stack

### Core Dependencies

```toml
[dependencies]
# UI Framework
floem = "0.1"              # Reactive UI framework

# Text Buffer Management
ropey = "1.6"              # Rope data structure for text
xi-rope = "0.3"            # Alternative/supplementary rope utilities

# Syntax Highlighting
syntect = "5.1"            # Syntax highlighting engine
tree-sitter = "0.20"       # Future: incremental parsing (Phase 2)

# File I/O
encoding_rs = "0.8"        # Character encoding detection
memmap2 = "0.9"            # Memory-mapped file I/O for large files

# Concurrency
tokio = { version = "1", features = ["full"] }
crossbeam = "0.8"          # Lock-free data structures

# Utilities
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"               # Configuration files
anyhow = "1.0"             # Error handling
thiserror = "1.0"          # Custom error types
tracing = "0.1"            # Structured logging

# Platform Integration
clipboard = "0.5"          # System clipboard
notify = "6.0"             # File system watching
```

### Justification

- **Floem**: Modern reactive UI framework with excellent performance
- **ropey**: Battle-tested rope implementation optimized for text editing
- **syntect**: Mature syntax highlighting with TextMate grammar support
- **tokio**: Industry-standard async runtime for file I/O and background tasks

---

## 3. Module Architecture

### 3.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Application                          │
│                    (main.rs, app.rs)                     │
└────────────────────┬────────────────────────────────────┘
                     │
          ┌──────────┼──────────┐
          │          │          │
┌─────────▼─────┐ ┌──▼──────┐ ┌▼─────────┐
│   UI Layer    │ │  Core   │ │  Plugin  │
│   (Floem)     │ │ Engine  │ │  System  │
└───────┬───────┘ └──┬──────┘ └──────────┘
        │            │
        │    ┌───────┴─────────┬───────────┬──────────┐
        │    │                 │           │          │
    ┌───▼────▼───┐  ┌─────────▼──┐  ┌────▼────┐  ┌──▼─────┐
    │  Document  │  │   Buffer   │  │  Syntax │  │  File  │
    │  Manager   │  │  (ropey)   │  │(syntect)│  │   I/O  │
    └────────────┘  └────────────┘  └─────────┘  └────────┘
```

### 3.2 Module Breakdown

#### Core Modules

1. **buffer**: Text buffer management with ropey
2. **document**: Document abstraction (buffer + metadata)
3. **editor**: Editor state and operations
4. **syntax**: Syntax highlighting and language support
5. **ui**: Floem-based UI components
6. **commands**: Command system and keybindings
7. **io**: File operations and encoding
8. **config**: Configuration management
9. **undo**: Undo/redo system
10. **selection**: Selection and cursor management

#### Support Modules

11. **clipboard**: System clipboard integration
12. **events**: Event handling and dispatching
13. **render**: Rendering abstractions
14. **utils**: Common utilities
15. **error**: Error types and handling

---

## 4. Core Data Structures

### 4.1 Buffer (`src/buffer/mod.rs`)

The buffer is the foundational data structure representing text content.

```rust
use ropey::Rope;
use std::path::PathBuf;

/// A text buffer backed by a rope data structure
pub struct Buffer {
    /// The underlying rope storing text content
    rope: Rope,

    /// Unique identifier for this buffer
    id: BufferId,

    /// File path (if associated with a file)
    path: Option<PathBuf>,

    /// Line ending style
    line_ending: LineEnding,

    /// Character encoding
    encoding: Encoding,

    /// Modification state
    modified: bool,

    /// Version counter (incremented on each change)
    version: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,    // Unix: \n
    CrLf,  // Windows: \r\n
    Cr,    // Classic Mac: \r
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Latin1,
    // Add more as needed
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self;

    /// Create buffer from string
    pub fn from_str(text: &str) -> Self;

    /// Create buffer from file
    pub async fn from_file(path: PathBuf) -> Result<Self, BufferError>;

    /// Get total number of lines
    pub fn line_count(&self) -> usize;

    /// Get line content by index (0-based)
    pub fn line(&self, idx: usize) -> Option<&str>;

    /// Get character at position
    pub fn char_at(&self, pos: Position) -> Option<char>;

    /// Insert text at position
    pub fn insert(&mut self, pos: Position, text: &str);

    /// Delete range
    pub fn delete(&mut self, range: Range);

    /// Get text in range
    pub fn slice(&self, range: Range) -> String;

    /// Get byte offset from position
    pub fn pos_to_offset(&self, pos: Position) -> usize;

    /// Get position from byte offset
    pub fn offset_to_pos(&self, offset: usize) -> Position;

    /// Save buffer to file
    pub async fn save(&mut self, path: Option<PathBuf>) -> Result<(), BufferError>;

    /// Check if buffer is modified
    pub fn is_modified(&self) -> bool;

    /// Get buffer content as string (for small buffers)
    pub fn to_string(&self) -> String;
}
```

### 4.2 Position and Range

```rust
/// Position in a text buffer (0-indexed)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,  // Character offset, not byte offset
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self;
    pub fn zero() -> Self;

    /// Move to start of line
    pub fn line_start(self) -> Self;

    /// Move to end of line (requires buffer context)
    pub fn line_end(self, buffer: &Buffer) -> Self;
}

/// A range in the buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self;

    /// Create zero-width range (cursor position)
    pub fn cursor(pos: Position) -> Self;

    /// Check if range is empty
    pub fn is_empty(&self) -> bool;

    /// Get length in characters
    pub fn len(&self, buffer: &Buffer) -> usize;

    /// Check if position is contained in range
    pub fn contains(&self, pos: Position) -> bool;

    /// Merge two ranges
    pub fn union(self, other: Range) -> Range;
}
```

### 4.3 Selection and Cursor

```rust
/// A selection with cursor position
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    /// The anchor point (where selection started)
    anchor: Position,

    /// The cursor position (active end)
    cursor: Position,
}

impl Selection {
    /// Create new selection
    pub fn new(anchor: Position, cursor: Position) -> Self;

    /// Create cursor (zero-width selection)
    pub fn cursor(pos: Position) -> Self;

    /// Get selection as range
    pub fn range(&self) -> Range;

    /// Check if selection is empty (just a cursor)
    pub fn is_cursor(&self) -> bool;

    /// Get the "head" position (cursor)
    pub fn head(&self) -> Position;

    /// Get the "tail" position (anchor)
    pub fn tail(&self) -> Position;

    /// Flip anchor and cursor
    pub fn flip(self) -> Self;

    /// Extend selection to new cursor position
    pub fn extend_to(&mut self, pos: Position);

    /// Move cursor (collapse selection)
    pub fn move_to(&mut self, pos: Position);
}

/// Multiple selections (for multi-cursor support)
#[derive(Debug, Clone)]
pub struct Selections {
    selections: Vec<Selection>,
    primary_idx: usize,
}

impl Selections {
    /// Create with single cursor
    pub fn single(pos: Position) -> Self;

    /// Get primary selection
    pub fn primary(&self) -> &Selection;

    /// Get mutable primary selection
    pub fn primary_mut(&mut self) -> &mut Selection;

    /// Add new selection
    pub fn add(&mut self, selection: Selection);

    /// Merge overlapping selections
    pub fn merge(&mut self);

    /// Get all selections
    pub fn iter(&self) -> impl Iterator<Item = &Selection>;

    /// Transform all selections
    pub fn transform(&mut self, f: impl Fn(&Selection) -> Selection);
}
```

### 4.4 Document

```rust
use crate::buffer::Buffer;
use crate::selection::Selections;
use crate::undo::UndoStack;
use crate::syntax::SyntaxHighlighter;

/// A document represents an editable text buffer with associated state
pub struct Document {
    /// The underlying text buffer
    buffer: Buffer,

    /// Current selections/cursors
    selections: Selections,

    /// Undo/redo stack
    undo_stack: UndoStack,

    /// Syntax highlighter (optional)
    highlighter: Option<SyntaxHighlighter>,

    /// Scroll position (in lines)
    scroll_offset: f32,

    /// Document-specific settings
    settings: DocumentSettings,
}

#[derive(Debug, Clone)]
pub struct DocumentSettings {
    /// Number of spaces per tab
    pub tab_width: usize,

    /// Use spaces instead of tabs
    pub use_spaces: bool,

    /// Auto-detect indentation
    pub auto_indent: bool,

    /// Show line numbers
    pub show_line_numbers: bool,

    /// Highlight current line
    pub highlight_current_line: bool,

    /// Line length guide column
    pub line_length_guide: Option<usize>,
}

impl Document {
    /// Create new empty document
    pub fn new() -> Self;

    /// Create from file
    pub async fn open(path: PathBuf) -> Result<Self, DocumentError>;

    /// Insert text at all cursors
    pub fn insert(&mut self, text: &str);

    /// Delete selected text or character at cursor
    pub fn delete(&mut self);

    /// Delete backward (backspace)
    pub fn delete_backward(&mut self);

    /// Undo last change
    pub fn undo(&mut self) -> bool;

    /// Redo last undone change
    pub fn redo(&mut self) -> bool;

    /// Move cursor(s)
    pub fn move_cursor(&mut self, movement: Movement);

    /// Select text
    pub fn select(&mut self, movement: Movement);

    /// Get current selections
    pub fn selections(&self) -> &Selections;

    /// Save document
    pub async fn save(&mut self) -> Result<(), DocumentError>;

    /// Save as new file
    pub async fn save_as(&mut self, path: PathBuf) -> Result<(), DocumentError>;

    /// Get buffer reference
    pub fn buffer(&self) -> &Buffer;
}

/// Cursor movement operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
    WordLeft,
    WordRight,
    DocumentStart,
    DocumentEnd,
    PageUp,
    PageDown,
}
```

### 4.5 Undo System

```rust
use crate::buffer::{Position, Range};

/// Undo/redo stack for a document
pub struct UndoStack {
    /// Stack of undo groups
    undo_stack: Vec<UndoGroup>,

    /// Stack of redo groups
    redo_stack: Vec<UndoGroup>,

    /// Current group being built
    current_group: Option<UndoGroup>,

    /// Maximum undo history
    max_size: usize,

    /// Time threshold for auto-grouping (milliseconds)
    group_timeout_ms: u64,

    /// Timestamp of last edit
    last_edit_time: Option<std::time::Instant>,
}

/// A group of related edits (merged together)
#[derive(Debug, Clone)]
struct UndoGroup {
    /// Individual edits in this group
    edits: Vec<Edit>,

    /// Cursor state before edits
    cursor_before: Selections,

    /// Cursor state after edits
    cursor_after: Selections,

    /// Timestamp
    timestamp: std::time::Instant,
}

/// A single edit operation
#[derive(Debug, Clone)]
enum Edit {
    Insert {
        position: Position,
        text: String,
    },
    Delete {
        range: Range,
        deleted_text: String,
    },
}

impl UndoStack {
    /// Create new undo stack
    pub fn new(max_size: usize, group_timeout_ms: u64) -> Self;

    /// Begin a new undo group
    pub fn begin_group(&mut self, cursor_state: Selections);

    /// End current undo group
    pub fn end_group(&mut self, cursor_state: Selections);

    /// Add edit to current group (auto-groups if needed)
    pub fn push_edit(&mut self, edit: Edit, cursor_state: Selections);

    /// Undo last group
    pub fn undo(&mut self, buffer: &mut Buffer) -> Option<Selections>;

    /// Redo last undone group
    pub fn redo(&mut self, buffer: &mut Buffer) -> Option<Selections>;

    /// Check if can undo
    pub fn can_undo(&self) -> bool;

    /// Check if can redo
    pub fn can_redo(&self) -> bool;

    /// Clear all history
    pub fn clear(&mut self);
}
```

### 4.6 Syntax Highlighting

```rust
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::{Theme, Highlighter, HighlightState};
use crate::buffer::Buffer;

/// Syntax highlighter for a document
pub struct SyntaxHighlighter {
    /// Syntax definition
    syntax: SyntaxReference,

    /// Syntax set (shared)
    syntax_set: SyntaxSet,

    /// Color theme
    theme: Theme,

    /// Highlighter instance
    highlighter: Highlighter<'static>,

    /// Cached highlight states per line
    line_states: Vec<Option<HighlightState>>,

    /// Last highlighted line
    last_highlighted: usize,
}

impl SyntaxHighlighter {
    /// Create new highlighter for language
    pub fn new(
        language: &str,
        syntax_set: SyntaxSet,
        theme: Theme,
    ) -> Result<Self, SyntaxError>;

    /// Highlight a single line
    pub fn highlight_line(
        &mut self,
        line_idx: usize,
        line_text: &str,
    ) -> Vec<HighlightedSpan>;

    /// Invalidate cached states from line onwards
    pub fn invalidate_from_line(&mut self, line_idx: usize);

    /// Get language name
    pub fn language(&self) -> &str;

    /// Change theme
    pub fn set_theme(&mut self, theme: Theme);
}

/// A highlighted span of text
#[derive(Debug, Clone)]
pub struct HighlightedSpan {
    /// Text content
    pub text: String,

    /// Foreground color (RGB)
    pub fg_color: (u8, u8, u8),

    /// Background color (RGB, optional)
    pub bg_color: Option<(u8, u8, u8)>,

    /// Font style
    pub style: FontStyle,
}

#[derive(Debug, Clone, Copy)]
pub struct FontStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}
```

### 4.7 Editor State

```rust
use crate::document::Document;
use crate::config::Config;

/// Main editor state
pub struct Editor {
    /// All open documents
    documents: Vec<Document>,

    /// Currently active document index
    active_document: usize,

    /// Global configuration
    config: Config,

    /// Clipboard content (for multiple cursors)
    clipboard: Vec<String>,

    /// Status message
    status_message: Option<StatusMessage>,

    /// Command mode state
    command_mode: Option<CommandMode>,
}

#[derive(Debug, Clone)]
pub struct StatusMessage {
    pub text: String,
    pub message_type: MessageType,
    pub expires_at: std::time::Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug)]
pub enum CommandMode {
    /// Open file dialog
    OpenFile { current_input: String },

    /// Save file dialog
    SaveFile { current_input: String },

    /// Go to line
    GoToLine { current_input: String },

    /// Find
    Find {
        query: String,
        case_sensitive: bool,
        regex: bool,
    },

    /// Replace
    Replace {
        find: String,
        replace: String,
        case_sensitive: bool,
        regex: bool,
    },
}

impl Editor {
    /// Create new editor
    pub fn new(config: Config) -> Self;

    /// Open a file
    pub async fn open_file(&mut self, path: PathBuf) -> Result<(), EditorError>;

    /// Create new empty document
    pub fn new_document(&mut self);

    /// Close current document
    pub fn close_document(&mut self) -> Result<(), EditorError>;

    /// Get active document
    pub fn active_document(&self) -> &Document;

    /// Get mutable active document
    pub fn active_document_mut(&mut self) -> &mut Document;

    /// Execute command
    pub fn execute_command(&mut self, command: Command);

    /// Show status message
    pub fn show_status(&mut self, message: String, message_type: MessageType);

    /// Enter command mode
    pub fn enter_command_mode(&mut self, mode: CommandMode);

    /// Exit command mode
    pub fn exit_command_mode(&mut self);
}
```

---

## 5. Event Handling System

### 5.1 Event Types

```rust
/// Events that can occur in the editor
#[derive(Debug, Clone)]
pub enum EditorEvent {
    /// Keyboard input
    KeyPress(KeyEvent),

    /// Mouse input
    Mouse(MouseEvent),

    /// Window events
    Window(WindowEvent),

    /// File system events
    FileSystem(FileSystemEvent),

    /// Command execution
    Command(Command),

    /// Timer/scheduled events
    Timer(TimerId),
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    pub text: Option<String>,  // For text input
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,  // Command on macOS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char),
    Backspace,
    Delete,
    Enter,
    Tab,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Click { position: ScreenPosition, button: MouseButton },
    DoubleClick { position: ScreenPosition },
    TripleClick { position: ScreenPosition },
    Drag { from: ScreenPosition, to: ScreenPosition },
    Scroll { delta: f32 },
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
pub enum WindowEvent {
    Resize { width: u32, height: u32 },
    Focus,
    Blur,
    Close,
}

#[derive(Debug, Clone)]
pub enum FileSystemEvent {
    FileChanged { path: PathBuf },
    FileDeleted { path: PathBuf },
    FileCreated { path: PathBuf },
}
```

### 5.2 Command System

```rust
/// Commands that can be executed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    // File operations
    NewFile,
    OpenFile,
    Save,
    SaveAs,
    Close,
    CloseAll,
    Quit,

    // Editing
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    Delete,
    DeleteLine,
    DuplicateLine,

    // Navigation
    MoveCursor(Movement),
    Select(Movement),
    GoToLine(usize),

    // Search
    Find,
    FindNext,
    FindPrevious,
    Replace,

    // View
    ToggleLineNumbers,
    ToggleHighlightCurrentLine,
    ZoomIn,
    ZoomOut,
    ZoomReset,

    // Multi-cursor
    AddCursorAbove,
    AddCursorBelow,
    AddCursorAtNextOccurrence,

    // Custom (for plugins)
    Custom { id: String, args: Vec<String> },
}

/// Keybinding map
pub struct KeyMap {
    bindings: HashMap<(Key, Modifiers), Command>,
}

impl KeyMap {
    /// Create default keymap
    pub fn default() -> Self;

    /// Bind key to command
    pub fn bind(&mut self, key: Key, modifiers: Modifiers, command: Command);

    /// Look up command for key event
    pub fn lookup(&self, event: &KeyEvent) -> Option<&Command>;

    /// Remove binding
    pub fn unbind(&mut self, key: Key, modifiers: Modifiers);
}
```

### 5.3 Event Dispatcher

```rust
use crossbeam::channel::{Sender, Receiver};

/// Event dispatcher handles routing events to handlers
pub struct EventDispatcher {
    /// Event queue
    event_tx: Sender<EditorEvent>,
    event_rx: Receiver<EditorEvent>,

    /// Keymap for translating key events to commands
    keymap: KeyMap,
}

impl EventDispatcher {
    pub fn new(keymap: KeyMap) -> Self;

    /// Send event to queue
    pub fn send(&self, event: EditorEvent);

    /// Process next event
    pub fn process(&self, editor: &mut Editor) -> Option<()>;

    /// Process all pending events
    pub fn process_all(&self, editor: &mut Editor);
}
```

---

## 6. Rendering Pipeline

### 6.1 Rendering Architecture

```
┌─────────────────────────────────────────────┐
│          Floem Reactive UI                   │
│  (Handles layout, painting, invalidation)   │
└───────────────┬─────────────────────────────┘
                │
        ┌───────┴───────┐
        │               │
┌───────▼──────┐  ┌────▼─────────┐
│ Editor View  │  │ Status View  │
│              │  │              │
│ ┌──────────┐ │  │              │
│ │  Gutter  │ │  └──────────────┘
│ └──────────┘ │
│ ┌──────────┐ │
│ │Text Area │ │
│ └──────────┘ │
└──────────────┘
```

### 6.2 View Components

```rust
use floem::views::*;
use floem::reactive::*;

/// Main editor view
pub struct EditorView {
    /// Editor state (reactive)
    editor: RwSignal<Editor>,

    /// View dimensions
    width: f32,
    height: f32,

    /// Line height in pixels
    line_height: f32,

    /// Font metrics
    font_size: f32,
    char_width: f32,

    /// Scroll state
    scroll_x: f32,
    scroll_y: f32,

    /// Visible line range
    first_visible_line: usize,
    last_visible_line: usize,
}

impl EditorView {
    pub fn new(editor: RwSignal<Editor>) -> Self;

    /// Build Floem view tree
    pub fn build(self) -> impl View;

    /// Calculate visible line range
    fn calculate_visible_lines(&self) -> (usize, usize);

    /// Convert screen position to document position
    fn screen_to_position(&self, screen: ScreenPosition) -> Position;

    /// Convert document position to screen position
    fn position_to_screen(&self, pos: Position) -> ScreenPosition;
}

/// Gutter view (line numbers)
pub struct GutterView {
    /// Reference to document
    document: RwSignal<Document>,

    /// Width in pixels
    width: f32,

    /// Line height
    line_height: f32,

    /// First visible line
    first_visible_line: usize,

    /// Last visible line
    last_visible_line: usize,
}

impl GutterView {
    pub fn new(document: RwSignal<Document>) -> Self;

    pub fn build(self) -> impl View;

    /// Calculate required width based on line count
    fn calculate_width(&self, line_count: usize) -> f32;
}

/// Text area view (main editing area)
pub struct TextAreaView {
    /// Reference to document
    document: RwSignal<Document>,

    /// Dimensions
    width: f32,
    height: f32,

    /// Font metrics
    line_height: f32,
    char_width: f32,

    /// Scroll offset
    scroll_x: f32,
    scroll_y: f32,
}

impl TextAreaView {
    pub fn new(document: RwSignal<Document>) -> Self;

    pub fn build(self) -> impl View;

    /// Render visible lines
    fn render_lines(&self, cx: &mut PaintCx);

    /// Render selections
    fn render_selections(&self, cx: &mut PaintCx);

    /// Render cursors
    fn render_cursors(&self, cx: &mut PaintCx);

    /// Render current line highlight
    fn render_current_line_highlight(&self, cx: &mut PaintCx);
}

/// Status bar view
pub struct StatusBarView {
    /// Reference to editor
    editor: RwSignal<Editor>,
}

impl StatusBarView {
    pub fn new(editor: RwSignal<Editor>) -> Self;

    pub fn build(self) -> impl View;
}
```

### 6.3 Rendering Strategy

1. **Incremental Rendering**: Only render visible lines
2. **Cached Layouts**: Cache line layouts between frames
3. **Dirty Tracking**: Only re-render changed regions
4. **Syntax Highlighting**: Highlight on-demand with caching
5. **60 FPS Target**: Budget 16.67ms per frame

```rust
/// Rendering context with caching
pub struct RenderCache {
    /// Cached line layouts
    line_layouts: Vec<Option<LineLayout>>,

    /// Cached syntax highlighting
    highlighted_lines: Vec<Option<Vec<HighlightedSpan>>>,

    /// Dirty regions
    dirty_lines: std::ops::Range<usize>,
}

#[derive(Clone)]
struct LineLayout {
    /// Positioned glyphs
    glyphs: Vec<PositionedGlyph>,

    /// Line width in pixels
    width: f32,

    /// Line number this layout is for
    line_number: usize,

    /// Buffer version when cached
    version: u64,
}

impl RenderCache {
    pub fn new() -> Self;

    /// Mark lines as dirty
    pub fn invalidate_lines(&mut self, range: std::ops::Range<usize>);

    /// Get or compute line layout
    pub fn get_line_layout(
        &mut self,
        line_idx: usize,
        buffer: &Buffer,
    ) -> &LineLayout;

    /// Clear all caches
    pub fn clear(&mut self);
}
```

---

## 7. State Management

### 7.1 Reactive State with Floem

Floem uses signals for reactive state management:

```rust
use floem::reactive::*;

/// Application state (reactive)
pub struct AppState {
    /// Editor state
    pub editor: RwSignal<Editor>,

    /// UI state
    pub ui: UiState,
}

pub struct UiState {
    /// Window dimensions
    pub window_size: RwSignal<(u32, u32)>,

    /// Font size
    pub font_size: RwSignal<f32>,

    /// Theme
    pub theme: RwSignal<Theme>,

    /// Show line numbers
    pub show_line_numbers: RwSignal<bool>,

    /// Status bar visibility
    pub show_status_bar: RwSignal<bool>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let editor = RwSignal::new(Editor::new(config));

        Self {
            editor,
            ui: UiState {
                window_size: RwSignal::new((1024, 768)),
                font_size: RwSignal::new(14.0),
                theme: RwSignal::new(Theme::default()),
                show_line_numbers: RwSignal::new(true),
                show_status_bar: RwSignal::new(true),
            },
        }
    }
}
```

### 7.2 State Updates

```rust
impl Editor {
    /// Execute command and update state
    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Insert(text) => {
                let doc = self.active_document_mut();
                doc.insert(&text);
            }

            Command::Undo => {
                let doc = self.active_document_mut();
                doc.undo();
            }

            Command::Save => {
                // Trigger async save
                let doc = self.active_document_mut();
                // ... spawn save task
            }

            // ... other commands
        }
    }
}
```

### 7.3 State Persistence

```rust
use serde::{Serialize, Deserialize};

/// Persistent editor state (saved between sessions)
#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    /// Recently opened files
    pub recent_files: Vec<PathBuf>,

    /// Window size and position
    pub window_geometry: WindowGeometry,

    /// Open files from last session
    pub open_files: Vec<PathBuf>,

    /// Active file index
    pub active_file: usize,
}

#[derive(Serialize, Deserialize)]
pub struct WindowGeometry {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

impl PersistedState {
    /// Load from disk
    pub fn load() -> Result<Self, StateError>;

    /// Save to disk
    pub fn save(&self) -> Result<(), StateError>;

    /// Get default state file path
    fn state_file_path() -> PathBuf;
}
```

---

## 8. Configuration Management

### 8.1 Configuration Structure

```rust
use serde::{Serialize, Deserialize};

/// Global configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Editor settings
    pub editor: EditorConfig,

    /// UI settings
    pub ui: UiConfig,

    /// Keymap settings
    pub keymap: KeymapConfig,

    /// Language-specific settings
    pub languages: HashMap<String, LanguageConfig>,

    /// Plugin settings
    pub plugins: HashMap<String, toml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Tab width
    pub tab_width: usize,

    /// Use spaces instead of tabs
    pub use_spaces: bool,

    /// Auto-detect indentation
    pub auto_detect_indentation: bool,

    /// Line ending preference
    pub line_ending: LineEnding,

    /// Auto-save interval (seconds, 0 = disabled)
    pub auto_save_interval: u64,

    /// Maximum file size to open (MB)
    pub max_file_size_mb: usize,

    /// Undo limit
    pub max_undo_history: usize,

    /// Undo grouping timeout (milliseconds)
    pub undo_group_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Font family
    pub font_family: String,

    /// Font size
    pub font_size: f32,

    /// Line height multiplier
    pub line_height: f32,

    /// Show line numbers
    pub show_line_numbers: bool,

    /// Highlight current line
    pub highlight_current_line: bool,

    /// Line length guide
    pub line_length_guide: Option<usize>,

    /// Color theme
    pub theme: String,

    /// Cursor blink rate (ms, 0 = no blink)
    pub cursor_blink_rate_ms: u64,

    /// Scroll sensitivity
    pub scroll_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeymapConfig {
    /// Keybinding preset (vim, emacs, default)
    pub preset: String,

    /// Custom keybindings
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// File extensions
    pub extensions: Vec<String>,

    /// Tab width override
    pub tab_width: Option<usize>,

    /// Use spaces override
    pub use_spaces: Option<bool>,

    /// Syntax file
    pub syntax: Option<String>,
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self, ConfigError>;

    /// Save configuration to file
    pub fn save(&self) -> Result<(), ConfigError>;

    /// Get default configuration
    pub fn default() -> Self;

    /// Get config file path
    fn config_file_path() -> PathBuf;

    /// Merge with user config
    pub fn merge(&mut self, user_config: Config);
}
```

### 8.2 Configuration File Format

Default location: `~/.config/rust-editor/config.toml`

```toml
[editor]
tab_width = 4
use_spaces = true
auto_detect_indentation = true
line_ending = "lf"
auto_save_interval = 0
max_file_size_mb = 100
max_undo_history = 10000
undo_group_timeout_ms = 300

[ui]
font_family = "JetBrains Mono"
font_size = 14.0
line_height = 1.4
show_line_numbers = true
highlight_current_line = true
line_length_guide = 80
theme = "monokai"
cursor_blink_rate_ms = 500
scroll_speed = 3.0

[keymap]
preset = "default"

[keymap.custom]
# Custom keybindings
"ctrl+shift+d" = "duplicate_line"
"ctrl+/" = "toggle_comment"

[languages.rust]
extensions = ["rs"]
tab_width = 4
use_spaces = true

[languages.python]
extensions = ["py"]
tab_width = 4
use_spaces = true

[languages.javascript]
extensions = ["js", "jsx", "mjs"]
tab_width = 2
use_spaces = true
```

---

## 9. Async Strategy

### 9.1 Async Runtime

Use Tokio for async operations:

```rust
use tokio::runtime::Runtime;

/// Async runtime for background tasks
pub struct AsyncRuntime {
    runtime: Runtime,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        Self { runtime }
    }

    /// Spawn async task
    pub fn spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    /// Block on async task (use sparingly)
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.runtime.block_on(future)
    }
}
```

### 9.2 Async File I/O

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Async file operations
pub struct FileIO;

impl FileIO {
    /// Read file asynchronously
    pub async fn read_file(path: &Path) -> Result<String, IoError> {
        let mut file = fs::File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        Ok(contents)
    }

    /// Write file asynchronously
    pub async fn write_file(path: &Path, contents: &str) -> Result<(), IoError> {
        let mut file = fs::File::create(path).await?;
        file.write_all(contents.as_bytes()).await?;
        file.sync_all().await?;
        Ok(())
    }

    /// Detect file encoding
    pub async fn detect_encoding(path: &Path) -> Result<Encoding, IoError> {
        let mut file = fs::File::open(path).await?;
        let mut buffer = vec![0u8; 8192];
        let n = file.read(&mut buffer).await?;
        buffer.truncate(n);

        // Use encoding_rs to detect
        let (encoding, _, _) = encoding_rs::Encoding::for_bom(&buffer)
            .unwrap_or((encoding_rs::UTF_8, false));

        Ok(Encoding::from_encoding_rs(encoding))
    }

    /// Detect line ending
    pub fn detect_line_ending(text: &str) -> LineEnding {
        if text.contains("\r\n") {
            LineEnding::CrLf
        } else if text.contains('\r') {
            LineEnding::Cr
        } else {
            LineEnding::Lf
        }
    }
}
```

### 9.3 File Watching

```rust
use notify::{Watcher, RecursiveMode, Event};
use tokio::sync::mpsc;

/// File system watcher
pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    event_rx: mpsc::Receiver<FileSystemEvent>,
}

impl FileWatcher {
    pub fn new() -> Result<Self, WatchError> {
        let (tx, rx) = mpsc::channel(100);

        let watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                // Convert notify event to FileSystemEvent
                // Send via tx
            }
        })?;

        Ok(Self {
            watcher,
            event_rx: rx,
        })
    }

    /// Watch a file
    pub fn watch(&mut self, path: &Path) -> Result<(), WatchError> {
        self.watcher.watch(path, RecursiveMode::NonRecursive)?;
        Ok(())
    }

    /// Stop watching a file
    pub fn unwatch(&mut self, path: &Path) -> Result<(), WatchError> {
        self.watcher.unwatch(path)?;
        Ok(())
    }

    /// Get next file system event
    pub async fn next_event(&mut self) -> Option<FileSystemEvent> {
        self.event_rx.recv().await
    }
}
```

### 9.4 Background Syntax Highlighting

```rust
use tokio::sync::mpsc;

/// Background syntax highlighting service
pub struct SyntaxService {
    /// Request channel
    request_tx: mpsc::Sender<SyntaxRequest>,

    /// Response channel
    response_rx: mpsc::Receiver<SyntaxResponse>,
}

struct SyntaxRequest {
    buffer_id: BufferId,
    line_range: std::ops::Range<usize>,
    lines: Vec<String>,
}

struct SyntaxResponse {
    buffer_id: BufferId,
    line_range: std::ops::Range<usize>,
    highlighted: Vec<Vec<HighlightedSpan>>,
}

impl SyntaxService {
    pub fn new() -> Self {
        let (req_tx, mut req_rx) = mpsc::channel(100);
        let (resp_tx, resp_rx) = mpsc::channel(100);

        // Spawn background worker
        tokio::spawn(async move {
            while let Some(request) = req_rx.recv().await {
                // Process syntax highlighting
                // Send response
            }
        });

        Self {
            request_tx: req_tx,
            response_rx: resp_rx,
        }
    }

    /// Request syntax highlighting
    pub async fn highlight_lines(
        &self,
        buffer_id: BufferId,
        line_range: std::ops::Range<usize>,
        lines: Vec<String>,
    ) -> Result<(), SyntaxError> {
        self.request_tx.send(SyntaxRequest {
            buffer_id,
            line_range,
            lines,
        }).await?;
        Ok(())
    }

    /// Poll for completed highlighting
    pub fn poll_response(&mut self) -> Option<SyntaxResponse> {
        self.response_rx.try_recv().ok()
    }
}
```

---

## 10. File Structure

```
rust-editor/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
│
├── src/
│   ├── main.rs                 # Application entry point
│   ├── app.rs                  # Application setup and main loop
│   ├── lib.rs                  # Library root (for testing)
│   │
│   ├── buffer/
│   │   ├── mod.rs              # Buffer module
│   │   ├── rope.rs             # Rope wrapper utilities
│   │   ├── position.rs         # Position and Range types
│   │   └── tests.rs            # Buffer tests
│   │
│   ├── document/
│   │   ├── mod.rs              # Document module
│   │   ├── selection.rs        # Selection and cursor
│   │   ├── settings.rs         # Document settings
│   │   └── tests.rs            # Document tests
│   │
│   ├── editor/
│   │   ├── mod.rs              # Editor state
│   │   ├── operations.rs       # Editing operations
│   │   └── tests.rs            # Editor tests
│   │
│   ├── ui/
│   │   ├── mod.rs              # UI module
│   │   ├── editor_view.rs      # Main editor view
│   │   ├── gutter.rs           # Gutter (line numbers)
│   │   ├── text_area.rs        # Text editing area
│   │   ├── status_bar.rs       # Status bar
│   │   ├── theme.rs            # UI theming
│   │   └── render_cache.rs     # Rendering cache
│   │
│   ├── syntax/
│   │   ├── mod.rs              # Syntax highlighting
│   │   ├── highlighter.rs      # Highlighter implementation
│   │   ├── languages.rs        # Language definitions
│   │   └── service.rs          # Background highlighting service
│   │
│   ├── commands/
│   │   ├── mod.rs              # Command system
│   │   ├── command.rs          # Command types
│   │   ├── keymap.rs           # Keybinding map
│   │   └── defaults.rs         # Default keybindings
│   │
│   ├── events/
│   │   ├── mod.rs              # Event system
│   │   ├── types.rs            # Event types
│   │   └── dispatcher.rs       # Event dispatcher
│   │
│   ├── io/
│   │   ├── mod.rs              # File I/O
│   │   ├── file.rs             # File operations
│   │   ├── encoding.rs         # Encoding detection
│   │   └── watcher.rs          # File watching
│   │
│   ├── undo/
│   │   ├── mod.rs              # Undo/redo system
│   │   ├── stack.rs            # Undo stack
│   │   └── tests.rs            # Undo tests
│   │
│   ├── config/
│   │   ├── mod.rs              # Configuration
│   │   ├── load.rs             # Config loading
│   │   └── defaults.rs         # Default configuration
│   │
│   ├── clipboard/
│   │   └── mod.rs              # Clipboard integration
│   │
│   └── utils/
│       ├── mod.rs              # Utilities
│       ├── paths.rs            # Path utilities
│       └── text.rs             # Text utilities
│
├── assets/
│   ├── themes/                 # Color themes
│   │   ├── monokai.json
│   │   ├── solarized-dark.json
│   │   └── solarized-light.json
│   │
│   └── syntaxes/               # Syntax definitions
│       └── (auto-downloaded by syntect)
│
├── tests/
│   ├── integration/
│   │   ├── basic_editing.rs
│   │   ├── file_io.rs
│   │   └── undo_redo.rs
│   └── fixtures/
│       └── sample_files/
│
└── benches/
    ├── buffer_operations.rs
    ├── syntax_highlighting.rs
    └── rendering.rs
```

---

## 11. API Design

### 11.1 Public API Surface

The editor exposes a clean public API for embedding:

```rust
// lib.rs

pub use crate::editor::Editor;
pub use crate::document::Document;
pub use crate::buffer::{Buffer, Position, Range};
pub use crate::config::Config;
pub use crate::commands::Command;

/// Create and run editor application
pub fn run(config: Config) -> Result<(), EditorError> {
    let app = App::new(config);
    app.run()
}

/// Create editor instance (for embedding)
pub fn create_editor(config: Config) -> Editor {
    Editor::new(config)
}
```

### 11.2 Buffer API

```rust
impl Buffer {
    // Creation
    pub fn new() -> Self;
    pub fn from_str(text: &str) -> Self;
    pub async fn from_file(path: PathBuf) -> Result<Self, BufferError>;

    // Queries
    pub fn line_count(&self) -> usize;
    pub fn len_chars(&self) -> usize;
    pub fn len_bytes(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn line(&self, idx: usize) -> Option<Cow<str>>;
    pub fn slice(&self, range: Range) -> String;

    // Modifications
    pub fn insert(&mut self, pos: Position, text: &str);
    pub fn delete(&mut self, range: Range) -> String;
    pub fn replace(&mut self, range: Range, text: &str) -> String;

    // Conversions
    pub fn pos_to_offset(&self, pos: Position) -> usize;
    pub fn offset_to_pos(&self, offset: usize) -> Position;

    // I/O
    pub async fn save(&mut self, path: Option<PathBuf>) -> Result<(), BufferError>;
    pub async fn reload(&mut self) -> Result<(), BufferError>;
}
```

### 11.3 Document API

```rust
impl Document {
    // Creation
    pub fn new() -> Self;
    pub async fn open(path: PathBuf) -> Result<Self, DocumentError>;

    // Editing
    pub fn insert(&mut self, text: &str);
    pub fn delete(&mut self);
    pub fn delete_backward(&mut self);
    pub fn delete_selection(&mut self);

    // Navigation
    pub fn move_cursor(&mut self, movement: Movement);
    pub fn select(&mut self, movement: Movement);
    pub fn set_selection(&mut self, selection: Selection);

    // Undo/Redo
    pub fn undo(&mut self) -> bool;
    pub fn redo(&mut self) -> bool;
    pub fn can_undo(&self) -> bool;
    pub fn can_redo(&self) -> bool;

    // Queries
    pub fn selections(&self) -> &Selections;
    pub fn buffer(&self) -> &Buffer;
    pub fn is_modified(&self) -> bool;
    pub fn path(&self) -> Option<&Path>;

    // I/O
    pub async fn save(&mut self) -> Result<(), DocumentError>;
    pub async fn save_as(&mut self, path: PathBuf) -> Result<(), DocumentError>;
}
```

### 11.4 Editor API

```rust
impl Editor {
    // Creation
    pub fn new(config: Config) -> Self;

    // Document management
    pub fn new_document(&mut self);
    pub async fn open_file(&mut self, path: PathBuf) -> Result<usize, EditorError>;
    pub fn close_document(&mut self, index: usize) -> Result<(), EditorError>;
    pub fn switch_document(&mut self, index: usize) -> Result<(), EditorError>;

    // Access
    pub fn active_document(&self) -> &Document;
    pub fn active_document_mut(&mut self) -> &mut Document;
    pub fn document(&self, index: usize) -> Option<&Document>;
    pub fn document_count(&self) -> usize;

    // Commands
    pub fn execute_command(&mut self, command: Command);

    // Status
    pub fn show_status(&mut self, message: String, message_type: MessageType);

    // Configuration
    pub fn config(&self) -> &Config;
    pub fn update_config(&mut self, config: Config);
}
```

---

## 12. Extension Points

### 12.1 Plugin Architecture (Future)

Design extensibility into core architecture:

```rust
/// Plugin trait (for future plugin system)
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Initialize plugin
    fn initialize(&mut self, editor: &mut Editor) -> Result<(), PluginError>;

    /// Handle command
    fn handle_command(&mut self, command: &Command) -> Option<()>;

    /// Handle event
    fn handle_event(&mut self, event: &EditorEvent) -> Option<()>;

    /// Cleanup
    fn shutdown(&mut self);
}

/// Plugin registry
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self;

    /// Register plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>);

    /// Initialize all plugins
    pub fn initialize_all(&mut self, editor: &mut Editor) -> Result<(), PluginError>;

    /// Dispatch command to plugins
    pub fn dispatch_command(&mut self, command: &Command);

    /// Dispatch event to plugins
    pub fn dispatch_event(&mut self, event: &EditorEvent);
}
```

### 12.2 Extension Points

1. **Commands**: Custom commands via plugin
2. **Keybindings**: Custom key mappings
3. **Syntax Definitions**: Additional languages
4. **Themes**: Custom color schemes
5. **UI Components**: Custom views (future)
6. **Event Handlers**: React to editor events
7. **File Type Handlers**: Custom file handling

### 12.3 Hook System

```rust
/// Event hooks for plugins
pub enum Hook {
    /// Before buffer is modified
    BeforeBufferChange {
        buffer_id: BufferId,
        range: Range,
        new_text: String,
    },

    /// After buffer is modified
    AfterBufferChange {
        buffer_id: BufferId,
        range: Range,
        old_text: String,
    },

    /// Before file is saved
    BeforeSave {
        path: PathBuf,
    },

    /// After file is saved
    AfterSave {
        path: PathBuf,
    },

    /// Document opened
    DocumentOpened {
        document_id: BufferId,
    },

    /// Document closed
    DocumentClosed {
        document_id: BufferId,
    },
}
```

---

## 13. Performance Targets

### 13.1 Startup Performance

- **Cold start**: < 100ms
- **Warm start**: < 50ms
- **Binary size**: < 20MB
- **Memory (no files open)**: < 30MB

### 13.2 Runtime Performance

- **Target FPS**: 60 FPS (16.67ms/frame)
- **Frame budget**:
  - Event processing: 2ms
  - State updates: 2ms
  - Layout: 3ms
  - Rendering: 8ms
  - Buffer: 2ms

### 13.3 Editing Operations

- **Keystroke latency**: < 5ms (perceived instant)
- **Insert operation**: O(log n) amortized
- **Delete operation**: O(log n) amortized
- **Undo/redo**: O(1) for operation lookup

### 13.4 File Operations

- **Open file (< 1MB)**: < 50ms
- **Save file**: < 100ms
- **Syntax highlight (1000 lines)**: < 10ms
- **Search (10,000 lines)**: < 100ms

### 13.5 Scalability

- **File size**: Handle files up to 100MB
- **Line count**: Efficiently handle 100,000+ lines
- **Multiple cursors**: Support 100+ simultaneous cursors
- **Open documents**: 50+ without performance degradation

---

## 14. Testing Strategy

### 14.1 Unit Tests

```rust
// src/buffer/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at_start() {
        let mut buffer = Buffer::from_str("hello");
        buffer.insert(Position::new(0, 0), "world ");
        assert_eq!(buffer.to_string(), "world hello");
    }

    #[test]
    fn test_delete_range() {
        let mut buffer = Buffer::from_str("hello world");
        buffer.delete(Range::new(
            Position::new(0, 0),
            Position::new(0, 6),
        ));
        assert_eq!(buffer.to_string(), "world");
    }

    #[test]
    fn test_multiline_operations() {
        let mut buffer = Buffer::from_str("line1\nline2\nline3");
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.line(1), Some("line2".into()));
    }
}
```

### 14.2 Integration Tests

```rust
// tests/integration/basic_editing.rs

use rust_editor::*;

#[tokio::test]
async fn test_basic_workflow() {
    let config = Config::default();
    let mut editor = Editor::new(config);

    // Create new document
    editor.new_document();

    // Insert text
    editor.execute_command(Command::Insert("Hello, world!".into()));

    // Verify
    let doc = editor.active_document();
    assert_eq!(doc.buffer().to_string(), "Hello, world!");

    // Undo
    editor.execute_command(Command::Undo);
    assert_eq!(doc.buffer().to_string(), "");

    // Redo
    editor.execute_command(Command::Redo);
    assert_eq!(doc.buffer().to_string(), "Hello, world!");
}

#[tokio::test]
async fn test_file_operations() {
    let config = Config::default();
    let mut editor = Editor::new(config);

    // Create temp file
    let temp_path = "/tmp/test_file.txt";
    std::fs::write(temp_path, "test content").unwrap();

    // Open file
    editor.open_file(temp_path.into()).await.unwrap();

    // Verify content
    let doc = editor.active_document();
    assert_eq!(doc.buffer().to_string(), "test content");

    // Modify
    editor.execute_command(Command::Insert(" modified".into()));

    // Save
    editor.execute_command(Command::Save);

    // Verify file content
    let saved = std::fs::read_to_string(temp_path).unwrap();
    assert_eq!(saved, "test content modified");

    // Cleanup
    std::fs::remove_file(temp_path).unwrap();
}
```

### 14.3 Performance Benchmarks

```rust
// benches/buffer_operations.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_editor::buffer::Buffer;

fn bench_insert(c: &mut Criterion) {
    c.bench_function("buffer insert", |b| {
        let mut buffer = Buffer::new();
        b.iter(|| {
            buffer.insert(Position::new(0, 0), black_box("x"));
        });
    });
}

fn bench_delete(c: &mut Criterion) {
    c.bench_function("buffer delete", |b| {
        let mut buffer = Buffer::from_str("x".repeat(10000).as_str());
        b.iter(|| {
            buffer.delete(Range::new(
                Position::new(0, 0),
                Position::new(0, 1),
            ));
        });
    });
}

criterion_group!(benches, bench_insert, bench_delete);
criterion_main!(benches);
```

### 14.4 Property-Based Tests

```rust
// Use proptest for property-based testing

#[cfg(test)]
mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_insert_then_delete_is_noop(
            text in ".*",
            pos_line in 0usize..100,
            pos_col in 0usize..100,
        ) {
            let mut buffer = Buffer::from_str(&text);
            let original = buffer.to_string();

            let pos = Position::new(pos_line, pos_col);
            if buffer.is_valid_position(pos) {
                buffer.insert(pos, "x");
                buffer.delete(Range::new(pos, pos.offset_right(1)));

                assert_eq!(buffer.to_string(), original);
            }
        }
    }
}
```

---

## 15. Implementation Phases

### Phase 1: Core MVP (Weeks 1-4)

**Week 1: Foundation**
- [ ] Project setup and dependencies
- [ ] Buffer implementation with ropey
- [ ] Position/Range types
- [ ] Basic unit tests

**Week 2: Document & Editing**
- [ ] Document abstraction
- [ ] Selection and cursor
- [ ] Basic editing operations
- [ ] Undo/redo system

**Week 3: UI Foundation**
- [ ] Floem integration
- [ ] Basic editor view
- [ ] Text rendering
- [ ] Gutter (line numbers)

**Week 4: Polish & Integration**
- [ ] Syntax highlighting (syntect)
- [ ] File I/O
- [ ] Keybindings
- [ ] Status bar
- [ ] Integration tests

### Phase 2: Enhancement (Weeks 5-8)

- [ ] Multi-cursor support
- [ ] Find/replace
- [ ] File watching
- [ ] Configuration system
- [ ] Theme support
- [ ] Performance optimization

### Phase 3: Advanced (Weeks 9-12)

- [ ] Plugin system foundation
- [ ] LSP client (basic)
- [ ] Project/workspace support
- [ ] Advanced editing (macros, snippets)
- [ ] Documentation

---

## 16. Risks and Mitigations

### Risk 1: Floem Maturity

**Risk**: Floem is relatively new and may have bugs or missing features.
**Mitigation**:
- Have fallback plan to switch to egui if needed
- Contribute fixes upstream
- Keep UI layer abstracted

### Risk 2: Performance

**Risk**: May not achieve 60 FPS target on all platforms.
**Mitigation**:
- Early performance testing
- Profiling at each phase
- Incremental rendering optimizations
- Render caching

### Risk 3: Rope Complexity

**Risk**: Ropey may have edge cases or performance issues.
**Mitigation**:
- Extensive unit testing
- Property-based testing
- Fallback to gap buffer if needed
- Contribute fixes to ropey

### Risk 4: Async Complexity

**Risk**: Async file I/O may introduce complexity.
**Mitigation**:
- Clear separation of sync/async boundaries
- Comprehensive error handling
- Use established patterns (tokio)

---

## 17. Success Metrics

### MVP Success Criteria

1. **Functionality**
   - ✓ Can edit text files
   - ✓ Undo/redo works correctly
   - ✓ Can save and load files
   - ✓ Syntax highlighting functional
   - ✓ Basic keyboard navigation

2. **Performance**
   - ✓ 60 FPS while editing
   - ✓ Startup < 100ms
   - ✓ Handles files up to 10MB

3. **Quality**
   - ✓ 80%+ test coverage
   - ✓ No data loss bugs
   - ✓ Stable on all platforms

4. **UX**
   - ✓ Pleasant editing experience
   - ✓ Responsive UI
   - ✓ Intuitive keybindings

---

## 18. Future Enhancements

### Post-MVP Features

1. **Multi-file editing**: Tabs, splits
2. **Project support**: File tree, fuzzy finder
3. **Language Server Protocol**: Full LSP support
4. **Advanced search**: Regex, multi-file
5. **Git integration**: Diff view, blame
6. **Terminal**: Integrated terminal
7. **Snippets**: Code snippet system
8. **Macros**: Keyboard macro recording
9. **Plugins**: Full plugin system with Lua/WASM
10. **Collaboration**: Real-time collaborative editing

---

## Conclusion

This architecture provides a solid foundation for building a fast, extensible text editor in Rust. The design emphasizes:

- **Simplicity**: Clean, understandable abstractions
- **Performance**: 60 FPS target with efficient data structures
- **Extensibility**: Plugin-ready from day one
- **Type Safety**: Leveraging Rust's strengths
- **Modularity**: Testable, maintainable components

The Phase 1 MVP focuses on core editing functionality while establishing patterns that will support future enhancements. By using proven libraries (ropey, syntect, Floem, tokio) and following Rust best practices, we can deliver a high-quality editor that serves as a foundation for more advanced features.

**Next Steps**: Begin implementation with Week 1 tasks (project setup, buffer implementation).
