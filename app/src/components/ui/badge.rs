use crate::brand::ThemeColors;
use gpui_kit::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeVariant {
    Primary,
    Success,
    Warning,
    Error,
    Neutral,
    Info,
}

impl BadgeVariant {
    pub fn colors(self, palette: &ThemeColors) -> (Rgba, Rgba) {
        match self {
            Self::Primary => (palette.accent_primary, palette.bg_badge),
            Self::Success => (palette.success, palette.bg_badge),
            Self::Warning => (palette.accent_gold, palette.bg_badge),
            Self::Error => (palette.error, palette.bg_badge),
            Self::Neutral => (palette.text_secondary, palette.bg_badge),
            Self::Info => (palette.accent_blue, palette.bg_badge),
        }
    }
}

pub fn badge(
    label: impl Into<SharedString>,
    variant: BadgeVariant,
    palette: ThemeColors,
) -> impl IntoElement {
    let (text_color, bg_color) = variant.colors(&palette);
    div()
        .flex()
        .items_center()
        .px_2p5()
        .py(px(3.))
        .rounded_full()
        .bg(bg_color)
        .border(px(1.5))
        .border_color(palette.border_color)
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(text_color)
        .child(label.into())
}

pub fn status_pill(
    label: impl Into<SharedString>,
    variant: BadgeVariant,
    palette: ThemeColors,
) -> impl IntoElement {
    let (text_color, bg_color) = variant.colors(&palette);
    div()
        .flex()
        .items_center()
        .gap_1p5()
        .px_2p5()
        .py_1()
        .rounded_full()
        .bg(bg_color)
        .border(px(1.5))
        .border_color(palette.border_color)
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(text_color)
        .child(div().w(px(6.)).h(px(6.)).rounded_full().bg(text_color))
        .child(label.into())
}
