use super::types::category_palette_colors;
use crate::brand::ThemeColors;
use crate::components::ui::{stateful_input_container, text_input};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::component::input::InputState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn render_color_picker(
    palette: ThemeColors,
    selected_color: &str,
    cat_active_color: Rgba,
    color_input: &Entity<InputState>,
    view_entity: Entity<DashboardView>,
    lang: Language,
) -> Div {
    let view_color = view_entity;

    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "modal.category_color")),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .child(
                            div()
                                .size(px(18.0))
                                .rounded_full()
                                .bg(cat_active_color)
                                .border(px(1.5))
                                .border_color(palette.border_color)
                                .shadow(palette.button_shadows()),
                        )
                        .child(div().w(px(95.0)).child(stateful_input_container(
                            palette,
                            color_input,
                            text_input(color_input, palette),
                        ))),
                ),
        )
        .child(div().flex().items_center().gap_2().flex_wrap().children(
            category_palette_colors().into_iter().map(|(hex, rgba)| {
                let is_selected = selected_color.eq_ignore_ascii_case(hex);
                let vc = view_color.clone();
                let hex_str = hex.to_string();
                div()
                    .id(format!("color-swatch-{}", hex.replace('#', "")))
                    .cursor_pointer()
                    .size(px(20.0))
                    .rounded_full()
                    .bg(rgba)
                    .border(if is_selected { px(2.0) } else { px(1.0) })
                    .border_color(if is_selected {
                        palette.text_primary
                    } else {
                        palette.border_color
                    })
                    .shadow(palette.button_shadows())
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(is_selected, |swatch| {
                        swatch.child(
                            icons::icon(icons::ICON_CHECK)
                                .size(px(10.0))
                                .text_color(rgb(0xFFFFFF)),
                        )
                    })
                    .on_click(move |_e, window, cx| {
                        let h = hex_str.clone();
                        vc.update(cx, |this, cx| {
                            this.entry_category_color = h.clone();
                            this.entry_category_color_input.update(cx, |s, cx| {
                                s.set_value(h, window, cx);
                            });
                            cx.notify();
                        });
                    })
            }),
        ))
}
