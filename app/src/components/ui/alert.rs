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
        // (text_and_border_color, bg_color, icon_name)
        match self {
            Self::Info => (
                palette.accent_blue,
                palette.accent_blue.opacity(0.1),
                icons::ICON_CIRCLE_INFO,
            ),
            Self::Success => (
                palette.success,
                palette.success.opacity(0.1),
                icons::ICON_CIRCLE_CHECK,
            ),
            Self::Warning => (
                palette.accent_gold,
                palette.accent_gold.opacity(0.1),
                icons::ICON_TRIANGLE_EXCLAMATION,
            ),
            Self::Error => (
                palette.error,
                palette.error.opacity(0.1),
                icons::ICON_CIRCLE_XMARK,
            ),
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
        .rounded_lg()
        .bg(bg_color)
        .border_1()
        .border_color(accent_color.opacity(0.3))
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
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(palette.text_primary)
                            .child(t.into()),
                    )
                })
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_body)
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
        .rounded_lg()
        .bg(bg_color)
        .border_1()
        .border_color(accent_color.opacity(0.3))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    icons::icon(icon_name)
                        .size(px(14.))
                        .text_color(accent_color),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_body)
                        .child(message.into()),
                ),
        )
        .child(
            div()
                .id("dismiss-icon")
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
