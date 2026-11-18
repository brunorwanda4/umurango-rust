use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn primary_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    primary_button_with_state(id, label, palette, false, false, on_click)
}

pub fn primary_button_with_state(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    loading: bool,
    disabled: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let is_inactive = loading || disabled;

    div()
        .id(id)
        .relative()
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .gap_2()
        .px_5()
        .py_3()
        .rounded(palette.radius_button)
        .bg(palette.accent_primary)
        .border(palette.border_width)
        .border_color(palette.border_color)
        .shadow(palette.button_shadows())
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(palette.text_on_accent)
        .when(is_inactive, |this| this.opacity(0.65).cursor_default())
        .when(!is_inactive, |this| {
            this.cursor_pointer()
                .hover(|style| {
                    style
                        .top(px(-1.5))
                        .bg(palette.accent_primary.opacity(0.95))
                        .border_color(palette.border_color)
                })
                .active(|style| style.top(px(4.0)).shadow(palette.button_active_shadows()))
                .on_click(on_click)
        })
        .child(label.into())
}

pub fn icon_action_button(
    id: impl Into<ElementId>,
    icon_name: &'static str,
    tooltip: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Button {
    let tooltip_str = tooltip.into().to_string();
    Button::new(id)
        .icon(icons::button_icon(icon_name).text_color(palette.text_secondary))
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
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .rounded(palette.radius_button)
        .bg(palette.bg_input)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.button_shadows())
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(palette.text_primary)
        .cursor_pointer()
        .hover(|style| {
            style
                .top(px(-1.5))
                .border_color(palette.accent_primary)
                .bg(palette.bg_input_focus)
        })
        .active(|style| style.top(px(4.0)).shadow(palette.button_active_shadows()))
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
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .px_4()
        .py_2()
        .rounded(palette.radius_button)
        .bg(palette.error.opacity(0.16))
        .border(palette.border_width_input)
        .border_color(palette.error)
        .shadow(palette.button_shadows())
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(palette.error)
        .cursor_pointer()
        .hover(|style| style.top(px(-1.5)).bg(palette.error.opacity(0.26)))
        .active(|style| style.top(px(4.0)).shadow(palette.button_active_shadows()))
        .child(label.into())
        .on_click(on_click)
}

pub fn compact_primary_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .gap_1p5()
        .px_4()
        .py_2()
        .rounded(px(18.0))
        .bg(palette.accent_primary)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.button_shadows())
        .text_xs()
        .font_weight(FontWeight::EXTRA_BOLD)
        .text_color(palette.text_on_accent)
        .cursor_pointer()
        .hover(|style| style.top(px(-1.5)).bg(palette.accent_primary.opacity(0.95)))
        .active(|style| style.top(px(3.0)).shadow(palette.button_active_shadows()))
        .child(label.into())
        .on_click(on_click)
}

pub fn pill_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    pill_button_colored(id, label, palette.text_primary, palette, on_click)
}

pub fn pill_button_colored(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    text_color: Rgba,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .gap_1p5()
        .px_3()
        .py_2()
        .rounded(px(15.0))
        .bg(palette.bg_badge)
        .border(px(2.0))
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .text_xs()
        .font_weight(FontWeight::EXTRA_BOLD)
        .text_color(text_color)
        .cursor_pointer()
        .hover(|style| {
            style
                .top(px(-1.0))
                .border_color(palette.accent_primary)
                .bg(palette.bg_elevated)
        })
        .active(|style| style.top(px(2.0)).shadow(palette.button_active_shadows()))
        .child(label.into())
        .on_click(on_click)
}

pub fn ghost_icon_button(
    id: impl Into<ElementId>,
    icon_name: &'static str,
    enabled: bool,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .relative()
        .w(px(36.0))
        .h(px(36.0))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(13.0))
        .bg(palette.bg_badge)
        .border(px(2.0))
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .when(!enabled, |this| this.opacity(0.38).cursor_default())
        .when(enabled, |this| {
            this.cursor_pointer()
                .hover(|s| {
                    s.top(px(-1.0))
                        .border_color(palette.accent_primary)
                        .bg(palette.bg_elevated)
                })
                .active(|s| s.top(px(2.0)))
                .on_click(on_click)
        })
        .child(
            icons::icon(icon_name)
                .size(px(14.0))
                .text_color(palette.text_primary),
        )
}
