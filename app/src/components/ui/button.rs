use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

pub fn icon_action_button(
    id: impl Into<ElementId>,
    icon_name: &'static str,
    tooltip: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Button {
    let tooltip_str = tooltip.into().to_string();
    Button::new(id)
        .icon(icons::button_icon(icon_name).text_color(palette.text_muted))
        .accessibility_label(tooltip_str.clone())
        .tooltip(tooltip_str)
        .text()
        .compact()
        .on_click(on_click)
}

pub fn secondary_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .rounded_lg()
        .bg(palette.surface)
        .border_1()
        .border_color(palette.border)
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(palette.text_primary)
        .cursor_pointer()
        .hover(|style| style.border_color(palette.border_hover).bg(palette.card))
        .child(label.into())
        .on_click(on_click)
}

pub fn destructive_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .rounded_lg()
        .bg(palette.error.opacity(0.12))
        .border_1()
        .border_color(palette.error.opacity(0.4))
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(palette.error)
        .cursor_pointer()
        .hover(|style| style.bg(palette.error.opacity(0.2)))
        .child(label.into())
        .on_click(on_click)
}
