pub(super) use crate::brand::{apply_theme, CozyMood};
use crate::brand::{colors, ThemeMode};
pub(super) use crate::components::dashboard::*;
pub(super) use crate::components::layouts::responsive::app_content_width;
pub(super) use crate::components::layouts::{
    app_layout, AppLayoutProps, AppSection, WorkspaceMode,
};
pub(super) use crate::components::scroll::touch_scrollable_fill;
pub(super) use crate::components::ui::*;
pub(super) use crate::i18n::{text, Language};
use crate::theme::{active_mood, set_cozy_mood};
pub(super) use crate::updater::{UpdateApplyOutcome, UpdateState};
pub(super) use gpui_kit::component::input::{InputState, TextareaState};
pub(super) use gpui_kit::component::select::{SelectEvent, SelectState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomCategoryItem {
    pub name: String,
    pub is_income: bool,
    pub icon: String,
    pub color: String,
    pub description: String,
}
use gpui_kit::base::{ToastManager, ToastMotion, ToastOptions, ToastStackState};
pub(super) use gpui_kit::prelude::FluentBuilder;
pub(super) use gpui_kit::*;
use std::rc::Rc;
use std::time::{Duration, Instant};

mod actions;
mod entry_modal;

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
    pub workspace_mode: WorkspaceMode,
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
    member_phone: String,
    member_email: String,
    pub user_dropdown_open: bool,
    pub family_name: String,
    search_state: Entity<InputState>,
    active_entry_modal: Option<EntryModalKind>,
    entry_title_input: Entity<InputState>,
    entry_secondary_input: Entity<InputState>,
    entry_amount_input: Entity<InputState>,
    pub(super) entry_category_select: Entity<SelectState<Vec<String>>>,
    pub(super) entry_account_select: Entity<SelectState<Vec<String>>>,
    pub(super) entry_date_picker: Entity<DatePickerState>,
    pub(super) entry_date_input: Entity<InputState>,
    pub(super) entry_notes_input: Entity<TextareaState>,
    entry_loan_is_lent: bool,
    pub(super) entry_category_is_income: bool,
    pub(super) entry_category_return_modal: Option<EntryModalKind>,
    pub(super) entry_category_icon: String,
    pub(super) entry_category_color: String,
    pub(super) entry_category_color_input: Entity<InputState>,
    pub(super) entry_category_desc_input: Entity<TextareaState>,
    pub(super) entry_category_icon_input: Entity<InputState>,
    pub(super) saved_tx_title: String,
    pub(super) saved_tx_amount: String,
    pub custom_categories: Vec<CustomCategoryItem>,
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
    pub report_period: ReportPeriodFilter,
    pub selected_report_point: usize,
    pub report_timeline_points: Vec<ReportTimelinePoint>,
    pub rule_multi_sig_over_30k: bool,
    pub rule_sunday_digest: bool,
    pub rule_auto_roundup_momo: bool,
    pub toast_notice: Option<String>,
    pub(super) toast_manager: ToastManager<u64, ToastItem>,
    pub(super) toast_stack_state: ToastStackState,
    pub(super) next_toast_id: u64,
    pub(super) toast_clock_running: bool,
    pub storage_sync_status: String,
    pub(super) update_state: UpdateState,
    pub(super) show_update_modal: bool,
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
        cx.observe(&search_state, |_this, _input, cx| {
            cx.notify();
        })
        .detach();

        let entry_title_input = cx.new(|cx| InputState::new(window, cx));
        let entry_secondary_input = cx.new(|cx| InputState::new(window, cx));
        let entry_amount_input =
            cx.new(|cx| InputState::new(window, cx).mask_pattern(money_mask_pattern("RWF")));
        let entry_category_select = cx.new(|cx| {
            SelectState::new(
                vec![
                    "Contribution".to_string(),
                    "Business".to_string(),
                    "Farming".to_string(),
                    "Salary".to_string(),
                ],
                None,
                window,
                cx,
            )
        });
        cx.subscribe_in(
            &entry_category_select,
            window,
            |this, _state, event: &SelectEvent<Vec<String>>, window, cx| {
                if let SelectEvent::Confirm(Some(cat)) = event {
                    let cat_val = cat.clone();
                    this.entry_secondary_input.update(cx, |input, cx| {
                        input.set_value(cat_val, window, cx);
                    });
                    cx.notify();
                }
            },
        )
        .detach();
        let entry_account_select = cx.new(|cx| {
            SelectState::new(
                vec![
                    "Mobile Money".to_string(),
                    "Cash".to_string(),
                    "Bank Account".to_string(),
                ],
                None,
                window,
                cx,
            )
        });
        let entry_date_picker =
            cx.new(|cx| DatePickerState::new(window, cx).date_format("%Y-%m-%d"));
        let entry_date_input = cx.new(|cx| InputState::new(window, cx));
        let entry_notes_input = cx.new(|cx| TextareaState::new(window, cx));
        let entry_category_desc_input = cx.new(|cx| TextareaState::new(window, cx));
        let entry_category_icon_input = cx.new(|cx| InputState::new(window, cx));
        cx.observe(&entry_category_icon_input, |_this, _input, cx| {
            cx.notify();
        })
        .detach();

        let entry_category_color_input = cx.new(|cx| InputState::new(window, cx));
        cx.observe(&entry_category_color_input, |this, input, cx| {
            let val = input.read(cx).value().trim().to_string();
            if !val.is_empty() {
                let hex = if val.starts_with('#') {
                    val
                } else {
                    format!("#{val}")
                };
                this.entry_category_color = hex;
            }
            cx.notify();
        })
        .detach();

        let current_user = crate::auth::client::get_current_user();
        let (initial_name, initial_phone, initial_email, initial_contact) =
            if let Some(ref u) = current_user {
                let name = if u.full_name.trim().is_empty() {
                    "Rwanda Bruno".to_string()
                } else {
                    u.full_name.clone()
                };
                let phone = u.phone_number.clone().unwrap_or_default();
                let email = u.email.clone().unwrap_or_default();
                let mut parts = Vec::new();
                if !phone.trim().is_empty() {
                    parts.push(phone.trim().to_string());
                }
                if !email.trim().is_empty() {
                    parts.push(email.trim().to_string());
                }
                let contact = if parts.is_empty() {
                    "brunorwanda4@gmail.com".to_string()
                } else {
                    parts.join(" · ")
                };
                (name, phone, email, contact)
            } else {
                (
                    "Rwanda Bruno".to_string(),
                    "+250 788 123 456".to_string(),
                    "brunorwanda4@gmail.com".to_string(),
                    "brunorwanda4@gmail.com".to_string(),
                )
            };
        let initial_family = crate::auth::client::get_current_family_name()
            .unwrap_or_else(|| "Ingabire Family".to_string());

        let mut view = Self {
            theme,
            mood,
            language,
            workspace_mode: WorkspaceMode::Family,
            active_section: AppSection::Home,
            money_sub_tab: MoneySubTab::Overview,
            chart_mode: ChartMode::Cashflow,
            loan_filter: LoanFilter::All,
            project_filter: ProjectFilter::Active,
            expanded_project_index: None,
            nav_history: Vec::new(),
            member_name: initial_name,
            member_role: String::new(),
            member_contact: initial_contact,
            member_phone: initial_phone,
            member_email: initial_email,
            user_dropdown_open: false,
            family_name: initial_family,
            search_state,
            active_entry_modal: None,
            entry_title_input,
            entry_secondary_input,
            entry_amount_input,
            entry_category_select,
            entry_account_select,
            entry_date_picker,
            entry_date_input,
            entry_notes_input,
            entry_loan_is_lent: true,
            entry_category_is_income: false,
            entry_category_return_modal: None,
            entry_category_icon: "tag".to_string(),
            entry_category_color: "#8CE0B8".to_string(),
            entry_category_color_input,
            entry_category_desc_input,
            entry_category_icon_input,
            saved_tx_title: String::new(),
            saved_tx_amount: String::new(),
            custom_categories: Vec::new(),
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
            report_period: ReportPeriodFilter::ThisMonth,
            selected_report_point: 2,
            report_timeline_points: derive_report_timeline(&[], ReportPeriodFilter::ThisMonth),
            rule_multi_sig_over_30k: true,
            rule_sunday_digest: true,
            rule_auto_roundup_momo: false,
            toast_notice: None,
            toast_manager: ToastManager::new(ToastMotion::sonner()),
            toast_stack_state: ToastStackState::default(),
            next_toast_id: 0,
            toast_clock_running: false,
            storage_sync_status: text(language, "dashboard.storage_health_desc").to_string(),
            update_state: UpdateState::Idle,
            show_update_modal: false,
        };
        view.check_for_app_update(false, cx);
        view
    }

    pub fn set_member_name(&mut self, name: String, cx: &mut Context<Self>) {
        if !name.trim().is_empty() {
            self.member_name = name;
            cx.notify();
        }
    }

    pub fn set_active_section(&mut self, section: AppSection, cx: &mut Context<Self>) {
        self.user_dropdown_open = false;
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

    pub fn set_workspace_mode(&mut self, mode: WorkspaceMode, cx: &mut Context<Self>) {
        if self.workspace_mode != mode {
            self.workspace_mode = mode;
            self.active_section = AppSection::Home;
            let mode_label = text(self.language, mode.label_key());
            let msg = format!(
                "{} {}",
                text(self.language, "workspace.switched_to"),
                mode_label
            );
            self.toast_info(msg, None::<&str>, cx);
            cx.notify();
        }
    }

    pub fn set_report_period(&mut self, period: ReportPeriodFilter, cx: &mut Context<Self>) {
        self.report_period = period;
        self.report_timeline_points = derive_report_timeline(&self.transactions, period);
        cx.notify();
    }

    pub fn select_report_point(&mut self, idx: usize, cx: &mut Context<Self>) {
        self.selected_report_point = idx;
        cx.notify();
    }

    pub fn download_report_csv(&mut self, cx: &mut Context<Self>) {
        match export_report_csv(&self.family_name, &self.transactions) {
            Ok(path) => {
                let msg = format!("Report saved to {}", path.display());
                self.toast_success("Export Succeeded", Some(msg), cx);
            }
            Err(err) => {
                self.toast_error("Export Failed", Some(err), cx);
            }
        }
    }

    pub fn download_report_summary(&mut self, cx: &mut Context<Self>) {
        let total_income: i64 = self
            .transactions
            .iter()
            .filter(|t| t.is_income)
            .map(|t| t.amount_rwf)
            .sum();
        let total_expenses: i64 = self
            .transactions
            .iter()
            .filter(|t| !t.is_income)
            .map(|t| t.amount_rwf)
            .sum();
        let total_contributions: i64 = self
            .transactions
            .iter()
            .filter(|t| t.is_income && t.category.to_lowercase().contains("contribution"))
            .map(|t| t.amount_rwf)
            .sum();
        match export_report_summary(
            &self.family_name,
            total_income,
            total_expenses,
            total_contributions,
            self.transactions.len(),
        ) {
            Ok(path) => {
                let msg = format!("Summary saved to {}", path.display());
                self.toast_success("Export Succeeded", Some(msg), cx);
            }
            Err(err) => {
                self.toast_error("Export Failed", Some(err), cx);
            }
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
        cx.emit(DashboardEvent::ThemeChanged(theme));
        cx.notify();
    }

    pub fn set_mood(&mut self, mood: CozyMood, cx: &mut Context<Self>) {
        self.mood = mood;
        set_cozy_mood(mood, cx);
        cx.emit(DashboardEvent::ThemeChanged(self.theme));
        self.toast_info(
            "Mood Updated",
            Some(format!(
                "{} {}",
                text(self.language, "cozy.toast_mood_switched"),
                mood.label()
            )),
            cx,
        );
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
        self.update_entry_modal_placeholders(language, window, cx);
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

    pub fn push_toast(&mut self, item: ToastItem, cx: &mut Context<Self>) {
        let id = item.id;
        let duration = item.duration;
        self.toast_manager.push(
            id,
            item,
            ToastOptions {
                timeout: Some(duration),
            },
            Instant::now(),
        );
        self.start_toast_clock(cx);
        cx.notify();
    }

    pub fn dismiss_toast(&mut self, id: u64, cx: &mut Context<Self>) {
        self.toast_manager.dismiss(&id, Instant::now());
        self.start_toast_clock(cx);
        cx.notify();
    }

    pub fn toast_success(
        &mut self,
        title: impl Into<SharedString>,
        message: Option<impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.next_toast_id += 1;
        let mut item = ToastItem::success(self.next_toast_id, title);
        if let Some(msg) = message {
            item = item.with_message(msg);
        }
        self.push_toast(item, cx);
    }

    pub fn toast_expense(
        &mut self,
        title: impl Into<SharedString>,
        message: Option<impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.next_toast_id += 1;
        let mut item = ToastItem::expense(self.next_toast_id, title);
        if let Some(msg) = message {
            item = item.with_message(msg);
        }
        self.push_toast(item, cx);
    }

    pub fn toast_info(
        &mut self,
        title: impl Into<SharedString>,
        message: Option<impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.next_toast_id += 1;
        let mut item = ToastItem::info(self.next_toast_id, title);
        if let Some(msg) = message {
            item = item.with_message(msg);
        }
        self.push_toast(item, cx);
    }

    pub fn toast_warning(
        &mut self,
        title: impl Into<SharedString>,
        message: Option<impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.next_toast_id += 1;
        let mut item = ToastItem::warning(self.next_toast_id, title);
        if let Some(msg) = message {
            item = item.with_message(msg);
        }
        self.push_toast(item, cx);
    }

    pub fn toast_error(
        &mut self,
        title: impl Into<SharedString>,
        message: Option<impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.next_toast_id += 1;
        let mut item = ToastItem::error(self.next_toast_id, title);
        if let Some(msg) = message {
            item = item.with_message(msg);
        }
        self.push_toast(item, cx);
    }

    fn start_toast_clock(&mut self, cx: &mut Context<Self>) {
        if self.toast_clock_running {
            return;
        }
        self.toast_clock_running = true;
        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let keep_running = this.update(cx, |view, cx| {
                let now = Instant::now();
                let paused = view.toast_stack_state.is_expanded();
                let advance = view.toast_manager.advance(now, paused);
                if advance.changed {
                    cx.notify();
                }
                let running = !view.toast_manager.is_empty();
                view.toast_clock_running = running;
                running
            });
            if !matches!(keep_running, Ok(true)) {
                break;
            }
        })
        .detach();
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
                    this.user_dropdown_open = false;
                    this.set_active_section(section, cx);
                });
            }
        };

        let select_workspace_mode = {
            let view = view_entity.clone();
            Rc::new(
                move |mode: WorkspaceMode, _window: &mut Window, cx: &mut App| {
                    view.update(cx, |this, cx| {
                        this.user_dropdown_open = false;
                        this.set_workspace_mode(mode, cx);
                    });
                },
            )
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

        let toggle_user_dropdown = {
            let view = view_entity.clone();
            Rc::new(move |_window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.user_dropdown_open = !this.user_dropdown_open;
                    cx.notify();
                });
            })
        };

        let close_user_dropdown = {
            let view = view_entity.clone();
            Rc::new(move |_window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    if this.user_dropdown_open {
                        this.user_dropdown_open = false;
                        cx.notify();
                    }
                });
            })
        };

        let entry_modal = self.render_entry_modal(palette, view_entity.clone(), cx);

        let active_release = match &self.update_state {
            UpdateState::Available(rel) => Some((rel.clone(), false)),
            UpdateState::Downloading(rel) => Some((rel.clone(), true)),
            _ => None,
        };

        let update_banner = active_release.as_ref().map(|(rel, is_downloading)| {
            let view_review = view_entity.clone();
            let view_install = view_entity.clone();
            update_notification_banner(
                rel,
                *is_downloading,
                self.language,
                palette,
                move |_e, _w, cx| {
                    view_review.update(cx, |this, cx| {
                        this.show_update_modal = true;
                        cx.notify();
                    });
                },
                move |_e, _w, cx| {
                    view_install.update(cx, |this, cx| {
                        this.install_app_update(cx);
                    });
                },
            )
        });

        let update_modal = if self.show_update_modal {
            active_release.as_ref().map(|(rel, is_downloading)| {
                let view_close = view_entity.clone();
                let view_install = view_entity.clone();
                render_update_modal(
                    rel,
                    *is_downloading,
                    self.language,
                    palette,
                    move |_w, cx| {
                        view_close.update(cx, |this, cx| {
                            this.show_update_modal = false;
                            cx.notify();
                        });
                    },
                    move |_e, _w, cx| {
                        view_install.update(cx, |this, cx| {
                            this.install_app_update(cx);
                        });
                    },
                )
            })
        } else {
            None
        };

        let screen_body: AnyElement = match self.active_section {
            AppSection::Home => {
                let view_for_sec = view_entity.clone();
                let view_for_tab = view_entity.clone();
                let view_for_qa = view_entity.clone();
                let view_for_cm = view_entity.clone();
                let view_for_boost = view_entity.clone();
                crate::home::home_page(crate::home::HomePageProps {
                    language: self.language,
                    palette,
                    stacked,
                    total_treasury: &self.total_treasury,
                    monthly_income_rwf: self.monthly_income_rwf,
                    monthly_expenses_rwf: self.monthly_expenses_rwf,
                    monthly_contributions: &self.monthly_contributions,
                    monthly_expenses: &self.monthly_expenses,
                    income_streams: &self.income_streams,
                    expense_categories: &self.expense_categories,
                    approvals: &self.approvals,
                    chart_mode: self.chart_mode,
                    monthly_cashflow: &self.monthly_cashflow,
                    projects: &self.projects,
                    transactions: &self.transactions,
                    on_navigate_section: Rc::new(move |sec, _w, cx| {
                        view_for_sec.update(cx, |this, cx| {
                            this.set_active_section(sec, cx);
                        });
                    }),
                    on_navigate_money_tab: Rc::new(move |tab, _w, cx| {
                        view_for_tab.update(cx, |this, cx| {
                            this.navigate_to_money_tab(tab, cx);
                        });
                    }),
                    on_quick_action: Rc::new(move |kind, window, cx| {
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
                    }),
                    on_change_chart_mode: Rc::new(move |mode, _w, cx| {
                        view_for_cm.update(cx, |this, cx| {
                            this.chart_mode = mode;
                            cx.notify();
                        });
                    }),
                    on_boost_project: Rc::new(move |idx, amount, _w, cx| {
                        view_for_boost.update(cx, |this, cx| {
                            this.boost_project(idx, amount, cx);
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Money => {
                let view_for_tab = view_entity.clone();
                let view_for_modal = view_entity.clone();
                let view_for_cat = view_entity.clone();
                let view_for_appr = view_entity.clone();
                let view_for_filter = view_entity.clone();
                let view_for_repay = view_entity.clone();
                crate::money::money_page(
                    crate::money::MoneyPageProps {
                        language: self.language,
                        palette,
                        stacked,
                        sub_tab: self.money_sub_tab,
                        loans: &self.loans,
                        monthly_income_rwf: self.monthly_income_rwf,
                        monthly_expenses_rwf: self.monthly_expenses_rwf,
                        total_treasury: &self.total_treasury,
                        monthly_contributions: &self.monthly_contributions,
                        monthly_expenses: &self.monthly_expenses,
                        income_streams: &self.income_streams,
                        expense_categories: &self.expense_categories,
                        monthly_cashflow: &self.monthly_cashflow,
                        approvals: &self.approvals,
                        transactions: &self.transactions,
                        loan_filter: self.loan_filter,
                        chart_mode: self.chart_mode,
                        search_state: &self.search_state,
                        on_select_sub_tab: Rc::new(move |tab, _w, cx| {
                            view_for_tab.update(cx, |this, cx| {
                                this.navigate_to_money_tab(tab, cx);
                            });
                        }),
                        on_open_entry_modal: Rc::new(move |kind, window, cx| {
                            view_for_modal.update(cx, |this, cx| {
                                this.open_entry_modal(kind, window, cx);
                            });
                        }),
                        on_open_category_modal: Rc::new(move |is_income, window, cx| {
                            view_for_cat.update(cx, |this, cx| {
                                this.open_category_modal(is_income, window, cx);
                            });
                        }),
                        on_resolve_approval: Rc::new(move |idx, approved, _w, cx| {
                            view_for_appr.update(cx, |this, cx| {
                                this.resolve_approval(idx, approved, cx);
                            });
                        }),
                        on_change_loan_filter: Rc::new(move |filter, _w, cx| {
                            view_for_filter.update(cx, |this, cx| {
                                this.loan_filter = filter;
                                cx.notify();
                            });
                        }),
                        on_repay_loan: Rc::new(move |idx, amount, _w, cx| {
                            view_for_repay.update(cx, |this, cx| {
                                this.record_loan_repayment(idx, amount, cx);
                            });
                        }),
                    },
                    cx,
                )
                .into_any_element()
            }
            AppSection::Campaigns => {
                let view_for_new = view_entity.clone();
                let view_for_contrib = view_entity.clone();
                crate::campaigns::campaigns_page(crate::campaigns::CampaignsPageProps {
                    language: self.language,
                    palette,
                    stacked,
                    on_new_campaign: Rc::new(move |_w, cx| {
                        view_for_new.update(cx, |this, cx| {
                            this.toast_info("Campaign creation dialog ready", None::<&str>, cx);
                        });
                    }),
                    on_contribute: Rc::new(move |name, amount, _w, cx| {
                        view_for_contrib.update(cx, |this, cx| {
                            this.toast_success(
                                format!("Pledged {} to {}", format_rwf(amount), name),
                                Some("Contribution recorded to campaign"),
                                cx,
                            );
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Projects => {
                let view_for_filter = view_entity.clone();
                let view_for_new = view_entity.clone();
                let view_for_boost = view_entity.clone();
                let view_for_expand = view_entity.clone();
                crate::projects::projects_page(crate::projects::ProjectsPageProps {
                    language: self.language,
                    palette,
                    projects: &self.projects,
                    project_filter: self.project_filter,
                    expanded_project_index: self.expanded_project_index,
                    on_change_filter: Rc::new(move |filter, _w, cx| {
                        view_for_filter.update(cx, |this, cx| {
                            this.project_filter = filter;
                            cx.notify();
                        });
                    }),
                    on_new_project: Rc::new(move |window, cx| {
                        view_for_new.update(cx, |this, cx| {
                            this.open_entry_modal(EntryModalKind::Project, window, cx);
                        });
                    }),
                    on_boost_project: Rc::new(move |idx, amount, _w, cx| {
                        view_for_boost.update(cx, |this, cx| {
                            this.boost_project(idx, amount, cx);
                        });
                    }),
                    on_toggle_expand: Rc::new(move |idx, _w, cx| {
                        view_for_expand.update(cx, |this, cx| {
                            if this.expanded_project_index == Some(idx) {
                                this.expanded_project_index = None;
                            } else {
                                this.expanded_project_index = Some(idx);
                            }
                            cx.notify();
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Events => {
                let view_for_new = view_entity.clone();
                let view_for_rsvp = view_entity.clone();
                crate::events::events_page(crate::events::EventsPageProps {
                    language: self.language,
                    palette,
                    stacked,
                    on_new_event: Rc::new(move |_w, cx| {
                        view_for_new.update(cx, |this, cx| {
                            this.toast_info("Event schedule dialog ready", None::<&str>, cx);
                        });
                    }),
                    on_rsvp: Rc::new(move |title, status, _w, cx| {
                        view_for_rsvp.update(cx, |this, cx| {
                            this.toast_success(
                                format!("RSVP {}: {}", status, title),
                                Some("Family calendar updated"),
                                cx,
                            );
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Vault => {
                let view_for_upload = view_entity.clone();
                let view_for_access = view_entity.clone();
                crate::vault::vault_page(crate::vault::VaultPageProps {
                    language: self.language,
                    palette,
                    stacked,
                    on_upload_doc: Rc::new(move |_w, cx| {
                        view_for_upload.update(cx, |this, cx| {
                            this.toast_info("Encrypted document upload ready", None::<&str>, cx);
                        });
                    }),
                    on_access_doc: Rc::new(move |title, _w, cx| {
                        view_for_access.update(cx, |this, cx| {
                            this.toast_success(
                                format!("Decrypted: {}", title),
                                Some("Audit key verified with biometric signature"),
                                cx,
                            );
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Members => {
                let view_for_invite = view_entity.clone();
                crate::members::members_page(crate::members::MembersPageProps {
                    language: self.language,
                    palette,
                    members: &self.members,
                    family_members: &self.family_members,
                    on_invite_member: Rc::new(move |window, cx| {
                        view_for_invite.update(cx, |this, cx| {
                            this.open_entry_modal(EntryModalKind::MemberInvite, window, cx);
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Profile => {
                let view_for_lang = view_entity.clone();
                let view_for_th = view_entity.clone();
                let view_for_st = view_entity.clone();
                let view_for_mood = view_entity.clone();
                let view_for_rule = view_entity.clone();
                let view_for_signout = view_entity.clone();
                let view_for_sync = view_entity.clone();
                let view_for_check_up = view_entity.clone();
                let view_for_inst_up = view_entity.clone();
                let next_lang = self.language.toggle();
                crate::settings::settings_page(crate::settings::SettingsPageProps {
                    language: self.language,
                    palette,
                    theme: self.theme,
                    mood: self.mood,
                    stacked,
                    member_name: &self.member_name,
                    member_role: &self.member_role,
                    member_contact: &self.member_contact,
                    family_name: &self.family_name,
                    storage_sync_status: &self.storage_sync_status,
                    rule_multi_sig_over_30k: self.rule_multi_sig_over_30k,
                    rule_sunday_digest: self.rule_sunday_digest,
                    rule_auto_roundup_momo: self.rule_auto_roundup_momo,
                    update_state: &self.update_state,
                    on_toggle_language: Rc::new(move |window, cx| {
                        view_for_lang.update(cx, |this, cx| {
                            this.set_language(next_lang, window, cx);
                            cx.emit(DashboardEvent::LanguageChanged(next_lang));
                        });
                    }),
                    on_change_theme: Rc::new(move |mode, cx| {
                        view_for_th.update(cx, |this, cx| {
                            this.set_theme(mode, cx);
                            cx.emit(DashboardEvent::ThemeChanged(mode));
                        });
                    }),
                    on_change_style: Rc::new(move |style, cx| {
                        crate::theme::set_ui_style(style, cx);
                        view_for_st.update(cx, |this, cx| {
                            cx.emit(DashboardEvent::ThemeChanged(this.theme));
                            cx.notify();
                        });
                    }),
                    on_change_mood: Rc::new(move |mood, cx| {
                        view_for_mood.update(cx, |this, cx| {
                            this.set_mood(mood, cx);
                        });
                    }),
                    on_toggle_rule: Rc::new(move |rule_key, _w, cx| {
                        view_for_rule.update(cx, |this, cx| {
                            match rule_key {
                                "rule_multi_sig_over_30k" => {
                                    this.rule_multi_sig_over_30k = !this.rule_multi_sig_over_30k;
                                }
                                "rule_sunday_digest" => {
                                    this.rule_sunday_digest = !this.rule_sunday_digest;
                                }
                                "rule_auto_roundup_momo" => {
                                    this.rule_auto_roundup_momo = !this.rule_auto_roundup_momo;
                                }
                                _ => {}
                            }
                            cx.notify();
                        });
                    }),
                    on_sign_out: Rc::new(move |_w, cx| {
                        view_for_signout.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::SignOutRequested);
                        });
                    }),
                    on_sync: Rc::new(move |_w, cx| {
                        view_for_sync.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::SyncRequested);
                        });
                    }),
                    on_check_update: Rc::new(move |_w, cx| {
                        view_for_check_up.update(cx, |this, cx| {
                            this.check_for_app_update(true, cx);
                        });
                    }),
                    on_install_update: Rc::new(move |_w, cx| {
                        view_for_inst_up.update(cx, |this, cx| {
                            this.install_app_update(cx);
                        });
                    }),
                })
                .into_any_element()
            }
            AppSection::Reports => {
                let view_for_period = view_entity.clone();
                let view_for_pt = view_entity.clone();
                let view_for_csv = view_entity.clone();
                let view_for_sum = view_entity.clone();
                crate::reports::reports_page(crate::reports::ReportsPageProps {
                    language: self.language,
                    palette,
                    stacked,
                    period: self.report_period,
                    selected_point: self.selected_report_point,
                    timeline_points: &self.report_timeline_points,
                    transactions: &self.transactions,
                    monthly_income_rwf: self.monthly_income_rwf,
                    monthly_expenses_rwf: self.monthly_expenses_rwf,
                    on_change_period: Rc::new(move |period, _w, cx| {
                        view_for_period.update(cx, |this, cx| {
                            this.report_period = period;
                            cx.notify();
                        });
                    }),
                    on_select_point: Rc::new(move |idx, _w, cx| {
                        view_for_pt.update(cx, |this, cx| {
                            this.selected_report_point = idx;
                            cx.notify();
                        });
                    }),
                    on_export_csv: Rc::new(move |_fmt, _w, cx| {
                        view_for_csv.update(cx, |this, cx| {
                            this.download_report_csv(cx);
                        });
                    }),
                    on_export_summary: Rc::new(move |_fmt, _w, cx| {
                        view_for_sum.update(cx, |this, cx| {
                            this.download_report_summary(cx);
                        });
                    }),
                })
                .into_any_element()
            }
        };

        let scale = window.scale_factor().max(1.0);
        let safe_area = crate::safe_area::get_safe_area();
        let safe_bottom = px(safe_area.bottom / scale);

        let toast_stack = if !self.toast_manager.is_empty() {
            let view_dismiss = view_entity.clone();
            let visible_toasts = self.toast_manager.visible(4).collect::<Vec<_>>();
            let is_stacked = stacked;
            Some(
                div()
                    .absolute()
                    .when(is_stacked, |this| {
                        this.bottom(px(72.0) + safe_bottom)
                            .right(px(16.0))
                            .left(px(16.0))
                    })
                    .when(!is_stacked, |this| {
                        this.bottom(px(24.0)).right(px(24.0)).w(px(360.0))
                    })
                    .child(render_toast_stack(
                        "dashboard-toasts",
                        self.toast_stack_state.clone(),
                        visible_toasts,
                        &palette,
                        move |id, _window, cx| {
                            view_dismiss.update(cx, |this, cx| {
                                this.dismiss_toast(id, cx);
                            });
                        },
                    )),
            )
        } else {
            None
        };

        let active_projects_count = self.projects.iter().filter(|p| !p.is_completed).count();

        let page_container = div()
            .relative()
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
                        .when_some(update_banner, |this, banner| this.child(banner))
                        .child(screen_body),
                ),
            )
            .when_some(entry_modal, |this, modal| this.child(modal))
            .when_some(update_modal, |this, modal| this.child(modal))
            .when_some(toast_stack, |this, stack| this.child(stack));

        app_layout(
            AppLayoutProps {
                viewport_width,
                safe_bottom,
                theme: self.theme,
                mood: self.mood,
                language: self.language,
                active_section: self.active_section,
                money_sub_tab: self.money_sub_tab,
                can_go_back: !self.nav_history.is_empty()
                    || self.active_section != AppSection::Home,
                member_name: &self.member_name,
                member_contact: &self.member_contact,
                member_phone: &self.member_phone,
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
                user_dropdown_open: self.user_dropdown_open,
                on_toggle_user_dropdown: Some(toggle_user_dropdown),
                on_close_user_dropdown: Some(close_user_dropdown),
                on_sign_out: Some(Rc::new(sign_out_action.clone())),
                workspace_mode: self.workspace_mode,
                on_select_workspace_mode: Some(select_workspace_mode),
            },
            select_section,
            change_theme,
            change_lang,
            sign_out_action,
            page_container,
        )
    }
}
