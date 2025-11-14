/// Theme system for the editor
///
/// Defines colors and styling for the editor UI

use floem::peniko::Color;

/// Editor theme
#[derive(Debug, Clone)]
pub struct Theme {
    /// Background color
    pub background: Color,

    /// Foreground/text color
    pub foreground: Color,

    /// Current line highlight
    pub current_line: Color,

    /// Selection background
    pub selection: Color,

    /// Cursor color
    pub cursor: Color,

    /// Line number color
    pub line_number: Color,

    /// Line number background
    pub line_number_bg: Color,

    /// Active line number color
    pub line_number_active: Color,

    /// Status bar background
    pub status_bar_bg: Color,

    /// Status bar foreground
    pub status_bar_fg: Color,

    /// Gutter background
    pub gutter_bg: Color,

    /// Comment color
    pub comment: Color,

    /// Keyword color
    pub keyword: Color,

    /// String color
    pub string: Color,

    /// Number color
    pub number: Color,

    /// Function color
    pub function: Color,

    /// Border color
    pub border: Color,
}

impl Theme {
    /// Create a dark theme (default)
    pub fn dark() -> Self {
        Self {
            // Main colors - Dark background with light text
            background: Color::rgb8(30, 30, 30),
            foreground: Color::rgb8(220, 220, 220),

            // Editor highlights
            current_line: Color::rgba8(50, 50, 50, 100),
            selection: Color::rgba8(70, 130, 180, 100),
            cursor: Color::rgb8(255, 255, 255),

            // Line numbers
            line_number: Color::rgb8(100, 100, 100),
            line_number_bg: Color::rgb8(25, 25, 25),
            line_number_active: Color::rgb8(200, 200, 200),

            // Status bar
            status_bar_bg: Color::rgb8(40, 40, 40),
            status_bar_fg: Color::rgb8(200, 200, 200),

            // Gutter
            gutter_bg: Color::rgb8(25, 25, 25),

            // Syntax highlighting
            comment: Color::rgb8(106, 153, 85),
            keyword: Color::rgb8(86, 156, 214),
            string: Color::rgb8(206, 145, 120),
            number: Color::rgb8(181, 206, 168),
            function: Color::rgb8(220, 220, 170),

            // UI elements
            border: Color::rgb8(60, 60, 60),
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            // Main colors - Light background with dark text
            background: Color::rgb8(255, 255, 255),
            foreground: Color::rgb8(30, 30, 30),

            // Editor highlights
            current_line: Color::rgba8(245, 245, 245, 100),
            selection: Color::rgba8(173, 214, 255, 150),
            cursor: Color::rgb8(0, 0, 0),

            // Line numbers
            line_number: Color::rgb8(150, 150, 150),
            line_number_bg: Color::rgb8(250, 250, 250),
            line_number_active: Color::rgb8(50, 50, 50),

            // Status bar
            status_bar_bg: Color::rgb8(240, 240, 240),
            status_bar_fg: Color::rgb8(50, 50, 50),

            // Gutter
            gutter_bg: Color::rgb8(250, 250, 250),

            // Syntax highlighting
            comment: Color::rgb8(0, 128, 0),
            keyword: Color::rgb8(0, 0, 255),
            string: Color::rgb8(163, 21, 21),
            number: Color::rgb8(9, 134, 88),
            function: Color::rgb8(121, 94, 38),

            // UI elements
            border: Color::rgb8(200, 200, 200),
        }
    }

    /// Create a solarized dark theme
    pub fn solarized_dark() -> Self {
        Self {
            background: Color::rgb8(0, 43, 54),
            foreground: Color::rgb8(131, 148, 150),

            current_line: Color::rgba8(7, 54, 66, 100),
            selection: Color::rgba8(42, 161, 152, 100),
            cursor: Color::rgb8(147, 161, 161),

            line_number: Color::rgb8(88, 110, 117),
            line_number_bg: Color::rgb8(0, 36, 46),
            line_number_active: Color::rgb8(147, 161, 161),

            status_bar_bg: Color::rgb8(7, 54, 66),
            status_bar_fg: Color::rgb8(147, 161, 161),

            gutter_bg: Color::rgb8(0, 36, 46),

            comment: Color::rgb8(88, 110, 117),
            keyword: Color::rgb8(38, 139, 210),
            string: Color::rgb8(42, 161, 152),
            number: Color::rgb8(211, 54, 130),
            function: Color::rgb8(181, 137, 0),

            border: Color::rgb8(7, 54, 66),
        }
    }

    /// Get default theme (dark)
    pub fn default() -> Self {
        Self::dark()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Font configuration
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Font family name
    pub family: String,

    /// Font size in points
    pub size: f32,

    /// Line height multiplier
    pub line_height: f32,

    /// Character width (monospace)
    pub char_width: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "monospace".to_string(),
            size: 14.0,
            line_height: 1.5,
            char_width: 8.0, // Approximate for monospace
        }
    }
}

impl FontConfig {
    /// Calculate pixel height for a line
    pub fn line_height_px(&self) -> f32 {
        self.size * self.line_height
    }

    /// Get approximate character width
    pub fn char_width_px(&self) -> f32 {
        self.char_width
    }
}
