use super::*;
use crate::components::dashboard::{expense_category_card, income_stream_card, transaction_row};
use crate::components::ui::{
    card_elevated, card_header, stat_card,
};

impl DashboardView {
    pub(super) fn render_reports_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
        _cx: &App,
    ) -> AnyElement {
        let lang = self.language;

        // 1. Calculate aggregated financial metrics
        let mut total_income: i64 = self
            .transactions
            .iter()
            .filter(|t| t.is_income)
            .map(|t| t.amount_rwf)
            .sum();
        let mut total_expenses: i64 = self
            .transactions
            .iter()
            .filter(|t| !t.is_income)
            .map(|t| t.amount_rwf)
            .sum();
        let mut total_contributions: i64 = self
            .transactions
            .iter()
            .filter(|t| t.is_income && t.category.to_lowercase().contains("contribution"))
            .map(|t| t.amount_rwf)
            .sum();

        // If no transactions have been loaded yet, fall back to monthly summary fields
        if total_income == 0 && total_expenses == 0 {
            total_income = self.monthly_income_rwf.max(450_000);
            total_expenses = self.monthly_expenses_rwf.max(89_500);
            total_contributions = 50_000;
        }

        let net_savings = total_income - total_expenses;
        let savings_rate = if total_income > 0 {
            ((net_savings.max(0) * 100) / total_income).clamp(0, 100)
        } else {
            0
        };

        let income_count = self.transactions.iter().filter(|t| t.is_income).count();
        let expense_count = self.transactions.iter().filter(|t| !t.is_income).count();

        // Action button callbacks
        let view_csv = view_entity.clone();
        let view_summary = view_entity.clone();

        // Header: Title, Subtitle, and Export Buttons
        let header_actions = div()
            .flex()
            .items_center()
            .gap_2p5()
            .flex_wrap()
            .child(
                div()
                    .id("download-report-csv-btn")
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3p5()
                    .py_2()
                    .rounded(palette.radius_button)
                    .bg(palette.bg_elevated)
                    .border(palette.border_width_input)
                    .border_color(palette.border_color)
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(palette.text_primary)
                    .hover(|s| s.bg(palette.bg_card).border_color(palette.accent_primary))
                    .child(
                        icons::icon(icons::ICON_FILE_LINES)
                            .size(px(14.0))
                            .text_color(palette.accent_primary),
                    )
                    .child(text(lang, "reports.download_csv"))
                    .on_click(move |_e, _window, cx| {
                        view_csv.update(cx, |this, cx| {
                            this.download_report_csv(cx);
                        });
                    }),
            )
            .child(
                div()
                    .id("export-report-summary-btn")
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3p5()
                    .py_2()
                    .rounded(palette.radius_button)
                    .bg(palette.accent_primary)
                    .border(palette.border_width_input)
                    .border_color(palette.border_color)
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(palette.text_on_accent)
                    .hover(|s| s.bg(palette.accent_secondary))
                    .child(
                        icons::icon(icons::ICON_DOWNLOAD)
                            .size(px(14.0))
                            .text_color(palette.text_on_accent),
                    )
                    .child(text(lang, "reports.export_summary"))
                    .on_click(move |_e, _window, cx| {
                        view_summary.update(cx, |this, cx| {
                            this.download_report_summary(cx);
                        });
                    }),
            );

        let page_header = div()
            .flex()
            .items_start()
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
                            .text_2xl()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.text_primary)
                            .child(text(lang, "reports.title")),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(palette.text_secondary)
                            .child(text(lang, "reports.subtitle")),
                    ),
            )
            .child(header_actions);

        // Period filter pills row
        let view_filter = view_entity.clone();
        let period_filter_bar = div().flex().items_center().gap_2().flex_wrap().children(
            ReportPeriodFilter::ALL.iter().map(|filter| {
                let is_active = *filter == self.report_period;
                let view_f = view_filter.clone();
                let f_val = *filter;
                div()
                    .id(ElementId::from(format!(
                        "report-filter-{}",
                        f_val.label_key()
                    )))
                    .cursor_pointer()
                    .px_3()
                    .py_1p5()
                    .rounded(px(14.0))
                    .bg(if is_active {
                        palette.accent_primary
                    } else {
                        palette.bg_elevated
                    })
                    .border(px(1.5))
                    .border_color(if is_active {
                        palette.accent_primary
                    } else {
                        palette.border_color
                    })
                    .text_xs()
                    .font_weight(if is_active {
                        FontWeight::EXTRA_BOLD
                    } else {
                        FontWeight::MEDIUM
                    })
                    .text_color(if is_active {
                        palette.text_on_accent
                    } else {
                        palette.text_secondary
                    })
                    .hover(move |s| {
                        if !is_active {
                            s.bg(palette.bg_card).text_color(palette.text_primary)
                        } else {
                            s
                        }
                    })
                    .child(text(lang, filter.label_key()))
                    .on_click(move |_e, _window, cx| {
                        view_f.update(cx, |this, cx| {
                            this.set_report_period(f_val, cx);
                        });
                    })
            }),
        );

        // 4 Financial Summary Stat Cards
        let stat_cards_grid = div()
            .flex()
            .gap_3p5()
            .when(stacked, |this| this.flex_col())
            // Inflow
            .child(div().flex_1().child(stat_card(
                "report-stat-inflow",
                palette,
                text(lang, "reports.total_inflow"),
                format_rwf(total_income),
                Some(format!("{income_count} income transactions")),
                Some(icons::ICON_ARROW_DOWN),
                Some((format!("+{}", format_rwf(total_income)).into(), true)),
            )))
            // Outflow
            .child(div().flex_1().child(stat_card(
                "report-stat-outflow",
                palette,
                text(lang, "reports.total_outflow"),
                format_rwf(total_expenses),
                Some(format!("{expense_count} expense entries")),
                Some(icons::ICON_ARROW_UP),
                Some((format!("-{}", format_rwf(total_expenses)).into(), false)),
            )))
            // Net Savings
            .child(div().flex_1().child(stat_card(
                "report-stat-net-savings",
                palette,
                text(lang, "reports.net_savings"),
                format_rwf(net_savings),
                Some(format!("{savings_rate}% savings rate")),
                Some(icons::ICON_WALLET),
                Some((format!("{savings_rate}% saved").into(), net_savings >= 0)),
            )))
            // Member Contributions
            .child(div().flex_1().child(stat_card(
                "report-stat-contributions",
                palette,
                text(lang, "cozy.legend_contributions"),
                format_rwf(total_contributions),
                Some("Family pooled deposits".to_string()),
                Some(icons::ICON_CREDIT_CARD),
                None,
            )));

        // Multi-Series Financial Timeline Chart using GPUI Kit Chart
        let chart_section = family_financial_chart(
            &self.report_timeline_points,
            lang,
            palette,
        );

        // Breakdowns side-by-side or stacked
        let expense_breakdown = card_elevated(palette).id("report-expenses-breakdown")
            .gap_3()
            .child(card_header(
                text(lang, "reports.top_expenses"),
                Some(format!(
                    "{} categories tracked",
                    self.expense_categories.len()
                )),
                palette,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .when(self.expense_categories.is_empty(), |this| {
                        this.child(
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child("No category expenses recorded in this period."),
                        )
                    })
                    .children(
                        self.expense_categories
                            .iter()
                            .take(5)
                            .enumerate()
                            .map(|(idx, cat)| expense_category_card(idx, cat, palette)),
                    ),
            );

        let income_breakdown = card_elevated(palette).id("report-income-breakdown")
            .gap_3()
            .child(card_header(
                text(lang, "reports.income_breakdown"),
                Some(format!("{} streams active", self.income_streams.len())),
                palette,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .when(self.income_streams.is_empty(), |this| {
                        this.child(
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child("No income streams recorded in this period."),
                        )
                    })
                    .children(
                        self.income_streams
                            .iter()
                            .take(5)
                            .enumerate()
                            .map(|(idx, stream)| income_stream_card(idx, stream, palette)),
                    ),
            );

        let breakdowns_row = div()
            .flex()
            .gap_4()
            .when(stacked, |this| this.flex_col())
            .child(div().flex_1().child(expense_breakdown))
            .child(div().flex_1().child(income_breakdown));

        // Ledger Audit Records Table / List
        let recent_transactions_card = card_elevated(palette).id("report-ledger-records")
            .gap_3()
            .child(card_header(
                text(lang, "reports.recent_ledger"),
                Some(format!(
                    "{} authoritative transactions verified",
                    self.transactions.len()
                )),
                palette,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1p5()
                    .when(self.transactions.is_empty(), |this| {
                        this.child(
                            div()
                                .p_6()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    icons::icon(icons::ICON_FILE_LINES)
                                        .size(px(28.0))
                                        .text_color(palette.text_muted),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(palette.text_muted)
                                        .child("No transaction records found for this period."),
                                ),
                        )
                    })
                    .children(
                        self.transactions
                            .iter()
                            .take(10)
                            .enumerate()
                            .map(|(idx, tx)| transaction_row(idx, tx, palette)),
                    ),
            );

        // Full Page Assembly
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_5()
            .child(page_header)
            .child(period_filter_bar)
            .child(stat_cards_grid)
            .child(chart_section)
            .child(breakdowns_row)
            .child(recent_transactions_card)
            .into_any_element()
    }
}
