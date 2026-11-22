#![allow(dead_code)]

use super::data::{
    format_rwf, format_signed_rwf, ApprovalItem, ChartMode, ExpenseCategoryRow, IncomeStreamRow,
    LoanItem, MemberContributionRow, MonthlyCashflowPoint, ProjectSummary, QuickEntryKind,
    TransactionRow,
};
use crate::brand::ThemeColors;
use crate::components::ui::{
    badge, card_elevated, card_surface, compact_primary_button, filter_pill_group, icon_badge_box,
    kawaii_progress_bar, member_avatar, pill_button, pill_button_colored, AvatarSize,
};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn treasury_hero_card<F>(
    total_treasury_label: impl Into<SharedString>,
    net_saved_label: impl Into<SharedString>,
    language: Language,
    palette: ThemeColors,
    on_quick_action: F,
) -> impl IntoElement
where
    F: Fn(QuickEntryKind, &mut Window, &mut App) + Clone + 'static,
{
    let act_income = on_quick_action.clone();
    let act_expense = on_quick_action.clone();
    let act_loan = on_quick_action.clone();
    let act_project = on_quick_action;

    card_surface(palette)
        .justify_between()
        .gap_4()
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .px_2p5()
                                .py_1()
                                .rounded_full()
                                .bg(palette.bg_badge)
                                .border(px(2.0))
                                .border_color(palette.border_color)
                                .child(
                                    icons::icon(icons::ICON_STAR)
                                        .size(px(11.0))
                                        .text_color(palette.accent_gold),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_secondary)
                                        .child(text(language, "cozy.hero_shared_pool")),
                                ),
                        )
                        .child(
                            div()
                                .text_2xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(total_treasury_label.into()),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.income_text)
                                .child(
                                    icons::icon(icons::ICON_ARROW_UP)
                                        .size(px(11.0))
                                        .text_color(palette.income_text),
                                )
                                .child(net_saved_label.into()),
                        ),
                )
                .child(
                    div()
                        .w(px(58.0))
                        .h(px(58.0))
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(20.0))
                        .bg(palette.bg_elevated)
                        .border(palette.border_width_input)
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                        .child(
                            icons::icon(icons::ICON_PIGGY_BANK)
                                .size(px(26.0))
                                .text_color(palette.accent_primary),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .flex_wrap()
                .gap_2()
                .child(
                    div()
                        .flex_1()
                        .min_w(px(105.0))
                        .child(compact_primary_button(
                            "hero-qa-income",
                            text(language, "cozy.qa_income"),
                            palette,
                            move |_e, w, cx| act_income(QuickEntryKind::Income, w, cx),
                        )),
                )
                .child(div().flex_1().min_w(px(105.0)).child(pill_button_colored(
                    "hero-qa-expense",
                    text(language, "cozy.qa_expense"),
                    palette.expense_text,
                    palette,
                    move |_e, w, cx| act_expense(QuickEntryKind::Expense, w, cx),
                )))
                .child(div().flex_1().min_w(px(95.0)).child(pill_button_colored(
                    "hero-qa-loan",
                    text(language, "cozy.qa_loan"),
                    palette.loan_text,
                    palette,
                    move |_e, w, cx| act_loan(QuickEntryKind::Loan, w, cx),
                )))
                .child(div().flex_1().min_w(px(95.0)).child(pill_button_colored(
                    "hero-qa-project",
                    text(language, "cozy.qa_project"),
                    palette.accent_primary,
                    palette,
                    move |_e, w, cx| act_project(QuickEntryKind::Project, w, cx),
                ))),
        )
}

pub fn pending_approvals_banner<F>(
    count: usize,
    first_item: Option<&ApprovalItem>,
    language: Language,
    palette: ThemeColors,
    on_review: F,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let subtitle = if let Some(item) = first_item {
        format!(
            "{} · {} ({})",
            item.requester,
            item.title,
            format_rwf(item.amount_rwf)
        )
    } else {
        text(language, "cozy.all_approvals_up_to_date")
    };

    card_surface(palette)
        .p_4()
        .flex_row()
        .items_center()
        .justify_between()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .min_w_0()
                .child(icon_badge_box(
                    icons::ICON_SHIELD_HALVED,
                    palette.loan_text,
                    palette.loan_bg,
                    palette.loan_border,
                    42.0,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(format!(
                                    "{count} {}",
                                    text(language, "cozy.pending_approvals_suffix")
                                )),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(subtitle),
                        ),
                ),
        )
        .child(pill_button(
            "home-review-approvals-btn",
            text(language, "cozy.review_arrow"),
            palette,
            on_review,
        ))
}

pub fn spending_donut_chart(categories: &[ExpenseCategoryRow]) -> impl IntoElement {
    let cats = categories.to_vec();
    canvas(
        |_bounds, _window, _cx| (),
        move |bounds, _, window, _cx| {
            use std::f32::consts::PI;

            let total: f32 = cats.iter().map(|c| c.pct as f32).sum();
            let total = if total <= 0.0 { 100.0 } else { total };

            let center = bounds.center();
            let center_x: f32 = center.x.into();
            let center_y: f32 = center.y.into();

            let r_out = 50.0_f32;
            let r_in = 33.0_f32;
            let cap_r = (r_out - r_in) / 2.0;

            let mut current_angle = -PI / 2.0;
            let gap = 0.08_f32;

            for cat in &cats {
                if cat.pct == 0 {
                    continue;
                }
                let fraction = (cat.pct as f32) / total;
                let delta = fraction * 2.0 * PI;

                let a1 = current_angle + gap / 2.0;
                let a2 = current_angle + delta - gap / 2.0;

                let large_arc = (a2 - a1) > PI;

                let x_out1 = center_x + r_out * a1.cos();
                let y_out1 = center_y + r_out * a1.sin();
                let x_out2 = center_x + r_out * a2.cos();
                let y_out2 = center_y + r_out * a2.sin();

                let x_in2 = center_x + r_in * a2.cos();
                let y_in2 = center_y + r_in * a2.sin();
                let x_in1 = center_x + r_in * a1.cos();
                let y_in1 = center_y + r_in * a1.sin();

                let mut builder = PathBuilder::fill();
                builder.move_to(point(px(x_out1), px(y_out1)));
                builder.arc_to(
                    point(px(r_out), px(r_out)),
                    px(0.0),
                    large_arc,
                    true,
                    point(px(x_out2), px(y_out2)),
                );
                builder.arc_to(
                    point(px(cap_r), px(cap_r)),
                    px(0.0),
                    false,
                    true,
                    point(px(x_in2), px(y_in2)),
                );
                builder.arc_to(
                    point(px(r_in), px(r_in)),
                    px(0.0),
                    large_arc,
                    false,
                    point(px(x_in1), px(y_in1)),
                );
                builder.arc_to(
                    point(px(cap_r), px(cap_r)),
                    px(0.0),
                    false,
                    true,
                    point(px(x_out1), px(y_out1)),
                );
                builder.close();

                if let Ok(path) = builder.build() {
                    window.paint_path(path, cat.color);
                }

                current_angle += delta;
            }
        },
    )
    .w(px(120.0))
    .h(px(120.0))
    .flex_none()
}

pub fn cashflow_insights_card<F>(
    mode: ChartMode,
    cashflow: &[MonthlyCashflowPoint],
    categories: &[ExpenseCategoryRow],
    language: Language,
    palette: ThemeColors,
    on_mode_change: F,
) -> impl IntoElement
where
    F: Fn(ChartMode, &mut Window, &mut App) + Clone + 'static,
{
    let pills = [
        (
            "chart-mode-cashflow",
            ChartMode::Cashflow,
            text(language, "cozy.chart_cashflow"),
        ),
        (
            "chart-mode-spending",
            ChartMode::Spending,
            text(language, "cozy.chart_spending"),
        ),
    ];

    let body: AnyElement = match mode {
        ChartMode::Cashflow => div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .h(px(148.0))
                    .px_2()
                    .pt_4()
                    .pb_2()
                    .rounded(px(18.0))
                    .bg(palette.bg_elevated)
                    .border(px(2.0))
                    .border_color(palette.border_color)
                    .flex()
                    .items_end()
                    .justify_around()
                    .gap_2()
                    .children(cashflow.iter().map(|pt| {
                        let inc_h = (pt.income_pct as f32).clamp(15.0, 96.0);
                        let exp_h = (pt.expense_pct as f32).clamp(12.0, 96.0);

                        div()
                            .flex_1()
                            .h_full()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_end()
                            .gap_1p5()
                            .child(
                                div()
                                    .flex()
                                    .items_end()
                                    .gap_1()
                                    .h(px(102.0))
                                    .child(
                                        div()
                                            .w(px(13.0))
                                            .h(px(inc_h))
                                            .rounded(px(6.0))
                                            .bg(palette.income_text)
                                            .border(px(1.5))
                                            .border_color(palette.border_color),
                                    )
                                    .child(
                                        div()
                                            .w(px(13.0))
                                            .h(px(exp_h))
                                            .rounded(px(6.0))
                                            .bg(palette.expense_text)
                                            .border(px(1.5))
                                            .border_color(palette.border_color),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(pt.month.clone()),
                            )
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .w(px(10.0))
                                    .h(px(10.0))
                                    .rounded(px(3.0))
                                    .bg(palette.income_text),
                            )
                            .child(
                                div()
                                    .text_color(palette.text_secondary)
                                    .child(text(language, "cozy.legend_income")),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .w(px(10.0))
                                    .h(px(10.0))
                                    .rounded(px(3.0))
                                    .bg(palette.expense_text),
                            )
                            .child(
                                div()
                                    .text_color(palette.text_secondary)
                                    .child(text(language, "cozy.legend_expenses")),
                            ),
                    ),
            )
            .into_any_element(),
        ChartMode::Spending => div()
            .flex()
            .items_center()
            .gap_6()
            .py_2()
            .child(
                div()
                    .w(px(120.0))
                    .h(px(120.0))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(spending_donut_chart(categories)),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(categories.iter().map(|cat| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2p5()
                                    .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(cat.color))
                                    .child(div().text_size(px(14.0)).child(cat.emoji))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(palette.text_primary)
                                            .child(cat.name.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_primary)
                                    .child(format!("{}%", cat.pct)),
                            )
                    })),
            )
            .into_any_element(),
    };

    card_surface(palette)
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .flex_wrap()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(text(language, "cozy.insights_title")),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(text(language, "cozy.insights_sub")),
                        ),
                )
                .child(filter_pill_group(&pills, mode, palette, on_mode_change)),
        )
        .child(body)
}

pub fn transaction_row(
    index: usize,
    transaction: &TransactionRow,
    palette: ThemeColors,
) -> impl IntoElement {
    let (badge_bg, badge_border, amount_color) = if transaction.is_loan {
        (palette.loan_bg, palette.loan_border, palette.loan_text)
    } else if transaction.is_income {
        (
            palette.income_bg,
            palette.income_border,
            palette.income_text,
        )
    } else {
        (
            palette.expense_bg,
            palette.expense_border,
            palette.expense_text,
        )
    };

    div()
        .id(("tx-row", index))
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .p_3()
        .rounded(px(18.0))
        .bg(palette.bg_elevated)
        .border(px(2.0))
        .border_color(palette.border_color)
        .hover(|s| {
            s.border_color(palette.accent_primary)
                .bg(palette.bg_input_focus)
        })
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .min_w_0()
                .child(icon_badge_box(
                    transaction.icon_name,
                    amount_color,
                    badge_bg,
                    badge_border,
                    38.0,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(transaction.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(palette.text_secondary)
                                .child(format!("{} · {}", transaction.actor, transaction.date)),
                        ),
                ),
        )
        .child(
            div().flex().items_center().gap_2().flex_shrink_0().child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(amount_color)
                    .child(transaction.amount.clone()),
            ),
        )
}

pub fn project_preview_row<F>(
    index: usize,
    project: &ProjectSummary,
    language: Language,
    palette: ThemeColors,
    on_boost: F,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    card_elevated(palette)
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2p5()
                        .min_w_0()
                        .child(icon_badge_box(
                            project.icon_name,
                            project.fill_color,
                            palette.bg_card,
                            palette.border_color,
                            36.0,
                        ))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .min_w_0()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(project.name.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(palette.text_secondary)
                                        .child(format!(
                                            "{} / {}",
                                            format_rwf(project.saved_rwf),
                                            format_rwf(project.target_rwf)
                                        )),
                                ),
                        ),
                )
                .child(pill_button_colored(
                    ("home-proj-boost", index),
                    text(language, "cozy.boost_50k"),
                    palette.income_text,
                    palette,
                    on_boost,
                )),
        )
        .child(kawaii_progress_bar(
            project.pct,
            project.fill_color,
            palette,
        ))
}

pub fn project_card(project: &ProjectSummary, palette: ThemeColors) -> impl IntoElement {
    let lang = Language::default();
    card_elevated(palette)
        .gap_2()
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(project.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(format!(
                                    "{} {} · {}",
                                    text(lang, "cozy.lead_prefix"),
                                    project.lead,
                                    project.deadline
                                )),
                        ),
                )
                .child(badge(project.progress.clone(), project.variant, palette)),
        )
        .child(kawaii_progress_bar(
            project.pct,
            project.fill_color,
            palette,
        ))
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .text_xs()
                .text_color(palette.text_secondary)
                .child(format!(
                    "{} {}",
                    text(lang, "cozy.saved_prefix"),
                    format_rwf(project.saved_rwf)
                ))
                .child(format!(
                    "{} {}",
                    text(lang, "cozy.goal_prefix"),
                    format_rwf(project.target_rwf)
                )),
        )
}

pub fn project_goal_card<B, D>(
    index: usize,
    project: &ProjectSummary,
    show_details: bool,
    language: Language,
    palette: ThemeColors,
    on_boost: B,
    on_toggle_details: D,
) -> impl IntoElement
where
    B: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    D: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    card_surface(palette)
        .gap_3()
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .min_w_0()
                        .child(icon_badge_box(
                            project.icon_name,
                            project.fill_color,
                            palette.bg_elevated,
                            palette.border_color,
                            46.0,
                        ))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .min_w_0()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(project.name.clone()),
                                )
                                .child(div().text_xs().text_color(palette.text_secondary).child(
                                    format!(
                                        "{} {} · {} {}",
                                        text(language, "cozy.lead_prefix"),
                                        project.lead,
                                        text(language, "cozy.target_prefix"),
                                        project.deadline
                                    ),
                                )),
                        ),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded_full()
                        .bg(palette.bg_elevated)
                        .border(px(2.0))
                        .border_color(palette.border_color)
                        .text_xs()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(project.fill_color)
                        .child(format!("{}%", project.pct)),
                ),
        )
        .child(kawaii_progress_bar(
            project.pct,
            project.fill_color,
            palette,
        ))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .child(div().text_color(palette.text_primary).child(format!(
                    "{} {}",
                    format_rwf(project.saved_rwf),
                    text(language, "cozy.saved_suffix")
                )))
                .child(div().text_color(palette.text_secondary).child(format!(
                    "{} {}",
                    text(language, "cozy.goal_prefix"),
                    format_rwf(project.target_rwf)
                ))),
        )
        .when(show_details, |this| {
            let remaining = (project.target_rwf - project.saved_rwf).max(0);
            this.child(
                card_elevated(palette)
                    .p_2p5()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.text_primary)
                            .child(format!(
                                "{} · {} {}",
                                project.contributors,
                                text(language, "cozy.remaining_to_goal"),
                                format_rwf(remaining)
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(palette.text_secondary)
                            .child(text(language, "cozy.multisig_protected_note")),
                    ),
            )
        })
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .pt_1()
                .child(div().flex_1().child(compact_primary_button(
                    ("proj-full-boost", index),
                    text(language, "cozy.quick_boost_50k"),
                    palette,
                    on_boost,
                )))
                .child(pill_button(
                    ("proj-full-details", index),
                    if show_details {
                        text(language, "cozy.hide_btn")
                    } else {
                        text(language, "cozy.details_btn")
                    },
                    palette,
                    on_toggle_details,
                )),
        )
}

pub fn approval_request_card<A, D>(
    index: usize,
    item: &ApprovalItem,
    language: Language,
    palette: ThemeColors,
    on_approve: A,
    on_decline: D,
) -> impl IntoElement
where
    A: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    D: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    card_elevated(palette)
        .gap_2p5()
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(item.title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(palette.text_secondary)
                                .child(format!(
                                    "{} {} · {}",
                                    text(language, "cozy.requested_by"),
                                    item.requester,
                                    item.category
                                )),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.expense_text)
                        .child(format_signed_rwf(item.amount_rwf, false)),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().flex_1().child(compact_primary_button(
                    ("approve-req-btn", index),
                    text(language, "cozy.approve_release_btn"),
                    palette,
                    on_approve,
                )))
                .child(pill_button(
                    ("decline-req-btn", index),
                    text(language, "cozy.decline_btn"),
                    palette,
                    on_decline,
                )),
        )
}

pub fn income_stream_card(
    index: usize,
    stream: &IncomeStreamRow,
    palette: ThemeColors,
) -> impl IntoElement {
    div()
        .id(("income-stream-card", index))
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .p_4()
        .rounded(px(24.0))
        .bg(palette.bg_card)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.button_shadows())
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .min_w_0()
                .child(icon_badge_box(
                    stream.icon_name,
                    palette.income_text,
                    palette.income_bg,
                    palette.income_border,
                    44.0,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(stream.name.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(palette.text_secondary)
                                .child(stream.cadence.clone()),
                        ),
                ),
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(palette.income_text)
                .child(format_signed_rwf(stream.amount_rwf, true)),
        )
}

pub fn expense_category_card(
    index: usize,
    cat: &ExpenseCategoryRow,
    palette: ThemeColors,
) -> impl IntoElement {
    let ratio_pct = if cat.budget_rwf <= 0 {
        100
    } else {
        ((cat.amount_rwf * 100) / cat.budget_rwf).clamp(0, 100) as u8
    };

    card_surface(palette)
        .id(("expense-cat-card", index))
        .p_4()
        .gap_2p5()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2p5()
                        .child(icon_badge_box(
                            cat.icon_name,
                            cat.color,
                            palette.bg_elevated,
                            palette.border_color,
                            38.0,
                        ))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(cat.name.clone()),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.expense_text)
                        .child(format!(
                            "{} / {}",
                            format_rwf(cat.amount_rwf),
                            format_rwf(cat.budget_rwf)
                        )),
                ),
        )
        .child(kawaii_progress_bar(ratio_pct, cat.color, palette))
}

pub fn loan_card<F>(
    index: usize,
    loan: &LoanItem,
    language: Language,
    palette: ThemeColors,
    on_repay: F,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let remaining = loan.remaining_rwf();
    let pct = loan.progress_pct();
    let (badge_bg, badge_border, badge_text_color, badge_label, fill_color) = if loan.is_lent {
        (
            palette.income_bg,
            palette.income_border,
            palette.income_text,
            text(language, "cozy.badge_lent"),
            palette.income_text,
        )
    } else {
        (
            palette.loan_bg,
            palette.loan_border,
            palette.loan_text,
            text(language, "cozy.badge_borrowed"),
            palette.loan_text,
        )
    };

    card_surface(palette)
        .id(("loan-card", index))
        .p_4()
        .gap_3()
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .min_w_0()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .px_2()
                                        .py(px(2.0))
                                        .rounded_full()
                                        .bg(badge_bg)
                                        .border(px(1.5))
                                        .border_color(badge_border)
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(badge_text_color)
                                        .child(badge_label),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(loan.person.clone()),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(format!(
                                    "{} · {} {}",
                                    loan.reason,
                                    text(language, "cozy.due_prefix"),
                                    loan.due
                                )),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_end()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(badge_text_color)
                                .child(format_rwf(remaining)),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(palette.text_secondary)
                                .child(text(language, "cozy.left_suffix")),
                        ),
                ),
        )
        .child(kawaii_progress_bar(pct, fill_color, palette))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_secondary)
                        .child(format!(
                            "{} {} / {} ({}%)",
                            text(language, "cozy.repaid_prefix"),
                            format_rwf(loan.paid_rwf),
                            format_rwf(loan.total_rwf),
                            pct
                        )),
                )
                .child(pill_button(
                    ("loan-repay-btn", index),
                    text(language, "cozy.repay_30k_btn"),
                    palette,
                    on_repay,
                )),
        )
}

pub fn member_card<R>(
    index: usize,
    member: &MemberContributionRow,
    language: Language,
    palette: ThemeColors,
    on_remind: R,
) -> impl IntoElement
where
    R: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let contact_text = member
        .phone
        .as_ref()
        .map(|p| p.to_string())
        .filter(|p| !p.trim().is_empty())
        .unwrap_or_else(|| "+250 788 000 000".to_string());

    card_surface(palette)
        .id(("member-card", index))
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_3()
                .child(member_avatar(
                    member.name.as_ref(),
                    Some(member.role.as_ref()),
                    AvatarSize::Lg,
                    palette,
                ))
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded_full()
                        .bg(palette.income_bg)
                        .border(px(1.5))
                        .border_color(palette.income_border)
                        .text_size(px(10.0))
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.income_text)
                        .child(member.status.clone()),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .pt_2()
                .border_t(palette.border_width_input)
                .border_color(palette.border_color)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            icons::icon(icons::ICON_PHONE)
                                .size(px(11.0))
                                .text_color(palette.text_secondary),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(palette.text_secondary)
                                .child(contact_text),
                        ),
                )
                .child(pill_button(
                    ("member-remind-btn", index),
                    text(language, "cozy.send_reminder_btn"),
                    palette,
                    on_remind,
                )),
        )
}

pub fn member_pledge_card<C, R>(
    index: usize,
    member: &MemberContributionRow,
    language: Language,
    palette: ThemeColors,
    on_log_contribution: C,
    on_remind: R,
) -> impl IntoElement
where
    C: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    R: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let pct = member.progress_pct();

    card_surface(palette)
        .id(("member-pledge-card", index))
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_3()
                .child(member_avatar(
                    member.name.as_ref(),
                    Some(member.role.as_ref()),
                    AvatarSize::Lg,
                    palette,
                ))
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded_full()
                        .bg(palette.income_bg)
                        .border(px(1.5))
                        .border_color(palette.income_border)
                        .text_size(px(10.0))
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.income_text)
                        .child(member.status.clone()),
                ),
        )
        .child(
            card_elevated(palette)
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_xs()
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_secondary)
                                .child(text(language, "cozy.monthly_contribution_label")),
                        )
                        .child(
                            div()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(format!(
                                    "{} / {}",
                                    format_rwf(member.contributed_rwf),
                                    format_rwf(member.monthly_goal_rwf)
                                )),
                        ),
                )
                .child(kawaii_progress_bar(pct, member.accent_color, palette)),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().flex_1().child(compact_primary_button(
                    ("member-contrib-btn", index),
                    text(language, "cozy.log_25k_btn"),
                    palette,
                    on_log_contribution,
                )))
                .child(pill_button(
                    ("member-remind-btn", index),
                    text(language, "cozy.send_reminder_btn"),
                    palette,
                    on_remind,
                )),
        )
}
