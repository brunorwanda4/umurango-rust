pub(super) use crate::brand::{apply_theme, CozyMood};
use crate::brand::{colors, ThemeColors, ThemeMode};
pub(super) use crate::components::dashboard::*;
pub(super) use crate::components::layouts::responsive::app_content_width;
pub(super) use crate::components::layouts::{app_layout, AppLayoutProps, AppSection};
pub(super) use crate::components::scroll::touch_scrollable_fill;
pub(super) use crate::components::ui::*;
pub(super) use crate::i18n::{text, Language};
pub(super) use crate::icons;
use crate::theme::{active_mood, set_cozy_mood};
pub(super) use crate::updater::{UpdateApplyOutcome, UpdateState};
pub(super) use gpui_kit::component::input::{InputState, TextareaState};
pub(super) use gpui_kit::component::select::{SelectEvent, SelectState};
pub(super) use gpui_kit::component::IndexPath;

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
mod home;
mod members;
mod money;
mod profile;
mod projects;

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
    pub rule_multi_sig_over_30k: bool,
    pub rule_sunday_digest: bool,
    pub rule_auto_roundup_momo: bool,
    pub toast_notice: Option<String>,
    pub(super) toast_manager: ToastManager<u64, ToastItem>,
    pub(super) toast_stack_state: ToastStackState,
    pub(super) next_toast_id: u64,
    pub(super) toast_clock_running: bool,
    pub storage_sync_status: String,
    pub update_state: UpdateState,
    pub show_update_modal: bool,
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
                Some(IndexPath::default()),
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
                Some(IndexPath::default()),
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
            },
            select_section,
            change_theme,
            change_lang,
            sign_out_action,
            page_container,
        )
    }
}
