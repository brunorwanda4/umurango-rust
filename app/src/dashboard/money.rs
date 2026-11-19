use super::*;

impl DashboardView {
    pub(super) fn render_money_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
        cx: &App,
    ) -> AnyElement {
        let lang = self.language;
        let lent_remaining: i64 = self
            .loans
            .iter()
            .filter(|l| l.is_lent)
            .map(|l| l.remaining_rwf())
            .sum();
        let borrowed_remaining: i64 = self
            .loans
            .iter()
            .filter(|l| !l.is_lent)
            .map(|l| l.remaining_rwf())
            .sum();
        let loans_count_str = self.loans.len().to_string();
        let inc_badge_str = format_compact_signed_rwf(self.monthly_income_rwf, true);
        let exp_badge_str = format_compact_signed_rwf(self.monthly_expenses_rwf, false);

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

        let view_tab = view_entity.clone();
        let sub_tab_bar = segmented_tab_bar(
            "money-subtab-bar",
            &sub_tab_items,
            self.money_sub_tab,
            palette,
            move |next_tab, _w, cx| {
                view_tab.update(cx, |this, cx| {
                    this.navigate_to_money_tab(next_tab, cx);
                });
            },
        );

        let sub_content: AnyElement = match self.money_sub_tab {
            MoneySubTab::Overview => {
                let total_flow = (self.monthly_income_rwf + self.monthly_expenses_rwf).max(1);
                let income_ratio =
                    ((self.monthly_income_rwf * 100) / total_flow).clamp(10, 95) as u8;
                let net_loans = lent_remaining - borrowed_remaining;

                let view_add = view_entity.clone();
                let view_to_inc = view_entity.clone();
                let view_to_exp = view_entity.clone();
                let view_to_loan = view_entity.clone();

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
                                            .child(self.total_treasury.clone()),
                                    ),
                            )
                            .child(compact_primary_button(
                                "money-ov-add-entry-btn",
                                text(lang, "cozy.add_entry_btn"),
                                palette,
                                move |_e, w, cx| {
                                    view_add.update(cx, |this, cx| {
                                        this.open_entry_modal(EntryModalKind::Income, w, cx);
                                    });
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
                                    .on_click(move |_e, _w, cx| {
                                        view_to_inc.update(cx, |this, cx| {
                                            this.navigate_to_money_tab(MoneySubTab::Income, cx);
                                        });
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
                                            .child(format!("+{}", self.monthly_contributions)),
                                    ),
                            )
                            .child(
                                card_elevated(palette)
                                    .id("ov-tile-expenses")
                                    .flex_1()
                                    .cursor_pointer()
                                    .hover(|s| s.border_color(palette.expense_text))
                                    .on_click(move |_e, _w, cx| {
                                        view_to_exp.update(cx, |this, cx| {
                                            this.navigate_to_money_tab(MoneySubTab::Expenses, cx);
                                        });
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
                                            .child(format!("-{}", self.monthly_expenses)),
                                    ),
                            )
                            .child(
                                card_elevated(palette)
                                    .id("ov-tile-loans")
                                    .flex_1()
                                    .cursor_pointer()
                                    .hover(|s| s.border_color(palette.loan_text))
                                    .on_click(move |_e, _w, cx| {
                                        view_to_loan.update(cx, |this, cx| {
                                            this.navigate_to_money_tab(MoneySubTab::Loans, cx);
                                        });
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
                                        self.approvals.len(),
                                        text(lang, "cozy.waiting_suffix")
                                    )),
                            ),
                    )
                    .child(if self.approvals.is_empty() {
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
                        div()
                            .flex()
                            .flex_col()
                            .gap_2p5()
                            .children(self.approvals.iter().enumerate().map(|(idx, item)| {
                                let view_app = view_entity.clone();
                                let view_dec = view_entity.clone();
                                approval_request_card(
                                    idx,
                                    item,
                                    lang,
                                    palette,
                                    move |_e, _w, cx| {
                                        view_app.update(cx, |this, cx| {
                                            this.resolve_approval(idx, true, cx);
                                        });
                                    },
                                    move |_e, _w, cx| {
                                        view_dec.update(cx, |this, cx| {
                                            this.resolve_approval(idx, false, cx);
                                        });
                                    },
                                )
                            }))
                            .into_any_element()
                    });

                let query = self
                    .search_state
                    .read(cx)
                    .value()
                    .to_string()
                    .to_lowercase();
                let filtered_txs: Vec<(usize, &TransactionRow)> = self
                    .transactions
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
                                &self.search_state,
                                search_input(&self.search_state, palette),
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
                let view_log_inc = view_entity.clone();
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
                                    .child(format!("+{}", self.monthly_contributions)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(palette.text_secondary)
                                    .child(text(lang, "cozy.monthly_inflow_sub")),
                            ),
                    )
                    .child(compact_primary_button(
                        "money-inc-log-btn",
                        text(lang, "cozy.log_income_btn"),
                        palette,
                        move |_e, w, cx| {
                            view_log_inc.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::Income, w, cx);
                            });
                        },
                    ));

                let streams_grid = div().flex().flex_col().gap_3().children(
                    self.income_streams
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
                            self.transactions
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
                let view_log_exp = view_entity.clone();
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
                                    .child(format!("-{}", self.monthly_expenses)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(palette.text_secondary)
                                    .child(text(lang, "cozy.monthly_spending_sub")),
                            ),
                    )
                    .child(pill_button_colored(
                        "money-exp-log-btn",
                        text(lang, "cozy.log_expense_btn"),
                        palette.expense_text,
                        palette,
                        move |_e, w, cx| {
                            view_log_exp.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::Expense, w, cx);
                            });
                        },
                    ));

                let categories_grid = div().flex().flex_col().gap_3().children(
                    self.expense_categories
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
                            self.transactions
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

                let view_lf = view_entity.clone();
                let view_new_loan = view_entity.clone();
                let filter_bar = div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .flex_wrap()
                    .gap_2()
                    .child(filter_pill_group(
                        &filter_pills,
                        self.loan_filter,
                        palette,
                        move |next_filter, _w, cx| {
                            view_lf.update(cx, |this, cx| {
                                this.loan_filter = next_filter;
                                cx.notify();
                            });
                        },
                    ))
                    .child(compact_primary_button(
                        "money-new-loan-btn",
                        text(lang, "cozy.new_loan_btn"),
                        palette,
                        move |_e, w, cx| {
                            view_new_loan.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::Loan, w, cx);
                            });
                        },
                    ));

                let current_loan_filter = self.loan_filter;
                let loans_list = div().flex().flex_col().gap_3().children(
                    self.loans
                        .iter()
                        .enumerate()
                        .filter(|(_, loan)| match current_loan_filter {
                            LoanFilter::All => true,
                            LoanFilter::Lent => loan.is_lent,
                            LoanFilter::Borrowed => !loan.is_lent,
                        })
                        .map(|(idx, loan)| {
                            let view_repay = view_entity.clone();
                            loan_card(idx, loan, lang, palette, move |_e, _w, cx| {
                                view_repay.update(cx, |this, cx| {
                                    this.record_loan_repayment(idx, 30_000, cx);
                                });
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

}
