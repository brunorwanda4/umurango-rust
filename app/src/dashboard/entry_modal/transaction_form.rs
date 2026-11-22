use crate::brand::ThemeColors;
use crate::components::ui::{
    date_picker_container, date_picker_input, money_input, select_container,
    select_field_with_action, select_input, stateful_input_container, stateful_textarea_container,
    text_input, textarea_input, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, WindowExt,
};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use gpui_kit::component::button::{Button, ButtonVariants};
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

    DialogContent::new()
        .gap_3()
        .child(
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
                ),
        )
        // Row 2: Account Select & DatePicker
        .child(
            div()
                .flex()
                .gap_3()
                .flex_wrap()
                // 4. Account (Mobile Money, Cash, Bank Account)
                .child(
                    div()
                        .flex_1()
                        .min_w(px(180.0))
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_secondary)
                                .child(text(lang, "modal.field_account")),
                        )
                        .child(select_container(
                            palette,
                            select_input(&dashboard.entry_account_select, palette)
                                .w_full()
                                .menu_width(px(220.0))
                                .placeholder(text(lang, "modal.field_account")),
                        )),
                )
                // 5. Date / Time (using GPUI Kit DatePicker component)
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
                                .child(text(lang, "modal.field_datetime")),
                        )
                        .child(date_picker_container(
                            palette,
                            date_picker_input(&dashboard.entry_date_picker, palette)
                                .w_full()
                                .placeholder(text(lang, "modal.datetime_ph")),
                        )),
                ),
        )
        // Row 3: Note (Textarea)
        .child(
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "modal.field_notes")),
                )
                .child(stateful_textarea_container(
                    palette,
                    &dashboard.entry_notes_input,
                    textarea_input(&dashboard.entry_notes_input, palette),
                )),
        )
        .into_any_element()
}

#[allow(dead_code)]
pub fn render_transaction_dialog(
    dashboard: &DashboardView,
    palette: ThemeColors,
    view_entity: Entity<DashboardView>,
    lang: Language,
    is_income: bool,
) -> AnyElement {
    render_transaction_form(dashboard, palette, view_entity, lang, is_income)
}

/// Opens a standalone transaction dialog using GPUI Kit's native declarative dialog API (`window.open_dialog`).
#[allow(dead_code)]
pub fn open_transaction_dialog(
    window: &mut Window,
    cx: &mut App,
    dashboard_view: Entity<DashboardView>,
    palette: ThemeColors,
    lang: Language,
    is_income: bool,
) {
    window.open_dialog(cx, move |dialog, _window, _cx| {
        let view = dashboard_view.clone();
        dialog
            .w(px(540.0))
            .overlay(true)
            .overlay_closable(true)
            .keyboard(true)
            .content(move |content, _window, cx| {
                let db = view.read(cx);
                let title = if is_income {
                    text(lang, "modal.income_title")
                } else {
                    text(lang, "modal.expense_title")
                };
                let desc = if is_income {
                    text(lang, "modal.field_category_income")
                } else {
                    text(lang, "modal.field_category_expense")
                };
                content
                    .child(
                        DialogHeader::new()
                            .child(DialogTitle::new().child(title))
                            .child(DialogDescription::new().child(desc)),
                    )
                    .child(render_transaction_form(
                        db,
                        palette,
                        view.clone(),
                        lang,
                        is_income,
                    ))
                    .child(
                        DialogFooter::new()
                            .child(
                                Button::new("transaction-dialog-cancel")
                                    .outline()
                                    .label(text(lang, "modal.cancel"))
                                    .on_click(|_, window, cx| {
                                        window.close_dialog(cx);
                                    }),
                            )
                            .child(
                                Button::new("transaction-dialog-save")
                                    .primary()
                                    .label(text(lang, "modal.save"))
                                    .on_click({
                                        let v = view.clone();
                                        move |_, window, cx| {
                                            v.update(cx, |this, cx| {
                                                this.submit_entry_modal(window, cx);
                                            });
                                            window.close_dialog(cx);
                                        }
                                    }),
                            ),
                    )
            })
    });
}
