use crate::brand::{apply_theme, colors, ThemeColors, ThemeMode};
use crate::components::dashboard::{
    approval_request_card, cashflow_insights_card, expense_category_card, format_rwf,
    format_signed_rwf, income_stream_card, loan_card, member_pledge_card, pending_approvals_banner,
    project_goal_card, project_preview_row, transaction_row, treasury_hero_card, ApprovalItem,
    ChartMode, ExpenseCategoryRow, IncomeStreamRow, LoanFilter, LoanItem, MemberContributionRow,
    MoneySubTab, ProjectFilter, ProjectSummary, QuickEntryKind, TransactionRow, APPROVALS,
    EXPENSE_CATEGORIES, FAMILY_MEMBERS, INCOME_STREAMS, LOANS, MONTHLY_CASHFLOW, PROJECTS,
    TRANSACTIONS,
};
use crate::components::layouts::responsive::app_content_width;
use crate::components::layouts::{app_layout, AppLayoutProps, AppSection};
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{
    card_elevated, card_header, card_surface, compact_primary_button, destructive_button,
    dismissible_alert, filter_pill_group, icon_badge_box, interactive_card, kawaii_progress_bar,
    member_avatar, pill_button, pill_button_colored, primary_button, search_input,
    secondary_button, segmented_tab_bar, stateful_input_container, toggle_field, AlertVariant,
    AvatarSize, BadgeVariant, SegmentedTabItem,
};
use crate::i18n::{text, Language};
use crate::icons;
use crate::theme::{active_mood, set_cozy_mood, CozyMood};
use gpui_kit::component::input::InputState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum DashboardEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    SignOutRequested,
    RecordContributionRequested,
    AddExpenseRequested,
    NewProjectRequested,
    SyncRequested,
}

pub struct DashboardView {
    theme: ThemeMode,
    mood: CozyMood,
    language: Language,
    active_section: AppSection,
    money_sub_tab: MoneySubTab,
    chart_mode: ChartMode,
    loan_filter: LoanFilter,
    project_filter: ProjectFilter,
    expanded_project_index: Option<usize>,
    nav_history: Vec<(AppSection, MoneySubTab)>,
    member_name: String,
    pub family_name: String,
    search_state: Entity<InputState>,
    show_approval_alert: bool,
    pending_approval_count: usize,
    pending_approval_tx_id: Option<String>,
    pending_approval_tx_title: Option<String>,
    pending_approval_tx_amount: Option<String>,
    pub total_treasury: String,
    pub monthly_contributions: String,
    pub monthly_expenses: String,
    pub emergency_reserve: String,
    treasury_balance_rwf: i64,
    monthly_income_rwf: i64,
    monthly_expenses_rwf: i64,
    pub transactions: Vec<TransactionRow>,
    pub projects: Vec<ProjectSummary>,
    pub members: Vec<(String, String, BadgeVariant)>,
    pub family_members: Vec<MemberContributionRow>,
    pub income_streams: Vec<IncomeStreamRow>,
    pub expense_categories: Vec<ExpenseCategoryRow>,
    pub loans: Vec<LoanItem>,
    pub approvals: Vec<ApprovalItem>,
    pub rule_multi_sig_over_30k: bool,
    pub rule_sunday_digest: bool,
    pub rule_auto_roundup_momo: bool,
    pub toast_notice: Option<String>,
    pub storage_sync_status: String,
}

impl EventEmitter<DashboardEvent> for DashboardView {}

impl DashboardView {
    pub fn new(
        window: &mut Window,
        theme: ThemeMode,
        language: Language,
        cx: &mut Context<Self>,
    ) -> Self {
        apply_theme(theme, cx);
        let mood = active_mood();
        let search_state = cx.new(|cx| {
            InputState::new(window, cx).placeholder(text(language, "dashboard.search_placeholder"))
        });

        let default_members = vec![
            (
                "Bruno N.".to_string(),
                "Co-Admin · Family Planner".to_string(),
                BadgeVariant::Primary,
            ),
            (
                "Agnes Ingabire".to_string(),
                "Co-Admin · Treasury Keeper".to_string(),
                BadgeVariant::Success,
            ),
            (
                "Papa Jean".to_string(),
                "Elder Advisor · Musanze".to_string(),
                BadgeVariant::Warning,
            ),
            (
                "Grandma Rose".to_string(),
                "Family Blessing · Gisenyi".to_string(),
                BadgeVariant::Info,
            ),
        ];

        let approvals = APPROVALS.to_vec();

        Self {
            theme,
            mood,
            language,
            active_section: AppSection::Home,
            money_sub_tab: MoneySubTab::Overview,
            chart_mode: ChartMode::Cashflow,
            loan_filter: LoanFilter::All,
            project_filter: ProjectFilter::Active,
            expanded_project_index: None,
            nav_history: Vec::new(),
            member_name: "Bruno N.".to_string(),
            family_name: "Kigali · 4 Members".to_string(),
            search_state,
            show_approval_alert: true,
            pending_approval_count: approvals.len(),
            pending_approval_tx_id: None,
            pending_approval_tx_title: Some("Musanze Farm Fertilizer & Seeds".to_string()),
            pending_approval_tx_amount: Some("75,000 RWF".to_string()),
            total_treasury: "4,280,000 RWF".to_string(),
            monthly_contributions: "850,000 RWF".to_string(),
            monthly_expenses: "410,000 RWF".to_string(),
            emergency_reserve: "1,500,000 RWF".to_string(),
            treasury_balance_rwf: 4_280_000,
            monthly_income_rwf: 850_000,
            monthly_expenses_rwf: 410_000,
            transactions: TRANSACTIONS.to_vec(),
            projects: PROJECTS.to_vec(),
            members: default_members,
            family_members: FAMILY_MEMBERS.to_vec(),
            income_streams: INCOME_STREAMS.to_vec(),
            expense_categories: EXPENSE_CATEGORIES.to_vec(),
            loans: LOANS.to_vec(),
            approvals,
            rule_multi_sig_over_30k: true,
            rule_sunday_digest: true,
            rule_auto_roundup_momo: false,
            toast_notice: None,
            storage_sync_status: text(language, "dashboard.storage_health_desc").to_string(),
        }
    }

    pub fn set_member_name(&mut self, name: String, cx: &mut Context<Self>) {
        if !name.trim().is_empty() {
            self.member_name = name;
            cx.notify();
        }
    }

    pub fn set_active_section(&mut self, section: AppSection, cx: &mut Context<Self>) {
        if self.active_section != section {
            self.nav_history
                .push((self.active_section, self.money_sub_tab));
            if self.nav_history.len() > 24 {
                self.nav_history.remove(0);
            }
            self.active_section = section;
            cx.notify();
        }
    }

    pub fn navigate_to_money_tab(&mut self, sub_tab: MoneySubTab, cx: &mut Context<Self>) {
        if self.active_section != AppSection::Money || self.money_sub_tab != sub_tab {
            self.nav_history
                .push((self.active_section, self.money_sub_tab));
            if self.nav_history.len() > 24 {
                self.nav_history.remove(0);
            }
            self.active_section = AppSection::Money;
            self.money_sub_tab = sub_tab;
            cx.notify();
        }
    }

    pub fn go_back(&mut self, cx: &mut Context<Self>) {
        if let Some((prev_section, prev_sub)) = self.nav_history.pop() {
            self.active_section = prev_section;
            self.money_sub_tab = prev_sub;
            cx.notify();
        } else if self.active_section != AppSection::Home {
            self.active_section = AppSection::Home;
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: ThemeMode, cx: &mut Context<Self>) {
        self.theme = theme;
        apply_theme(theme, cx);
        cx.notify();
    }

    pub fn set_mood(&mut self, mood: CozyMood, cx: &mut Context<Self>) {
        self.mood = mood;
        set_cozy_mood(mood, cx);
        self.toast_notice = Some(format!("✨ Switched to {} Cozy Mood", mood.label()));
        cx.notify();
    }

    pub fn set_language(
        &mut self,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.language = language;
        let placeholder = text(language, "dashboard.search_placeholder");
        self.search_state.update(cx, |state, cx| {
            state.set_placeholder(placeholder, window, cx);
        });
        cx.notify();
    }

    pub fn set_storage_status(&mut self, status: String, cx: &mut Context<Self>) {
        self.storage_sync_status = status;
        cx.notify();
    }

    #[allow(dead_code)]
    pub fn set_family_name(&mut self, name: String, cx: &mut Context<Self>) {
        if !name.trim().is_empty() {
            self.family_name = name;
            cx.notify();
        }
    }

    fn sync_formatted_totals(&mut self) {
        self.total_treasury = format_rwf(self.treasury_balance_rwf);
        self.monthly_contributions = format_rwf(self.monthly_income_rwf);
        self.monthly_expenses = format_rwf(self.monthly_expenses_rwf);
        self.pending_approval_count = self.approvals.len();
    }

    pub fn boost_project(&mut self, index: usize, amount_rwf: i64, cx: &mut Context<Self>) {
        if let Some(proj) = self.projects.get_mut(index) {
            proj.boost(amount_rwf);
            let proj_name = proj.name.clone();
            self.transactions.insert(
                0,
                TransactionRow {
                    actor: format!("{} · Family Pool", self.member_name).into(),
                    title: format!("Boosted {proj_name}").into(),
                    date: "Just now".into(),
                    amount: format_signed_rwf(amount_rwf, true).into(),
                    amount_rwf,
                    is_income: true,
                    is_loan: false,
                    icon_name: icons::ICON_BULLSEYE,
                    category: "Project Boost".into(),
                    category_variant: BadgeVariant::Success,
                    status: "Approved".into(),
                    status_variant: BadgeVariant::Success,
                },
            );
            self.toast_notice = Some(format!(
                "✨ Added +{} boost to {}!",
                format_rwf(amount_rwf),
                proj_name
            ));
            cx.notify();
        }
    }

    pub fn record_loan_repayment(&mut self, index: usize, amount_rwf: i64, cx: &mut Context<Self>) {
        if let Some(loan) = self.loans.get_mut(index) {
            let step = amount_rwf.min(loan.remaining_rwf());
            if step <= 0 {
                self.toast_notice = Some(format!("✨ {} is already fully settled!", loan.person));
                cx.notify();
                return;
            }
            loan.paid_rwf += step;
            let person = loan.person.clone();
            let is_lent = loan.is_lent;
            if is_lent {
                self.treasury_balance_rwf += step;
            } else {
                self.treasury_balance_rwf = (self.treasury_balance_rwf - step).max(0);
            }
            self.sync_formatted_totals();
            self.transactions.insert(
                0,
                TransactionRow {
                    actor: format!("{person} · MoMo").into(),
                    title: format!("Loan Repayment · {person}").into(),
                    date: "Just now".into(),
                    amount: format_signed_rwf(step, is_lent).into(),
                    amount_rwf: step,
                    is_income: is_lent,
                    is_loan: true,
                    icon_name: icons::ICON_HANDSHAKE,
                    category: "Loan".into(),
                    category_variant: BadgeVariant::Warning,
                    status: "Approved".into(),
                    status_variant: BadgeVariant::Success,
                },
            );
            self.toast_notice = Some(format!(
                "🤝 Recorded {} repayment for {}!",
                format_rwf(step),
                person
            ));
            cx.notify();
        }
    }

    pub fn resolve_approval(&mut self, index: usize, approve: bool, cx: &mut Context<Self>) {
        if index < self.approvals.len() {
            let item = self.approvals.remove(index);
            if approve {
                if let Some(id) = self.pending_approval_tx_id.clone() {
                    if let Some(fid) = crate::auth::client::get_current_family_id() {
                        let _ = crate::auth::client::api_approve_transaction(&fid, &id);
                    }
                }
                self.treasury_balance_rwf = (self.treasury_balance_rwf - item.amount_rwf).max(0);
                self.monthly_expenses_rwf += item.amount_rwf;
                self.transactions.insert(
                    0,
                    TransactionRow {
                        actor: format!("{} · 2/2 Approved", item.requester).into(),
                        title: item.title.clone(),
                        date: "Just now".into(),
                        amount: format_signed_rwf(item.amount_rwf, false).into(),
                        amount_rwf: item.amount_rwf,
                        is_income: false,
                        is_loan: false,
                        icon_name: icons::ICON_CIRCLE_CHECK,
                        category: item.category.clone(),
                        category_variant: BadgeVariant::Success,
                        status: "Approved".into(),
                        status_variant: BadgeVariant::Success,
                    },
                );
                self.toast_notice = Some(format!(
                    "✓ Approved & released {} ({})",
                    item.title,
                    format_rwf(item.amount_rwf)
                ));
            } else {
                self.toast_notice = Some(format!("✕ Declined request: {}", item.title));
            }
            self.sync_formatted_totals();
            cx.notify();
        }
    }

    pub fn log_member_contribution(
        &mut self,
        index: usize,
        amount_rwf: i64,
        cx: &mut Context<Self>,
    ) {
        if let Some(member) = self.family_members.get_mut(index) {
            member.contributed_rwf += amount_rwf;
            let name = member.name.clone();
            self.treasury_balance_rwf += amount_rwf;
            self.monthly_income_rwf += amount_rwf;
            self.sync_formatted_totals();
            self.transactions.insert(
                0,
                TransactionRow {
                    actor: format!("{name} · MTN MoMo").into(),
                    title: format!("{name} Family Circle Pledge").into(),
                    date: "Just now".into(),
                    amount: format_signed_rwf(amount_rwf, true).into(),
                    amount_rwf,
                    is_income: true,
                    is_loan: false,
                    icon_name: icons::ICON_COINS,
                    category: "Contribution".into(),
                    category_variant: BadgeVariant::Success,
                    status: "Approved".into(),
                    status_variant: BadgeVariant::Success,
                },
            );
            self.toast_notice = Some(format!(
                "💚 Logged +{} contribution from {}!",
                format_rwf(amount_rwf),
                name
            ));
            cx.notify();
        }
    }

    pub fn refresh_from_backend(&mut self, cx: &mut Context<Self>) {
        let family_id = crate::auth::client::get_current_family_id().or_else(|| {
            if let Ok(fams) = crate::auth::client::api_list_families() {
                if let Some(first) = fams.first() {
                    crate::auth::client::set_current_family(
                        Some(first.id.to_string()),
                        Some(first.name.clone()),
                    );
                    return Some(first.id.to_string());
                }
            }
            None
        });

        if let Some(ref fam_name) = crate::auth::client::get_current_family_name() {
            self.family_name = fam_name.clone();
        }

        if let Some(ref fid) = family_id {
            if let Ok(summary) = crate::auth::client::api_get_treasury_summary(fid) {
                self.treasury_balance_rwf = summary.total_treasury_minor_units / 100;
                self.monthly_income_rwf = summary.monthly_contributions_minor_units / 100;
                self.monthly_expenses_rwf = summary.monthly_expenses_minor_units / 100;
                self.total_treasury = umuryango_shared::format_money(
                    summary.total_treasury_minor_units,
                    &summary.currency,
                );
                self.monthly_contributions = umuryango_shared::format_money(
                    summary.monthly_contributions_minor_units,
                    &summary.currency,
                );
                self.monthly_expenses = umuryango_shared::format_money(
                    summary.monthly_expenses_minor_units,
                    &summary.currency,
                );
                self.emergency_reserve = umuryango_shared::format_money(
                    summary.emergency_reserve_minor_units,
                    &summary.currency,
                );
            }

            if let Ok(txs) = crate::auth::client::api_list_transactions(fid, None) {
                let pending_items: Vec<ApprovalItem> = txs
                    .iter()
                    .filter(|t| t.status == "PENDING")
                    .map(|p| ApprovalItem {
                        id: p.id.clone().into(),
                        requester: p.actor_name.clone().into(),
                        title: p.title.clone().into(),
                        category: p.category.clone().into(),
                        amount_rwf: p.amount_minor_units / 100,
                        votes: "1 of 2 Admins Approved".into(),
                    })
                    .collect();

                if let Some(first) = pending_items.first() {
                    self.pending_approval_count = pending_items.len();
                    self.pending_approval_tx_id = Some(first.id.to_string());
                    self.pending_approval_tx_title = Some(first.title.to_string());
                    self.pending_approval_tx_amount = Some(format_rwf(first.amount_rwf));
                    self.approvals = pending_items;
                    self.show_approval_alert = true;
                }
                if !txs.is_empty() {
                    self.transactions = txs
                        .into_iter()
                        .map(|dto| TransactionRow::from_dto(&dto))
                        .collect();
                }
            }

            if let Ok(projs) = crate::auth::client::api_list_projects(fid) {
                if !projs.is_empty() {
                    self.projects = projs
                        .into_iter()
                        .map(|dto| ProjectSummary::from_dto(&dto))
                        .collect();
                }
            }

            if let Ok(mems) = crate::auth::client::api_list_members(fid) {
                if !mems.is_empty() {
                    self.members = mems
                        .into_iter()
                        .map(|m| {
                            let role_desc = match m.role.as_str() {
                                "HEAD" => "Family Head / Admin",
                                "TREASURER" => "Treasurer",
                                "ELDER" => "Elder",
                                "YOUTH" => "Youth Member",
                                _ => "Member",
                            };
                            let variant = match m.role.as_str() {
                                "HEAD" => BadgeVariant::Primary,
                                "TREASURER" => BadgeVariant::Success,
                                "ELDER" => BadgeVariant::Warning,
                                "YOUTH" => BadgeVariant::Info,
                                _ => BadgeVariant::Neutral,
                            };
                            (m.full_name, role_desc.to_string(), variant)
                        })
                        .collect();
                }
            }
        }
        cx.notify();
    }

    pub fn sync_preferences(
        &mut self,
        theme: ThemeMode,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.theme != theme {
            self.set_theme(theme, cx);
        }
        if self.language != language {
            self.set_language(language, window, cx);
        }
    }

    fn render_home_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let net_saved_rwf = (self.monthly_income_rwf - self.monthly_expenses_rwf).max(0);
        let net_saved_label = format!("+{} net saved this month", format_rwf(net_saved_rwf));

        let view_for_qa = view_entity.clone();
        let hero_card = treasury_hero_card(
            self.total_treasury.clone(),
            net_saved_label,
            palette,
            move |kind, _window, cx| {
                view_for_qa.update(cx, |this, cx| match kind {
                    QuickEntryKind::Income => {
                        cx.emit(DashboardEvent::RecordContributionRequested);
                    }
                    QuickEntryKind::Expense => {
                        cx.emit(DashboardEvent::AddExpenseRequested);
                    }
                    QuickEntryKind::Loan => {
                        this.navigate_to_money_tab(MoneySubTab::Loans, cx);
                    }
                    QuickEntryKind::Project => {
                        cx.emit(DashboardEvent::NewProjectRequested);
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
                            .child("+12%"),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(palette.text_secondary)
                    .child("Monthly Income"),
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
                            .child("-4%"),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(palette.text_secondary)
                    .child("Monthly Expenses"),
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
            &MONTHLY_CASHFLOW,
            &self.expense_categories,
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
                                    .child("Planned Family Projects"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(palette.text_secondary)
                                    .child("Shared goals & milestone progress"),
                            ),
                    )
                    .child(pill_button(
                        "home-see-all-projects-btn",
                        "See all ->",
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
                            project_preview_row(idx, proj, palette, move |_e, _w, cx| {
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
                                    .child("Recent Family Activity"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(palette.text_secondary)
                                    .child("Latest income, expenses & loan updates"),
                            ),
                    )
                    .child(pill_button(
                        "home-full-ledger-btn",
                        "Full Ledger ->",
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

    fn render_money_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
        cx: &App,
    ) -> AnyElement {
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

        let sub_tab_items = [
            SegmentedTabItem {
                id: "money-subtab-overview",
                value: MoneySubTab::Overview,
                label: "Overview",
                icon: Some(icons::ICON_WALLET),
                dot_color: None,
                badge_text: None,
            },
            SegmentedTabItem {
                id: "money-subtab-income",
                value: MoneySubTab::Income,
                label: "Income",
                icon: None,
                dot_color: Some(palette.income_text),
                badge_text: Some("+850k"),
            },
            SegmentedTabItem {
                id: "money-subtab-expenses",
                value: MoneySubTab::Expenses,
                label: "Expenses",
                icon: None,
                dot_color: Some(palette.expense_text),
                badge_text: Some("-410k"),
            },
            SegmentedTabItem {
                id: "money-subtab-loans",
                value: MoneySubTab::Loans,
                label: "Loan & Borrowed",
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
                                            .child("Combined Family Treasury"),
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
                                "+ Add Entry",
                                palette,
                                move |_e, _w, cx| {
                                    view_add.update(cx, |_this, cx| {
                                        cx.emit(DashboardEvent::RecordContributionRequested);
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
                                            .child("Income"),
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
                                            .child("Expenses"),
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
                                            .child("Loans Net"),
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
                                            .child("Pending Multi-Sig Approvals"),
                                    )
                                    .child(
                                        div().text_xs().text_color(palette.text_secondary).child(
                                            "Expenses over 30,000 RWF require 2 family admins",
                                        ),
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
                                    .child(format!("{} Waiting", self.approvals.len())),
                            ),
                    )
                    .child(if self.approvals.is_empty() {
                        card_elevated(palette)
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.income_text)
                                    .child("All family requests approved! ✨"),
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
                                            .child("Combined Family Ledger"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(palette.text_secondary)
                                            .child("All income, expenses & loan movements"),
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
                                    .child("Monthly Family Inflow"),
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
                                    .child("Salaries, consulting, rental plot & Ibimina dividend"),
                            ),
                    )
                    .child(compact_primary_button(
                        "money-inc-log-btn",
                        "+ Log Income",
                        palette,
                        move |_e, _w, cx| {
                            view_log_inc.update(cx, |_this, cx| {
                                cx.emit(DashboardEvent::RecordContributionRequested);
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
                            .child("Recorded Income Deposits"),
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
                                    .child("Monthly Family Spending"),
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
                                    .child("78% of 525,000 RWF monthly household budget"),
                            ),
                    )
                    .child(pill_button_colored(
                        "money-exp-log-btn",
                        "- Log Expense",
                        palette.expense_text,
                        palette,
                        move |_e, _w, cx| {
                            view_log_exp.update(cx, |_this, cx| {
                                cx.emit(DashboardEvent::AddExpenseRequested);
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
                            .child("Expense Receipts"),
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
                                    .child("Lent Out (Owed to Us)"),
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
                                    .child("Active family support bridges"),
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
                                    .child("Borrowed (We Owe)"),
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
                                    .child("Kigali Family Ibimina solar advance"),
                            ),
                    );

                let filter_pills = [
                    ("loan-filter-all", LoanFilter::All, "All".to_string()),
                    ("loan-filter-lent", LoanFilter::Lent, "Lent Out".to_string()),
                    (
                        "loan-filter-borrowed",
                        LoanFilter::Borrowed,
                        "Borrowed".to_string(),
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
                        "+ New Loan / Borrow",
                        palette,
                        move |_e, _w, cx| {
                            view_new_loan.update(cx, |this, cx| {
                                this.loans.insert(
                                    0,
                                    LoanItem {
                                        id: format!("l{}", this.loans.len() + 1).into(),
                                        is_lent: true,
                                        person: "Uncle Olivier".into(),
                                        reason: "Seed Potato Transport Bridge".into(),
                                        total_rwf: 120_000,
                                        paid_rwf: 30_000,
                                        due: "Feb 20, 2027".into(),
                                    },
                                );
                                this.toast_notice = Some(
                                    "🤝 Created new family loan bridge for Uncle Olivier!"
                                        .to_string(),
                                );
                                cx.notify();
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
                            loan_card(idx, loan, palette, move |_e, _w, cx| {
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

    fn render_projects_screen(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let total_saved: i64 = self.projects.iter().map(|p| p.saved_rwf).sum();
        let active_count = self.projects.iter().filter(|p| !p.is_completed).count();
        let completed_count = self.projects.iter().filter(|p| p.is_completed).count();

        let view_new_proj = view_entity.clone();
        let hero_card = card_surface(palette)
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
                            .child("Total Saved Across Family Plans"),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.accent_primary)
                            .child(format_rwf(total_saved)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_secondary)
                            .child(format!(
                                "{} active goals · {} completed milestone",
                                active_count, completed_count
                            )),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(filter_pill_group(
                        &[
                            (
                                "proj-filter-active",
                                ProjectFilter::Active,
                                format!("Active Plans ({active_count})"),
                            ),
                            (
                                "proj-filter-done",
                                ProjectFilter::Completed,
                                format!("Completed ({completed_count})"),
                            ),
                        ],
                        self.project_filter,
                        palette,
                        {
                            let view_pf = view_entity.clone();
                            move |next_filter, _w, cx| {
                                view_pf.update(cx, |this, cx| {
                                    this.project_filter = next_filter;
                                    cx.notify();
                                });
                            }
                        },
                    ))
                    .child(compact_primary_button(
                        "projects-new-btn",
                        "+ New Project",
                        palette,
                        move |_e, _w, cx| {
                            view_new_proj.update(cx, |_this, cx| {
                                cx.emit(DashboardEvent::NewProjectRequested);
                            });
                        },
                    )),
            );

        let filter = self.project_filter;
        let expanded = self.expanded_project_index;
        let projects_grid = div().flex().flex_col().gap_3().children(
            self.projects
                .iter()
                .enumerate()
                .filter(|(_, p)| match filter {
                    ProjectFilter::Active => !p.is_completed,
                    ProjectFilter::Completed => p.is_completed,
                })
                .map(|(idx, proj)| {
                    let view_boost = view_entity.clone();
                    let view_details = view_entity.clone();
                    let is_expanded = expanded == Some(idx);
                    project_goal_card(
                        idx,
                        proj,
                        is_expanded,
                        palette,
                        move |_e, _w, cx| {
                            view_boost.update(cx, |this, cx| {
                                this.boost_project(idx, 50_000, cx);
                            });
                        },
                        move |_e, _w, cx| {
                            view_details.update(cx, |this, cx| {
                                if this.expanded_project_index == Some(idx) {
                                    this.expanded_project_index = None;
                                } else {
                                    this.expanded_project_index = Some(idx);
                                }
                                cx.notify();
                            });
                        },
                    )
                }),
        );

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(hero_card)
            .child(projects_grid)
            .into_any_element()
    }

    fn render_people_screen(&self, palette: ThemeColors, view_entity: Entity<Self>) -> AnyElement {
        let total_pledge: i64 = self.family_members.iter().map(|m| m.monthly_goal_rwf).sum();
        let total_contributed: i64 = self.family_members.iter().map(|m| m.contributed_rwf).sum();
        let pledge_pct = if total_pledge <= 0 {
            100
        } else {
            ((total_contributed * 100) / total_pledge).clamp(0, 100) as u8
        };

        let view_invite = view_entity.clone();
        let hero_card = card_surface(palette)
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
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child("Family Circle Monthly Pledge"),
                            )
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child(format!(
                                        "{} / {} ({}%)",
                                        format_rwf(total_contributed),
                                        format_rwf(total_pledge),
                                        pledge_pct
                                    )),
                            ),
                    )
                    .child(compact_primary_button(
                        "people-invite-member-btn",
                        "+ Invite",
                        palette,
                        move |_e, _w, cx| {
                            view_invite.update(cx, |this, cx| {
                                this.family_members.push(MemberContributionRow {
                                    id: format!("m{}", this.family_members.len() + 1).into(),
                                    name: "Cousin Clarisse".into(),
                                    role: "Family Member · Huye".into(),
                                    monthly_goal_rwf: 75_000,
                                    contributed_rwf: 25_000,
                                    status: "Joined ✨".into(),
                                    accent_color: palette.accent_primary,
                                });
                                this.toast_notice = Some(
                                    "💌 Invited Cousin Clarisse to the Umuryango Circle!"
                                        .to_string(),
                                );
                                cx.notify();
                            });
                        },
                    )),
            )
            .child(kawaii_progress_bar(
                pledge_pct,
                palette.accent_primary,
                palette,
            ));

        let members_grid =
            div()
                .flex()
                .flex_col()
                .gap_3()
                .children(self.family_members.iter().enumerate().map(|(idx, member)| {
                    let view_contrib = view_entity.clone();
                    let view_remind = view_entity.clone();
                    let member_name = member.name.to_string();
                    member_pledge_card(
                        idx,
                        member,
                        palette,
                        move |_e, _w, cx| {
                            view_contrib.update(cx, |this, cx| {
                                this.log_member_contribution(idx, 25_000, cx);
                            });
                        },
                        move |_e, _w, cx| {
                            let target_name = member_name.clone();
                            view_remind.update(cx, |this, cx| {
                                this.toast_notice = Some(format!(
                                "📲 Sent gentle Kinyarwanda MoMo pledge reminder to {target_name}!"
                            ));
                                cx.notify();
                            });
                        },
                    )
                }));

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(hero_card)
            .child(members_grid)
            .into_any_element()
    }

    fn render_profile_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let next_theme = self.theme.toggle();
        let next_lang = self.language.toggle();
        let view_theme = view_entity.clone();
        let view_lang = view_entity.clone();
        let view_sign_out = view_entity.clone();

        let identity_card = card_surface(palette)
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(member_avatar(
                        &self.member_name,
                        Some("Co-Admin · Family Planner"),
                        AvatarSize::Xl,
                        palette,
                    ))
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(palette.income_bg)
                            .border(px(1.5))
                            .border_color(palette.income_border)
                            .text_xs()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.income_text)
                            .child("Co-Admin · Family Planner"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_secondary)
                            .child("bruno@umuryango.rw · +250 788 123 456"),
                    ),
            )
            .child(
                card_elevated(palette)
                    .gap_3()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.text_primary)
                            .child("Cozy Theme & Pastel Mood Studio"),
                    )
                    .child(primary_button(
                        "profile-toggle-cozy-theme-btn",
                        format!("Switch to {}", next_theme.label()),
                        palette,
                        move |_e, _w, cx| {
                            view_theme.update(cx, |this, cx| {
                                this.set_theme(next_theme, cx);
                                cx.emit(DashboardEvent::ThemeChanged(next_theme));
                            });
                        },
                    ))
                    .child(div().flex().items_center().gap_2().children(
                        CozyMood::ALL.into_iter().map(|mood| {
                            let is_active = self.mood == mood;
                            let view_m = view_entity.clone();
                            let btn_id = match mood {
                                CozyMood::Matcha => "profile-mood-matcha",
                                CozyMood::Peach => "profile-mood-peach",
                                CozyMood::Berry => "profile-mood-berry",
                            };
                            div()
                                .id(btn_id)
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap_1p5()
                                .py_2()
                                .rounded(px(14.0))
                                .border(px(2.0))
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
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .cursor_pointer()
                                .child(
                                    div()
                                        .w(px(10.0))
                                        .h(px(10.0))
                                        .rounded_full()
                                        .bg(mood.swatch_color()),
                                )
                                .child(mood.label())
                                .on_click(move |_e, _w, cx| {
                                    view_m.update(cx, |this, cx| {
                                        this.set_mood(mood, cx);
                                    });
                                })
                        }),
                    ))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .pt_1()
                            .child(secondary_button(
                                "profile-switch-lang-btn",
                                format!("Language: {}", self.language.label()),
                                palette,
                                move |_e, w, cx| {
                                    view_lang.update(cx, |this, cx| {
                                        this.set_language(next_lang, w, cx);
                                        cx.emit(DashboardEvent::LanguageChanged(next_lang));
                                    });
                                },
                            ))
                            .child(destructive_button(
                                "profile-signout-btn",
                                text(self.language, "dashboard.sign_out"),
                                palette,
                                move |_e, _w, cx| {
                                    view_sign_out.update(cx, |_this, cx| {
                                        cx.emit(DashboardEvent::SignOutRequested);
                                    });
                                },
                            )),
                    ),
            );

        let view_r1 = view_entity.clone();
        let view_r2 = view_entity.clone();
        let view_r3 = view_entity.clone();
        let r1 = self.rule_multi_sig_over_30k;
        let r2 = self.rule_sunday_digest;
        let r3 = self.rule_auto_roundup_momo;

        let governance_card = card_surface(palette)
            .gap_2p5()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(palette.text_primary)
                    .child("Family Governance Rules"),
            )
            .child(card_elevated(palette).child(toggle_field(
                "rule-multisig-30k",
                "Require 2-Admin Approval over 30k RWF",
                None::<&str>,
                r1,
                palette,
                move |_e, _w, cx| {
                    view_r1.update(cx, |this, cx| {
                        this.rule_multi_sig_over_30k = !this.rule_multi_sig_over_30k;
                        cx.notify();
                    });
                },
            )))
            .child(card_elevated(palette).child(toggle_field(
                "rule-sunday-digest",
                "Sunday Evening Family Treasury Digest",
                None::<&str>,
                r2,
                palette,
                move |_e, _w, cx| {
                    view_r2.update(cx, |this, cx| {
                        this.rule_sunday_digest = !this.rule_sunday_digest;
                        cx.notify();
                    });
                },
            )))
            .child(card_elevated(palette).child(toggle_field(
                "rule-auto-roundup",
                "Auto-Round Up MoMo Change to Projects",
                None::<&str>,
                r3,
                palette,
                move |_e, _w, cx| {
                    view_r3.update(cx, |this, cx| {
                        this.rule_auto_roundup_momo = !this.rule_auto_roundup_momo;
                        cx.notify();
                    });
                },
            )));

        let channels_card = card_surface(palette)
            .gap_2p5()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(palette.text_primary)
                    .child("Connected Rwandan Channels"),
            )
            .child(
                div()
                    .flex()
                    .gap_2p5()
                    .when(stacked, |this| this.flex_col())
                    .child(
                        card_elevated(palette)
                            .flex_1()
                            .items_center()
                            .gap_1()
                            .child(
                                icons::icon(icons::ICON_PHONE)
                                    .size(px(16.0))
                                    .text_color(palette.accent_gold),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child("MTN MoMo"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.income_text)
                                    .child("Linked"),
                            ),
                    )
                    .child(
                        card_elevated(palette)
                            .flex_1()
                            .items_center()
                            .gap_1()
                            .child(
                                icons::icon(icons::ICON_CREDIT_CARD)
                                    .size(px(16.0))
                                    .text_color(palette.expense_text),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child("Airtel Money"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.income_text)
                                    .child("Linked"),
                            ),
                    )
                    .child(
                        card_elevated(palette)
                            .flex_1()
                            .items_center()
                            .gap_1()
                            .child(
                                icons::icon(icons::ICON_BUILDING_COLUMNS)
                                    .size(px(16.0))
                                    .text_color(palette.accent_primary),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child("I&M Bank"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.income_text)
                                    .child("Treasury"),
                            ),
                    ),
            );

        let view_sync = view_entity;
        let storage_card = card_surface(palette)
            .child(card_header(
                text(self.language, "dashboard.storage_health"),
                Some("Authoritative PostgreSQL • Local SQLite replica & outbox"),
                palette,
            ))
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
                            .items_center()
                            .gap_2()
                            .child(
                                icons::icon(icons::ICON_CIRCLE_CHECK)
                                    .size(px(14.0))
                                    .text_color(palette.income_text),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_primary)
                                    .child(self.storage_sync_status.clone()),
                            ),
                    )
                    .child(pill_button(
                        "profile-sync-now-btn",
                        text(self.language, "dashboard.sync_now"),
                        palette,
                        move |_e, _w, cx| {
                            view_sync.update(cx, |_this, cx| {
                                cx.emit(DashboardEvent::SyncRequested);
                            });
                        },
                    )),
            );

        div()
            .flex()
            .gap_4()
            .when(stacked, |this| this.flex_col())
            .child(div().flex_1().min_w_0().child(identity_card))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(governance_card)
                    .child(channels_card)
                    .child(storage_card),
            )
            .into_any_element()
    }
}

impl Render for DashboardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = colors(self.theme);
        let view_entity = cx.entity().clone();
        let viewport_size = window.viewport_size();
        let viewport_width = viewport_size.width;
        let content_width = app_content_width(viewport_width);
        let stacked = content_width < px(760.0);

        let select_section = {
            let view = view_entity.clone();
            move |section: AppSection, _window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_active_section(section, cx);
                });
            }
        };

        let select_money_tab = {
            let view = view_entity.clone();
            Rc::new(
                move |tab: MoneySubTab, _window: &mut Window, cx: &mut App| {
                    view.update(cx, |this, cx| {
                        this.navigate_to_money_tab(tab, cx);
                    });
                },
            )
        };

        let quick_entry_action = {
            let view = view_entity.clone();
            Rc::new(move |_window: &mut Window, cx: &mut App| {
                view.update(cx, |_this, cx| {
                    cx.emit(DashboardEvent::RecordContributionRequested);
                });
            })
        };

        let back_action = {
            let view = view_entity.clone();
            Rc::new(move |_window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.go_back(cx);
                });
            })
        };

        let mood_action = {
            let view = view_entity.clone();
            Rc::new(move |next_mood: CozyMood, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_mood(next_mood, cx);
                });
            })
        };

        let change_theme = {
            let view = view_entity.clone();
            move |next_theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(next_theme, cx);
                    cx.emit(DashboardEvent::ThemeChanged(next_theme));
                });
            }
        };

        let change_lang = {
            let view = view_entity.clone();
            move |next_lang: Language, window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_language(next_lang, window, cx);
                    cx.emit(DashboardEvent::LanguageChanged(next_lang));
                });
            }
        };

        let sign_out_action = {
            let view = view_entity.clone();
            move |_window: &mut Window, cx: &mut App| {
                view.update(cx, |_this, cx| {
                    cx.emit(DashboardEvent::SignOutRequested);
                });
            }
        };

        let toast_banner = self.toast_notice.as_ref().map(|msg| {
            let view_dismiss = view_entity.clone();
            dismissible_alert(
                "cozy-toast-banner",
                AlertVariant::Success,
                msg.clone(),
                palette,
                move |_e, _w, cx| {
                    view_dismiss.update(cx, |this, cx| {
                        this.toast_notice = None;
                        cx.notify();
                    });
                },
            )
        });

        let screen_body: AnyElement = match self.active_section {
            AppSection::Home => self.render_home_screen(stacked, palette, view_entity.clone()),
            AppSection::Money => {
                self.render_money_screen(stacked, palette, view_entity.clone(), cx)
            }
            AppSection::Projects => self.render_projects_screen(palette, view_entity.clone()),
            AppSection::Members => self.render_people_screen(palette, view_entity.clone()),
            AppSection::Profile => {
                self.render_profile_screen(stacked, palette, view_entity.clone())
            }
        };

        let active_projects_count = self.projects.iter().filter(|p| !p.is_completed).count();

        let page_container = div()
            .size_full()
            .bg(palette.bg_canvas)
            .text_color(palette.text_primary)
            .child(
                touch_scrollable_fill("dashboard-main-scroll").child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .p_5()
                        .when_some(toast_banner, |this, banner| this.child(banner))
                        .child(screen_body),
                ),
            );

        app_layout(
            AppLayoutProps {
                viewport_width,
                theme: self.theme,
                mood: self.mood,
                language: self.language,
                active_section: self.active_section,
                money_sub_tab: self.money_sub_tab,
                can_go_back: !self.nav_history.is_empty()
                    || self.active_section != AppSection::Home,
                member_name: &self.member_name,
                family_name: &self.family_name,
                total_treasury: &self.total_treasury,
                money_badge_count: self.transactions.len(),
                projects_badge_count: active_projects_count,
                members_badge_count: self.family_members.len(),
                pending_approvals_count: self.approvals.len(),
                on_select_money_tab: Some(select_money_tab),
                on_quick_entry: Some(quick_entry_action),
                on_back: Some(back_action),
                on_mood_change: Some(mood_action),
            },
            select_section,
            change_theme,
            change_lang,
            sign_out_action,
            page_container,
        )
    }
}
