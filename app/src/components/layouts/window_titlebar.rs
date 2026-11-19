use crate::brand::{brand_logo, colors, StyleTokens, ThemeMode};
use crate::i18n::{text, Language};
use crate::icons;
use crate::theme::UiStyle;
use gpui_kit::component::InteractiveElementExt;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub const WINDOW_TITLEBAR_HEIGHT: Pixels = px(44.0);

pub fn window_control_radius(style: UiStyle) -> Pixels {
    match style {
        UiStyle::CozyKawaii => px(10.0),
        UiStyle::NeoBrutalism => px(4.0),
        UiStyle::Claymorphism3D => px(12.0),
    }
}

pub fn maximize_icon_for_state(is_maximized: bool) -> &'static str {
    if is_maximized {
        icons::ICON_WINDOW_RESTORE
    } else {
        icons::ICON_WINDOW_MAXIMIZE
    }
}

fn window_control_button(
    id: &'static str,
    icon_name: &'static str,
    control_area: WindowControlArea,
    is_danger: bool,
    palette: StyleTokens,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let radius = window_control_radius(palette.style);

    let (base_bg, base_border, base_text, hover_bg, hover_border, hover_text) = if is_danger {
        (
            palette.expense_bg,
            palette.expense_border,
            palette.expense_text,
            palette.error,
            palette.border_color,
            palette.text_on_accent,
        )
    } else {
        (
            palette.bg_card,
            palette.border_color,
            palette.text_primary,
            palette.accent_primary,
            palette.border_color,
            palette.text_on_accent,
        )
    };

    let active_offset = match palette.style {
        UiStyle::CozyKawaii | UiStyle::NeoBrutalism => px(1.5),
        UiStyle::Claymorphism3D => px(1.0),
    };

    div()
        .id(id)
        .window_control_area(control_area)
        .cursor_pointer()
        .w(px(30.0))
        .h(px(28.0))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .rounded(radius)
        .bg(base_bg)
        .border(px(2.0))
        .border_color(base_border)
        .shadow(palette.checkbox_shadows())
        .text_color(base_text)
        .hover(move |s| {
            s.bg(hover_bg)
                .border_color(hover_border)
                .text_color(hover_text)
        })
        .active(move |s| {
            s.top(active_offset)
                .bg(hover_bg)
                .text_color(hover_text)
                .shadow(vec![])
        })
        .on_mouse_down(MouseButton::Left, |_, window, cx| {
            window.prevent_default();
            cx.stop_propagation();
        })
        .on_click(move |event, window, cx| {
            cx.stop_propagation();
            on_click(event, window, cx);
        })
        .child(icons::icon(icon_name).size(px(12.0)))
}

pub fn app_window_titlebar(
    theme: ThemeMode,
    _language: Language,
    is_maximized: bool,
) -> impl IntoElement {
    let palette = colors(theme);
    let badge_radius = match palette.style {
        UiStyle::CozyKawaii => px(999.0),
        UiStyle::NeoBrutalism => px(4.0),
        UiStyle::Claymorphism3D => px(12.0),
    };
    let icon_box_radius = window_control_radius(palette.style);
    let max_icon = maximize_icon_for_state(is_maximized);

    div()
        .id("umuryango-window-titlebar")
        .w_full()
        .h(WINDOW_TITLEBAR_HEIGHT)
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_between()
        .px_3p5()
        .bg(palette.bg_nav)
        .border_b(px(2.5))
        .border_color(palette.border_color)
        .font_family(palette.font_family)
        .on_double_click(|_, window, _| {
            window.zoom_window();
        })
        .child(
            div()
                .id("window-titlebar-drag-zone")
                .window_control_area(WindowControlArea::Drag)
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .gap_2p5()
                .on_mouse_down(MouseButton::Left, |_, window, _| {
                    window.start_window_move();
                })
                .on_mouse_down(MouseButton::Right, |ev, window, _| {
                    window.show_window_menu(ev.position);
                })
                .child(
                    div()
                        .w(px(28.0))
                        .h(px(28.0))
                        .rounded(icon_box_radius)
                        .bg(palette.bg_card)
                        .border(px(2.0))
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(brand_logo(theme, px(16.0))),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.text_primary)
                        .child("Umuryango"),
                )
                .child(
                    div()
                        .px_2()
                        .py(px(2.0))
                        .rounded(badge_radius)
                        .bg(palette.bg_card)
                        .border(px(1.5))
                        .border_color(palette.border_color)
                        .text_size(px(9.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(palette.style.label()),
                ),
        )
        .child(
            div()
                .id("window-titlebar-controls")
                .flex()
                .items_center()
                .gap_2()
                .child(window_control_button(
                    "window-btn-minimize",
                    icons::ICON_MINUS,
                    WindowControlArea::Min,
                    false,
                    palette,
                    |_e, window, _cx| {
                        window.minimize_window();
                    },
                ))
                .child(window_control_button(
                    "window-btn-maximize",
                    max_icon,
                    WindowControlArea::Max,
                    false,
                    palette,
                    |_e, window, _cx| {
                        window.zoom_window();
                    },
                ))
                .child(window_control_button(
                    "window-btn-close",
                    icons::ICON_XMARK,
                    WindowControlArea::Close,
                    true,
                    palette,
                    |_e, window, _cx| {
                        window.remove_window();
                    },
                )),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[core::prelude::v1::test]
    fn window_control_geometry_and_icons_follow_style_and_maximize_state() {
        assert_eq!(window_control_radius(UiStyle::CozyKawaii), px(10.0));
        assert_eq!(window_control_radius(UiStyle::NeoBrutalism), px(4.0));
        assert_eq!(window_control_radius(UiStyle::Claymorphism3D), px(12.0));

        assert_eq!(maximize_icon_for_state(false), icons::ICON_WINDOW_MAXIMIZE);
        assert_eq!(maximize_icon_for_state(true), icons::ICON_WINDOW_RESTORE);
    }
}
