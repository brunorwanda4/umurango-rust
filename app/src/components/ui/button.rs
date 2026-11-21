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

    if palette.is_minimalist() {
        return div()
            .id(id)
            .relative()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .px_4()
            .py_2()
            .rounded(palette.radius_button)
            .bg(palette.accent_primary)
            .border(palette.border_width_input)
            .border_color(palette.border_color)
            .shadow(palette.button_shadows())
            .text_sm()
            .font_weight(FontWeight::BOLD)
            .text_color(palette.text_on_accent)
            .when(is_inactive, |this| this.opacity(0.5).cursor_default())
            .when(!is_inactive, |this| {
                this.cursor_pointer()
                    .hover(|style| style.bg(palette.accent_secondary))
                    .active(|style| style.bg(palette.accent_rim).shadow(vec![]))
                    .on_click(on_click)
            })
            .child(label.into());
    }

    let rim_color = if palette.income_border == rgb(0x489974) {
        rgb(0x489974)
    } else {
        palette.accent_rim
    };

    let hover_bg =
        if palette.accent_primary == rgb(0x9BE0B8) || palette.accent_primary == rgb(0x8CE0B8) {
            rgb(0xA0EBC7)
        } else {
            palette.accent_secondary
        };

    let idle_shadow = vec![BoxShadow {
        color: rim_color.into(),
        offset: point(px(0.0), px(6.0)),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
        inset: false,
    }];

    let hover_shadow = vec![BoxShadow {
        color: rim_color.into(),
        offset: point(px(0.0), px(9.0)),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
        inset: false,
    }];

    let active_shadow = vec![BoxShadow {
        color: rim_color.into(),
        offset: point(px(0.0), px(2.0)),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
        inset: false,
    }];

    div()
        .id(id)
        .relative()
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .gap_2()
        .px_5()
        .py(px(13.0))
        .rounded_full()
        .bg(palette.accent_primary)
        .border(px(2.0))
        .border_color(rgb(0xFDF6EC))
        .shadow(idle_shadow)
        .text_sm()
        .font_weight(FontWeight::BOLD)
        .text_color(palette.text_on_accent)
        .when(is_inactive, |this| this.opacity(0.65).cursor_default())
        .when(!is_inactive, |this| {
            this.cursor_pointer()
                .hover(move |style| {
                    style
                        .top(px(-3.0))
                        .bg(hover_bg)
                        .border_color(rgb(0xFDF6EC))
                        .shadow(hover_shadow)
                })
                .active(move |style| style.top(px(4.0)).shadow(active_shadow))
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
    let (hover_offset, active_offset) = if palette.is_minimalist() {
        (px(0.0), px(0.0))
    } else {
        (px(-1.5), px(4.0))
    };
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
        .hover(move |style| {
            style
                .top(hover_offset)
                .border_color(palette.accent_primary)
                .bg(palette.bg_input_focus)
        })
        .active(move |style| {
            style
                .top(active_offset)
                .shadow(palette.button_active_shadows())
        })
        .child(label.into())
        .on_click(on_click)
}

pub fn destructive_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let (hover_offset, active_offset) = if palette.is_minimalist() {
        (px(0.0), px(0.0))
    } else {
        (px(-1.5), px(4.0))
    };
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
        .hover(move |style| style.top(hover_offset).bg(palette.error.opacity(0.26)))
        .active(move |style| {
            style
                .top(active_offset)
                .shadow(palette.button_active_shadows())
        })
        .child(label.into())
        .on_click(on_click)
}

pub fn compact_primary_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    if palette.is_minimalist() {
        return div()
            .id(id)
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .gap_1p5()
            .px_3()
            .py_1p5()
            .rounded(palette.radius_button)
            .bg(palette.accent_primary)
            .border(palette.border_width_input)
            .border_color(palette.border_color)
            .shadow(palette.button_shadows())
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(palette.text_on_accent)
            .cursor_pointer()
            .hover(|style| style.bg(palette.accent_secondary))
            .active(|style| style.bg(palette.accent_rim).shadow(vec![]))
            .child(label.into())
            .on_click(on_click);
    }

    let rim_color = if palette.income_border == rgb(0x489974) {
        rgb(0x489974)
    } else {
        palette.accent_rim
    };
    let hover_bg =
        if palette.accent_primary == rgb(0x9BE0B8) || palette.accent_primary == rgb(0x8CE0B8) {
            rgb(0xA0EBC7)
        } else {
            palette.accent_secondary
        };
    let idle_shadow = vec![BoxShadow {
        color: rim_color.into(),
        offset: point(px(0.0), px(4.0)),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
        inset: false,
    }];
    let hover_shadow = vec![BoxShadow {
        color: rim_color.into(),
        offset: point(px(0.0), px(6.0)),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
        inset: false,
    }];
    let active_shadow = vec![BoxShadow {
        color: rim_color.into(),
        offset: point(px(0.0), px(1.5)),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
        inset: false,
    }];

    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .gap_1p5()
        .px_4()
        .py_2()
        .rounded_full()
        .bg(palette.accent_primary)
        .border(px(2.0))
        .border_color(rgb(0xFDF6EC))
        .shadow(idle_shadow)
        .text_xs()
        .font_weight(FontWeight::EXTRA_BOLD)
        .text_color(palette.text_on_accent)
        .cursor_pointer()
        .hover(move |style| {
            style
                .top(px(-2.0))
                .bg(hover_bg)
                .border_color(rgb(0xFDF6EC))
                .shadow(hover_shadow)
        })
        .active(move |style| style.top(px(2.5)).shadow(active_shadow))
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
    let radius = if palette.is_minimalist() {
        palette.radius_button
    } else {
        px(15.0)
    };
    let border_w = if palette.is_minimalist() {
        palette.border_width_input
    } else {
        px(2.0)
    };
    let (hover_offset, active_offset) = if palette.is_minimalist() {
        (px(0.0), px(0.0))
    } else {
        (px(-1.0), px(2.0))
    };
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .gap_1p5()
        .px_3()
        .py_2()
        .rounded(radius)
        .bg(palette.bg_badge)
        .border(border_w)
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .text_xs()
        .font_weight(FontWeight::EXTRA_BOLD)
        .text_color(text_color)
        .cursor_pointer()
        .hover(move |style| {
            style
                .top(hover_offset)
                .border_color(palette.accent_primary)
                .bg(palette.bg_elevated)
        })
        .active(move |style| {
            style
                .top(active_offset)
                .shadow(palette.button_active_shadows())
        })
        .child(label.into())
        .on_click(on_click)
}

/// A true ghost icon button with completely transparent idle background, border, and shadow.
/// Displays a smooth subtle background only when hovered or pressed.
pub fn ghost_icon_button(
    id: impl Into<ElementId>,
    icon_name: &'static str,
    enabled: bool,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let radius = if palette.is_minimalist() {
        palette.radius_button
    } else {
        px(10.0)
    };
    let (hover_offset, active_offset) = if palette.is_minimalist() {
        (px(0.0), px(0.0))
    } else {
        (px(-0.5), px(1.0))
    };
    let hover_bg = if palette.mode == crate::brand::ThemeMode::Dark {
        palette.bg_elevated
    } else {
        palette.bg_badge
    };

    div()
        .id(id)
        .relative()
        .w(px(32.0))
        .h(px(32.0))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .rounded(radius)
        // True ghost styling: NO background, NO border, and NO shadow when idle!
        .when(!enabled, |this| this.opacity(0.35).cursor_default())
        .when(enabled, |this| {
            this.cursor_pointer()
                .hover(move |s| s.top(hover_offset).bg(hover_bg))
                .active(move |s| s.top(active_offset).bg(palette.bg_input_focus))
                .on_click(on_click)
        })
        .child(
            icons::icon(icon_name)
                .size(px(14.0))
                .text_color(if enabled {
                    palette.text_primary
                } else {
                    palette.text_muted
                }),
        )
}
