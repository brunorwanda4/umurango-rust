use crate::brand::ThemeColors;
use gpui_kit::*;

pub fn kawaii_progress_bar(pct: u8, fill_color: Rgba, palette: ThemeColors) -> impl IntoElement {
    let clamped = pct.min(100);
    let ratio = (clamped as f32 / 100.0).max(0.04);

    div()
        .w_full()
        .h(px(14.0))
        .p(px(2.0))
        .rounded_full()
        .bg(palette.bg_input)
        .border(px(2.0))
        .border_color(palette.border_color)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(relative(ratio))
                .rounded_full()
                .bg(fill_color),
        )
}
