use crate::brand::ThemeColors;
use crate::components::ui::{
    money_input, pill_button_colored, stateful_input_container, text_input,
};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use gpui_kit::*;

pub fn render_loan_form(
    dashboard: &DashboardView,
    palette: ThemeColors,
    view_entity: Entity<DashboardView>,
    lang: Language,
) -> AnyElement {
    let view_toggle_lent = view_entity.clone();
    let is_lent = dashboard.entry_loan_is_lent;

    let lent_label = if is_lent {
        format!("✓ {}", text(lang, "modal.loan_lent_out"))
    } else {
        text(lang, "modal.loan_lent_out")
    };

    let borrowed_label = if !is_lent {
        format!("✓ {}", text(lang, "modal.loan_borrowed"))
    } else {
        text(lang, "modal.loan_borrowed")
    };

    div()
        .flex()
        .flex_col()
        .gap_3()
        // 1. Lent vs Borrowed switch
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(pill_button_colored(
                    "entry-loan-type-lent",
                    lent_label,
                    if is_lent {
                        palette.income_text
                    } else {
                        palette.text_secondary
                    },
                    palette,
                    {
                        let vt = view_toggle_lent.clone();
                        move |_e, _w, cx| {
                            vt.update(cx, |this, cx| {
                                this.entry_loan_is_lent = true;
                                cx.notify();
                            });
                        }
                    },
                ))
                .child(pill_button_colored(
                    "entry-loan-type-borrowed",
                    borrowed_label,
                    if !is_lent {
                        palette.loan_text
                    } else {
                        palette.text_secondary
                    },
                    palette,
                    move |_e, _w, cx| {
                        view_toggle_lent.update(cx, |this, cx| {
                            this.entry_loan_is_lent = false;
                            cx.notify();
                        });
                    },
                )),
        )
        // 2. Form Fields Row
        .child(
            div()
                .flex()
                .gap_3()
                .flex_wrap()
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
                                .child(text(lang, "modal.field_loan_reason")),
                        )
                        .child(stateful_input_container(
                            palette,
                            &dashboard.entry_title_input,
                            text_input(&dashboard.entry_title_input, palette),
                        )),
                )
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
                                .child(text(lang, "modal.field_counterparty")),
                        )
                        .child(stateful_input_container(
                            palette,
                            &dashboard.entry_secondary_input,
                            text_input(&dashboard.entry_secondary_input, palette),
                        )),
                )
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
                                .child(text(lang, "modal.field_loan_principal")),
                        )
                        .child(money_input(
                            "entry-loan-amount-input",
                            &dashboard.entry_amount_input,
                            "RWF",
                            palette,
                        )),
                ),
        )
        .into_any_element()
}
