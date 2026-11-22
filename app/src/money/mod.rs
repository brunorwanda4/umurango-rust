use crate::brand::ThemeColors;
use crate::components::dashboard::{
    approval_request_card, expense_category_card, format_compact_signed_rwf, format_signed_rwf,
    income_stream_card, loan_card, transaction_row, ApprovalItem, ChartMode, ExpenseCategoryRow,
    IncomeStreamRow, LoanFilter, LoanItem, MoneySubTab, MonthlyCashflowPoint, TransactionRow,
};
use crate::components::ui::*;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::component::input::InputState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

pub type SubTabSelectHandler = Rc<dyn Fn(MoneySubTab, &mut Window, &mut App)>;
pub type ModalOpenHandler =
    Rc<dyn Fn(crate::components::dashboard::EntryModalKind, &mut Window, &mut App)>;
pub type CategoryModalHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;
pub type ApprovalResolveHandler = Rc<dyn Fn(usize, bool, &mut Window, &mut App)>;
pub type LoanFilterHandler = Rc<dyn Fn(LoanFilter, &mut Window, &mut App)>;
pub type LoanRepayHandler = Rc<dyn Fn(usize, i64, &mut Window, &mut App)>;

pub struct MoneyPageProps<'a> {
    pub language: Language,
    pub palette: ThemeColors,
    pub stacked: bool,
    pub sub_tab: MoneySubTab,
    pub loans: &'a [LoanItem],
    pub monthly_income_rwf: i64,
    pub monthly_expenses_rwf: i64,
    pub total_treasury: &'a str,
    pub monthly_contributions: &'a str,
    pub monthly_expenses: &'a str,
    pub income_streams: &'a [IncomeStreamRow],
    pub expense_categories: &'a [ExpenseCategoryRow],
    pub monthly_cashflow: &'a [MonthlyCashflowPoint],
    pub approvals: &'a [ApprovalItem],
    pub transactions: &'a [TransactionRow],
    pub loan_filter: LoanFilter,
    pub chart_mode: ChartMode,
    pub search_state: &'a Entity<InputState>,
    pub on_select_sub_tab: SubTabSelectHandler,
    pub on_open_entry_modal: ModalOpenHandler,
    pub on_open_category_modal: CategoryModalHandler,
    pub on_resolve_approval: ApprovalResolveHandler,
    pub on_change_loan_filter: LoanFilterHandler,
    pub on_repay_loan: LoanRepayHandler,
}

pub fn money_page(props: MoneyPageProps<'_>, cx: &App) -> impl IntoElement {
    let MoneyPageProps {
        language: lang,
        palette,
        stacked,
        sub_tab,
        loans,
        monthly_income_rwf,
        monthly_expenses_rwf,
        total_treasury,
        monthly_contributions,
        monthly_expenses,
        income_streams,
        expense_categories,
        monthly_cashflow: _,
        approvals,
        transactions,
        loan_filter,
        chart_mode: _,
        search_state,
        on_select_sub_tab,
        on_open_entry_modal,
        on_open_category_modal,
        on_resolve_approval,
        on_change_loan_filter,
        on_repay_loan,
    } = props;

    let lent_remaining: i64 = loans
        .iter()
        .filter(|l| l.is_lent)
        .map(|l| l.remaining_rwf())
        .sum();
    let borrowed_remaining: i64 = loans
        .iter()
        .filter(|l| !l.is_lent)
        .map(|l| l.remaining_rwf())
        .sum();
    let loans_count_str = loans.len().to_string();
    let inc_badge_str = format_compact_signed_rwf(monthly_income_rwf, true);
    let exp_badge_str = format_compact_signed_rwf(monthly_expenses_rwf, false);

    let tab_ov_label = MoneySubTab::Overview.label_for(lang);
    let tab_inc_label = MoneySubTab::Income.label_for(lang);
    let tab_exp_label = MoneySubTab::Expenses.label_for(lang);
    let tab_loan_label = MoneySubTab::Loans.label_for(lang);

    let sub_tab_items = [
        SegmentedTabItem {
            id: "money-subtab-overview",
            value: MoneySubTab::Overview,
            label: &tab_ov_label,
            icon: Some(icons::ICON_WALLET),
            dot_color: None,
            badge_text: None,
        },
        SegmentedTabItem {
            id: "money-subtab-income",
            value: MoneySubTab::Income,
            label: &tab_inc_label,
            icon: None,
            dot_color: Some(palette.income_text),
            badge_text: Some(inc_badge_str.as_str()),
        },
        SegmentedTabItem {
            id: "money-subtab-expenses",
            value: MoneySubTab::Expenses,
            label: &tab_exp_label,
            icon: None,
            dot_color: Some(palette.expense_text),
            badge_text: Some(exp_badge_str.as_str()),
        },
        SegmentedTabItem {
            id: "money-subtab-loans",
            value: MoneySubTab::Loans,
            label: &tab_loan_label,
            icon: None,
            dot_color: Some(palette.loan_text),
            badge_text: Some(loans_count_str.as_str()),
        },
    ];

    let on_tab = on_select_sub_tab.clone();
    let sub_tab_bar = segmented_tab_bar(
        "money-subtab-bar",
        &sub_tab_items,
        sub_tab,
        palette,
        move |next_tab, w, cx| {
            on_tab(next_tab, w, cx);
        },
    );

    let sub_content: AnyElement = match sub_tab {
        MoneySubTab::Overview => {
            let total_flow = (monthly_income_rwf + monthly_expenses_rwf).max(1);
            let income_ratio = ((monthly_income_rwf * 100) / total_flow).clamp(10, 95) as u8;
            let net_loans = lent_remaining - borrowed_remaining;

            let open_inc_entry = on_open_entry_modal.clone();
            let nav_inc = on_select_sub_tab.clone();
            let nav_exp = on_select_sub_tab.clone();
            let nav_loans = on_select_sub_tab.clone();

            let ratio_card = card_surface(palette)
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.combined_treasury")),
                                )
                                .child(
                                    div()
                                        .text_xl()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(total_treasury.to_string()),
                                ),
                        )
                        .child(compact_primary_button(
                            "money-ov-add-entry-btn",
                            text(lang, "cozy.add_entry_btn"),
                            palette,
                            move |_e, w, cx| {
                                open_inc_entry(
                                    crate::components::dashboard::EntryModalKind::Income,
                                    w,
                                    cx,
                                );
                            },
                        )),
                )
                .child(kawaii_progress_bar(
                    income_ratio,
                    palette.income_text,
                    palette,
                ))
                .child(
                    div()
                        .flex()
                        .gap_2p5()
                        .when(stacked, |this| this.flex_col())
                        .child(
                            card_elevated(palette)
                                .id("ov-tile-income")
                                .flex_1()
                                .cursor_pointer()
                                .hover(|s| s.border_color(palette.income_text))
                                .on_click(move |_e, w, cx| {
                                    nav_inc(MoneySubTab::Income, w, cx);
                                })
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.tab_income")),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.income_text)
                                        .child(format!("+{}", monthly_contributions)),
                                ),
                        )
                        .child(
                            card_elevated(palette)
                                .id("ov-tile-expenses")
                                .flex_1()
                                .cursor_pointer()
                                .hover(|s| s.border_color(palette.expense_text))
                                .on_click(move |_e, w, cx| {
                                    nav_exp(MoneySubTab::Expenses, w, cx);
                                })
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.tab_expenses")),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.expense_text)
                                        .child(format!("-{}", monthly_expenses)),
                                ),
                        )
                        .child(
                            card_elevated(palette)
                                .id("ov-tile-loans")
                                .flex_1()
                                .cursor_pointer()
                                .hover(|s| s.border_color(palette.loan_text))
                                .on_click(move |_e, w, cx| {
                                    nav_loans(MoneySubTab::Loans, w, cx);
                                })
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.loans_net")),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.loan_text)
                                        .child(format_signed_rwf(net_loans, net_loans >= 0)),
                                ),
                        ),
                );

            let approvals_card = card_surface(palette)
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
                                        .child(text(lang, "cozy.multisig_title")),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.multisig_sub")),
                                ),
                        )
                        .child(
                            div()
                                .px_2p5()
                                .py_1()
                                .rounded_full()
                                .bg(palette.loan_bg)
                                .border(px(1.5))
                                .border_color(palette.loan_border)
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.loan_text)
                                .child(format!(
                                    "{} {}",
                                    approvals.len(),
                                    text(lang, "cozy.waiting_suffix")
                                )),
                        ),
                )
                .child(if approvals.is_empty() {
                    card_elevated(palette)
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.income_text)
                                .child(text(lang, "cozy.all_approved")),
                        )
                        .into_any_element()
                } else {
                    let resolve_app_cb = on_resolve_approval.clone();
                    div()
                        .flex()
                        .flex_col()
                        .gap_2p5()
                        .children(approvals.iter().enumerate().map(|(idx, item)| {
                            let res_yes = resolve_app_cb.clone();
                            let res_no = resolve_app_cb.clone();
                            approval_request_card(
                                idx,
                                item,
                                lang,
                                palette,
                                move |_e, w, cx| {
                                    res_yes(idx, true, w, cx);
                                },
                                move |_e, w, cx| {
                                    res_no(idx, false, w, cx);
                                },
                            )
                        }))
                        .into_any_element()
                });

            let query = search_state.read(cx).value().to_string().to_lowercase();
            let filtered_txs: Vec<(usize, &TransactionRow)> = transactions
                .iter()
                .enumerate()
                .filter(|(_, tx)| {
                    if query.trim().is_empty() {
                        true
                    } else {
                        tx.title.to_lowercase().contains(&query)
                            || tx.actor.to_lowercase().contains(&query)
                            || tx.category.to_lowercase().contains(&query)
                    }
                })
                .collect();

            let ledger_card = card_surface(palette)
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .flex_wrap()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(text(lang, "cozy.combined_ledger_title")),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(palette.text_secondary)
                                        .child(text(lang, "cozy.combined_ledger_sub")),
                                ),
                        )
                        .child(div().w(px(260.0)).child(stateful_input_container(
                            palette,
                            search_state,
                            search_input(search_state, palette),
                        ))),
                )
                .child(
                    div().flex().flex_col().gap_2().children(
                        filtered_txs
                            .into_iter()
                            .map(|(idx, tx)| transaction_row(idx, tx, palette)),
                    ),
                );

            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .flex()
                        .gap_4()
                        .when(stacked, |this| this.flex_col())
                        .child(div().flex_1().min_w_0().child(ratio_card))
                        .child(div().flex_1().min_w_0().child(approvals_card)),
                )
                .child(ledger_card)
                .into_any_element()
        }
        MoneySubTab::Income => {
            let open_inc = on_open_entry_modal.clone();
            let open_cat = on_open_category_modal.clone();
            let banner = card_surface(palette)
                .flex_row()
                .items_center()
                .justify_between()
                .flex_wrap()
                .gap_3()
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
                                .child(text(lang, "cozy.monthly_inflow_title")),
                        )
                        .child(
                            div()
                                .text_2xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.income_text)
                                .child(format!("+{}", monthly_contributions)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.monthly_inflow_sub")),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(pill_button(
                            "money-inc-add-cat-btn",
                            text(lang, "modal.add_category_btn"),
                            palette,
                            move |_e, w, cx| {
                                open_cat(true, w, cx);
                            },
                        ))
                        .child(compact_primary_button(
                            "money-inc-log-btn",
                            text(lang, "cozy.log_income_btn"),
                            palette,
                            move |_e, w, cx| {
                                open_inc(
                                    crate::components::dashboard::EntryModalKind::Income,
                                    w,
                                    cx,
                                );
                            },
                        )),
                );

            let streams_grid = div().flex().flex_col().gap_3().children(
                income_streams
                    .iter()
                    .enumerate()
                    .map(|(idx, stream)| income_stream_card(idx, stream, palette)),
            );

            let deposits_card = card_surface(palette)
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.text_primary)
                        .child(text(lang, "cozy.recorded_deposits_title")),
                )
                .child(
                    div().flex().flex_col().gap_2().children(
                        transactions
                            .iter()
                            .filter(|tx| tx.is_income && !tx.is_loan)
                            .enumerate()
                            .map(|(idx, tx)| transaction_row(idx, tx, palette)),
                    ),
                );

            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(banner)
                .child(streams_grid)
                .child(deposits_card)
                .into_any_element()
        }
        MoneySubTab::Expenses => {
            let open_exp = on_open_entry_modal.clone();
            let open_cat = on_open_category_modal.clone();
            let banner = card_surface(palette)
                .flex_row()
                .items_center()
                .justify_between()
                .flex_wrap()
                .gap_3()
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
                                .child(text(lang, "cozy.monthly_spending_title")),
                        )
                        .child(
                            div()
                                .text_2xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.expense_text)
                                .child(format!("-{}", monthly_expenses)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.monthly_spending_sub")),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(pill_button(
                            "money-exp-add-cat-btn",
                            text(lang, "modal.add_category_btn"),
                            palette,
                            move |_e, w, cx| {
                                open_cat(false, w, cx);
                            },
                        ))
                        .child(pill_button_colored(
                            "money-exp-log-btn",
                            text(lang, "cozy.log_expense_btn"),
                            palette.expense_text,
                            palette,
                            move |_e, w, cx| {
                                open_exp(
                                    crate::components::dashboard::EntryModalKind::Expense,
                                    w,
                                    cx,
                                );
                            },
                        )),
                );

            let categories_grid = div().flex().flex_col().gap_3().children(
                expense_categories
                    .iter()
                    .enumerate()
                    .map(|(idx, cat)| expense_category_card(idx, cat, palette)),
            );

            let receipts_card = card_surface(palette)
                .gap_3()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.text_primary)
                        .child(text(lang, "cozy.expense_receipts_title")),
                )
                .child(
                    div().flex().flex_col().gap_2().children(
                        transactions
                            .iter()
                            .filter(|tx| !tx.is_income)
                            .enumerate()
                            .map(|(idx, tx)| transaction_row(idx, tx, palette)),
                    ),
                );

            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(banner)
                .child(categories_grid)
                .child(receipts_card)
                .into_any_element()
        }
        MoneySubTab::Loans => {
            let summary_row = div()
                .flex()
                .gap_4()
                .when(stacked, |this| this.flex_col())
                .child(
                    card_surface(palette)
                        .flex_1()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.lent_out_title")),
                        )
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.income_text)
                                .child(format_signed_rwf(lent_remaining, true)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.lent_out_sub")),
                        ),
                )
                .child(
                    card_surface(palette)
                        .flex_1()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.borrowed_title")),
                        )
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.loan_text)
                                .child(format_signed_rwf(borrowed_remaining, false)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(text(lang, "cozy.borrowed_sub")),
                        ),
                );

            let filter_pills = [
                (
                    "loan-filter-all",
                    LoanFilter::All,
                    text(lang, "cozy.filter_all").to_string(),
                ),
                (
                    "loan-filter-lent",
                    LoanFilter::Lent,
                    text(lang, "cozy.filter_lent").to_string(),
                ),
                (
                    "loan-filter-borrowed",
                    LoanFilter::Borrowed,
                    text(lang, "cozy.filter_borrowed").to_string(),
                ),
            ];

            let change_filter = on_change_loan_filter.clone();
            let open_loan = on_open_entry_modal.clone();
            let filter_bar = div()
                .flex()
                .items_center()
                .justify_between()
                .flex_wrap()
                .gap_2()
                .child(filter_pill_group(
                    &filter_pills,
                    loan_filter,
                    palette,
                    move |next_filter, w, cx| {
                        change_filter(next_filter, w, cx);
                    },
                ))
                .child(compact_primary_button(
                    "money-new-loan-btn",
                    text(lang, "cozy.new_loan_btn"),
                    palette,
                    move |_e, w, cx| {
                        open_loan(crate::components::dashboard::EntryModalKind::Loan, w, cx);
                    },
                ));

            let repay_cb = on_repay_loan.clone();
            let current_loan_filter = loan_filter;
            let loans_list = div().flex().flex_col().gap_3().children(
                loans
                    .iter()
                    .enumerate()
                    .filter(|(_, loan)| match current_loan_filter {
                        LoanFilter::All => true,
                        LoanFilter::Lent => loan.is_lent,
                        LoanFilter::Borrowed => !loan.is_lent,
                    })
                    .map(|(idx, loan)| {
                        let repay = repay_cb.clone();
                        loan_card(idx, loan, lang, palette, move |_e, w, cx| {
                            repay(idx, 30_000, w, cx);
                        })
                    }),
            );

            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(summary_row)
                .child(filter_bar)
                .child(loans_list)
                .into_any_element()
        }
    };

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(sub_tab_bar)
        .child(sub_content)
        .into_any_element()
}
