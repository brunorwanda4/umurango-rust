use super::data::{ProjectSummary, TransactionRow};
use crate::brand::ThemeColors;
use crate::components::ui::{badge, BadgeVariant};
use crate::icons;
use gpui_kit::*;

pub fn transaction_row(
    index: usize,
    transaction: &TransactionRow,
    palette: ThemeColors,
) -> impl IntoElement {
    let TransactionRow {
        actor,
        title,
        date,
        amount,
        is_income,
        category,
        category_variant,
        status,
        status_variant,
    } = transaction;

    div()
        .id(("tx-row", index))
        .flex()
        .items_center()
        .justify_between()
        .p_3()
        .rounded_lg()
        .bg(palette.card)
        .border_1()
        .border_color(palette.border)
        .hover(|s| s.border_color(palette.border_hover))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .w(px(34.))
                        .h(px(34.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(if *is_income {
                            palette.success.opacity(0.15)
                        } else {
                            palette.primary.opacity(0.15)
                        })
                        .child(
                            icons::icon(if *is_income {
                                icons::ICON_ARROW_DOWN
                            } else {
                                icons::ICON_ARROW_UP
                            })
                            .size(px(14.))
                            .text_color(if *is_income {
                                palette.success
                            } else {
                                palette.primary_light
                            }),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(palette.text_primary)
                                .child(*title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(format!("{actor} • {date}")),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(badge(*category, *category_variant, palette))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if *is_income {
                            palette.success
                        } else {
                            palette.text_primary
                        })
                        .child(*amount),
                )
                .child(badge(*status, *status_variant, palette)),
        )
}

pub fn project_card(project: &ProjectSummary, palette: ThemeColors) -> impl IntoElement {
    div()
        .p_3()
        .rounded_lg()
        .bg(palette.card)
        .border_1()
        .border_color(palette.border)
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .mb_2()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(palette.text_primary)
                                .child(project.name),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(project.target),
                        ),
                )
                .child(badge(project.progress, project.variant, palette)),
        )
        .child(
            div()
                .w_full()
                .h(px(8.))
                .rounded_full()
                .bg(palette.surface)
                .border_1()
                .border_color(palette.border)
                .child(div().h_full().w(px(240.)).rounded_full().bg(
                    if project.variant == BadgeVariant::Success {
                        palette.success
                    } else {
                        palette.primary
                    },
                )),
        )
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .mt_2()
                .text_xs()
                .text_color(palette.text_muted)
                .child(project.contributors)
                .child(project.deadline),
        )
}
