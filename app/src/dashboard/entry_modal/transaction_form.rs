use crate::brand::ThemeColors;
use crate::components::ui::{
    money_input, select_field_with_action, select_input, stateful_input_container, text_input,
};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
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

    let view_add_cat = view_entity;

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
        // 2. Category with "+ Category" button and styled Select dropdown
        .child(
            div()
                .flex_1()
                .min_w(px(180.0))
                .child(select_field_with_action(
                    field2_label,
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
                    palette,
                    select_input(&dashboard.entry_category_select, palette)
                        .w_full()
                        .menu_width(px(240.0))
                        .placeholder(if is_income {
                            text(lang, "modal.income_cat_ph")
                        } else {
                            text(lang, "modal.expense_cat_ph")
                        }),
                )),
        )
        // 3. Amount (using MoneyInput via gpui-kit InputGroup)
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
                .child(money_input(
                    "entry-amount-input",
                    &dashboard.entry_amount_input,
                    "RWF",
                    palette,
                )),
        )
        .into_any_element()
}
