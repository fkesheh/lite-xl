/// Terminal Tab Bar Component
///
/// Manages multiple terminal instances with tabs
///
/// Features:
/// - Tab creation and deletion
/// - Tab switching
/// - Visual indication of active tab
/// - Close buttons on tabs
/// - Add new tab button
/// - Tab labels with terminal info

use floem::{
    event::{Event, EventListener},
    peniko::Color,
    reactive::{RwSignal, SignalGet, SignalUpdate},
    style::{AlignItems, CursorStyle, JustifyContent},
    View,
    views::{container, h_stack, label, svg, Decorators},
};

use super::theme::Theme;
use super::terminal_canvas::TerminalGrid;

/// Terminal tab information
#[derive(Debug, Clone)]
pub struct TerminalTab {
    /// Unique tab ID
    pub id: usize,
    /// Tab title
    pub title: String,
    /// Terminal grid state
    pub grid: TerminalGrid,
    /// Whether this tab is currently active
    pub active: bool,
    /// Working directory
    pub working_dir: String,
    /// Whether the terminal is running a process
    pub is_busy: bool,
}

impl TerminalTab {
    /// Create a new terminal tab
    pub fn new(id: usize, title: String, cols: usize, rows: usize) -> Self {
        Self {
            id,
            title,
            grid: TerminalGrid::new(cols, rows),
            active: false,
            working_dir: "~".to_string(),
            is_busy: false,
        }
    }

    /// Create a default tab
    pub fn default(id: usize) -> Self {
        Self::new(id, format!("Terminal {}", id + 1), 80, 24)
    }
}

/// Terminal tab manager state
#[derive(Debug, Clone)]
pub struct TabManager {
    /// List of all tabs
    pub tabs: Vec<TerminalTab>,
    /// Currently active tab index
    pub active_index: usize,
    /// Next tab ID
    pub next_id: usize,
}

impl TabManager {
    /// Create a new tab manager with one initial tab
    pub fn new() -> Self {
        let initial_tab = TerminalTab::default(0);
        Self {
            tabs: vec![initial_tab],
            active_index: 0,
            next_id: 1,
        }
    }

    /// Add a new tab
    pub fn add_tab(&mut self, title: Option<String>) {
        let title = title.unwrap_or_else(|| format!("Terminal {}", self.next_id + 1));
        let tab = TerminalTab::new(self.next_id, title, 80, 24);
        self.next_id += 1;
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
    }

    /// Close a tab by index
    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() > 1 && index < self.tabs.len() {
            self.tabs.remove(index);

            // Adjust active index
            if self.active_index >= self.tabs.len() {
                self.active_index = self.tabs.len() - 1;
            } else if self.active_index > index {
                self.active_index -= 1;
            }
        }
    }

    /// Switch to a specific tab
    pub fn switch_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_index = index;
        }
    }

    /// Get the currently active tab
    pub fn active_tab(&self) -> Option<&TerminalTab> {
        self.tabs.get(self.active_index)
    }

    /// Get the currently active tab (mutable)
    pub fn active_tab_mut(&mut self) -> Option<&mut TerminalTab> {
        self.tabs.get_mut(self.active_index)
    }

    /// Switch to next tab
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_index = (self.active_index + 1) % self.tabs.len();
        }
    }

    /// Switch to previous tab
    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_index = if self.active_index == 0 {
                self.tabs.len() - 1
            } else {
                self.active_index - 1
            };
        }
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Terminal tab bar view
pub fn terminal_tab_bar_view(
    tab_manager: RwSignal<TabManager>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        h_stack((
            // Tab list
            container({
                // This is a simplified version - in a real implementation,
                // you'd use dyn_stack or similar for dynamic tabs
                label(|| "Tabs (placeholder)")
                    .style(move |s| {
                        let theme_val = theme.get();
                        s.font_size(12.0)
                            .color(theme_val.line_number)
                    })
            })
            .style(|s| s.flex_grow(1.0).padding(4.0)),

            // Add new tab button
            add_tab_button(tab_manager, theme),
        ))
        .style(|s| s.gap(4.0)),
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height(32.0)
            .background(theme_val.status_bar_bg)
            .border_bottom(1.0)
            .border_color(theme_val.border)
            .padding_left(8.0)
            .padding_right(8.0)
            .align_items(AlignItems::Center)
            .flex_shrink(0.0)
    })
}

/// Single tab view
pub fn tab_view(
    index: usize,
    tab: TerminalTab,
    is_active: bool,
    theme: RwSignal<Theme>,
    on_select: impl Fn() + 'static,
    on_close: impl Fn() + 'static,
) -> impl View {
    let title = tab.title.clone();
    let is_busy = tab.is_busy;

    container(
        h_stack((
            // Tab label
            label(move || title.clone()).style(move |s| {
                let theme_val = theme.get();
                s.font_size(12.0)
                    .color(if is_active {
                        theme_val.foreground
                    } else {
                        theme_val.line_number
                    })
                    .font_family("sans-serif".to_string())
            }),

            // Busy indicator (optional)
            if is_busy {
                container(
                    label(|| "●").style(move |s| {
                        s.font_size(10.0)
                            .color(Color::rgb8(100, 200, 100))
                    }),
                )
                .style(|s| s.margin_left(4.0))
            } else {
                container(label(|| "")).style(|s| s.width(0.0))
            },

            // Close button
            close_button(theme, on_close),
        ))
        .style(|s| s.gap(8.0).align_items(AlignItems::Center)),
    )
    .on_click_stop(move |_| on_select())
    .style(move |s| {
        let theme_val = theme.get();
        let base_style = s
            .padding(6.0)
            .padding_left(12.0)
            .padding_right(12.0)
            .border_radius(4.0)
            .cursor(CursorStyle::Pointer);

        if is_active {
            base_style
                .background(theme_val.background)
                .border(1.0)
                .border_color(theme_val.border)
        } else {
            base_style
                .background(Color::rgba8(40, 40, 40, 100))
                .hover(|s| s.background(Color::rgba8(50, 50, 50, 150)))
        }
    })
}

/// Close button for tabs
fn close_button(theme: RwSignal<Theme>, on_close: impl Fn() + 'static) -> impl View {
    container(
        label(|| "×").style(move |s| {
            let theme_val = theme.get();
            s.font_size(16.0)
                .color(theme_val.line_number)
                .font_weight(floem::text::Weight::BOLD)
        }),
    )
    .on_click_stop(move |_| on_close())
    .style(move |s| {
        s.width(20.0)
            .height(20.0)
            .border_radius(3.0)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .cursor(CursorStyle::Pointer)
            .hover(move |s| {
                let theme_val = theme.get();
                s.background(theme_val.selection)
                    .color(theme_val.foreground)
            })
    })
}

/// Add new tab button
fn add_tab_button(
    tab_manager: RwSignal<TabManager>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        label(|| "+").style(move |s| {
            let theme_val = theme.get();
            s.font_size(16.0)
                .color(theme_val.foreground)
                .font_weight(floem::text::Weight::BOLD)
        }),
    )
    .on_click_stop(move |_| {
        tab_manager.update(|manager| {
            manager.add_tab(None);
        });
    })
    .style(move |s| {
        let theme_val = theme.get();
        s.width(28.0)
            .height(28.0)
            .border_radius(4.0)
            .border(1.0)
            .border_color(theme_val.border)
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .cursor(CursorStyle::Pointer)
            .hover(move |s| s.background(theme_val.selection))
    })
}

/// Render dynamic tab list
pub fn render_tab_list(
    tab_manager: RwSignal<TabManager>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        label(|| "Tab items (placeholder)")
            .style(move |s| {
                let theme_val = theme.get();
                s.font_size(12.0)
                    .color(theme_val.line_number)
            })
    )
    .style(|s| s.padding(4.0))
}

/// Helper to create tabs dynamically
pub fn create_tab_items(
    tab_manager: RwSignal<TabManager>,
    theme: RwSignal<Theme>,
) -> Vec<Box<dyn View>> {
    let manager = tab_manager.get();
    let mut items: Vec<Box<dyn View>> = Vec::new();

    for (index, tab) in manager.tabs.iter().enumerate() {
        let is_active = index == manager.active_index;
        let tab_clone = tab.clone();
        let idx = index;

        let tab_view_item = tab_view(
            index,
            tab_clone,
            is_active,
            theme,
            move || {
                tab_manager.update(|mgr| {
                    mgr.switch_to_tab(idx);
                });
            },
            move || {
                tab_manager.update(|mgr| {
                    mgr.close_tab(idx);
                });
            },
        );

        // Note: This is a conceptual representation. In practice, you'd need to
        // handle the type conversion properly
        // items.push(Box::new(tab_view_item));
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_manager_creation() {
        let manager = TabManager::new();
        assert_eq!(manager.tabs.len(), 1);
        assert_eq!(manager.active_index, 0);
    }

    #[test]
    fn test_add_tab() {
        let mut manager = TabManager::new();
        manager.add_tab(None);
        assert_eq!(manager.tabs.len(), 2);
        assert_eq!(manager.active_index, 1);
    }

    #[test]
    fn test_close_tab() {
        let mut manager = TabManager::new();
        manager.add_tab(None);
        manager.add_tab(None);
        assert_eq!(manager.tabs.len(), 3);

        manager.close_tab(1);
        assert_eq!(manager.tabs.len(), 2);
    }

    #[test]
    fn test_switch_tab() {
        let mut manager = TabManager::new();
        manager.add_tab(None);
        manager.add_tab(None);

        manager.switch_to_tab(1);
        assert_eq!(manager.active_index, 1);

        manager.switch_to_tab(0);
        assert_eq!(manager.active_index, 0);
    }

    #[test]
    fn test_next_prev_tab() {
        let mut manager = TabManager::new();
        manager.add_tab(None);
        manager.add_tab(None);

        manager.next_tab();
        assert_eq!(manager.active_index, 1);

        manager.next_tab();
        assert_eq!(manager.active_index, 2);

        manager.next_tab();
        assert_eq!(manager.active_index, 0); // Wraps around

        manager.prev_tab();
        assert_eq!(manager.active_index, 2);
    }

    #[test]
    fn test_active_tab() {
        let mut manager = TabManager::new();
        assert!(manager.active_tab().is_some());

        let tab = manager.active_tab().unwrap();
        assert_eq!(tab.id, 0);
    }

    #[test]
    fn test_cannot_close_last_tab() {
        let mut manager = TabManager::new();
        assert_eq!(manager.tabs.len(), 1);

        manager.close_tab(0);
        assert_eq!(manager.tabs.len(), 1); // Should still have 1 tab
    }

    #[test]
    fn test_tab_creation() {
        let tab = TerminalTab::new(0, "Test".to_string(), 80, 24);
        assert_eq!(tab.title, "Test");
        assert_eq!(tab.grid.cols, 80);
        assert_eq!(tab.grid.rows, 24);
    }
}
