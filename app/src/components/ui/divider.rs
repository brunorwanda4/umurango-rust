use crate::brand::ThemeColors;
use gpui_kit::*;

pub fn divider(palette: ThemeColors) -> impl IntoElement {
    div().w_full().h(px(1.)).bg(palette.border)
}

pub fn vertical_divider(palette: ThemeColors) -> impl IntoElement {
    div().h_full().w(px(1.)).bg(palette.border)
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
        .child(div().absolute().w_full().h(px(1.)).bg(palette.border))
        .child(
            div()
                .relative()
                .px_3()
                .bg(palette.card)
                .text_xs()
                .text_color(palette.text_muted)
                .child(label.into()),
        )
}
