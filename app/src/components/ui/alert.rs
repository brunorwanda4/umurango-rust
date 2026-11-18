use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    pub fn config(self, palette: &ThemeColors) -> (Rgba, Rgba, &'static str) {
        match self {
            Self::Info => (
                palette.accent_blue,
                palette.bg_input,
                icons::ICON_CIRCLE_INFO,
            ),
            Self::Success => (palette.success, palette.bg_input, icons::ICON_CIRCLE_CHECK),
            Self::Warning => (
                palette.accent_gold,
                palette.bg_input,
                icons::ICON_TRIANGLE_EXCLAMATION,
            ),
            Self::Error => (palette.error, palette.bg_input, icons::ICON_CIRCLE_XMARK),
        }
    }
}

pub fn alert(
    variant: AlertVariant,
    title: Option<impl Into<SharedString>>,
    message: impl Into<SharedString>,
    palette: ThemeColors,
) -> impl IntoElement {
    let (accent_color, bg_color, icon_name) = variant.config(&palette);

    div()
        .w_full()
        .flex()
        .items_start()
        .gap_3()
        .p_3()
        .rounded(palette.radius_input)
        .bg(bg_color)
        .border(palette.border_width_input)
        .border_color(accent_color)
        .shadow(palette.checkbox_shadows())
        .child(
            div().pt(px(2.)).child(
                icons::icon(icon_name)
                    .size(px(14.))
                    .text_color(accent_color),
            ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .when_some(title, |this, t| {
                    this.child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(palette.text_primary)
                            .child(t.into()),
                    )
                })
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_secondary)
                        .child(message.into()),
                ),
        )
}

pub fn dismissible_alert(
    id: impl Into<ElementId>,
    variant: AlertVariant,
    message: impl Into<SharedString>,
    palette: ThemeColors,
    on_dismiss: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let (accent_color, bg_color, icon_name) = variant.config(&palette);

    div()
        .id(id)
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .p_3()
        .rounded(palette.radius_input)
        .bg(bg_color)
        .border(palette.border_width_input)
        .border_color(accent_color)
        .shadow(palette.checkbox_shadows())
        .child(
            div()
                .flex()
                .items_center()
                .gap_2p5()
                .child(
                    icons::icon(icon_name)
                        .size(px(14.))
                        .text_color(accent_color),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette.text_secondary)
                        .child(message.into()),
                ),
        )
        .child(
            div()
                .id("alert-dismiss-btn")
                .cursor_pointer()
                .p_1()
                .child(
                    icons::icon(icons::ICON_XMARK)
                        .size(px(12.))
                        .text_color(palette.text_muted),
                )
                .on_click(on_dismiss),
        )
}
