# Lite XL - Comprehensive Feature Specification

**Version:** 2.1.7
**Last Updated:** November 2025
**License:** MIT

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Core Features](#core-features)
3. [Editor Capabilities](#editor-capabilities)
4. [User Interface](#user-interface)
5. [File and Project Management](#file-and-project-management)
6. [Search and Navigation](#search-and-navigation)
7. [Customization and Configuration](#customization-and-configuration)
8. [Plugin System](#plugin-system)
9. [Built-in Plugins](#built-in-plugins)
10. [Extended Plugin Ecosystem](#extended-plugin-ecosystem)
11. [Language Support](#language-support)
12. [Performance Characteristics](#performance-characteristics)
13. [Platform Support](#platform-support)
14. [Technical Architecture](#technical-architecture)
15. [Accessibility Features](#accessibility-features)
16. [Integration Capabilities](#integration-capabilities)

---

## 1. Executive Summary

Lite XL is a lightweight, fast, and extensible text editor designed for developers who value simplicity, performance, and customizability. Written primarily in Lua with a C backend for rendering and system operations, Lite XL provides a modern editing experience while maintaining a minimal footprint.

### Key Characteristics

- **Lightweight**: Small binary size (~10MB), minimal memory footprint
- **Fast**: Smooth 60 FPS rendering, instant startup, responsive UI
- **Extensible**: Powerful Lua-based plugin system with 100+ available plugins
- **Cross-Platform**: Native support for Windows, Linux, macOS, and FreeBSD
- **Open Source**: MIT licensed, community-driven development

### Target Users

- Software developers seeking a lightweight alternative to heavyweight IDEs
- System administrators needing a fast, reliable text editor
- Writers and content creators who prefer minimalist interfaces
- Users who want deep customization without complexity

---

## 2. Core Features

### 2.1 Modern Text Editing

#### Multi-Cursor Support
- **Unlimited Cursors**: Create and manage unlimited cursors simultaneously
- **Smart Merging**: Automatically merges overlapping cursors
- **Synchronized Operations**: All editing commands work across all cursors
- **Cursor Creation Methods**:
  - `Ctrl+Click`: Add cursor at click position
  - `Ctrl+D`: Select next occurrence of current word
  - `Ctrl+Shift+L`: Select all occurrences
  - `Ctrl+Shift+Up/Down`: Add cursor above/below

#### Advanced Selection
- **Multiple Selection Types**:
  - Character-based selection
  - Word selection (double-click)
  - Line selection (triple-click)
  - Rectangular/column selection
- **Selection Modes**:
  - Continuous selection (click and drag)
  - Incremental selection (Shift+arrow keys)
  - Word-at-a-time selection (Ctrl+Shift+arrows)
  - Extend to line boundaries
- **Smart Selection**: Automatically expands to word/line boundaries

#### Undo/Redo System
- **Per-Document History**: Each file maintains independent undo stack
- **Configurable Stack Size**: Default 10,000 operations per document
- **Time-Based Merging**: Groups rapid edits within 0.3 seconds
- **Full State Preservation**: Restores text and cursor positions
- **Memory Efficient**: Stores deltas, not full document states

### 2.2 Syntax Highlighting

#### Intelligent Tokenization
- **Incremental Parsing**: Only tokenizes visible content + buffer
- **State-Based**: Maintains parsing state across lines
- **Multi-Language Support**: 50+ built-in language definitions
- **Performance Optimized**: 0.5ms timeout per frame to maintain 60 FPS
- **Nested Syntax**: Supports embedded languages (e.g., JavaScript in HTML)

#### Token Categories
- Keywords (primary and secondary)
- Strings and string interpolation
- Numbers (integers, floats, hex, binary)
- Comments (line and block)
- Functions and methods
- Operators and punctuation
- Symbols and identifiers
- Literals (true, false, null)

#### Pattern Matching
- **Lua Patterns**: Fast, simple pattern matching
- **PCRE2 Regex**: Full Perl-compatible regular expressions
- **Custom Delimiters**: Define language-specific string/comment syntax
- **Escape Sequences**: Proper handling of escape characters

### 2.3 Code Intelligence

#### Autocomplete System
- **Symbol Extraction**: Automatically builds symbol cache from open documents
- **Scope Control**:
  - **Global**: All open documents
  - **Local**: Current document only
  - **Related**: Documents with same file type
  - **None**: Manual symbols only
- **Fuzzy Matching**: Intelligent substring matching
- **Configurable Trigger**: Minimum 1-5 characters (default: 3)
- **Rich Suggestions**:
  - Symbol name
  - Icon/type indicator
  - Description/documentation
  - Keyboard shortcuts shown in tooltips
- **Callbacks**: `onhover`, `onselect` for plugin extensions
- **Performance Limits**: Max 4,000 symbols cached per document

#### Smart Indentation
- **Auto-Detection**: Analyzes file to detect tabs vs. spaces
- **Configurable Defaults**:
  - Indent size: 2-8 spaces (default: 2)
  - Tab type: soft (spaces) or hard (tabs)
- **Language-Aware**: Respects language-specific indentation rules
- **Indent Guides**: Visual indicators for indentation levels (via plugin)

#### Bracket Matching
- **Visual Indicators**: Underlines matching bracket pairs
- **Supported Brackets**: `()`, `[]`, `{}`, `<>`
- **Smart Navigation**: Jump to matching bracket
- **Rainbow Parentheses**: Color-coded nested brackets (via plugin)

### 2.4 Command System

#### Command Palette
- **Fuzzy Search**: Find commands by typing partial names
- **Keyboard-Centric**: Access all functionality without mouse
- **Context-Aware**: Shows only applicable commands
- **Categorized**: Organized by function (file, edit, view, etc.)
- **Maximum Visible**: 10 commands shown simultaneously (configurable)
- **Recent Commands**: Prioritizes frequently used commands

#### Command Categories
- **Core**: Application-level commands (quit, reload, etc.)
- **File Operations**: New, open, save, close, rename
- **Document Editing**: Undo, redo, cut, copy, paste, select all
- **Search/Replace**: Find, find next, replace, replace all
- **View Management**: Split, close view, switch view
- **Navigation**: Go to line, go to definition, find file
- **Project**: Add directory, open project folder
- **Window**: Toggle fullscreen, toggle sidebar

#### Context Menus
- **Right-Click Menus**: Context-sensitive options
- **View-Specific Items**: Different menus per view type
- **Keyboard Navigation**: Arrow keys and Enter
- **Extensible**: Plugins can add menu items

---

## 3. Editor Capabilities

### 3.1 Text Manipulation

#### Basic Operations
- **Insert/Delete**: Character, word, line, selection
- **Cut/Copy/Paste**: Standard clipboard operations
- **Duplicate**: Duplicate line or selection
- **Move Lines**: Move line(s) up or down
- **Join Lines**: Merge current line with next
- **Split Lines**: Break line at cursor

#### Advanced Transformations
- **Case Conversion**: Upper, lower, title case
- **Sort Lines**: Alphabetical sorting of selected lines
- **Indentation**: Indent/outdent single or multiple lines
- **Reflow Paragraphs**: Rewrap text to line width (via plugin)
- **Tabularize**: Align columns by delimiter (via plugin)
- **Quote/Unquote**: Toggle quotation marks (via plugin)

#### Whitespace Management
- **Trim Trailing**: Remove trailing whitespace
- **Trim Leading**: Remove leading whitespace
- **Normalize Whitespace**: Convert multiple spaces to single
- **Draw Whitespace**: Visualize spaces, tabs, line endings (via plugin)
- **Preserve on Newline**: Optionally keep trailing whitespace

#### Line Endings
- **Auto-Detection**: Detects existing line ending format
- **Conversion**: Convert between CRLF (Windows) and LF (Unix)
- **Default Setting**: Configurable per-platform (Windows: CRLF, Others: LF)
- **Mixed Line Endings**: Handles files with inconsistent endings

### 3.2 Macro Recording

#### Recording Capabilities
- **Keystroke Recording**: Captures all keyboard input
- **Playback**: Replay recorded actions
- **Simple Interface**: Start/stop/play commands
- **Single Macro Storage**: One macro at a time
- **Use Cases**: Repetitive editing tasks, bulk transformations

### 3.3 Find and Replace

#### Search Features
- **Plain Text Search**: Fast literal string matching
- **Regular Expressions**: Full PCRE2 regex support
- **Case Sensitivity**: Toggle case-sensitive/insensitive
- **Whole Word**: Match complete words only
- **Incremental Search**: Live preview while typing
- **Search Direction**: Forward and backward
- **Wrap Around**: Continue from beginning/end

#### Replace Operations
- **Single Replace**: Replace current match
- **Replace All**: Replace all matches in document
- **Interactive Replace**: Confirm each replacement
- **Undo Support**: Full undo for all replace operations
- **Multi-File Replace**: Via project search (see §5.3)

#### Search History
- **Recent Searches**: Remember previous search terms
- **Recent Replacements**: Remember replacement strings
- **Persistent**: Saved across sessions

### 3.4 Line Wrapping

#### Soft Wrap
- **Visual Wrapping**: Wrap long lines without modifying file
- **Configurable Width**: Wrap at window edge or custom column
- **Indent Continuation**: Indent wrapped portions
- **Performance Optimized**: Minimal impact on scrolling
- **Per-Document**: Enable/disable per file

#### Hard Wrap
- **Insert Line Breaks**: Actually modify file content
- **Reflow Paragraphs**: Intelligently rewrap paragraph text
- **Preserve Structure**: Respects blank lines and indentation

---

## 4. User Interface

### 4.1 Window Management

#### Main Window
- **Borderless Mode**: Custom window decorations (optional)
- **Fullscreen Support**: Distraction-free editing
- **Window Persistence**: Remembers size and position
- **Multi-Window**: Support for multiple editor windows (upcoming)
- **Retina Display**: High-DPI support on macOS
- **HiDPI Scaling**: Automatic scaling on Windows and Linux

#### Tab System
- **Multiple Tabs**: Unlimited tabs per window
- **Tab Navigation**:
  - `Ctrl+Tab`: Next tab
  - `Ctrl+Shift+Tab`: Previous tab
  - `Alt+1-9`: Jump to tab number
- **Tab Dragging**: Reorder tabs by dragging
- **Tab Closing**: Close button or keyboard shortcuts
- **Max Visible Tabs**: Shows 8 tabs by default, scrolls for more
- **Always Show Tabs**: Option to show tab bar with single tab
- **Tab Close Button**: Optional close buttons (configurable)

#### Split Views
- **Flexible Splits**: Unlimited horizontal and vertical splits
- **Split Commands**:
  - `Alt+Shift+I`: Split up
  - `Alt+Shift+K`: Split down
  - `Alt+Shift+J`: Split left
  - `Alt+Shift+L`: Split right
- **View Navigation**:
  - `Alt+I/K/J/L`: Navigate between splits
- **Resizable**: Drag dividers to resize splits
- **Tab Per Split**: Each split can have multiple tabs
- **Drag-and-Drop**: Drag tabs between splits

### 4.2 Sidebar Components

#### TreeView (File Browser)
- **Features**:
  - Hierarchical file/folder display
  - Expand/collapse folders
  - Show/hide hidden files
  - Show/hide ignored files
  - File type icons
  - Focus on active file (optional)
  - Auto-scroll to active file (optional)
- **Configuration**:
  - Default width: 200px (customizable)
  - Highlight focused file: enabled
  - Show hidden files: disabled by default
  - Show ignored files: enabled by default
- **Operations**:
  - Click to open file
  - Double-click to expand folder
  - Right-click for context menu
  - Create new files/folders
  - Rename files/folders
  - Delete files/folders
  - Drag to resize
- **Real-Time Updates**: Auto-refreshes on external changes

#### Terminal Plugin
- **Integrated Terminal**: Full terminal emulator within editor
- **ANSI Support**: Color codes, cursor positioning, text formatting
- **Multiple Terminals**: Create multiple terminal instances
- **Split Integration**: Can be placed in any split
- **Command Execution**: Run shell commands directly
- **Working Directory**: Inherits project directory

### 4.3 Status Bar

#### Left Section
- **File Information**:
  - File name and path
  - Modified indicator
  - Read-only indicator
  - Line ending type (CRLF/LF)
  - File encoding

#### Right Section
- **Cursor Position**: Line and column numbers
- **Selection Info**: Character/line count when selecting
- **File Type**: Detected syntax/language
- **Indent Info**: Tab type and size
- **Git Branch**: Current branch (via plugin)

#### Messages
- **Temporary Messages**: Auto-dismiss after 5 seconds (configurable)
- **Tooltips**: Hover information
- **Status Updates**: Plugin notifications

### 4.4 Visual Customization

#### Color Schemes
- **Built-in Themes**:
  - Default (dark)
  - Summer
  - Fall
  - Textadept
- **Extended Themes**: 50+ themes via lite-xl-colors repository
- **Easy Switching**: Change themes without restart
- **Custom Themes**: Create your own in Lua

#### Font Configuration
- **UI Fonts**: FiraSans (default)
- **Code Fonts**: JetBrains Mono (default)
- **Icon Fonts**: Custom icon font for UI elements
- **Font Sizes**: Fully customizable
- **Font Rendering Options**:
  - Hinting: none, slight, full
  - Anti-aliasing: grayscale, subpixel
- **Per-Token Fonts**: Different fonts for different syntax elements

#### Visual Effects
- **Smooth Animations**: 60 FPS rendering
- **Configurable Transitions**:
  - Scrolling
  - Command view suggestions
  - Context menu show/hide
  - Log view navigation
  - Status bar notifications
  - Tab scrolling and dragging
- **Animation Speed**: Adjustable rate (default: 1.0x)
- **Disable Animations**: Global or per-feature toggle
- **Cursor Blinking**: Configurable period (0.8s default) or disabled

#### UI Scaling
- **Global Scale**: Scale entire UI (via plugin)
- **Keyboard Shortcuts**:
  - `Ctrl+ScrollWheel`: Zoom in/out
  - `Ctrl+0`: Reset zoom
- **Font Scaling**: Scale fonts independently
- **Retina Support**: Automatic HiDPI handling

### 4.5 Line Display

#### Line Numbers
- **Gutter Display**: Line numbers in left margin
- **Relative Numbers**: Show distance from cursor (via plugin)
- **Current Line**: Highlighted current line number
- **Color Customization**: Via theme files

#### Line Guides
- **Column Guide**: Vertical line at column limit (default: 80)
- **Multiple Guides**: Support for multiple column markers (via plugin)
- **Indentation Guides**: Vertical lines showing indent levels (via plugin)
- **Current Line Highlight**: Full-width background highlight (optional)

#### Minimap
- **Document Overview**: Miniature view of entire document (via plugin)
- **Syntax Colors**: Shows syntax highlighting
- **Viewport Indicator**: Shows visible portion
- **Click to Navigate**: Click minimap to jump to location
- **Configurable Width**: Adjustable minimap size

---

## 5. File and Project Management

### 5.1 File Operations

#### Opening Files
- **Methods**:
  - Command palette: `Ctrl+O`
  - Drag-and-drop to window
  - Command-line arguments
  - TreeView file browser
  - Fuzzy file finder: `Ctrl+P`
  - System file picker (optional)
- **Multiple Files**: Open multiple files simultaneously
- **Large File Handling**:
  - Size limit: 10 MB (configurable)
  - Warning for files exceeding limit
  - Option to open anyway

#### Saving Files
- **Save**: `Ctrl+S` - Save current file
- **Save As**: `Ctrl+Shift+S` - Save with new name
- **Auto-Save**: Automatic periodic saving (via plugin)
- **Save All**: Save all modified files
- **Modified Indicator**: Visual indicator for unsaved changes
- **Save on Focus Loss**: Optional auto-save when switching away

#### File Monitoring
- **Auto-Reload**: Detect external changes and prompt to reload
- **Real-Time Watching**: Uses OS-native file watching
- **Conflict Detection**: Warns if file modified externally while editing
- **Directory Watching**: Monitors entire project structure

#### File Types
- **Auto-Detection**: Detects file type by extension and shebang
- **Manual Override**: Manually set syntax highlighting
- **Custom Associations**: Define custom file type patterns
- **Unknown Files**: Defaults to plain text

### 5.2 Project Management

#### Multi-Root Projects
- **Multiple Directories**: Open multiple project roots simultaneously
- **Primary Project**: First directory is primary
- **Add/Remove Roots**: Dynamically manage project directories
- **Per-Root Configuration**: `.lite_project.lua` for project-specific settings

#### Project Files
- **Ignore Patterns**: Configurable file/folder exclusions
- **Default Ignores**:
  - Version control: `.git/`, `.svn/`, `.hg/`, `CVS/`
  - Dependencies: `node_modules/`, `__pycache__/`, `.cache/`
  - Binaries: `.exe`, `.dll`, `.so`, `.dylib`, `.o`, `.a`
  - IDE files: `.suo`, `.pdb`, `.idb`, `.class`
  - System files: `.DS_Store`, `desktop.ini`, `.directory`
- **Custom Patterns**: Define project-specific ignore rules
- **Size Filtering**: Exclude files above size threshold

#### Session Management
- **Session Persistence**: Remembers open files and window state
- **Recent Projects**: Quick access to recently opened projects
- **Workspace Plugin**: Advanced session management
  - Saves open files per project
  - Restores cursor positions
  - Preserves split layout
  - Multiple workspace slots per project

### 5.3 File Navigation

#### Fuzzy File Finder
- **Quick Open**: `Ctrl+P` to search all project files
- **Fuzzy Matching**: Type partial filename to find matches
- **Keyboard Navigation**: Arrow keys to select, Enter to open
- **Real-Time Filtering**: Updates as you type
- **Case-Insensitive**: Matches regardless of case
- **Path Display**: Shows relative path for disambiguation

#### Recent Files
- **File History**: Access recently opened files
- **Quick Switch**: Jump between recent files
- **Persistent**: Saved across sessions

---

## 6. Search and Navigation

### 6.1 Document Search

#### Find in File
- **Activation**: `Ctrl+F`
- **Features**:
  - Incremental search (live preview)
  - Highlight all matches
  - Match count display
  - Wrap around option
- **Navigation**:
  - `F3` / `Ctrl+G`: Find next
  - `Shift+F3`: Find previous
  - `Ctrl+D`: Select next occurrence
  - `Ctrl+Shift+L`: Select all occurrences

#### Replace in File
- **Activation**: `Ctrl+R`
- **Operations**:
  - Replace current match
  - Replace all matches
  - Replace with confirmation
- **Preview**: Shows replacement before applying
- **Undo**: Full undo support for replacements

### 6.2 Project-Wide Search

#### Project Search
- **Activation**: `Ctrl+Shift+F`
- **Features**:
  - Search all files in project
  - Regular expression support
  - Case sensitivity toggle
  - File type filtering
  - Ignore file support
- **Results View**:
  - Live results as files are scanned
  - Click to jump to match
  - Line preview with context
  - Match count per file
  - Horizontal scrolling for long lines
  - Ellipsis for truncated content
- **Performance**:
  - Coroutine-based (non-blocking)
  - Incremental display
  - Cancellable search

#### Find File
- **Fuzzy Search**: Find files by name pattern
- **Path Matching**: Match against full path
- **Real-Time Updates**: Instant results
- **Keyboard-Only**: Full keyboard navigation

### 6.3 Code Navigation

#### Go to Line
- **Command**: `Ctrl+L`
- **Features**:
  - Jump to specific line number
  - Line:column syntax support
  - Validates input
  - Centers line in view

#### Go to Definition
- **LSP Support**: Via Language Server Protocol plugins
- **Symbol Navigation**: Jump to function/class definitions
- **Cross-File**: Navigate across project files

#### Symbol Navigation
- **Document Symbols**: Browse symbols in current file
- **Workspace Symbols**: Search symbols across project
- **Symbol Types**: Functions, classes, variables, constants

#### Breadcrumbs
- **File Path**: Shows current file location
- **Symbol Context**: Current function/class (via plugins)

---

## 7. Customization and Configuration

### 7.1 Configuration Files

#### User Directory Structure
```
~/.config/lite-xl/           (Linux/macOS)
C:\Users\<name>\.config\lite-xl\  (Windows)
├── init.lua                 # User configuration
├── plugins/                 # User plugins
│   ├── plugin1.lua
│   └── plugin2/
│       └── init.lua
├── colors/                  # Custom color schemes
│   └── mytheme.lua
└── session.lua              # Auto-saved session
```

#### Project Configuration
- **Location**: `.lite_project.lua` in project root
- **Scope**: Applies only to specific project
- **Priority**: Loaded after user config
- **Use Cases**:
  - Project-specific ignore patterns
  - Custom build commands
  - Team coding standards

### 7.2 Configuration Options

#### Performance Settings
- **FPS Limit**: Frame rate (default: 60)
- **Max Log Items**: Log retention (default: 800)
- **Max Undos**: Undo history per document (default: 10,000)
- **Max Symbols**: Autocomplete cache size (default: 4,000)

#### Editor Behavior
- **Symbol Pattern**: Regex for symbol matching
- **Non-Word Characters**: Word boundary definition
- **Undo Merge Timeout**: Time window for merging edits (default: 0.3s)
- **Indent Size**: Spaces per level (default: 2)
- **Tab Type**: Soft (spaces) or hard (tabs)
- **Keep Newline Whitespace**: Preserve trailing whitespace
- **Line Endings**: CRLF or LF
- **Line Limit**: Column guide position (default: 80)

#### UI Settings
- **Message Timeout**: Status message duration (default: 5s)
- **Mouse Wheel Scroll**: Scroll distance (default: 50px)
- **Animate Drag Scroll**: Scrollbar inertia
- **Scroll Past End**: Allow scrolling beyond document
- **Scrollbar Status**: Always expanded/contracted/auto
- **Max Tabs**: Maximum visible tabs (default: 8)
- **Max Visible Commands**: Command palette entries (default: 10)
- **Always Show Tabs**: Show tab bar with single tab
- **Highlight Current Line**: Current line background
- **Line Height**: Line spacing multiplier (default: 1.2)
- **Tab Close Button**: Show/hide close buttons

#### Visual Effects
- **Transitions**: Master toggle for all animations
- **Per-Feature Transitions**: Individual animation controls
- **Animation Rate**: Speed multiplier (default: 1.0)
- **Blink Period**: Cursor blink rate (default: 0.8s)
- **Disable Blink**: Turn off cursor blinking

#### Advanced
- **Borderless**: Custom window decorations
- **Max Clicks**: Multi-click recognition (default: 3)
- **Skip Plugin Version Check**: Disable version validation
- **Use System File Picker**: Native file dialogs

### 7.3 Keybinding Customization

#### Keymap Modification
- **Custom Bindings**: Override default shortcuts
- **Multi-Key Sequences**: Support for key chords
- **Modifier Keys**: Ctrl, Alt, Shift, Cmd (macOS)
- **Context-Aware**: Different bindings per view type
- **Conflict Detection**: Warns about conflicting bindings

#### Platform-Specific
- **macOS**: Uses Cmd instead of Ctrl where appropriate
- **Windows**: Includes AltGr support
- **Linux**: Standard Ctrl/Alt/Shift modifiers

#### Mouse Bindings
- **Click Bindings**: Single, double, triple-click
- **Modifier+Click**: Ctrl+click, Shift+click, etc.
- **Scroll Bindings**: Wheel, horizontal scroll
- **Drag Bindings**: Selection, tab dragging

### 7.4 Style Customization

#### Dimensions
- **Divider Size**: Split separator thickness
- **Scrollbar Sizes**: Normal and expanded widths
- **Minimum Thumb Size**: Smallest scrollbar handle
- **Caret Width**: Cursor thickness
- **Tab Width**: Maximum tab button width
- **Padding**: Element spacing (x and y)

#### Colors
- **Background**: Window and editor background
- **Text**: Default text color
- **Caret**: Cursor color
- **Selection**: Selection background
- **Line Number**: Gutter text
- **Line Number2**: Current line number
- **Line Highlight**: Current line background
- **Scrollbar**: Scrollbar colors
- **Divider**: Split separator
- **Syntax Colors**: Per-token type colors

---

## 8. Plugin System

### 8.1 Architecture

#### Plugin Loading
- **Discovery**: Scans `plugins/` directories
- **Version Matching**: Checks compatibility with editor version
- **Priority System**: Controls load order (lower number = earlier)
- **Load Locations**:
  1. `$USERDIR/init.lua` (priority: -2)
  2. `.lite_project.lua` (priority: -1)
  3. `$DATADIR/plugins/*` (priority: 100)
  4. `$USERDIR/plugins/*` (overrides system plugins)

#### Plugin Structure
```lua
-- mod-version:4
-- priority: 100

local core = require "core"
local config = require "core.config"
local command = require "core.command"

-- Plugin configuration
config.plugins.myplugin = {
  enabled = true,
  option1 = "value",
  option2 = 123
}

-- Plugin implementation
-- ...

return plugin_object
```

#### Version Management
- **Version Declaration**: `-- mod-version:MAJOR.MINOR.PATCH`
- **Compatibility Check**: Must match editor's MOD_VERSION
- **Version Mismatch**: Plugin refused unless check disabled
- **Skip Version Check**: `config.skip_plugins_version = true`

### 8.2 Plugin Capabilities

#### Core API Access
- **System API**: File I/O, clipboard, dialogs, process spawning
- **Renderer API**: Drawing primitives, fonts, colors
- **Document API**: Text manipulation, cursor control
- **View API**: UI components, event handling
- **Command API**: Add commands and keybindings
- **Syntax API**: Define language syntax
- **Process API**: Spawn and manage child processes
- **Regex API**: PCRE2 pattern matching

#### Extension Points
- **Commands**: Add new commands to command palette
- **Keybindings**: Define keyboard shortcuts
- **Views**: Create custom UI components
- **Syntax**: Add language support
- **Context Menus**: Add menu items
- **Status Bar**: Add status indicators
- **Document Hooks**: React to text changes
- **Event Handlers**: Respond to user input

#### Inter-Plugin Communication
- **Shared State**: Via Lua global tables
- **Module System**: Plugins can require each other
- **Events**: Callback-based communication
- **Service Registration**: Provide APIs for other plugins

### 8.3 Plugin Management

#### Installation Methods
1. **Manual**: Copy files to `~/.config/lite-xl/plugins/`
2. **Plugin Manager**: Use `lpm` command-line tool
3. **Git Submodules**: For version-controlled configs
4. **Package Managers**: Distribution-specific packages

#### Configuration
- **Per-Plugin Config**: `config.plugins.pluginname`
- **Enable/Disable**: `config.plugins.pluginname = false`
- **Runtime Configuration**: Some plugins configurable without restart
- **Persistent Settings**: Saved in user config

#### Updating
- **Manual Updates**: Replace plugin files
- **LPM Updates**: `lpm upgrade` for all plugins
- **Selective Updates**: Update individual plugins
- **Version Pinning**: Lock plugins to specific versions

---

## 9. Built-in Plugins

### 9.1 Editor Enhancement Plugins

#### Autocomplete
- **Symbol Caching**: Builds index of all symbols in open files
- **Fuzzy Matching**: Intelligent substring matching
- **Scope Control**: Global, local, related, or manual
- **Configurable**:
  - Minimum trigger length: 1-5 characters
  - Max suggestions: 100
  - Max visible height: 6 items
  - Description font size: 12pt
  - Icon position: left or right
  - Hide icons/info: optional
- **Rich Display**: Icons, descriptions, type information
- **Callbacks**: Custom hover and select handlers

#### DetectIndent
- **Auto-Detection**: Analyzes file to determine indent style
- **Detection Criteria**:
  - Tabs vs. spaces
  - Indent size (2, 4, 8 spaces)
- **File Patterns**: Configurable file type associations
- **Status Display**: Shows detected indent in status bar

#### Line Wrapping
- **Soft Wrap**: Visual wrapping without modifying file
- **Configurable Width**: Window edge or custom column
- **Indent Continuation**: Wrapped lines indented
- **Toggle Command**: Enable/disable per document
- **Performance**: Optimized for minimal overhead

#### Line Guide
- **Column Marker**: Vertical line at specified column
- **Multiple Guides**: Support for multiple columns
- **Configurable**:
  - Column positions
  - Line color and width
  - Enable/disable per language

#### Draw Whitespace
- **Visual Indicators**:
  - Spaces: middle dot (·)
  - Tabs: right-pointing arrow (→)
  - Line endings: pilcrow (¶)
- **Configurable**:
  - Show/hide each type
  - Custom colors
  - Custom characters
- **Performance**: Minimal rendering overhead

#### Trim Whitespace
- **Auto-Trim**: Remove trailing whitespace on save
- **Manual Trim**: Command to trim current file
- **Configurable**:
  - Enable/disable auto-trim
  - File type exclusions
  - Preserve specific patterns

### 9.2 File Management Plugins

#### TreeView
- **File Browser**: Hierarchical file/folder display
- **Features**:
  - Expand/collapse folders
  - Show/hide hidden files
  - Show/hide ignored files
  - Focus on active file
  - Auto-scroll to active file
  - File type icons
  - Drag to resize
- **Operations**:
  - Create files/folders
  - Rename files/folders
  - Delete files/folders
  - Open files
- **Real-Time Updates**: Auto-refreshes on file system changes
- **Configuration**:
  - Default width: 200px
  - Highlight focused file: yes
  - Expand dirs to focused: no
  - Scroll to focused: no
  - Animate scroll: yes

#### Workspace
- **Session Management**: Save and restore project state
- **Per-Project Sessions**: Multiple workspace slots per project
- **Saved State**:
  - Open files and tabs
  - Cursor positions and selections
  - Split layout
  - Scroll positions
- **Auto-Load**: Restores workspace on project open
- **Manual Control**: Commands to save/load workspaces

#### FindFile
- **Fuzzy Finder**: Quick file opening by name
- **Activation**: `Ctrl+P`
- **Features**:
  - Real-time filtering
  - Path display
  - Keyboard navigation
  - Case-insensitive matching

### 9.3 Search and Navigation Plugins

#### ProjectSearch
- **Multi-File Search**: Search across entire project
- **Features**:
  - Regular expression support
  - Case sensitivity toggle
  - File type filtering
  - Ignore file support
- **Results Display**:
  - Live incremental results
  - Click to jump to match
  - Line preview with context
  - Match counts
  - Horizontal scrolling
- **Performance**:
  - Non-blocking coroutine-based search
  - Cancellable
  - Handles large projects

### 9.4 Editing Tool Plugins

#### Macro
- **Record**: Capture keystroke sequences
- **Playback**: Replay recorded actions
- **Commands**:
  - Start recording
  - Stop recording
  - Play macro
- **Storage**: Single macro slot
- **Use Cases**: Repetitive edits, bulk changes

#### Quote
- **Toggle Quotes**: Cycle through quote types
- **Supported**: Single quotes, double quotes, backticks
- **Smart**: Preserves inner quotes
- **Selection-Aware**: Works on selected text

#### Reflow
- **Paragraph Reformatting**: Rewrap text to line width
- **Smart Wrapping**: Respects sentence boundaries
- **Indentation**: Preserves paragraph indentation
- **Use Cases**: Comments, documentation, prose

#### Tabularize
- **Column Alignment**: Align text by delimiter
- **Configurable Delimiter**: Any character or pattern
- **Multiple Lines**: Works on selected lines
- **Padding**: Adjustable spacing
- **Use Cases**: ASCII tables, aligned assignments

### 9.5 Appearance Plugins

#### Scale
- **UI Scaling**: Zoom entire interface
- **Keybindings**:
  - `Ctrl+Scroll`: Zoom in/out
  - `Ctrl+0`: Reset zoom
  - `Ctrl+=/-`: Zoom in/out
- **Font Scaling**: Scales all fonts proportionally
- **Persistent**: Saved across sessions

#### ToolbarView
- **Custom Toolbar**: Add buttons to toolbar
- **Features**:
  - Icon buttons
  - Command execution
  - Tooltips
  - Keyboard shortcuts shown
- **Configurable**: Add/remove buttons
- **Extensible**: Plugins can add toolbar items

### 9.6 System Integration Plugins

#### AutoReload
- **File Watching**: Monitors open files for external changes
- **Auto-Reload**: Automatically reloads changed files
- **Prompt Mode**: Ask before reloading
- **Conflict Detection**: Warns if file has unsaved changes
- **Per-File**: Independent monitoring per document

#### AutoRestart
- **Crash Recovery**: Automatically restarts editor on crash
- **Session Preservation**: Saves session before restart
- **Logging**: Records crash information
- **Configurable**: Enable/disable auto-restart

### 9.7 Terminal Plugin

#### Integrated Terminal
- **Full Terminal**: Complete terminal emulator
- **ANSI Support**:
  - Color codes (16 colors + 256 color mode)
  - Cursor positioning
  - Text formatting (bold, italic, underline)
  - Clear screen, clear line
- **Features**:
  - Multiple terminal instances
  - Tab integration
  - Split view compatible
  - Scrollback buffer
  - Copy/paste support
- **Shell Integration**: Works with bash, zsh, fish, PowerShell
- **Working Directory**: Inherits from project

### 9.8 Language Support Plugins

#### Built-in Languages
- **C** (`language_c.lua`): C syntax with preprocessor support
- **C++** (`language_cpp.lua`): C++ with templates and STL
- **CSS** (`language_css.lua`): CSS3 properties and selectors
- **HTML** (`language_html.lua`): HTML5 tags and attributes
- **JavaScript** (`language_js.lua`): ES6+ syntax
- **Lua** (`language_lua.lua`): Lua 5.1-5.4 syntax
- **Markdown** (`language_md.lua`): CommonMark + extensions
- **Python** (`language_python.lua`): Python 3 syntax
- **XML** (`language_xml.lua`): XML/XHTML syntax

---

## 10. Extended Plugin Ecosystem

### 10.1 Plugin Categories

The lite-xl-plugins repository (https://github.com/lite-xl/lite-xl-plugins) contains 100+ additional plugins organized into categories:

#### Editing & Text Manipulation
- **align_carets**: Align multiple carets and selections
- **autoinsert**: Auto-insert closing brackets, quotes, tags
- **bracketmatch**: Highlight matching brackets
- **editorconfig**: EditorConfig support
- **ephemeral_tabs**: Preview files without opening
- **indentguide**: Show indent guides
- **rainbowparen**: Rainbow-colored nested parentheses
- **selectionhighlight**: Highlight all instances of selection
- **sort**: Sort selected lines
- **textwrap**: Intelligent text wrapping

#### Navigation & Search
- **navigate**: Navigate forward/backward in jump history
- **markers**: Bookmark locations in files
- **tab_switcher**: Fuzzy search through open tabs
- **centerdoc**: Center document in view
- **gofmt**: Go code formatting
- **bracketmatch**: Jump to matching bracket

#### Visual Enhancements
- **minimap**: Document overview minimap
- **indentguide**: Visual indent guides
- **motiontrail**: Animated cursor trail
- **colorpicker**: Color picker dialog
- **rainbowparen**: Rainbow parentheses
- **scroller**: Enhanced scrollbar
- **theme_switcher**: Quick theme switching UI
- **nonicons/nerdicons/devicons**: File icon packs

#### Language Server Protocol (LSP)
- **lsp**: Full LSP client implementation
  - Auto-completion
  - Go to definition
  - Find references
  - Hover documentation
  - Diagnostics (errors/warnings)
  - Code actions
  - Formatting
- **Language-specific LSP configs**: Pre-configured for popular languages

#### Development Tools
- **build**: Build system integration
  - Run build commands
  - Parse compiler errors
  - Jump to error locations
  - Configurable per project
- **debugger**: Debug adapter protocol support
  - Breakpoints
  - Step through code
  - Variable inspection
  - Call stack navigation
- **formatter**: Code formatting integration
- **linter**: Linting integration
- **exec**: Execute shell commands and capture output
- **console**: Interactive console/REPL

#### Source Control
- **gitdiff**: Show git diff in gutter
- **gitblame**: Show git blame information
- **gitstatus**: Git status indicators in TreeView
- **gitopen**: Open file in GitHub/GitLab
- **diffview**: Side-by-side diff viewer

#### File Management
- **extend-treeview**: Enhanced TreeView features
- **project_manager**: Manage multiple projects
- **openfilelocation**: Open file location in file manager
- **copyfilelocation**: Copy file path to clipboard
- **opener**: Open files with external programs

#### Language Support
Over 50 additional language syntax definitions:
- **Assembly**: NASM, GAS, MASM
- **Scripting**: Bash, PowerShell, Fish, AWK
- **Systems**: Rust, Go, Zig, V, Nim, Crystal
- **Web**: TypeScript, JSX, Vue, Svelte, SCSS, LESS
- **JVM**: Java, Kotlin, Scala, Groovy
- **Functional**: Haskell, OCaml, F#, Elm, Elixir
- **Data**: YAML, TOML, JSON, Protobuf, GraphQL
- **Markup**: reStructuredText, AsciiDoc, LaTeX
- **Config**: Dockerfile, Nginx, Apache
- **Database**: SQL, PLpgSQL, Cypher
- **Mobile**: Swift, Dart, Kotlin
- **Game Dev**: GLSL, HLSL, GDScript, Lua
- **Other**: R, Julia, MATLAB, Verilog, VHDL

#### Productivity
- **autosave**: Automatic file saving
- **autobackup**: Automatic backup creation
- **autorestore**: Restore unsaved changes
- **smartopenselected**: Open selected path/URL
- **bracketmatch**: Enhanced bracket operations
- **titlebar**: Custom title bar
- **workspace**: Advanced workspace management

#### Integration & Export
- **export-html**: Export syntax-highlighted HTML
- **latex**: LaTeX support and compilation
- **pdf**: PDF viewer integration
- **todotree**: TODO comment scanner
- **ctags**: Ctags integration
- **ghmarkdown**: GitHub-flavored Markdown preview

### 10.2 Plugin Manager (LPM)

#### Installation
```bash
# Install LPM itself
curl -L https://github.com/lite-xl/lite-xl-plugin-manager/releases/download/latest/lpm.linux-x86_64 -o lpm
chmod +x lpm
```

#### Commands
- `lpm install <plugin>`: Install plugin
- `lpm uninstall <plugin>`: Remove plugin
- `lpm upgrade`: Update all plugins
- `lpm list`: Show installed plugins
- `lpm search <term>`: Search for plugins
- `lpm info <plugin>`: Show plugin details

#### Features
- **Version Management**: Install specific plugin versions
- **Dependency Resolution**: Automatically installs dependencies
- **Repository Management**: Multiple plugin sources
- **Verification**: Checksum and signature verification
- **Offline Support**: Cache for offline installation

---

## 11. Language Support

### 11.1 Built-in Languages

#### C Language
- **Features**: Preprocessor directives, C99/C11 syntax
- **Highlighting**: Keywords, types, macros, strings, comments
- **Recognition**: `.c`, `.h` files

#### C++
- **Features**: Templates, STL, C++11/14/17/20 syntax
- **Highlighting**: All C features + C++ keywords, namespaces
- **Recognition**: `.cpp`, `.hpp`, `.cc`, `.cxx`, `.h++` files

#### CSS
- **Features**: CSS3 properties, selectors, at-rules
- **Highlighting**: Properties, values, colors, selectors
- **Recognition**: `.css` files

#### HTML
- **Features**: HTML5 tags, attributes
- **Nested Syntax**: CSS in `<style>`, JavaScript in `<script>`
- **Highlighting**: Tags, attributes, text content
- **Recognition**: `.html`, `.htm` files

#### JavaScript
- **Features**: ES6+ syntax, JSX support
- **Highlighting**: Keywords, literals, operators, regex
- **Recognition**: `.js`, `.jsx`, `.mjs` files

#### Lua
- **Features**: Lua 5.1-5.4 syntax
- **Highlighting**: Keywords, functions, strings, numbers
- **Recognition**: `.lua` files

#### Markdown
- **Features**: CommonMark + extensions
- **Highlighting**: Headers, bold, italic, code, links, lists
- **Recognition**: `.md`, `.markdown` files

#### Python
- **Features**: Python 3 syntax
- **Highlighting**: Keywords, decorators, f-strings, comments
- **Recognition**: `.py`, `.pyw` files, `#!/usr/bin/python`

#### XML
- **Features**: XML/XHTML syntax
- **Highlighting**: Tags, attributes, CDATA, comments
- **Recognition**: `.xml` files

### 11.2 Extended Languages (via Plugins)

Categories include:
- **Systems Programming**: Rust, Go, Zig, V, Nim, Crystal, D
- **Web Development**: TypeScript, JSX, Vue, Svelte, Angular, SCSS, LESS, Sass
- **JVM Languages**: Java, Kotlin, Scala, Groovy, Clojure
- **Functional Languages**: Haskell, OCaml, F#, Elm, Elixir, Erlang
- **Scripting**: Bash, PowerShell, Fish, Zsh, AWK, Perl, Ruby
- **Data Formats**: YAML, TOML, JSON, Protobuf, GraphQL, MessagePack
- **Markup**: reStructuredText, AsciiDoc, LaTeX, Org-mode
- **Configuration**: Dockerfile, Nginx, Apache, systemd
- **Database**: SQL, PLpgSQL, MySQL, PostgreSQL, Cypher
- **Mobile**: Swift, Objective-C, Dart, Kotlin
- **Game Development**: GLSL, HLSL, Cg, GDScript, Godot, Unity C#
- **Scientific**: R, Julia, MATLAB, Octave, Fortran
- **Hardware**: Verilog, VHDL, SystemVerilog
- **Other**: CMake, Make, Meson, Bazel, Assembly variants

### 11.3 Language Server Protocol Support

Via LSP plugin:
- **C/C++**: clangd, ccls
- **Python**: pyright, pylsp, jedi
- **JavaScript/TypeScript**: typescript-language-server
- **Rust**: rust-analyzer
- **Go**: gopls
- **Java**: jdtls
- **Many more**: Support for 40+ language servers

---

## 12. Performance Characteristics

### 12.1 Startup Performance

- **Cold Start**: ~50-150ms on modern hardware
- **Warm Start**: ~20-50ms with OS cache
- **Session Restoration**: +10-50ms depending on open files
- **Plugin Loading**: ~1-5ms per plugin
- **Binary Size**: ~10MB (varies by platform)

### 12.2 Runtime Performance

#### Rendering
- **Target FPS**: 60 FPS (configurable)
- **Frame Budget**: ~16.67ms per frame
- **Actual Performance**: Typically 60 FPS on modern hardware
- **Large Files**: Maintains 60 FPS up to 10,000+ lines
- **Scrolling**: Smooth with syntax highlighting

#### Syntax Highlighting
- **Incremental**: Only highlights visible lines + buffer
- **Timeout**: 0.5ms per frame to maintain FPS
- **Background Processing**: Uses coroutines for heavy files
- **Cache**: Line-by-line token caching
- **Large Files**: Graceful degradation on very large files

#### Memory Usage
- **Baseline**: ~20-50 MB with no files open
- **Per Document**: ~1-5 MB depending on size and complexity
- **Symbol Cache**: Limited to 4,000 symbols per document
- **Undo Stack**: Limited to 10,000 operations per document
- **Large Projects**: Efficient file enumeration

#### File Operations
- **Open File**: Instant for files < 1 MB
- **Save File**: Near-instant for most files
- **File Watching**: OS-native, minimal overhead
- **Project Scan**: Coroutine-based, non-blocking

### 12.3 Scalability

- **File Size**: Handles files up to 10 MB (configurable limit)
- **Project Size**: Tested with projects containing 10,000+ files
- **Open Files**: Practically unlimited (memory-bound)
- **Multiple Cursors**: Handles 100+ cursors efficiently
- **Long Lines**: Graceful handling of lines > 1,000 characters

---

## 13. Platform Support

### 13.1 Operating Systems

#### Linux
- **Distributions**: Ubuntu 18.04+, Fedora, Arch, Debian, Gentoo, etc.
- **Requirements**: glibc 2.27+
- **Display Servers**: X11, Wayland
- **Package Formats**: AppImage, tar.gz, distribution packages
- **Desktop Integration**: .desktop file, icon, MIME types

#### macOS
- **Versions**: macOS 10.11 (El Capitan) and later
- **Architecture**: x86_64, Apple Silicon (ARM64)
- **Retina Support**: Full HiDPI support
- **Distribution**: DMG installer, application bundle
- **Code Signing**: Self-signed (requires manual approval on first launch)

#### Windows
- **Versions**: Windows 7 and later (7, 8, 10, 11)
- **Architecture**: x86_64
- **Distribution**: ZIP archive, MSI installer
- **Portable Mode**: Create `user/` folder for portable installation
- **Integration**: File associations, context menu (optional)

#### FreeBSD
- **Support**: Community-maintained
- **Distribution**: Source build, ports system

#### SerenityOS
- **Support**: Experimental
- **Distribution**: Source build

### 13.2 Hardware Requirements

#### Minimum
- **CPU**: Any x86_64 processor (2010+)
- **RAM**: 256 MB
- **Storage**: 50 MB
- **Display**: 800x600, 16-bit color

#### Recommended
- **CPU**: Modern x86_64 (2015+)
- **RAM**: 512 MB+
- **Storage**: 100 MB (including plugins)
- **Display**: 1920x1080, 24-bit color

#### Optimal
- **CPU**: Multi-core x86_64 or ARM64
- **RAM**: 1 GB+
- **Storage**: SSD
- **Display**: HiDPI/Retina

---

## 14. Technical Architecture

### 14.1 Core Components

#### C Backend (`src/`)
- **main.c**: Application entry point, SDL initialization
- **renderer.c**: Font rendering, drawing primitives
- **rencache.c**: Command batching, render optimization
- **renwindow.c**: Window management, multi-window support
- **api/**: Lua C API bindings
  - `system.c`: OS interaction (files, clipboard, dialogs)
  - `renderer.c`: Drawing API
  - `process.c`: Child process management
  - `regex.c`: PCRE2 regular expressions
  - `utf8.c`: UTF-8 string handling

#### Lua Core (`data/core/`)
- **init.lua**: Core initialization, main loop
- **start.lua**: Bootstrap, environment setup
- **common.lua**: Utility functions
- **config.lua**: Configuration management
- **command.lua**: Command system
- **keymap.lua**: Keyboard binding system
- **view.lua**: Base view class
- **doc.lua**: Document model
- **docview.lua**: Document editor view
- **rootview.lua**: Layout manager
- **statusview.lua**: Status bar
- **commandview.lua**: Command palette
- **contextmenu.lua**: Context menu system
- **syntax.lua**: Syntax definition system
- **tokenizer.lua**: Syntax tokenizer
- **highlighter.lua**: Syntax highlighter
- **project.lua**: Project management

### 14.2 Dependencies

#### Required
- **SDL3**: Window management, input, rendering backend
- **Lua 5.4**: Scripting engine
- **PCRE2**: Regular expression engine
- **FreeType2**: Font rendering

#### Optional
- **Platform Libraries** (Linux):
  - libX11, libXi, libXcursor (X11 backend)
  - libxkbcommon (keyboard input)
  - wayland, wayland-protocols (Wayland backend)
  - dbus, ibus (input method support)

### 14.3 Build System

- **Build Tool**: Meson 0.63+
- **Backend**: Ninja
- **C Compiler**: GCC, Clang, or MSVC
- **Dependency Management**: Meson wraps (automatic download)
- **Build Profiles**: Debug, release, profile-guided optimization (PGO)

---

## 15. Accessibility Features

### 15.1 Keyboard Accessibility

- **Full Keyboard Control**: All features accessible via keyboard
- **Customizable Shortcuts**: Remap any keyboard shortcut
- **Command Palette**: Access all commands without memorizing shortcuts
- **Tab Navigation**: Navigate all UI elements with Tab key
- **Focus Indicators**: Clear visual focus indicators

### 15.2 Visual Accessibility

- **High Contrast Themes**: Multiple high-contrast color schemes
- **Customizable Colors**: Full control over all colors
- **Font Scaling**: Adjust all font sizes
- **UI Scaling**: Scale entire interface
- **Cursor Customization**: Adjustable cursor width and blink rate
- **No Cursor Blink**: Option to disable cursor blinking

### 15.3 Input Method Support

- **IME Support**: Full Input Method Editor support for CJK languages
- **Composition Preview**: Shows composition string while typing
- **Candidate Window**: Proper positioning of candidate selection
- **Multiple IMEs**: Works with system-configured IMEs

---

## 16. Integration Capabilities

### 16.1 Command-Line Interface

#### Arguments
- `lite-xl [files/directories]`: Open files or directories
- `lite-xl -v`: Show version information
- `lite-xl -h`: Show help
- Environment variables for customization

#### Exit Codes
- `0`: Normal exit
- `1`: Error

### 16.2 External Tool Integration

#### Version Control
- **Git**: GitDiff, GitBlame, GitStatus plugins
- **SVN, Mercurial**: Via exec plugin

#### Build Systems
- **Make, CMake, Meson, etc.**: Via build plugin
- **Custom Build Commands**: Configurable per project
- **Error Parsing**: Jump to compiler errors

#### Debuggers
- **Debug Adapter Protocol**: Via debugger plugin
- **GDB, LLDB**: Integration available

#### Formatters
- **clang-format, prettier, black, etc.**: Via formatter plugins
- **Auto-format**: On save or manual

#### Linters
- **ESLint, pylint, etc.**: Via linter plugins
- **Inline Diagnostics**: Errors/warnings in editor

### 16.3 File Format Support

#### Text Files
- **Encoding**: UTF-8 (primary), ASCII
- **Line Endings**: LF, CRLF, auto-detection
- **Large Files**: Up to 10 MB (configurable)

#### Binary Files
- **Exclusion**: Binary files excluded from project search
- **Detection**: Automatic binary file detection
- **Warning**: Warns before opening binary files

### 16.4 Clipboard Integration

- **Standard Operations**: Cut, copy, paste
- **System Clipboard**: Full integration with OS clipboard
- **Rich Text**: Preserves formatting when appropriate
- **Cross-Application**: Works with all applications

### 16.5 File System Integration

#### File Associations
- **Linux**: MIME type associations via .desktop file
- **macOS**: Document type associations in Info.plist
- **Windows**: File extension associations (optional)

#### Drag and Drop
- **Files**: Drag files to window to open
- **Directories**: Drag folders to add as project root
- **Text**: Drag text from other applications

#### External Changes
- **File Watching**: Detects external modifications
- **Auto-Reload**: Optional automatic reloading
- **Conflict Resolution**: Warns on save conflicts

---

## Conclusion

Lite XL is a comprehensive yet lightweight text editor that successfully balances simplicity with powerful features. Its extensible architecture, combined with a robust plugin ecosystem, makes it suitable for a wide range of use cases from quick text editing to full-scale software development.

### Key Strengths

1. **Performance**: Fast startup, smooth 60 FPS rendering, efficient resource usage
2. **Extensibility**: Powerful Lua-based plugin system with 100+ available plugins
3. **Simplicity**: Clean, distraction-free interface with sensible defaults
4. **Customization**: Deep customization through configuration files and plugins
5. **Cross-Platform**: Consistent experience across Windows, Linux, and macOS
6. **Open Source**: MIT licensed, community-driven, transparent development

### Ideal Use Cases

- Lightweight alternative to Electron-based editors (VS Code, Atom)
- Fast editor for remote/SSH development
- Minimalist development environment
- Learning platform for Lua programming
- Quick file editing and system administration
- Embedded systems and resource-constrained environments
- Custom editor development (fork and extend)

### Community and Support

- **GitHub**: https://github.com/lite-xl/lite-xl
- **Discord**: Active community for support and development
- **Documentation**: https://lite-xl.com
- **Plugins**: https://github.com/lite-xl/lite-xl-plugins
- **Colors**: https://github.com/lite-xl/lite-xl-colors

---

**Document Version**: 1.0
**Specification Coverage**: Lite XL 2.1.7
**Generated**: November 2025
