/// Terminal Plugin Core Components Demo
///
/// This example demonstrates the core data structures and functionality
/// of the terminal plugin without requiring UI or PTY dependencies.
///
/// Run with: cargo run --example terminal_core_demo

use std::collections::VecDeque;

// ============================================================================
// Cell and Grid Implementation
// ============================================================================

/// RGB Color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    const RED: Color = Color { r: 205, g: 49, b: 49 };
    const GREEN: Color = Color { r: 13, g: 188, b: 121 };
    const YELLOW: Color = Color { r: 229, g: 229, b: 16 };
    const BLUE: Color = Color { r: 36, g: 114, b: 200 };

    fn from_ansi_color(code: u8) -> Color {
        match code {
            0 => Color::BLACK,
            1 => Color::RED,
            2 => Color::GREEN,
            3 => Color::YELLOW,
            4 => Color::BLUE,
            7 => Color::WHITE,
            _ => Color::WHITE,
        }
    }
}

/// Cell attributes using bitflags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellAttributes(u8);

impl CellAttributes {
    const NONE: u8 = 0b0000_0000;
    const BOLD: u8 = 0b0000_0001;
    const DIM: u8 = 0b0000_0010;
    const ITALIC: u8 = 0b0000_0100;
    const UNDERLINE: u8 = 0b0000_1000;
    const REVERSE: u8 = 0b0010_0000;

    fn empty() -> Self {
        Self(Self::NONE)
    }

    fn contains(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }

    fn insert(&mut self, flag: u8) {
        self.0 |= flag;
    }

    fn remove(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    fn clear(&mut self) {
        self.0 = Self::NONE;
    }
}

/// A single terminal cell
#[derive(Debug, Clone, PartialEq)]
struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
    attrs: CellAttributes,
}

impl Cell {
    fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::WHITE,
            bg: Color::BLACK,
            attrs: CellAttributes::empty(),
        }
    }

    fn default() -> Self {
        Self::new(' ')
    }

    fn reset(&mut self) {
        self.ch = ' ';
        self.fg = Color::WHITE;
        self.bg = Color::BLACK;
        self.attrs = CellAttributes::empty();
    }

    fn with_foreground(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    fn with_background(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    fn with_bold(mut self) -> Self {
        self.attrs.insert(CellAttributes::BOLD);
        self
    }

    fn is_bold(&self) -> bool {
        self.attrs.contains(CellAttributes::BOLD)
    }

    fn is_underline(&self) -> bool {
        self.attrs.contains(CellAttributes::UNDERLINE)
    }
}

/// Terminal grid (2D array of cells)
struct Grid {
    cols: usize,
    rows: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(cols: usize, rows: usize) -> Self {
        let cells = vec![vec![Cell::default(); cols]; rows];
        Self { cols, rows, cells }
    }

    fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row).and_then(|r| r.get_mut(col))
    }

    fn set(&mut self, row: usize, col: usize, cell: Cell) {
        if let Some(target) = self.get_mut(row, col) {
            *target = cell;
        }
    }

    fn resize(&mut self, cols: usize, rows: usize) {
        self.cols = cols;
        self.rows = rows;
        self.cells.resize(rows, vec![Cell::default(); cols]);
        for row in &mut self.cells {
            row.resize(cols, Cell::default());
        }
    }

    fn scroll_up(&mut self, lines: usize) {
        for _ in 0..lines {
            if !self.cells.is_empty() {
                self.cells.remove(0);
                self.cells.push(vec![Cell::default(); self.cols]);
            }
        }
    }

    fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.reset();
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        if let Some(row_cells) = self.cells.get_mut(row) {
            for cell in row_cells {
                cell.reset();
            }
        }
    }

    /// Render grid to string (for debugging/testing)
    fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.ch).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Render grid with ANSI colors (for terminal output)
    fn to_ansi_string(&self) -> String {
        let mut result = String::new();

        for row in &self.cells {
            for cell in row {
                // Add color codes
                result.push_str(&format!(
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
                    cell.fg.r, cell.fg.g, cell.fg.b,
                    cell.bg.r, cell.bg.g, cell.bg.b
                ));

                // Add attributes
                if cell.is_bold() {
                    result.push_str("\x1b[1m");
                }
                if cell.is_underline() {
                    result.push_str("\x1b[4m");
                }

                // Add character
                result.push(cell.ch);

                // Reset
                result.push_str("\x1b[0m");
            }
            result.push('\n');
        }

        result
    }
}

// ============================================================================
// Scrollback Buffer
// ============================================================================

struct Scrollback {
    max_lines: usize,
    lines: VecDeque<Vec<Cell>>,
    scroll_offset: usize,
}

impl Scrollback {
    fn new(max_lines: usize) -> Self {
        Self {
            max_lines,
            lines: VecDeque::with_capacity(max_lines),
            scroll_offset: 0,
        }
    }

    fn push_line(&mut self, line: Vec<Cell>) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    fn get_line(&self, index: usize) -> Option<&Vec<Cell>> {
        self.lines.get(index)
    }

    fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = (self.scroll_offset + lines).min(self.lines.len());
    }

    fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    fn is_at_bottom(&self) -> bool {
        self.scroll_offset == 0
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    fn len(&self) -> usize {
        self.lines.len()
    }
}

// ============================================================================
// Cursor
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum CursorStyle {
    Block,
    Underline,
    Bar,
}

#[derive(Debug, Clone, Copy)]
struct Cursor {
    row: usize,
    col: usize,
    visible: bool,
    style: CursorStyle,
}

impl Cursor {
    fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            visible: true,
            style: CursorStyle::Block,
        }
    }

    fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn move_up(&mut self, n: usize) {
        self.row = self.row.saturating_sub(n);
    }

    fn move_down(&mut self, n: usize, max_row: usize) {
        self.row = (self.row + n).min(max_row);
    }

    fn move_forward(&mut self, n: usize, max_col: usize) {
        self.col = (self.col + n).min(max_col);
    }

    fn move_backward(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }

    fn carriage_return(&mut self) {
        self.col = 0;
    }

    fn newline(&mut self, max_row: usize) {
        self.row = (self.row + 1).min(max_row);
    }
}

// ============================================================================
// Simple ANSI Parser (subset)
// ============================================================================

#[derive(Debug, Clone)]
enum AnsiAction {
    Print(char),
    MoveCursor { row: usize, col: usize },
    ClearScreen,
    ClearLine,
    SetForeground(Color),
    SetBackground(Color),
    SetBold,
    Reset,
    CarriageReturn,
    Newline,
}

struct SimpleAnsiParser {
    state: ParserState,
    params: Vec<u8>,
    current_fg: Color,
    current_bg: Color,
    current_bold: bool,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    Normal,
    Escape,
    Csi,
}

impl SimpleAnsiParser {
    fn new() -> Self {
        Self {
            state: ParserState::Normal,
            params: Vec::new(),
            current_fg: Color::WHITE,
            current_bg: Color::BLACK,
            current_bold: false,
        }
    }

    fn parse(&mut self, byte: u8) -> Option<AnsiAction> {
        match self.state {
            ParserState::Normal => {
                match byte {
                    b'\x1b' => {
                        self.state = ParserState::Escape;
                        None
                    }
                    b'\r' => Some(AnsiAction::CarriageReturn),
                    b'\n' => Some(AnsiAction::Newline),
                    ch if ch >= 32 && ch < 127 => Some(AnsiAction::Print(ch as char)),
                    _ => None,
                }
            }
            ParserState::Escape => {
                match byte {
                    b'[' => {
                        self.state = ParserState::Csi;
                        self.params.clear();
                        None
                    }
                    _ => {
                        self.state = ParserState::Normal;
                        None
                    }
                }
            }
            ParserState::Csi => {
                match byte {
                    b'0'..=b'9' => {
                        self.params.push(byte - b'0');
                        None
                    }
                    b';' => {
                        // Parameter separator
                        None
                    }
                    b'H' => {
                        // Cursor position
                        self.state = ParserState::Normal;
                        Some(AnsiAction::MoveCursor { row: 0, col: 0 })
                    }
                    b'J' => {
                        // Clear screen
                        self.state = ParserState::Normal;
                        Some(AnsiAction::ClearScreen)
                    }
                    b'K' => {
                        // Clear line
                        self.state = ParserState::Normal;
                        Some(AnsiAction::ClearLine)
                    }
                    b'm' => {
                        // SGR (colors/attributes)
                        self.state = ParserState::Normal;
                        self.parse_sgr()
                    }
                    _ => {
                        self.state = ParserState::Normal;
                        None
                    }
                }
            }
        }
    }

    fn parse_sgr(&mut self) -> Option<AnsiAction> {
        if self.params.is_empty() || self.params[0] == 0 {
            // Reset
            self.current_fg = Color::WHITE;
            self.current_bg = Color::BLACK;
            self.current_bold = false;
            return Some(AnsiAction::Reset);
        }

        if self.params[0] == 1 {
            // Bold
            self.current_bold = true;
            return Some(AnsiAction::SetBold);
        }

        if self.params[0] >= 30 && self.params[0] <= 37 {
            // Foreground color
            let color = Color::from_ansi_color(self.params[0] - 30);
            self.current_fg = color;
            return Some(AnsiAction::SetForeground(color));
        }

        if self.params[0] >= 40 && self.params[0] <= 47 {
            // Background color
            let color = Color::from_ansi_color(self.params[0] - 40);
            self.current_bg = color;
            return Some(AnsiAction::SetBackground(color));
        }

        None
    }
}

// ============================================================================
// Terminal Emulator (simplified)
// ============================================================================

struct SimpleTerminal {
    grid: Grid,
    scrollback: Scrollback,
    cursor: Cursor,
    parser: SimpleAnsiParser,
    current_fg: Color,
    current_bg: Color,
    current_bold: bool,
}

impl SimpleTerminal {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            grid: Grid::new(cols, rows),
            scrollback: Scrollback::new(1000),
            cursor: Cursor::new(),
            parser: SimpleAnsiParser::new(),
            current_fg: Color::WHITE,
            current_bg: Color::BLACK,
            current_bold: false,
        }
    }

    fn process_bytes(&mut self, data: &[u8]) {
        for &byte in data {
            if let Some(action) = self.parser.parse(byte) {
                self.apply_action(action);
            }
        }
    }

    fn apply_action(&mut self, action: AnsiAction) {
        match action {
            AnsiAction::Print(ch) => {
                // Write character at cursor
                let mut cell = Cell::new(ch)
                    .with_foreground(self.current_fg)
                    .with_background(self.current_bg);

                if self.current_bold {
                    cell = cell.with_bold();
                }

                self.grid.set(self.cursor.row, self.cursor.col, cell);

                // Move cursor forward
                self.cursor.move_forward(1, self.grid.cols - 1);
            }
            AnsiAction::MoveCursor { row, col } => {
                self.cursor.move_to(row, col);
            }
            AnsiAction::ClearScreen => {
                self.grid.clear();
                self.cursor.move_to(0, 0);
            }
            AnsiAction::ClearLine => {
                self.grid.clear_row(self.cursor.row);
                self.cursor.carriage_return();
            }
            AnsiAction::SetForeground(color) => {
                self.current_fg = color;
            }
            AnsiAction::SetBackground(color) => {
                self.current_bg = color;
            }
            AnsiAction::SetBold => {
                self.current_bold = true;
            }
            AnsiAction::Reset => {
                self.current_fg = Color::WHITE;
                self.current_bg = Color::BLACK;
                self.current_bold = false;
            }
            AnsiAction::CarriageReturn => {
                self.cursor.carriage_return();
            }
            AnsiAction::Newline => {
                self.cursor.newline(self.grid.rows - 1);

                // If we're at the bottom, scroll
                if self.cursor.row >= self.grid.rows {
                    self.cursor.row = self.grid.rows - 1;
                    self.grid.scroll_up(1);
                }
            }
        }
    }

    fn render(&self) -> String {
        self.grid.to_string()
    }

    fn render_ansi(&self) -> String {
        self.grid.to_ansi_string()
    }
}

// ============================================================================
// Demo
// ============================================================================

fn main() {
    println!("═══════════════════════════════════════════════");
    println!("  Terminal Plugin Core Components Demo");
    println!("═══════════════════════════════════════════════\n");

    // Demo 1: Basic Grid
    println!("Demo 1: Basic Grid Operations");
    println!("─────────────────────────────────────────────\n");

    let mut grid = Grid::new(20, 5);

    // Write some text
    let text = "Hello, Terminal!";
    for (i, ch) in text.chars().enumerate() {
        grid.set(0, i, Cell::new(ch));
    }

    println!("Grid (20x5):");
    println!("{}\n", grid.to_string());

    // Demo 2: Colored Cells
    println!("Demo 2: Colored Cells");
    println!("─────────────────────────────────────────────\n");

    let mut grid2 = Grid::new(30, 3);

    // Red text
    for (i, ch) in "Red".chars().enumerate() {
        grid2.set(0, i, Cell::new(ch).with_foreground(Color::RED));
    }

    // Green text
    for (i, ch) in "Green".chars().enumerate() {
        grid2.set(1, i, Cell::new(ch).with_foreground(Color::GREEN));
    }

    // Blue text
    for (i, ch) in "Blue".chars().enumerate() {
        grid2.set(2, i, Cell::new(ch).with_foreground(Color::BLUE));
    }

    println!("Colored Grid:");
    println!("{}", grid2.to_ansi_string());

    // Demo 3: ANSI Parsing
    println!("Demo 3: ANSI Sequence Parsing");
    println!("─────────────────────────────────────────────\n");

    let mut terminal = SimpleTerminal::new(40, 10);

    // Simulate shell output with ANSI codes
    let output = b"$ \x1b[32mls\x1b[0m -la\r\n\
                   total 64\r\n\
                   drwxr-xr-x  5 user  staff  160\r\n\
                   -rw-r--r--  1 user  staff 1234\r\n\
                   \x1b[34mREADME.md\x1b[0m\r\n\
                   \x1b[31m*.log\x1b[0m\r\n";

    terminal.process_bytes(output);

    println!("Terminal Output:");
    println!("{}", terminal.render_ansi());

    // Demo 4: Scrollback
    println!("Demo 4: Scrollback Buffer");
    println!("─────────────────────────────────────────────\n");

    let mut scrollback = Scrollback::new(100);

    // Add some lines
    for i in 0..10 {
        let line = format!("Line {}", i)
            .chars()
            .map(Cell::new)
            .collect::<Vec<_>>();
        scrollback.push_line(line);
    }

    println!("Scrollback buffer:");
    println!("  Total lines: {}", scrollback.len());
    println!("  At bottom: {}", scrollback.is_at_bottom());

    scrollback.scroll_up(3);
    println!("  After scrolling up 3:");
    println!("    At bottom: {}", scrollback.is_at_bottom());

    scrollback.scroll_to_bottom();
    println!("  After scroll_to_bottom:");
    println!("    At bottom: {}\n", scrollback.is_at_bottom());

    // Demo 5: Cursor Movement
    println!("Demo 5: Cursor Movement");
    println!("─────────────────────────────────────────────\n");

    let mut cursor = Cursor::new();
    println!("Initial position: ({}, {})", cursor.row, cursor.col);

    cursor.move_down(3, 10);
    cursor.move_forward(5, 20);
    println!("After move_down(3) and move_forward(5): ({}, {})", cursor.row, cursor.col);

    cursor.move_to(0, 0);
    println!("After move_to(0, 0): ({}, {})", cursor.row, cursor.col);

    cursor.carriage_return();
    println!("After carriage_return: ({}, {})\n", cursor.row, cursor.col);

    // Demo 6: Complex ANSI
    println!("Demo 6: Complex ANSI Sequences");
    println!("─────────────────────────────────────────────\n");

    let mut terminal2 = SimpleTerminal::new(60, 8);

    let complex_output = b"\x1b[2J\x1b[H\
                          \x1b[1;31m╔═══════════════════════════╗\x1b[0m\r\n\
                          \x1b[1;31m║\x1b[0m \x1b[1mTerminal Demo\x1b[0m           \x1b[1;31m║\x1b[0m\r\n\
                          \x1b[1;31m╚═══════════════════════════╝\x1b[0m\r\n\
                          \r\n\
                          Status: \x1b[32m✓ Ready\x1b[0m\r\n\
                          Error:  \x1b[31m✗ Failed\x1b[0m\r\n";

    terminal2.process_bytes(complex_output);

    println!("Complex ANSI output:");
    println!("{}", terminal2.render_ansi());

    println!("═══════════════════════════════════════════════");
    println!("  Demo Complete!");
    println!("═══════════════════════════════════════════════");
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::default();
        assert_eq!(cell.ch, ' ');
        assert_eq!(cell.fg, Color::WHITE);
        assert_eq!(cell.bg, Color::BLACK);
    }

    #[test]
    fn test_cell_attributes() {
        let cell = Cell::new('A').with_bold();
        assert!(cell.is_bold());
    }

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(80, 24);
        assert_eq!(grid.cols, 80);
        assert_eq!(grid.rows, 24);
    }

    #[test]
    fn test_grid_set_get() {
        let mut grid = Grid::new(10, 10);
        let cell = Cell::new('X');
        grid.set(5, 5, cell.clone());

        assert_eq!(grid.get(5, 5).unwrap().ch, 'X');
    }

    #[test]
    fn test_grid_resize() {
        let mut grid = Grid::new(80, 24);
        grid.resize(120, 30);
        assert_eq!(grid.cols, 120);
        assert_eq!(grid.rows, 30);
    }

    #[test]
    fn test_scrollback() {
        let mut scrollback = Scrollback::new(10);

        for i in 0..5 {
            let line = vec![Cell::new('A'); i];
            scrollback.push_line(line);
        }

        assert_eq!(scrollback.len(), 5);
        assert!(scrollback.is_at_bottom());
    }

    #[test]
    fn test_cursor_movement() {
        let mut cursor = Cursor::new();

        cursor.move_down(5, 10);
        assert_eq!(cursor.row, 5);

        cursor.move_forward(3, 10);
        assert_eq!(cursor.col, 3);

        cursor.carriage_return();
        assert_eq!(cursor.col, 0);
    }

    #[test]
    fn test_ansi_parser_basic() {
        let mut parser = SimpleAnsiParser::new();

        let action = parser.parse(b'A');
        assert!(matches!(action, Some(AnsiAction::Print('A'))));
    }

    #[test]
    fn test_simple_terminal() {
        let mut term = SimpleTerminal::new(80, 24);

        term.process_bytes(b"Hello");
        let output = term.render();

        assert!(output.contains("Hello"));
    }
}
