use crate::brand::{apply_theme, colors, ThemeColors, ThemeMode};
use crate::components::dashboard::{
    approval_request_card, cashflow_insights_card, derive_expense_categories,
    derive_income_streams, derive_monthly_cashflow, expense_category_card,
    format_compact_signed_rwf, format_rwf, format_signed_rwf, income_stream_card, loan_card,
    member_pledge_card, pending_approvals_banner, project_goal_card, project_preview_row,
    transaction_row, treasury_hero_card, ApprovalItem, ChartMode, EntryModalKind,
    ExpenseCategoryRow, IncomeStreamRow, LoanFilter, LoanItem, MemberContributionRow, MoneySubTab,
    MonthlyCashflowPoint, ProjectFilter, ProjectSummary, QuickEntryKind, TransactionRow,
};
use crate::components::layouts::responsive::app_content_width;
use crate::components::layouts::{app_layout, AppLayoutProps, AppSection};
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{
    card_elevated, card_header, card_surface, compact_primary_button, destructive_button,
    dismissible_alert, filter_pill_group, icon_badge_box, interactive_card, kawaii_progress_bar,
    member_avatar, pill_button, pill_button_colored, primary_button, search_input,
    secondary_button, segmented_tab_bar, stateful_input_container, text_input, toggle_field,
    AlertVariant, AvatarSize, BadgeVariant, SegmentedTabItem,
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
    SyncRequested,
    QueueOutboxMutation { command: String, payload: String },
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
    member_role: String,
    member_contact: String,
    pub family_name: String,
    search_state: Entity<InputState>,
    active_entry_modal: Option<EntryModalKind>,
    entry_title_input: Entity<InputState>,
    entry_secondary_input: Entity<InputState>,
    entry_amount_input: Entity<InputState>,
    entry_loan_is_lent: bool,
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
    pub monthly_cashflow: Vec<MonthlyCashflowPoint>,
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
        let entry_title_input = cx.new(|cx| InputState::new(window, cx));
        let entry_secondary_input = cx.new(|cx| InputState::new(window, cx));
        let entry_amount_input = cx.new(|cx| InputState::new(window, cx));

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
            member_name: String::new(),
            member_role: String::new(),
            member_contact: String::new(),
            family_name: String::new(),
            search_state,
            active_entry_modal: None,
            entry_title_input,
            entry_secondary_input,
            entry_amount_input,
            entry_loan_is_lent: true,
            show_approval_alert: false,
            pending_approval_count: 0,
            pending_approval_tx_id: None,
            pending_approval_tx_title: None,
            pending_approval_tx_amount: None,
            total_treasury: "0 RWF".to_string(),
            monthly_contributions: "0 RWF".to_string(),
            monthly_expenses: "0 RWF".to_string(),
            emergency_reserve: "0 RWF".to_string(),
            treasury_balance_rwf: 0,
            monthly_income_rwf: 0,
            monthly_expenses_rwf: 0,
            transactions: Vec::new(),
            projects: Vec::new(),
            members: Vec::new(),
            family_members: Vec::new(),
            income_streams: Vec::new(),
            expense_categories: Vec::new(),
            monthly_cashflow: derive_monthly_cashflow(&[]),
            loans: Vec::new(),
            approvals: Vec::new(),
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
        self.toast_notice = Some(format!(
            "{} {}",
            text(self.language, "cozy.toast_mood_switched"),
            mood.label()
        ));
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

    pub fn open_entry_modal(
        &mut self,
        kind: EntryModalKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (title_ph, title_val, sec_ph, sec_val, amt_ph, amt_val) = match kind {
            EntryModalKind::Income => (
                "Description (e.g. Monthly Family Contribution)",
                "Monthly Family Contribution",
                "Category (e.g. Contribution, Salary, Rent)",
                "Contribution",
                "Amount in RWF",
                "50000",
            ),
            EntryModalKind::Expense => (
                "Description (e.g. Market Groceries & Supplies)",
                "Market Groceries & Supplies",
                "Category (e.g. Groceries, Education, Utilities)",
                "Groceries",
                "Amount in RWF",
                "35000",
            ),
            EntryModalKind::Loan => (
                "Reason / Purpose (e.g. Seed Potato Transport Bridge)",
                "Family Bridge Support",
                "Person / Counterparty Name",
                "Family Contact",
                "Total Loan Amount in RWF",
                "120000",
            ),
            EntryModalKind::Project => (
                "Project Name (e.g. Solar Roof & Battery)",
                "Community Solar Water System",
                "Category (HOUSE, EDUCATION, BUSINESS, VEHICLE, EVENT)",
                "HOUSE",
                "Target Amount in RWF",
                "1200000",
            ),
            EntryModalKind::MemberInvite => (
                "Member Full Name",
                "",
                "Phone Number (e.g. +250788123456)",
                "+25078",
                "Role (MEMBER, ADMIN, HEAD)",
                "MEMBER",
            ),
        };

        self.entry_title_input.update(cx, |state, cx| {
            state.set_placeholder(title_ph, window, cx);
            state.set_value(title_val, window, cx);
        });
        self.entry_secondary_input.update(cx, |state, cx| {
            state.set_placeholder(sec_ph, window, cx);
            state.set_value(sec_val, window, cx);
        });
        self.entry_amount_input.update(cx, |state, cx| {
            state.set_placeholder(amt_ph, window, cx);
            state.set_value(amt_val, window, cx);
        });
        self.active_entry_modal = Some(kind);
        cx.notify();
    }

    pub fn submit_entry_modal(&mut self, cx: &mut Context<Self>) {
        let Some(kind) = self.active_entry_modal else {
            return;
        };
        let title_str = self.entry_title_input.read(cx).value().trim().to_string();
        let secondary_str = self
            .entry_secondary_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let amount_raw = self.entry_amount_input.read(cx).value().trim().to_string();
        let parsed_amount: i64 = amount_raw
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i64>()
            .unwrap_or(0);

        let Some(fid) = crate::auth::client::get_current_family_id() else {
            self.toast_notice = Some("No active family workspace selected.".to_string());
            self.active_entry_modal = None;
            cx.notify();
            return;
        };

        match kind {
            EntryModalKind::Income | EntryModalKind::Expense => {
                let is_income = kind == EntryModalKind::Income;
                let amount = if parsed_amount > 0 {
                    parsed_amount
                } else {
                    50_000
                };
                let description = if title_str.is_empty() {
                    if is_income {
                        "Family Contribution".to_string()
                    } else {
                        "Family Expense".to_string()
                    }
                } else {
                    title_str
                };
                let category = if secondary_str.is_empty() {
                    if is_income {
                        "Contribution".to_string()
                    } else {
                        "General".to_string()
                    }
                } else {
                    secondary_str
                };

                let req = umuryango_shared::CreateTransactionRequest {
                    amount_minor_units: amount,
                    is_income,
                    category,
                    description: description.clone(),
                    notes: None,
                    idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
                };

                match crate::auth::client::api_create_transaction(&fid, &req) {
                    Ok(_) => {
                        self.toast_notice = Some(format!(
                            "{} ({})",
                            description,
                            format_signed_rwf(amount, is_income)
                        ));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        if let Ok(payload) = serde_json::to_string(&req) {
                            let command = if is_income {
                                "CREATE_CONTRIBUTION".to_string()
                            } else {
                                "CREATE_EXPENSE".to_string()
                            };
                            cx.emit(DashboardEvent::QueueOutboxMutation { command, payload });
                        }
                        self.toast_notice = Some(format!("Queued offline ({err})"));
                        self.active_entry_modal = None;
                        cx.notify();
                    }
                }
            }
            EntryModalKind::Project => {
                let target = if parsed_amount > 0 {
                    parsed_amount
                } else {
                    1_000_000
                };
                let name = if title_str.is_empty() {
                    "Family Project".to_string()
                } else {
                    title_str
                };
                let category = if secondary_str.is_empty() {
                    "OTHER".to_string()
                } else {
                    secondary_str
                };
                let req = umuryango_shared::CreateProjectRequest {
                    name: name.clone(),
                    category,
                    target_amount_minor_units: target,
                    description: Some(self.member_name.clone()),
                    target_date: None,
                };
                match crate::auth::client::api_create_project(&fid, &req) {
                    Ok(_) => {
                        self.toast_notice = Some(format!("Created project: {name}"));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        self.toast_notice = Some(err);
                        cx.notify();
                    }
                }
            }
            EntryModalKind::Loan => {
                let total = if parsed_amount > 0 {
                    parsed_amount
                } else {
                    100_000
                };
                let title = if title_str.is_empty() {
                    "Family Bridge Loan".to_string()
                } else {
                    title_str
                };
                let counterparty_name = if secondary_str.is_empty() {
                    "Family Contact".to_string()
                } else {
                    secondary_str
                };
                let req = umuryango_shared::CreateLoanRequest {
                    title,
                    counterparty_name: counterparty_name.clone(),
                    is_lent: self.entry_loan_is_lent,
                    total_amount_minor_units: total,
                    paid_amount_minor_units: 0,
                    due_date: None,
                };
                match crate::auth::client::api_create_loan(&fid, &req) {
                    Ok(_) => {
                        self.toast_notice = Some(format!(
                            "{} · {counterparty_name} ({})",
                            text(self.language, "cozy.toast_new_loan"),
                            format_rwf(total)
                        ));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        self.toast_notice = Some(err);
                        cx.notify();
                    }
                }
            }
            EntryModalKind::MemberInvite => {
                if secondary_str.is_empty() {
                    self.toast_notice = Some("Phone number is required.".to_string());
                    cx.notify();
                    return;
                }
                let req = umuryango_shared::InviteFamilyMemberRequest {
                    phone: secondary_str,
                    full_name: if title_str.is_empty() {
                        None
                    } else {
                        Some(title_str)
                    },
                    role: if amount_raw.is_empty() {
                        None
                    } else {
                        Some(amount_raw)
                    },
                };
                match crate::auth::client::api_invite_member(&fid, &req) {
                    Ok(member) => {
                        self.toast_notice = Some(format!(
                            "{} ({})",
                            text(self.language, "cozy.toast_invited"),
                            member.full_name
                        ));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        self.toast_notice = Some(err);
                        cx.notify();
                    }
                }
            }
        }
    }

    pub fn boost_project(&mut self, index: usize, amount_rwf: i64, cx: &mut Context<Self>) {
        let Some((project_id, proj_name)) = self
            .projects
            .get(index)
            .map(|p| (p.id.to_string(), p.name.to_string()))
        else {
            return;
        };

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let req = umuryango_shared::ProjectContributionRequest {
                amount_minor_units: amount_rwf,
                note: Some(format!("Boosted {proj_name}")),
            };
            match crate::auth::client::api_contribute_project(&fid, &project_id, &req) {
                Ok(_) => {
                    let tx_req = umuryango_shared::CreateTransactionRequest {
                        amount_minor_units: amount_rwf,
                        is_income: true,
                        category: "Project Boost".to_string(),
                        description: format!("Boosted {proj_name}"),
                        notes: None,
                        idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
                    };
                    let _ = crate::auth::client::api_create_transaction(&fid, &tx_req);
                    self.toast_notice = Some(format!(
                        "{} {proj_name}!",
                        text(self.language, "cozy.toast_boosted")
                    ));
                    self.refresh_from_backend(cx);
                    return;
                }
                Err(err) => {
                    self.toast_notice = Some(err);
                    cx.notify();
                    return;
                }
            }
        }
    }

    pub fn record_loan_repayment(&mut self, index: usize, amount_rwf: i64, cx: &mut Context<Self>) {
        let Some((loan_id, person, remaining)) = self
            .loans
            .get(index)
            .map(|l| (l.id.to_string(), l.person.to_string(), l.remaining_rwf()))
        else {
            return;
        };

        let step = amount_rwf.min(remaining);
        if step <= 0 {
            self.toast_notice = Some(format!(
                "{} {person}",
                text(self.language, "cozy.toast_loan_settled")
            ));
            cx.notify();
            return;
        }

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let req = umuryango_shared::LoanRepaymentRequest {
                amount_minor_units: step,
                note: Some(format!("Loan Repayment · {person}")),
            };
            match crate::auth::client::api_repay_loan(&fid, &loan_id, &req) {
                Ok(_) => {
                    self.toast_notice = Some(format!(
                        "{} {person} ({})!",
                        text(self.language, "cozy.toast_loan_repaid"),
                        format_rwf(step)
                    ));
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    pub fn resolve_approval(&mut self, index: usize, approve: bool, cx: &mut Context<Self>) {
        let Some(item) = self.approvals.get(index).cloned() else {
            return;
        };
        let tx_id = item.id.to_string();

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let res = if approve {
                crate::auth::client::api_approve_transaction(&fid, &tx_id)
            } else {
                crate::auth::client::api_reject_transaction(&fid, &tx_id)
            };

            match res {
                Ok(_) => {
                    self.toast_notice = if approve {
                        Some(format!(
                            "{} {} ({})",
                            text(self.language, "cozy.toast_approved"),
                            item.title,
                            format_rwf(item.amount_rwf)
                        ))
                    } else {
                        Some(format!(
                            "{} {}",
                            text(self.language, "cozy.toast_declined"),
                            item.title
                        ))
                    };
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    pub fn log_member_contribution(
        &mut self,
        index: usize,
        amount_rwf: i64,
        cx: &mut Context<Self>,
    ) {
        let Some(name) = self.family_members.get(index).map(|m| m.name.to_string()) else {
            return;
        };

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let req = umuryango_shared::CreateTransactionRequest {
                amount_minor_units: amount_rwf,
                is_income: true,
                category: "Contribution".to_string(),
                description: format!("{name} Family Circle Pledge"),
                notes: Some(format!("Recorded for {name}")),
                idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
            };
            match crate::auth::client::api_create_transaction(&fid, &req) {
                Ok(_) => {
                    self.toast_notice = Some(format!(
                        "{} {name}!",
                        text(self.language, "cozy.toast_member_contrib")
                    ));
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    pub fn refresh_from_backend(&mut self, cx: &mut Context<Self>) {
        // 1. Refresh authenticated user profile
        let current_user = crate::auth::client::get_current_user()
            .or_else(|| crate::auth::client::api_get_me().ok());
        if let Some(ref user) = current_user {
            if !user.full_name.trim().is_empty() {
                self.member_name = user.full_name.clone();
            }
            let mut contact_parts = Vec::new();
            if let Some(ref email) = user.email {
                if !email.trim().is_empty() {
                    contact_parts.push(email.trim().to_string());
                }
            }
            if let Some(ref phone) = user.phone_number {
                if !phone.trim().is_empty() {
                    contact_parts.push(phone.trim().to_string());
                }
            }
            self.member_contact = contact_parts.join(" · ");
        }

        if let Some(role) = crate::auth::client::get_current_role() {
            self.member_role = match role.as_str() {
                "HEAD" => "Family Head · Co-Admin".to_string(),
                "ADMIN" => "Co-Admin · Treasury Keeper".to_string(),
                _ => "Family Member".to_string(),
            };
        }

        // 2. Resolve active family workspace
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
            // 3. Authoritative treasury summary
            if let Ok(summary) = crate::auth::client::api_get_treasury_summary(fid) {
                self.treasury_balance_rwf = summary.total_treasury_minor_units;
                self.monthly_income_rwf = summary.monthly_contributions_minor_units;
                self.monthly_expenses_rwf = summary.monthly_expenses_minor_units;
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
                self.pending_approval_count = summary.pending_approval_count;
            }

            // 4. Authoritative transactions + dynamic income streams, expense categories, cashflow, and approvals
            let mut raw_txs = Vec::new();
            if let Ok(txs) = crate::auth::client::api_list_transactions(fid, None) {
                let pending_items: Vec<ApprovalItem> = txs
                    .iter()
                    .filter(|t| t.status == "PENDING")
                    .map(|p| ApprovalItem {
                        id: p.id.clone().into(),
                        requester: p.actor_name.clone().into(),
                        title: p.title.clone().into(),
                        category: p.category.clone().into(),
                        amount_rwf: p.amount_minor_units,
                        votes: "Pending Admin Approval".into(),
                    })
                    .collect();

                self.pending_approval_count = pending_items.len();
                if let Some(first) = pending_items.first() {
                    self.pending_approval_tx_id = Some(first.id.to_string());
                    self.pending_approval_tx_title = Some(first.title.to_string());
                    self.pending_approval_tx_amount = Some(format_rwf(first.amount_rwf));
                    self.show_approval_alert = true;
                } else {
                    self.pending_approval_tx_id = None;
                    self.pending_approval_tx_title = None;
                    self.pending_approval_tx_amount = None;
                    self.show_approval_alert = false;
                }
                self.approvals = pending_items;

                self.income_streams = derive_income_streams(&txs);
                self.expense_categories = derive_expense_categories(&txs);
                self.monthly_cashflow = derive_monthly_cashflow(&txs);
                self.transactions = txs.iter().map(TransactionRow::from_dto).collect();
                raw_txs = txs;
            }

            // 5. Authoritative communal projects
            if let Ok(projs) = crate::auth::client::api_list_projects(fid) {
                self.projects = projs.iter().map(ProjectSummary::from_dto).collect();
            }

            // 6. Authoritative family loans
            if let Ok(loans) = crate::auth::client::api_list_loans(fid) {
                self.loans = loans.iter().map(LoanItem::from_dto).collect();
            }

            // 7. Authoritative family members roster
            if let Ok(mems) = crate::auth::client::api_list_members(fid) {
                if let Some(ref user) = current_user {
                    if let Some(me_member) = mems.iter().find(|m| m.user_id == user.id) {
                        self.member_role = match me_member.role.as_str() {
                            "HEAD" => "Family Head · Co-Admin".to_string(),
                            "ADMIN" | "TREASURER" => "Co-Admin · Treasury Keeper".to_string(),
                            "ELDER" => "Elder Advisor".to_string(),
                            _ => "Family Member".to_string(),
                        };
                    }
                }

                self.family_members = mems
                    .iter()
                    .enumerate()
                    .map(|(idx, m)| MemberContributionRow::from_dto(m, idx, &raw_txs))
                    .collect();

                let base_fam_name = crate::auth::client::get_current_family_name()
                    .unwrap_or_else(|| self.family_name.clone());
                if !base_fam_name.is_empty() {
                    self.family_name = format!("{} · {} Members", base_fam_name, mems.len());
                }

                self.members = mems
                    .into_iter()
                    .map(|m| {
                        let role_desc = match m.role.as_str() {
                            "HEAD" => "Family Head / Admin",
                            "ADMIN" | "TREASURER" => "Treasurer / Admin",
                            "ELDER" => "Elder",
                            "YOUTH" => "Youth Member",
                            _ => "Member",
                        };
                        let variant = match m.role.as_str() {
                            "HEAD" => BadgeVariant::Primary,
                            "ADMIN" | "TREASURER" => BadgeVariant::Success,
                            "ELDER" => BadgeVariant::Warning,
                            "YOUTH" => BadgeVariant::Info,
                            _ => BadgeVariant::Neutral,
                        };
                        (m.full_name, role_desc.to_string(), variant)
                    })
                    .collect();
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

    fn render_money_screen(
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

    fn render_projects_screen(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let lang = self.language;
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
                            .child(text(lang, "cozy.total_saved_plans")),
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
                                "{} {} · {} {}",
                                active_count,
                                text(lang, "cozy.active_goals_word"),
                                completed_count,
                                text(lang, "cozy.completed_milestone_word")
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
                                format!(
                                    "{} ({active_count})",
                                    text(lang, "cozy.filter_active_plans")
                                ),
                            ),
                            (
                                "proj-filter-done",
                                ProjectFilter::Completed,
                                format!(
                                    "{} ({completed_count})",
                                    text(lang, "cozy.filter_completed")
                                ),
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
                        text(lang, "cozy.new_project_btn"),
                        palette,
                        move |_e, w, cx| {
                            view_new_proj.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::Project, w, cx);
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
                        lang,
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
        let lang = self.language;
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
                                    .child(text(lang, "cozy.circle_monthly_pledge")),
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
                        text(lang, "cozy.invite_btn"),
                        palette,
                        move |_e, w, cx| {
                            view_invite.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::MemberInvite, w, cx);
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
                        lang,
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
                                    "{} {target_name}!",
                                    text(this.language, "cozy.toast_reminded")
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
        let lang = self.language;
        let next_theme = self.theme.toggle();
        let next_lang = self.language.toggle();
        let view_theme = view_entity.clone();
        let view_lang = view_entity.clone();
        let view_sign_out = view_entity.clone();

        let role_subtitle = if self.member_role.is_empty() {
            text(lang, "cozy.coadmin_planner").to_string()
        } else {
            self.member_role.clone()
        };

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
                        Some(&role_subtitle),
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
                            .child(role_subtitle.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_secondary)
                            .child(self.member_contact.clone()),
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
                            .child(text(lang, "cozy.theme_studio_title")),
                    )
                    .child(primary_button(
                        "profile-toggle-cozy-theme-btn",
                        format!(
                            "{} {}",
                            text(lang, "cozy.switch_to_prefix"),
                            next_theme.label()
                        ),
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
                                format!(
                                    "{}: {}",
                                    text(lang, "section.language_label"),
                                    self.language.label()
                                ),
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
                                text(lang, "dashboard.sign_out"),
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
                    .child(text(lang, "cozy.governance_rules_title")),
            )
            .child(card_elevated(palette).child(toggle_field(
                "rule-multisig-30k",
                text(lang, "cozy.rule_multisig_30k"),
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
                text(lang, "cozy.rule_sunday_digest"),
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
                text(lang, "cozy.rule_auto_roundup"),
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
                    .child(text(lang, "cozy.connected_channels_title")),
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
                                    .child(text(lang, "cozy.channel_linked")),
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
                                    .child(text(lang, "cozy.channel_linked")),
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
                                    .child(text(lang, "cozy.channel_treasury")),
                            ),
                    ),
            );

        let view_sync = view_entity;
        let storage_card = card_surface(palette)
            .child(card_header(
                text(lang, "dashboard.storage_health"),
                Some(text(lang, "cozy.storage_sub")),
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
                        text(lang, "dashboard.sync_now"),
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

    fn render_entry_modal(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> Option<AnyElement> {
        let kind = self.active_entry_modal?;
        let (modal_title, field1_label, field2_label, field3_label) = match kind {
            EntryModalKind::Income => (
                "Record Family Income / Contribution",
                "Description / Title",
                "Category (e.g. Contribution, Farming, Business)",
                "Amount (RWF)",
            ),
            EntryModalKind::Expense => (
                "Record Family Expense",
                "Description / Purpose",
                "Category (e.g. Education, Healthcare, Farming)",
                "Amount (RWF)",
            ),
            EntryModalKind::Loan => (
                "Create Family Loan Agreement",
                "Loan Reason / Purpose",
                "Counterparty / Member Name",
                "Total Loan Principal (RWF)",
            ),
            EntryModalKind::Project => (
                "Create Communal Family Project",
                "Project Name",
                "Category (e.g. Infrastructure, Agriculture, Education)",
                "Target Goal Amount (RWF)",
            ),
            EntryModalKind::MemberInvite => (
                "Add or Invite Family Member",
                "Full Name",
                "Phone Number (e.g. +250788000000)",
                "Role (MEMBER, ADMIN, ELDER, TREASURER)",
            ),
        };

        let view_close = view_entity.clone();
        let view_submit = view_entity.clone();
        let view_toggle_lent = view_entity.clone();
        let is_lent = self.entry_loan_is_lent;

        let card = card_surface(palette)
            .gap_3()
            .border_color(palette.accent_primary)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.text_primary)
                            .child(modal_title),
                    )
                    .child(pill_button(
                        "entry-modal-cancel-top-btn",
                        "Cancel",
                        palette,
                        move |_e, _w, cx| {
                            view_close.update(cx, |this, cx| {
                                this.active_entry_modal = None;
                                cx.notify();
                            });
                        },
                    )),
            )
            .when(kind == EntryModalKind::Loan, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(pill_button_colored(
                            "entry-loan-type-lent",
                            if is_lent {
                                "✓ Lent Out by Treasury"
                            } else {
                                "Lent Out by Treasury"
                            },
                            if is_lent {
                                palette.income_text
                            } else {
                                palette.text_secondary
                            },
                            palette,
                            {
                                let vt = view_toggle_lent.clone();
                                move |_e, _w, cx| {
                                    vt.update(cx, |this, cx| {
                                        this.entry_loan_is_lent = true;
                                        cx.notify();
                                    });
                                }
                            },
                        ))
                        .child(pill_button_colored(
                            "entry-loan-type-borrowed",
                            if !is_lent {
                                "✓ Borrowed by Family"
                            } else {
                                "Borrowed by Family"
                            },
                            if !is_lent {
                                palette.loan_text
                            } else {
                                palette.text_secondary
                            },
                            palette,
                            move |_e, _w, cx| {
                                view_toggle_lent.update(cx, |this, cx| {
                                    this.entry_loan_is_lent = false;
                                    cx.notify();
                                });
                            },
                        )),
                )
            })
            .child(
                div()
                    .flex()
                    .gap_3()
                    .flex_wrap()
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(200.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(field1_label),
                            )
                            .child(stateful_input_container(
                                palette,
                                &self.entry_title_input,
                                text_input(&self.entry_title_input, palette),
                            )),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(180.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(field2_label),
                            )
                            .child(stateful_input_container(
                                palette,
                                &self.entry_secondary_input,
                                text_input(&self.entry_secondary_input, palette),
                            )),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(150.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(field3_label),
                            )
                            .child(stateful_input_container(
                                palette,
                                &self.entry_amount_input,
                                text_input(&self.entry_amount_input, palette),
                            )),
                    ),
            )
            .child(div().flex().justify_end().child(compact_primary_button(
                "entry-modal-submit-btn",
                "Save to Backend",
                palette,
                move |_e, _w, cx| {
                    view_submit.update(cx, |this, cx| {
                        this.submit_entry_modal(cx);
                    });
                },
            )));

        Some(card.into_any_element())
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
            Rc::new(move |window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    let modal_kind = match this.active_section {
                        AppSection::Projects => EntryModalKind::Project,
                        AppSection::Members => EntryModalKind::MemberInvite,
                        AppSection::Money => match this.money_sub_tab {
                            MoneySubTab::Expenses => EntryModalKind::Expense,
                            MoneySubTab::Loans => EntryModalKind::Loan,
                            _ => EntryModalKind::Income,
                        },
                        _ => EntryModalKind::Income,
                    };
                    this.open_entry_modal(modal_kind, window, cx);
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

        let entry_modal = self.render_entry_modal(palette, view_entity.clone());

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
                        .when_some(entry_modal, |this, modal| this.child(modal))
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
