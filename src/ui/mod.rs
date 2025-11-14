/// UI module
///
/// Contains all UI components for the editor

pub mod editor_view;
pub mod gutter;
pub mod statusbar;
pub mod theme;

pub use editor_view::editor_view;
pub use gutter::gutter_view;
pub use statusbar::statusbar_view;
pub use theme::{FontConfig, Theme};

use floem::{
    reactive::{RwSignal, SignalGet},
    style::AlignItems,
    View,
    views::{container, v_stack, Decorators},
};

use crate::editor::EditorState;

/// Create the complete application UI
pub fn app_view(
    editor: RwSignal<EditorState>,
    theme: RwSignal<Theme>,
    font_config: RwSignal<FontConfig>,
) -> impl View {
    container(
        v_stack((
            // Main editor area
            container(
                editor_view(editor, theme, font_config)
            )
            .style(|s| s.flex_grow(1.0).width_full()),

            // Status bar
            statusbar_view(editor, theme),
        ))
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width_full()
            .height_full()
            .background(theme_val.background)
            .align_items(AlignItems::Stretch)
    })
}
