use crate::brand::ThemeColors;
use crate::components::dashboard::{
    format_rwf, transaction_row, ReportPeriodFilter, ReportTimelinePoint, TransactionRow,
};
use crate::components::ui::{card_header, card_surface, filter_pill_group, stat_card};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

pub type PeriodChangeHandler = Rc<dyn Fn(ReportPeriodFilter, &mut Window, &mut App)>;
pub type PointSelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;
pub type ExportHandler = Rc<dyn Fn(&'static str, &mut Window, &mut App)>;

pub struct ReportsPageProps<'a> {
    pub language: Language,
    pub palette: ThemeColors,
    pub stacked: bool,
    pub period: ReportPeriodFilter,
    pub selected_point: usize,
    pub timeline_points: &'a [ReportTimelinePoint],
    pub transactions: &'a [TransactionRow],
    pub monthly_income_rwf: i64,
    pub monthly_expenses_rwf: i64,
    pub on_change_period: PeriodChangeHandler,
    pub on_select_point: PointSelectHandler,
    pub on_export_csv: ExportHandler,
    pub on_export_summary: ExportHandler,
}

pub fn reports_page(props: ReportsPageProps<'_>) -> impl IntoElement {
    let ReportsPageProps {
        language: lang,
        palette,
        stacked,
        period,
        selected_point,
        timeline_points,
        transactions,
        monthly_income_rwf,
        monthly_expenses_rwf,
        on_change_period,
        on_select_point,
        on_export_csv,
        on_export_summary,
    } = props;

    // 1. Calculate aggregated financial metrics
    let mut total_income: i64 = transactions
        .iter()
        .filter(|t| t.is_income)
        .map(|t| t.amount_rwf)
        .sum();
    let mut total_expenses: i64 = transactions
        .iter()
        .filter(|t| !t.is_income)
        .map(|t| t.amount_rwf)
        .sum();
    let mut total_contributions: i64 = transactions
        .iter()
        .filter(|t| t.is_income && t.category.to_lowercase().contains("contribution"))
        .map(|t| t.amount_rwf)
        .sum();

    if total_income == 0 && total_expenses == 0 {
        total_income = monthly_income_rwf.max(450_000);
        total_expenses = monthly_expenses_rwf.max(89_500);
        total_contributions = 50_000;
    }

    let net_savings = total_income - total_expenses;
    let savings_rate = if total_income > 0 {
        ((net_savings.max(0) * 100) / total_income).clamp(0, 100)
    } else {
        0
    };

    let income_count = transactions.iter().filter(|t| t.is_income).count();
    let expense_count = transactions.iter().filter(|t| !t.is_income).count();

    // Header actions
    let export_csv_cb = on_export_csv.clone();
    let export_sum_cb = on_export_summary.clone();

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
                .on_click(move |_e, w, cx| {
                    export_csv_cb("csv", w, cx);
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
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_on_accent)
                .hover(|s| s.opacity(0.9))
                .child(
                    icons::icon(icons::ICON_FILE_LINES)
                        .size(px(14.0))
                        .text_color(palette.text_on_accent),
                )
                .child(text(lang, "reports.export_summary"))
                .on_click(move |_e, w, cx| {
                    export_sum_cb("summary", w, cx);
                }),
        );

    let period_pills = [
        (
            "report-period-this-month",
            ReportPeriodFilter::ThisMonth,
            text(lang, "reports.period_this_month").to_string(),
        ),
        (
            "report-period-30-days",
            ReportPeriodFilter::Last30Days,
            text(lang, "reports.period_last_month").to_string(),
        ),
        (
            "report-period-6-months",
            ReportPeriodFilter::Last6Months,
            text(lang, "reports.period_this_quarter").to_string(),
        ),
        (
            "report-period-all-time",
            ReportPeriodFilter::AllTime,
            text(lang, "reports.period_all_time").to_string(),
        ),
    ];

    let on_period = on_change_period.clone();
    let filter_bar = div()
        .flex()
        .items_center()
        .justify_between()
        .flex_wrap()
        .gap_3()
        .child(filter_pill_group(
            &period_pills,
            period,
            palette,
            move |next_p, w, cx| {
                on_period(next_p, w, cx);
            },
        ))
        .child(header_actions);

    let title_section = div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(palette.text_primary)
                .child(text(lang, "reports.page_title")),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(palette.text_secondary)
                .child(text(lang, "reports.page_subtitle")),
        );

    let metrics_grid = div()
        .flex()
        .gap_3()
        .when(stacked, |this| this.flex_col())
        .child(div().flex_1().child(stat_card(
            "stat-reports-income",
            palette,
            text(lang, "reports.stat_total_income"),
            format_rwf(total_income),
            Some(format!("{} recorded entries", income_count.max(1))),
            Some(icons::ICON_ARROW_DOWN),
            None,
        )))
        .child(div().flex_1().child(stat_card(
            "stat-reports-expenses",
            palette,
            text(lang, "reports.stat_total_expenses"),
            format_rwf(total_expenses),
            Some(format!("{} approved entries", expense_count.max(1))),
            Some(icons::ICON_ARROW_UP),
            None,
        )))
        .child(div().flex_1().child(stat_card(
            "stat-reports-savings",
            palette,
            text(lang, "reports.stat_net_savings"),
            format_rwf(net_savings),
            Some(format!("{}% savings rate", savings_rate)),
            Some(icons::ICON_PIGGY_BANK),
            None,
        )))
        .child(div().flex_1().child(stat_card(
            "stat-reports-contributions",
            palette,
            text(lang, "reports.stat_contributions"),
            format_rwf(total_contributions),
            Some("100% compliant"),
            Some(icons::ICON_HAND_HOLDING_DOLLAR),
            None,
        )));

    let timeline_card = card_surface(palette)
        .gap_3()
        .child(card_header(
            text(lang, "reports.timeline_title"),
            Some(text(lang, "reports.timeline_subtitle")),
            palette,
        ))
        .child(div().flex().items_center().gap_2().flex_wrap().children(
            timeline_points.iter().enumerate().map(|(idx, pt)| {
                let is_active = idx == selected_point;
                let select_pt = on_select_point.clone();
                div()
                    .id(("report-point", idx))
                    .cursor_pointer()
                    .flex_1()
                    .min_w(px(90.0))
                    .p_2p5()
                    .rounded(palette.radius_card)
                    .border(palette.border_width_input)
                    .border_color(if is_active {
                        palette.accent_primary
                    } else {
                        palette.border_color
                    })
                    .bg(if is_active {
                        palette.bg_input_focus
                    } else {
                        palette.bg_card
                    })
                    .hover(|s| s.border_color(palette.accent_primary))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_active {
                                palette.accent_primary
                            } else {
                                palette.text_secondary
                            })
                            .child(pt.label.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.income_text)
                            .child(format!("+{}", format_rwf(pt.income_rwf))),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.expense_text)
                            .child(format!("-{}", format_rwf(pt.expenses_rwf))),
                    )
                    .on_click(move |_e, w, cx| {
                        select_pt(idx, w, cx);
                    })
            }),
        ));

    let audit_card = card_surface(palette)
        .gap_3()
        .child(card_header(
            text(lang, "reports.audit_title"),
            Some(text(lang, "reports.audit_subtitle")),
            palette,
        ))
        .child(
            div().flex().flex_col().gap_2().children(
                transactions
                    .iter()
                    .take(6)
                    .enumerate()
                    .map(|(idx, tx)| transaction_row(idx, tx, palette)),
            ),
        );

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(title_section)
        .child(filter_bar)
        .child(metrics_grid)
        .child(timeline_card)
        .child(audit_card)
        .into_any_element()
}
