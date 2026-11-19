use super::*;

impl DashboardView {
    pub(super) fn render_home_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let lang = self.language;
        let net_saved_rwf = (self.monthly_income_rwf - self.monthly_expenses_rwf).max(0);
        let net_saved_label = format!(
            "+{} {}",
            format_rwf(net_saved_rwf),
            text(lang, "cozy.hero_net_saved_suffix")
        );

        let view_for_qa = view_entity.clone();
        let hero_card = treasury_hero_card(
            self.total_treasury.clone(),
            net_saved_label,
            lang,
            palette,
            move |kind, window, cx| {
                view_for_qa.update(cx, |this, cx| match kind {
                    QuickEntryKind::Income => {
                        this.open_entry_modal(EntryModalKind::Income, window, cx);
                    }
                    QuickEntryKind::Expense => {
                        this.open_entry_modal(EntryModalKind::Expense, window, cx);
                    }
                    QuickEntryKind::Loan => {
                        this.navigate_to_money_tab(MoneySubTab::Loans, cx);
                        this.open_entry_modal(EntryModalKind::Loan, window, cx);
                    }
                    QuickEntryKind::Project => {
                        this.set_active_section(AppSection::Projects, cx);
                        this.open_entry_modal(EntryModalKind::Project, window, cx);
                    }
                });
            },
        );

        let view_inc = view_entity.clone();
        let income_summary_card =
            interactive_card("home-summary-income", palette, move |_e, _w, cx| {
                view_inc.update(cx, |this, cx| {
                    this.navigate_to_money_tab(MoneySubTab::Income, cx);
                });
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
                            .child(format!("{} streams", self.income_streams.len())),
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
                    .child(format!("+{}", self.monthly_contributions)),
            );

        let view_exp = view_entity.clone();
        let expense_summary_card =
            interactive_card("home-summary-expense", palette, move |_e, _w, cx| {
                view_exp.update(cx, |this, cx| {
                    this.navigate_to_money_tab(MoneySubTab::Expenses, cx);
                });
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
                            .child(format!("{} cats", self.expense_categories.len())),
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
                    .child(format!("-{}", self.monthly_expenses)),
            );

        let view_rev = view_entity.clone();
        let approvals_banner = pending_approvals_banner(
            self.approvals.len(),
            self.approvals.first(),
            lang,
            palette,
            move |_e, _w, cx| {
                view_rev.update(cx, |this, cx| {
                    this.navigate_to_money_tab(MoneySubTab::Overview, cx);
                });
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

        let view_chart = view_entity.clone();
        let insights_card = cashflow_insights_card(
            self.chart_mode,
            &self.monthly_cashflow,
            &self.expense_categories,
            lang,
            palette,
            move |next_mode, _w, cx| {
                view_chart.update(cx, |this, cx| {
                    this.chart_mode = next_mode;
                    cx.notify();
                });
            },
        );

        let view_see_projects = view_entity.clone();
        let projects_preview_card = card_surface(palette)
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
                        move |_e, _w, cx| {
                            view_see_projects.update(cx, |this, cx| {
                                this.set_active_section(AppSection::Projects, cx);
                            });
                        },
                    )),
            )
            .child(
                div().flex().flex_col().gap_2p5().children(
                    self.projects
                        .iter()
                        .enumerate()
                        .filter(|(_, p)| !p.is_completed)
                        .take(2)
                        .map(|(idx, proj)| {
                            let view_boost = view_entity.clone();
                            project_preview_row(idx, proj, lang, palette, move |_e, _w, cx| {
                                view_boost.update(cx, |this, cx| {
                                    this.boost_project(idx, 50_000, cx);
                                });
                            })
                        }),
                ),
            );

        let middle_row = div()
            .flex()
            .gap_4()
            .when(stacked, |this| this.flex_col())
            .child(div().flex_1().min_w_0().child(insights_card))
            .child(div().flex_1().min_w_0().child(projects_preview_card));

        let view_full_ledger = view_entity;
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
                        move |_e, _w, cx| {
                            view_full_ledger.update(cx, |this, cx| {
                                this.navigate_to_money_tab(MoneySubTab::Overview, cx);
                            });
                        },
                    )),
            )
            .child(
                div().flex().flex_col().gap_2().children(
                    self.transactions
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

}
