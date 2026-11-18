use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn modal_backdrop(palette: ThemeColors) -> Div {
    div()
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .bg(palette.bg_canvas.opacity(0.8))
        .p_6()
}

pub fn modal_card(id: impl Into<ElementId>, palette: ThemeColors) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .w(px(480.))
        .max_w_full()
        .p_6()
        .rounded(palette.radius_card)
        .bg(palette.bg_card)
        .border(palette.border_width)
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .text_color(palette.text_primary)
}

pub fn modal_header(
    title: impl Into<SharedString>,
    palette: ThemeColors,
    on_close: Option<impl Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .mb_4()
        .pb_3()
        .border_b(palette.border_width_input)
        .border_color(palette.border_color)
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_primary)
                .child(title.into()),
        )
        .when_some(on_close, |this, close_fn| {
            this.child(
                div()
                    .id("modal-close-icon")
                    .cursor_pointer()
                    .p_1()
                    .child(
                        icons::icon(icons::ICON_XMARK)
                            .size(px(14.))
                            .text_color(palette.text_muted),
                    )
                    .on_click(close_fn),
            )
        })
}

pub fn modal_footer(palette: ThemeColors) -> Div {
    div()
        .flex()
        .items_center()
        .justify_end()
        .gap_3()
        .mt_5()
        .pt_3()
        .border_t(palette.border_width_input)
        .border_color(palette.border_color)
}
