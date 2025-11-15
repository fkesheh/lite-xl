/// Terminal Panel Component
///
/// Main terminal panel view with docking support
///
/// Features:
/// - Dockable panel (bottom, left, right)
/// - Resizable with drag handle
/// - Integrates tab bar and terminal canvas
/// - Collapsible/expandable
/// - Keyboard shortcuts for show/hide
/// - Theme integration
/// - Multiple terminal instances via tabs

use floem::{
    event::{Event, EventListener},
    peniko::Color,
    pointer::{PointerButton, PointerInputEvent},
    reactive::{RwSignal, SignalGet, SignalUpdate},
    style::{AlignItems, CursorStyle, FlexDirection},
    View,
    views::{container, empty, h_stack, label, v_stack, Decorators},
};

use super::theme::{FontConfig, Theme};
use super::terminal_canvas::{terminal_canvas_view, TerminalGrid};
use super::terminal_tabs::{terminal_tab_bar_view, TabManager};

/// Terminal panel docking position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockPosition {
    /// Docked at the bottom of the editor
    Bottom,
    /// Docked on the left side
    Left,
    /// Docked on the right side
    Right,
}

impl Default for DockPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Terminal panel state
#[derive(Debug, Clone)]
pub struct TerminalPanelState {
    /// Whether the panel is visible
    pub visible: bool,
    /// Docking position
    pub position: DockPosition,
    /// Panel size (width for left/right, height for bottom)
    pub size: f64,
    /// Minimum panel size
    pub min_size: f64,
    /// Maximum panel size (as fraction of window)
    pub max_size_fraction: f64,
    /// Tab manager
    pub tab_manager: TabManager,
    /// Whether panel is being resized
    pub is_resizing: bool,
}

impl TerminalPanelState {
    /// Create a new terminal panel state
    pub fn new() -> Self {
        Self {
            visible: false,
            position: DockPosition::Bottom,
            size: 300.0,
            min_size: 100.0,
            max_size_fraction: 0.8,
            tab_manager: TabManager::new(),
            is_resizing: false,
        }
    }

    /// Toggle panel visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Show the panel
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the panel
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Set docking position
    pub fn set_position(&mut self, position: DockPosition) {
        self.position = position;

        // Adjust default size based on position
        self.size = match position {
            DockPosition::Bottom => 300.0,
            DockPosition::Left | DockPosition::Right => 400.0,
        };
    }

    /// Resize the panel
    pub fn resize(&mut self, delta: f64, window_size: f64) {
        let max_size = window_size * self.max_size_fraction;
        self.size = (self.size + delta).clamp(self.min_size, max_size);
    }
}

impl Default for TerminalPanelState {
    fn default() -> Self {
        Self::new()
    }
}

/// Terminal panel view
pub fn terminal_panel_view(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    container(
        // Use dynamic visibility based on panel state
        container({
            let panel_content = create_panel_content(panel_state, theme, font_config);

            v_stack((
                // Top/resize handle (for bottom position)
                container({
                    let is_bottom = move || panel_state.get().position == DockPosition::Bottom;
                    container(resize_handle(panel_state, theme, ResizeDirection::Vertical))
                        .style(move |s| {
                            if is_bottom() {
                                s.display(floem::style::Display::Flex)
                            } else {
                                s.display(floem::style::Display::None)
                            }
                        })
                }),
                // Main content row
                h_stack((
                    // Left resize handle (for right position)
                    container({
                        let is_right = move || panel_state.get().position == DockPosition::Right;
                        container(resize_handle(panel_state, theme, ResizeDirection::Horizontal))
                            .style(move |s| {
                                if is_right() {
                                    s.display(floem::style::Display::Flex)
                                } else {
                                    s.display(floem::style::Display::None)
                                }
                            })
                    }),
                    // Panel content
                    panel_content,
                    // Right resize handle (for left position)
                    container({
                        let is_left = move || panel_state.get().position == DockPosition::Left;
                        container(resize_handle(panel_state, theme, ResizeDirection::Horizontal))
                            .style(move |s| {
                                if is_left() {
                                    s.display(floem::style::Display::Flex)
                                } else {
                                    s.display(floem::style::Display::None)
                                }
                            })
                    }),
                ))
                .style(|s| s.flex_grow(1.0).width_full()),
            ))
        })
        .style(move |s| {
            let state = panel_state.get();
            let theme_val = theme.get();
            let mut style = s.flex_shrink(0.0);

            match state.position {
                DockPosition::Bottom => {
                    style = style
                        .width_full()
                        .height(state.size)
                        .border_top(1.0);
                }
                DockPosition::Left => {
                    style = style
                        .width(state.size)
                        .height_full()
                        .border_right(1.0);
                }
                DockPosition::Right => {
                    style = style
                        .width(state.size)
                        .height_full()
                        .border_left(1.0);
                }
            }

            style.border_color(theme_val.border)
        })
    )
    .style(move |s| {
        let visible = panel_state.get().visible;
        if visible {
            s.display(floem::style::Display::Flex)
        } else {
            s.display(floem::style::Display::None)
        }
    })
}

/// Resize handle direction
#[derive(Debug, Clone, Copy, PartialEq)]
enum ResizeDirection {
    Horizontal,
    Vertical,
}

/// Create resize handle
fn resize_handle(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    direction: ResizeDirection,
) -> impl View {
    let is_dragging = RwSignal::new(false);
    let drag_start_pos = RwSignal::new(0.0);
    let initial_size = RwSignal::new(0.0);

    container(empty())
        .on_event_stop(EventListener::PointerDown, move |event| {
            if let Event::PointerDown(pointer_event) = event {
                if pointer_event.button == PointerButton::Primary {
                    is_dragging.set(true);
                    drag_start_pos.set(match direction {
                        ResizeDirection::Horizontal => pointer_event.pos.x,
                        ResizeDirection::Vertical => pointer_event.pos.y,
                    });
                    initial_size.set(panel_state.get().size);
                    panel_state.update(|state| state.is_resizing = true);
                }
            }
        })
        .on_event_stop(EventListener::PointerMove, move |event| {
            if let Event::PointerMove(pointer_event) = event {
                if is_dragging.get() {
                    let current_pos = match direction {
                        ResizeDirection::Horizontal => pointer_event.pos.x,
                        ResizeDirection::Vertical => pointer_event.pos.y,
                    };

                    let delta = match (direction, panel_state.get().position) {
                        (ResizeDirection::Vertical, DockPosition::Bottom) => {
                            drag_start_pos.get() - current_pos
                        }
                        (ResizeDirection::Horizontal, DockPosition::Left) => {
                            current_pos - drag_start_pos.get()
                        }
                        (ResizeDirection::Horizontal, DockPosition::Right) => {
                            drag_start_pos.get() - current_pos
                        }
                        _ => 0.0,
                    };

                    let new_size = initial_size.get() + delta;
                    panel_state.update(|state| {
                        state.size = new_size.clamp(state.min_size, 1000.0);
                    });
                }
            }
        })
        .on_event_stop(EventListener::PointerUp, move |_event| {
            is_dragging.set(false);
            panel_state.update(|state| state.is_resizing = false);
        })
        .style(move |s| {
            let theme_val = theme.get();
            let base_style = match direction {
                ResizeDirection::Horizontal => s
                    .width(4.0)
                    .height_full()
                    .cursor(CursorStyle::ColResize),
                ResizeDirection::Vertical => s
                    .height(4.0)
                    .width_full()
                    .cursor(CursorStyle::RowResize),
            };

            base_style
                .background(if is_dragging.get() {
                    theme_val.selection
                } else {
                    Color::TRANSPARENT
                })
                .hover(|s| s.background(theme_val.border))
        })
}

/// Create the main panel content
fn create_panel_content(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    container(
        v_stack((
            // Header with tab bar
            create_panel_header(panel_state, theme),

            // Terminal content area
            create_terminal_content(panel_state, theme, font_config),
        )),
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height_full()
            .background(theme_val.background)
            .flex_direction(FlexDirection::Column)
    })
}

/// Create panel header with controls
fn create_panel_header(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    container(
        h_stack((
            // Tab bar (left side)
            container({
                // Create a derived signal for the tab manager
                let tab_manager_signal = RwSignal::new(panel_state.get().tab_manager.clone());
                terminal_tab_bar_view(tab_manager_signal, theme)
            })
            .style(|s| s.flex_grow(1.0)),

            // Control buttons (right side)
            create_panel_controls(panel_state, theme),
        )),
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height(32.0)
            .background(theme_val.status_bar_bg)
            .border_bottom(1.0)
            .border_color(theme_val.border)
            .align_items(AlignItems::Center)
            .flex_shrink(0.0)
    })
}

/// Create panel control buttons
fn create_panel_controls(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
) -> impl View {
    h_stack((
        // Position toggle button
        control_button("⊟", "Change position", theme, move || {
            panel_state.update(|state| {
                state.position = match state.position {
                    DockPosition::Bottom => DockPosition::Right,
                    DockPosition::Right => DockPosition::Left,
                    DockPosition::Left => DockPosition::Bottom,
                };
            });
        }),

        // Close button
        control_button("×", "Close terminal", theme, move || {
            panel_state.update(|state| state.hide());
        }),
    ))
    .style(|s| s.gap(4.0).padding_right(8.0))
}

/// Create a control button
fn control_button(
    icon: &'static str,
    tooltip: &'static str,
    theme: RwSignal<Theme>,
    on_click: impl Fn() + 'static,
) -> impl View {
    container(
        label(move || icon.to_string()).style(move |s| {
            let theme_val = theme.get();
            s.font_size(16.0)
                .color(theme_val.foreground)
                .font_weight(floem::text::Weight::BOLD)
        }),
    )
    .on_click_stop(move |_| on_click())
    .style(move |s| {
        let theme_val = theme.get();
        s.width(24.0)
            .height(24.0)
            .border_radius(3.0)
            .justify_content(floem::style::JustifyContent::Center)
            .align_items(AlignItems::Center)
            .cursor(CursorStyle::Pointer)
            .hover(|s| s.background(theme_val.selection))
    })
}

/// Create terminal content area
fn create_terminal_content(
    panel_state: RwSignal<TerminalPanelState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    // Create a simple placeholder for now
    // In a real implementation, this should be reactive and show the actual terminal
    container(
        label(|| "Terminal view (placeholder)")
            .style(move |s| {
                let theme_val = theme.get();
                s.font_size(14.0)
                    .color(theme_val.line_number)
            }),
    )
    .style(|s| {
        s.width_full()
            .height_full()
            .justify_content(floem::style::JustifyContent::Center)
            .align_items(AlignItems::Center)
    })
}

/// Helper to create terminal panel in application
pub fn create_terminal_panel(
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> (RwSignal<TerminalPanelState>, impl View) {
    let panel_state = RwSignal::new(TerminalPanelState::new());

    let view = terminal_panel_view(panel_state, theme, font_config);

    (panel_state, view)
}

/// Keyboard shortcut handlers
pub fn handle_terminal_shortcuts(
    panel_state: RwSignal<TerminalPanelState>,
    key: &str,
    ctrl: bool,
) -> bool {
    if ctrl {
        match key {
            // Ctrl+` to toggle terminal
            "`" | "~" => {
                panel_state.update(|state| state.toggle());
                true
            }
            // Ctrl+Shift+T to create new tab
            "T" => {
                panel_state.update(|state| {
                    state.tab_manager.add_tab(None);
                    state.show();
                });
                true
            }
            // Ctrl+Shift+W to close current tab
            "W" => {
                panel_state.update(|state| {
                    let active_idx = state.tab_manager.active_index;
                    state.tab_manager.close_tab(active_idx);
                });
                true
            }
            // Ctrl+Tab to switch to next tab
            "Tab" => {
                panel_state.update(|state| {
                    state.tab_manager.next_tab();
                });
                true
            }
            _ => false,
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_state_creation() {
        let state = TerminalPanelState::new();
        assert!(!state.visible);
        assert_eq!(state.position, DockPosition::Bottom);
        assert_eq!(state.size, 300.0);
    }

    #[test]
    fn test_toggle_visibility() {
        let mut state = TerminalPanelState::new();
        assert!(!state.visible);

        state.toggle();
        assert!(state.visible);

        state.toggle();
        assert!(!state.visible);
    }

    #[test]
    fn test_show_hide() {
        let mut state = TerminalPanelState::new();

        state.show();
        assert!(state.visible);

        state.hide();
        assert!(!state.visible);
    }

    #[test]
    fn test_set_position() {
        let mut state = TerminalPanelState::new();

        state.set_position(DockPosition::Left);
        assert_eq!(state.position, DockPosition::Left);
        assert_eq!(state.size, 400.0);

        state.set_position(DockPosition::Bottom);
        assert_eq!(state.position, DockPosition::Bottom);
        assert_eq!(state.size, 300.0);
    }

    #[test]
    fn test_resize() {
        let mut state = TerminalPanelState::new();
        let initial_size = state.size;

        state.resize(50.0, 1000.0);
        assert_eq!(state.size, initial_size + 50.0);

        state.resize(-100.0, 1000.0);
        assert_eq!(state.size, initial_size - 50.0);
    }

    #[test]
    fn test_resize_clamping() {
        let mut state = TerminalPanelState::new();

        // Try to resize below minimum
        state.resize(-1000.0, 1000.0);
        assert_eq!(state.size, state.min_size);

        // Try to resize above maximum
        state.resize(2000.0, 1000.0);
        assert_eq!(state.size, 1000.0 * state.max_size_fraction);
    }

    #[test]
    fn test_dock_position_default() {
        let pos = DockPosition::default();
        assert_eq!(pos, DockPosition::Bottom);
    }
}
