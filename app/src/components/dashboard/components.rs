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
        .rounded(palette.radius_input)
        .bg(palette.bg_input)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .hover(|s| {
            s.border_color(palette.accent_primary)
                .bg(palette.bg_input_focus)
        })
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .w(px(36.))
                        .h(px(36.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(palette.radius_checkbox)
                        .bg(palette.bg_badge)
                        .border(px(2.0))
                        .border_color(palette.border_color)
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
                                palette.accent_primary
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
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_primary)
                                .child(title.clone()),
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
                .child(badge(category.clone(), *category_variant, palette))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if *is_income {
                            palette.success
                        } else {
                            palette.text_primary
                        })
                        .child(amount.clone()),
                )
                .child(badge(status.clone(), *status_variant, palette)),
        )
}

pub fn project_card(project: &ProjectSummary, palette: ThemeColors) -> impl IntoElement {
    div()
        .p_3()
        .rounded(palette.radius_input)
        .bg(palette.bg_input)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
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
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_primary)
                                .child(project.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(project.target.clone()),
                        ),
                )
                .child(badge(project.progress.clone(), project.variant, palette)),
        )
        .child(
            div()
                .w_full()
                .h(px(10.))
                .rounded_full()
                .bg(palette.bg_canvas)
                .border(px(1.5))
                .border_color(palette.border_color)
                .child(div().h_full().w(px(240.)).rounded_full().bg(
                    if project.variant == BadgeVariant::Success {
                        palette.success
                    } else {
                        palette.accent_primary
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
                .child(project.contributors.clone())
                .child(project.deadline.clone()),
        )
}
