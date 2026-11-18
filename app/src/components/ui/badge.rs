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
        // Returns (text_color, bg_color)
        match self {
            Self::Primary => (palette.primary_light, palette.primary.opacity(0.15)),
            Self::Success => (palette.success, palette.success.opacity(0.15)),
            Self::Warning => (palette.accent_gold, palette.accent_gold.opacity(0.15)),
            Self::Error => (palette.error, palette.error.opacity(0.15)),
            Self::Neutral => (palette.text_muted, palette.border),
            Self::Info => (palette.accent_blue, palette.accent_blue.opacity(0.15)),
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
        .px_2()
        .py(px(2.))
        .rounded_full()
        .bg(bg_color)
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
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
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(text_color)
        .child(div().w(px(6.)).h(px(6.)).rounded_full().bg(text_color))
        .child(label.into())
}
