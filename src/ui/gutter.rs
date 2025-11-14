/// Line number gutter component
///
/// Displays line numbers on the left side of the editor

use floem::{
    reactive::{RwSignal, SignalGet},
    style::{AlignItems, CursorStyle, JustifyContent},
    View,
    views::{container, dyn_stack, label, Decorators},
};

use crate::editor::EditorState;
use super::theme::Theme;

/// Create a gutter view showing line numbers
pub fn gutter_view(
    editor: RwSignal<EditorState>,
    theme: RwSignal<Theme>,
    scroll_offset: RwSignal<f64>,
    line_height: f32,
    visible_lines: usize,
) -> impl View {
    container(
        dyn_stack(
            move || {
                let editor_state = editor.get();
                let scroll = scroll_offset.get();
                let first_visible = scroll.floor() as usize;
                let last_visible = (first_visible + visible_lines).min(editor_state.line_count());
                first_visible..last_visible
            },
            |line_num| *line_num,
            move |line_num| {
                let editor_state = editor.get();
                let theme_val = theme.get();
                let is_current = line_num == editor_state.cursor().line;
                let line_color = if is_current {
                    theme_val.line_number_active
                } else {
                    theme_val.line_number
                };

                container(
                    label(move || format!("{:>4}", line_num + 1))
                        .style(move |s| {
                            s.font_size(14.0)
                                .color(line_color)
                                .font_family("monospace".to_string())
                        })
                )
                .style(move |s| {
                    s.height(line_height as f64)
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::End)
                        .padding_right(8.0)
                })
            }
        )
        .style(|s| s.flex_col())
    )
    .style(move |s| {
        let theme_val = theme.get();
        s.width(60.0)
            .background(theme_val.gutter_bg)
            .border_right(1.0)
            .border_color(theme_val.border)
            .flex_shrink(0.0)
            .cursor(CursorStyle::Default)
    })
}
