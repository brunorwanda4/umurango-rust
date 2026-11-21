use std::rc::Rc;

use crate::brand::ThemeColors;
use crate::components::ui::{compact_primary_button, is_escape_key, pill_button};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn render_modal_header<C>(
    modal_icon: &'static str,
    modal_title: impl Into<SharedString>,
    is_category: bool,
    category_icon: &str,
    cat_active_color: Rgba,
    palette: ThemeColors,
    on_close: C,
) -> Div
where
    C: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .pb_2()
        .border_b(palette.border_width_input)
        .border_color(palette.border_color)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .when(is_category, |hdr| {
                    hdr.child(
                        div()
                            .size(px(28.0))
                            .rounded_full()
                            .bg(cat_active_color.opacity(0.20))
                            .border(px(1.5))
                            .border_color(cat_active_color)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                icons::safe_icon(category_icon)
                                    .size(px(14.0))
                                    .text_color(cat_active_color),
                            ),
                    )
                })
                .when(!is_category, |hdr| {
                    hdr.child(
                        icons::icon(modal_icon)
                            .size(px(16.0))
                            .text_color(palette.accent_primary),
                    )
                })
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.text_primary)
                        .child(modal_title.into()),
                ),
        )
        .child(
            div()
                .id("entry-modal-close-icon")
                .cursor_pointer()
                .p_1()
                .child(
                    icons::icon(icons::ICON_XMARK)
                        .size(px(14.0))
                        .text_color(palette.text_secondary),
                )
                .on_click(on_close),
        )
}

pub fn render_modal_footer<C, S>(
    cancel_text: impl Into<SharedString>,
    submit_text: impl Into<SharedString>,
    palette: ThemeColors,
    on_cancel: C,
    on_submit: S,
) -> Div
where
    C: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    S: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .flex()
        .items_center()
        .justify_end()
        .gap_2p5()
        .pt_2()
        .border_t(palette.border_width_input)
        .border_color(palette.border_color)
        .child(pill_button(
            "entry-modal-cancel-btn",
            cancel_text,
            palette,
            on_cancel,
        ))
        .child(compact_primary_button(
            "entry-modal-submit-btn",
            submit_text,
            palette,
            on_submit,
        ))
}

pub fn render_modal_card(
    header: Div,
    body: AnyElement,
    footer: Div,
    palette: ThemeColors,
) -> Stateful<Div> {
    div()
        .id("entry-modal-card")
        .w(px(520.0))
        .max_w_full()
        .p_4()
        .rounded(palette.radius_card)
        .bg(palette.bg_card)
        .border(px(1.5))
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .text_color(palette.text_primary)
        .flex()
        .flex_col()
        .gap_2p5()
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
        .child(header)
        .child(body)
        .child(footer)
}

pub fn render_modal_backdrop<C>(
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
        .id("entry-modal-backdrop")
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
