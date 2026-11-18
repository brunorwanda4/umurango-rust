use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn checkbox_indicator(palette: ThemeColors, checked: bool) -> impl IntoElement {
    div()
        .w(px(18.))
        .h(px(18.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(3.))
        .border_1()
        .border_color(palette.border)
        .when(checked, |this| {
            this.bg(palette.primary)
                .border_color(palette.primary)
                .child(
                    icons::icon(icons::ICON_CHECK)
                        .size(px(12.))
                        .text_color(palette.text_primary),
                )
        })
}

pub fn checkbox(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    checked: bool,
    label: impl Into<SharedString>,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap_2()
        .cursor_pointer()
        .child(checkbox_indicator(palette, checked))
        .child(div().text_color(palette.text_body).child(label.into()))
        .on_click(on_click)
}

pub fn checkbox_with_description(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    checked: bool,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_start()
        .gap_2()
        .cursor_pointer()
        .child(div().pt(px(2.)).child(checkbox_indicator(palette, checked)))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette.text_primary)
                        .child(title.into()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_muted)
                        .child(description.into()),
                ),
        )
        .on_click(on_click)
}

pub fn checkbox_with_link(
    id: impl Into<ElementId>,
    link_id: impl Into<ElementId>,
    palette: ThemeColors,
    checked: bool,
    label: impl Into<SharedString>,
    link_label: impl Into<SharedString>,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_link_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_1()
        .flex_wrap()
        .text_xs()
        .child(
            div()
                .id(id)
                .flex()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .child(checkbox_indicator(palette, checked))
                .child(div().text_color(palette.text_body).child(label.into()))
                .on_click(on_toggle),
        )
        .child(
            div()
                .id(link_id)
                .cursor_pointer()
                .text_color(palette.primary_light)
                .child(link_label.into())
                .on_click(on_link_click),
        )
}
