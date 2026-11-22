use crate::brand::ThemeColors;
use crate::components::dashboard::{
    cashflow_insights_card, format_rwf, pending_approvals_banner, project_preview_row,
    transaction_row, treasury_hero_card, ApprovalItem, ChartMode, ExpenseCategoryRow,
    IncomeStreamRow, MoneySubTab, MonthlyCashflowPoint, ProjectSummary, QuickEntryKind,
    TransactionRow,
};
use crate::components::layouts::AppSection;
use crate::components::ui::*;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

pub type SectionNavHandler = Rc<dyn Fn(AppSection, &mut Window, &mut App)>;
pub type MoneyTabNavHandler = Rc<dyn Fn(MoneySubTab, &mut Window, &mut App)>;
pub type QuickActionHandler = Rc<dyn Fn(QuickEntryKind, &mut Window, &mut App)>;
pub type ChartModeChangeHandler = Rc<dyn Fn(ChartMode, &mut Window, &mut App)>;
pub type BoostActionHandler = Rc<dyn Fn(usize, i64, &mut Window, &mut App)>;

pub struct HomePageProps<'a> {
    pub language: Language,
    pub palette: ThemeColors,
    pub stacked: bool,
    pub total_treasury: &'a str,
    pub monthly_income_rwf: i64,
    pub monthly_expenses_rwf: i64,
    pub monthly_contributions: &'a str,
    pub monthly_expenses: &'a str,
    pub income_streams: &'a [IncomeStreamRow],
    pub expense_categories: &'a [ExpenseCategoryRow],
    pub approvals: &'a [ApprovalItem],
    pub chart_mode: ChartMode,
    pub monthly_cashflow: &'a [MonthlyCashflowPoint],
    pub projects: &'a [ProjectSummary],
    pub transactions: &'a [TransactionRow],
    pub on_navigate_section: SectionNavHandler,
    pub on_navigate_money_tab: MoneyTabNavHandler,
    pub on_quick_action: QuickActionHandler,
    pub on_change_chart_mode: ChartModeChangeHandler,
    pub on_boost_project: BoostActionHandler,
}

pub fn home_page(props: HomePageProps<'_>) -> impl IntoElement {
    let HomePageProps {
        language: lang,
        palette,
        stacked,
        total_treasury,
        monthly_income_rwf,
        monthly_expenses_rwf,
        monthly_contributions,
        monthly_expenses,
        income_streams,
        expense_categories,
        approvals,
        chart_mode,
        monthly_cashflow,
        projects,
        transactions,
        on_navigate_section,
        on_navigate_money_tab,
        on_quick_action,
        on_change_chart_mode,
        on_boost_project,
    } = props;

    let net_saved_rwf = (monthly_income_rwf - monthly_expenses_rwf).max(0);
    let net_saved_label = format!(
        "+{} {}",
        format_rwf(net_saved_rwf),
        text(lang, "cozy.hero_net_saved_suffix")
    );

    let qa_cb = on_quick_action.clone();
    let hero_card = treasury_hero_card(
        total_treasury.to_string(),
        net_saved_label,
        lang,
        palette,
        move |kind, window, cx| {
            qa_cb(kind, window, cx);
        },
    );

    let tab_inc_cb = on_navigate_money_tab.clone();
    let income_summary_card = interactive_card("home-summary-income", palette, move |_e, w, cx| {
        tab_inc_cb(MoneySubTab::Income, w, cx);
    })
    .gap_2()
    .child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(icon_badge_box(
                icons::ICON_ARROW_DOWN,
                palette.income_text,
                palette.income_bg,
                palette.income_border,
                36.0,
            ))
            .child(
                div()
                    .px_2()
                    .py(px(1.0))
                    .rounded_full()
                    .bg(palette.income_bg)
                    .border(px(1.5))
                    .border_color(palette.income_border)
                    .text_size(px(10.0))
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(palette.income_text)
                    .child(format!("{} streams", income_streams.len())),
            ),
    )
    .child(
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(palette.text_secondary)
            .child(text(lang, "cozy.monthly_income")),
    )
    .child(
        div()
            .text_base()
            .font_weight(FontWeight::EXTRA_BOLD)
            .text_color(palette.income_text)
            .child(format!("+{}", monthly_contributions)),
    );

    let tab_exp_cb = on_navigate_money_tab.clone();
    let expense_summary_card =
        interactive_card("home-summary-expense", palette, move |_e, w, cx| {
            tab_exp_cb(MoneySubTab::Expenses, w, cx);
        })
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(icon_badge_box(
                    icons::ICON_ARROW_UP,
                    palette.expense_text,
                    palette.expense_bg,
                    palette.expense_border,
                    36.0,
                ))
                .child(
                    div()
                        .px_2()
                        .py(px(1.0))
                        .rounded_full()
                        .bg(palette.expense_bg)
                        .border(px(1.5))
                        .border_color(palette.expense_border)
                        .text_size(px(10.0))
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.expense_text)
                        .child(format!("{} cats", expense_categories.len())),
                ),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_secondary)
                .child(text(lang, "cozy.monthly_expenses")),
        )
        .child(
            div()
                .text_base()
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(palette.expense_text)
                .child(format!("-{}", monthly_expenses)),
        );

    let tab_ov_cb = on_navigate_money_tab.clone();
    let approvals_banner = pending_approvals_banner(
        approvals.len(),
        approvals.first(),
        lang,
        palette,
        move |_e, w, cx| {
            tab_ov_cb(MoneySubTab::Overview, w, cx);
        },
    );

    let top_row = div()
        .flex()
        .gap_4()
        .when(stacked, |this| this.flex_col())
        .child(div().flex_1().min_w_0().child(hero_card))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .when(stacked, |this| this.flex_col())
                        .child(div().flex_1().child(income_summary_card))
                        .child(div().flex_1().child(expense_summary_card)),
                )
                .child(approvals_banner),
        );

    let chart_cb = on_change_chart_mode.clone();
    let insights_card = cashflow_insights_card(
        chart_mode,
        monthly_cashflow,
        expense_categories,
        lang,
        palette,
        move |next_mode, w, cx| {
            chart_cb(next_mode, w, cx);
        },
    );

    let has_planned_projects = projects.iter().any(|p| !p.is_completed);
    let projects_preview_card = if has_planned_projects {
        let sec_proj_cb = on_navigate_section.clone();
        let boost_cb = on_boost_project.clone();
        Some(
            card_surface(palette)
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(text(lang, "cozy.planned_projects_title")),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.planned_projects_sub")),
                                ),
                        )
                        .child(pill_button(
                            "home-see-all-projects-btn",
                            text(lang, "cozy.see_all_arrow"),
                            palette,
                            move |_e, w, cx| {
                                sec_proj_cb(AppSection::Projects, w, cx);
                            },
                        )),
                )
                .child(
                    div().flex().flex_col().gap_2p5().children(
                        projects
                            .iter()
                            .enumerate()
                            .filter(|(_, p)| !p.is_completed)
                            .take(2)
                            .map(|(idx, proj)| {
                                let boost = boost_cb.clone();
                                project_preview_row(idx, proj, lang, palette, move |_e, w, cx| {
                                    boost(idx, 50_000, w, cx);
                                })
                            }),
                    ),
                ),
        )
    } else {
        None
    };

    let middle_row = div()
        .flex()
        .gap_4()
        .when(stacked, |this| this.flex_col())
        .child(div().flex_1().min_w_0().child(insights_card))
        .when_some(projects_preview_card, |this, card| {
            this.child(div().flex_1().min_w_0().child(card))
        });

    let nav_ov_cb = on_navigate_money_tab.clone();
    let recent_activity_card = card_surface(palette)
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(text(lang, "cozy.recent_activity_title")),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.recent_activity_sub")),
                        ),
                )
                .child(pill_button(
                    "home-full-ledger-btn",
                    text(lang, "cozy.full_ledger_arrow"),
                    palette,
                    move |_e, w, cx| {
                        nav_ov_cb(MoneySubTab::Overview, w, cx);
                    },
                )),
        )
        .child(
            div().flex().flex_col().gap_2().children(
                transactions
                    .iter()
                    .take(4)
                    .enumerate()
                    .map(|(idx, tx)| transaction_row(idx, tx, palette)),
            ),
        );

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(top_row)
        .child(middle_row)
        .child(recent_activity_card)
        .into_any_element()
}
