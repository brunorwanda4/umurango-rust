use crate::brand::ThemeColors;
use gpui_kit::*;

pub fn divider(palette: ThemeColors) -> impl IntoElement {
    div()
        .w_full()
        .h(palette.border_width_input)
        .bg(palette.border_color)
}

pub fn vertical_divider(palette: ThemeColors) -> impl IntoElement {
    div()
        .h_full()
        .w(palette.border_width_input)
        .bg(palette.border_color)
}

pub fn divider_with_label(
    label: impl Into<SharedString>,
    palette: ThemeColors,
) -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .my_2()
        .child(
            div()
                .absolute()
                .w_full()
                .h(palette.border_width_input)
                .bg(palette.border_color),
        )
        .child(
            div()
                .relative()
                .px_3()
                .bg(palette.bg_card)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_muted)
                .child(label.into()),
        )
}
