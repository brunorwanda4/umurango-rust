use crate::brand::ThemeColors;
use crate::components::ui::{stateful_input_container, text_input};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn render_transaction_form(
    dashboard: &DashboardView,
    palette: ThemeColors,
    view_entity: Entity<DashboardView>,
    lang: Language,
    is_income: bool,
) -> AnyElement {
    let (field1_label, field2_label, field3_label) = if is_income {
        (
            text(lang, "modal.field_desc_title"),
            text(lang, "modal.field_category_income"),
            text(lang, "modal.field_amount_rwf"),
        )
    } else {
        (
            text(lang, "modal.field_desc_purpose"),
            text(lang, "modal.field_category_expense"),
            text(lang, "modal.field_amount_rwf"),
        )
    };

    let view_add_cat = view_entity.clone();
    let avail_cats = dashboard.get_available_categories(is_income);

    div()
        .flex()
        .gap_3()
        .flex_wrap()
        // 1. Description / Title
        .child(
            div()
                .flex_1()
                .min_w(px(200.0))
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(field1_label),
                )
                .child(stateful_input_container(
                    palette,
                    &dashboard.entry_title_input,
                    text_input(&dashboard.entry_title_input, palette),
                )),
        )
        // 2. Category with "+ Category" button and quick pills
        .child(
            div()
                .flex_1()
                .min_w(px(180.0))
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
                                .child(field2_label),
                        )
                        .child(
                            div()
                                .id("btn-open-category-dialog")
                                .cursor_pointer()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(palette.accent_primary.opacity(0.12))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.accent_primary)
                                .hover(|s| {
                                    s.bg(palette.accent_primary)
                                        .text_color(palette.text_on_accent)
                                })
                                .child(text(lang, "modal.add_category_btn"))
                                .on_click(move |_e, window, cx| {
                                    view_add_cat.update(cx, |this, cx| {
                                        this.open_category_modal(is_income, window, cx);
                                    });
                                }),
                        ),
                )
                .child(stateful_input_container(
                    palette,
                    &dashboard.entry_secondary_input,
                    text_input(&dashboard.entry_secondary_input, palette),
                ))
                .when(!avail_cats.is_empty(), |col| {
                    let chips_view = view_entity.clone();
                    col.child(div().flex().flex_wrap().gap_1().pt_1().children(
                        avail_cats.into_iter().take(5).map(|cat| {
                            let cv = chips_view.clone();
                            let cat_val = cat.clone();
                            div()
                                .id(format!("cat-pill-{}", cat.replace(' ', "-")))
                                .cursor_pointer()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(palette.bg_elevated)
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .hover(|s| {
                                    s.bg(palette.accent_primary.opacity(0.12))
                                        .text_color(palette.accent_primary)
                                })
                                .child(cat)
                                .on_click(move |_e, window, cx| {
                                    let c = cat_val.clone();
                                    cv.update(cx, |this, cx| {
                                        this.entry_secondary_input.update(cx, |state, cx| {
                                            state.set_value(c, window, cx);
                                        });
                                    });
                                })
                        }),
                    ))
                }),
        )
        // 3. Amount
        .child(
            div()
                .flex_1()
                .min_w(px(150.0))
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(field3_label),
                )
                .child(stateful_input_container(
                    palette,
                    &dashboard.entry_amount_input,
                    text_input(&dashboard.entry_amount_input, palette),
                )),
        )
        .into_any_element()
}
