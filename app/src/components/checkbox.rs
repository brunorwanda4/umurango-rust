use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

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
        .child(
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
                            icons::icon("check")
                                .size(px(12.))
                                .text_color(palette.text_primary),
                        )
                }),
        )
        .child(div().text_color(palette.text_body).child(label.into()))
        .on_click(on_click)
}
