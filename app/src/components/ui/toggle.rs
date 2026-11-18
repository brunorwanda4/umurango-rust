use crate::brand::ThemeColors;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn toggle(
    id: impl Into<ElementId>,
    checked: bool,
    palette: ThemeColors,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let bg_color = if checked {
        palette.primary
    } else {
        palette.border
    };

    let thumb_bg = if checked {
        palette.text_primary
    } else {
        palette.text_muted
    };

    div()
        .id(id)
        .w(px(38.))
        .h(px(22.))
        .p(px(2.))
        .flex()
        .items_center()
        .rounded_full()
        .bg(bg_color)
        .cursor_pointer()
        .when(checked, |this| this.justify_end())
        .child(div().w(px(18.)).h(px(18.)).rounded_full().bg(thumb_bg))
        .on_click(on_toggle)
}

pub fn toggle_field(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    description: Option<impl Into<SharedString>>,
    checked: bool,
    palette: ThemeColors,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
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
                        .child(label.into()),
                )
                .when_some(description, |this, desc| {
                    this.child(
                        div()
                            .text_xs()
                            .text_color(palette.text_muted)
                            .child(desc.into()),
                    )
                }),
        )
        .child(toggle(id, checked, palette, on_toggle))
}
