use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn card(palette: ThemeColors) -> Div {
    div()
        .flex()
        .flex_col()
        .w_full()
        .p_5()
        .rounded_xl()
        .bg(palette.card)
        .border_1()
        .border_color(palette.border)
        .text_color(palette.text_primary)
}

pub fn card_surface(palette: ThemeColors) -> Div {
    div()
        .flex()
        .flex_col()
        .w_full()
        .p_4()
        .rounded_lg()
        .bg(palette.surface)
        .border_1()
        .border_color(palette.border)
        .text_color(palette.text_primary)
}

pub fn interactive_card(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .w_full()
        .p_4()
        .rounded_lg()
        .bg(palette.surface)
        .border_1()
        .border_color(palette.border)
        .cursor_pointer()
        .hover(|style| style.border_color(palette.border_hover))
        .on_click(on_click)
}

pub fn card_header(
    title: impl Into<SharedString>,
    subtitle: Option<impl Into<SharedString>>,
    palette: ThemeColors,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .mb_3()
        .child(
            div()
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(palette.text_primary)
                .child(title.into()),
        )
        .when_some(subtitle, |this, sub| {
            this.child(
                div()
                    .text_xs()
                    .text_color(palette.text_muted)
                    .child(sub.into()),
            )
        })
}

pub fn card_footer(palette: ThemeColors) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .mt_4()
        .pt_3()
        .border_t_1()
        .border_color(palette.border)
}

pub fn option_card(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    icon_name: &'static str,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    selected: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .p_4()
        .rounded_lg()
        .border_1()
        .border_color(if selected {
            palette.primary
        } else {
            palette.border
        })
        .bg(palette.surface)
        .cursor_pointer()
        .hover(|style| style.border_color(palette.border_hover))
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .w(px(36.))
                        .h(px(36.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_lg()
                        .bg(palette.card)
                        .border_1()
                        .border_color(palette.border)
                        .child(
                            icons::icon(icon_name)
                                .size(px(16.))
                                .text_color(palette.primary_light),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(palette.text_primary)
                                .child(title.into()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .whitespace_normal()
                                .text_color(palette.text_muted)
                                .child(description.into()),
                        ),
                ),
        )
        .child(
            icons::icon(icons::ICON_ARROW_RIGHT)
                .size(px(14.))
                .text_color(if selected {
                    palette.primary_light
                } else {
                    palette.text_muted
                }),
        )
        .on_click(on_click)
}
