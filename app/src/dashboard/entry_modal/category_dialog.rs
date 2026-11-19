use super::types::{category_palette_colors, parse_color_hex, CATEGORY_FONTAWESOME_ICONS};
use crate::brand::ThemeColors;
use crate::components::ui::{
    pill_button_colored, stateful_input_container, stateful_textarea_container, text_input,
    textarea_input,
};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn render_category_dialog(
    dashboard: &DashboardView,
    palette: ThemeColors,
    view_entity: Entity<DashboardView>,
    lang: Language,
) -> AnyElement {
    let view_toggle_cat_inc = view_entity.clone();
    let view_toggle_cat_exp = view_entity.clone();
    let view_color = view_entity.clone();
    let view_icon = view_entity.clone();

    let cat_is_income = dashboard.entry_category_is_income;
    let cat_active_color = parse_color_hex(&dashboard.entry_category_color);
    let selected_color = dashboard.entry_category_color.clone();
    let selected_icon = dashboard.entry_category_icon.clone();

    let cat_income_label = if cat_is_income {
        format!("✓ {}", text(lang, "modal.category_income"))
    } else {
        text(lang, "modal.category_income")
    };

    let cat_expense_label = if !cat_is_income {
        format!("✓ {}", text(lang, "modal.category_expense"))
    } else {
        text(lang, "modal.category_expense")
    };

    div()
        .flex()
        .flex_col()
        .gap_2()
        // 1. Type Switcher
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(pill_button_colored(
                    "entry-cat-type-income",
                    cat_income_label,
                    if cat_is_income {
                        palette.income_text
                    } else {
                        palette.text_secondary
                    },
                    palette,
                    {
                        let vt = view_toggle_cat_inc.clone();
                        move |_e, _w, cx| {
                            vt.update(cx, |this, cx| {
                                this.entry_category_is_income = true;
                                cx.notify();
                            });
                        }
                    },
                ))
                .child(pill_button_colored(
                    "entry-cat-type-expense",
                    cat_expense_label,
                    if !cat_is_income {
                        palette.expense_text
                    } else {
                        palette.text_secondary
                    },
                    palette,
                    move |_e, _w, cx| {
                        view_toggle_cat_exp.update(cx, |this, cx| {
                            this.entry_category_is_income = false;
                            cx.notify();
                        });
                    },
                )),
        )
        // 2. Category Name
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "modal.category_name")),
                )
                .child(stateful_input_container(
                    palette,
                    &dashboard.entry_title_input,
                    text_input(&dashboard.entry_title_input, palette),
                )),
        )
        // 3. Color Picker Swatches
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "modal.category_color")),
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
                            .on_click(move |_e, _w, cx| {
                                let h = hex_str.clone();
                                vc.update(cx, |this, cx| {
                                    this.entry_category_color = h;
                                    cx.notify();
                                });
                            })
                    }),
                )),
        )
        // 4. FontAwesome Icon Picker
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
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
                                .child(text(lang, "modal.category_icon")),
                        )
                        .child(div().w(px(130.0)).child(stateful_input_container(
                            palette,
                            &dashboard.entry_category_icon_input,
                            text_input(&dashboard.entry_category_icon_input, palette),
                        ))),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_1()
                        .p_1()
                        .rounded(palette.radius_input)
                        .bg(palette.bg_elevated)
                        .border(palette.border_width_input)
                        .border_color(palette.border_color)
                        .children(CATEGORY_FONTAWESOME_ICONS.into_iter().map(|icon_name| {
                            let is_selected = selected_icon.eq_ignore_ascii_case(icon_name);
                            let vi = view_icon.clone();
                            let ic_name = icon_name.to_string();
                            div()
                                .id(format!("fa-icon-choice-{icon_name}"))
                                .cursor_pointer()
                                .size(px(25.0))
                                .rounded(px(5.0))
                                .bg(if is_selected {
                                    cat_active_color.opacity(0.20)
                                } else {
                                    palette.bg_card
                                })
                                .border(if is_selected { px(1.5) } else { px(1.0) })
                                .border_color(if is_selected {
                                    cat_active_color
                                } else {
                                    palette.border_color
                                })
                                .hover(|s| s.border_color(cat_active_color))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(icons::safe_icon(icon_name).size(px(11.0)).text_color(
                                    if is_selected {
                                        cat_active_color
                                    } else {
                                        palette.text_secondary
                                    },
                                ))
                                .on_click(move |_e, window, cx| {
                                    let name = ic_name.clone();
                                    vi.update(cx, |this, cx| {
                                        this.entry_category_icon = name.clone();
                                        this.entry_category_icon_input.update(cx, |state, cx| {
                                            state.set_value(name, window, cx);
                                        });
                                        cx.notify();
                                    });
                                })
                        })),
                ),
        )
        // 5. Description Textarea
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "modal.category_desc")),
                )
                .child(stateful_textarea_container(
                    palette,
                    &dashboard.entry_category_desc_input,
                    textarea_input(&dashboard.entry_category_desc_input, palette),
                )),
        )
        .into_any_element()
}
