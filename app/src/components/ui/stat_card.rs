use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn stat_card(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    title: impl Into<SharedString>,
    value: impl Into<SharedString>,
    subtitle: Option<impl Into<SharedString>>,
    icon_name: Option<&'static str>,
    trend: Option<(SharedString, bool)>,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .gap_2()
        .p_4()
        .rounded_lg()
        .bg(palette.surface)
        .border_1()
        .border_color(palette.border)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette.text_muted)
                        .child(title.into()),
                )
                .when_some(icon_name, |this, name| {
                    this.child(
                        icons::icon(name)
                            .size(px(16.))
                            .text_color(palette.primary_light),
                    )
                }),
        )
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_primary)
                .child(value.into()),
        )
        .when(subtitle.is_some() || trend.is_some(), |this| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .when_some(trend, |this, (trend_text, is_positive)| {
                        let color = if is_positive {
                            palette.success
                        } else {
                            palette.error
                        };
                        let arrow_icon = if is_positive {
                            icons::ICON_ARROW_UP
                        } else {
                            icons::ICON_ARROW_DOWN
                        };
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(color)
                                .child(icons::icon(arrow_icon).size(px(10.)).text_color(color))
                                .child(trend_text),
                        )
                    })
                    .when_some(subtitle, |this, sub| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(palette.text_dim)
                                .child(sub.into()),
                        )
                    }),
            )
        })
}
