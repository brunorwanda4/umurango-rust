#![allow(dead_code)]

use std::rc::Rc;

use crate::brand::ThemeColors;
use crate::icons;
pub use gpui_kit::component::dialog::{
    Dialog, DialogAction, DialogButtonProps, DialogClose, DialogContent, DialogDescription,
    DialogEntrance, DialogFooter, DialogHeader, DialogTitle,
};
pub use gpui_kit::component::WindowExt;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Checks if a keystroke key string corresponds to the Escape key.
pub fn is_escape_key(key: &str) -> bool {
    key.eq_ignore_ascii_case("escape") || key.eq_ignore_ascii_case("esc")
}

/// Creates a styled dialog container adhering to the gpui-kit dialog specification.
pub fn dialog_container(id: impl Into<ElementId>, palette: ThemeColors) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .flex_col()
        .w(px(520.0))
        .max_w_full()
        .p_4()
        .rounded(palette.radius_card)
        .bg(palette.bg_card)
        .border(px(1.5))
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .text_color(palette.text_primary)
}

/// Creates a styled dialog header with theme-appropriate spacing and divider.
pub fn dialog_header(palette: ThemeColors) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .pb_2()
        .border_b(palette.border_width_input)
        .border_color(palette.border_color)
}

/// Creates a styled dialog title.
pub fn dialog_title(title: impl Into<SharedString>, palette: ThemeColors) -> Div {
    div()
        .text_sm()
        .font_weight(FontWeight::EXTRA_BOLD)
        .text_color(palette.text_primary)
        .child(title.into())
}

/// Creates a styled dialog description with muted styling.
pub fn dialog_description(description: impl Into<SharedString>, palette: ThemeColors) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::NORMAL)
        .text_color(palette.text_secondary)
        .child(description.into())
}

/// Creates a styled dialog content area.
pub fn dialog_content() -> Div {
    div().flex().flex_col().gap_2p5().w_full()
}

/// Creates a styled dialog footer container.
pub fn dialog_footer(palette: ThemeColors) -> Div {
    div()
        .flex()
        .items_center()
        .justify_end()
        .gap_2p5()
        .pt_2()
        .border_t(palette.border_width_input)
        .border_color(palette.border_color)
}

/// Creates a styled dialog overlay backdrop that closes when clicking outside or pressing the Esc key.
pub fn dialog_backdrop<C>(
    card: impl IntoElement,
    palette: ThemeColors,
    on_close: C,
) -> Stateful<Div>
where
    C: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    let on_close = Rc::new(on_close);
    let on_close_click = on_close.clone();
    let on_close_key = on_close;

    div()
        .id("dialog-backdrop")
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .bg(palette.bg_canvas.opacity(0.85))
        .p_4()
        .on_key_down(move |event: &KeyDownEvent, window, cx| {
            if is_escape_key(event.keystroke.key.as_str()) {
                on_close_key(&MouseDownEvent::default(), window, cx);
            }
        })
        .on_mouse_down(MouseButton::Left, move |e, window, cx| {
            on_close_click(e, window, cx);
        })
        .child(card)
}

/// Creates a styled dialog overlay backdrop with a simplified action callback that closes on outside click or Esc key.
pub fn dialog_backdrop_action<C>(
    card: impl IntoElement,
    palette: ThemeColors,
    on_close: C,
) -> Stateful<Div>
where
    C: Fn(&mut Window, &mut App) + 'static,
{
    let on_close = Rc::new(on_close);
    let on_close_click = on_close.clone();
    let on_close_key = on_close;

    div()
        .id("dialog-backdrop")
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .bg(palette.bg_canvas.opacity(0.85))
        .p_4()
        .on_key_down(move |event: &KeyDownEvent, window, cx| {
            if is_escape_key(event.keystroke.key.as_str()) {
                on_close_key(window, cx);
            }
        })
        .on_mouse_down(MouseButton::Left, move |_e, window, cx| {
            on_close_click(window, cx);
        })
        .child(card)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[core::prelude::v1::test]
    fn test_is_escape_key() {
        assert!(is_escape_key("escape"));
        assert!(is_escape_key("Escape"));
        assert!(is_escape_key("esc"));
        assert!(is_escape_key("ESC"));
        assert!(!is_escape_key("enter"));
        assert!(!is_escape_key("tab"));
        assert!(!is_escape_key("space"));
    }
}
