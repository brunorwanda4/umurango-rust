use crate::brand::{apply_theme, colors, ThemeMode};
use crate::components::dashboard::{
    project_card, transaction_row, ProjectSummary, TransactionRow, PROJECTS, TRANSACTIONS,
};
use crate::components::layouts::responsive::app_content_width;
use crate::components::layouts::sections::{
    members_section, money_section, profile_section, projects_section,
};
use crate::components::layouts::{app_layout, AppLayoutProps, AppSection};
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{
    badge, card_header, card_surface, dismissible_alert, icon_action_button, member_avatar,
    primary_button, search_input, secondary_button, stat_card, stateful_input_container,
    vertical_divider, AlertVariant, AvatarSize, BadgeVariant,
};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::component::input::InputState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActivityFilter {
    #[default]
    All,
    Contributions,
    Expenses,
}

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
    language: Language,
    active_section: AppSection,
    member_name: String,
    pub family_name: String,
    search_state: Entity<InputState>,
    activity_filter: ActivityFilter,
    show_approval_alert: bool,
    pending_approval_count: usize,
    pending_approval_tx_id: Option<String>,
    pending_approval_tx_title: Option<String>,
    pending_approval_tx_amount: Option<String>,
    pub total_treasury: String,
    pub monthly_contributions: String,
    pub monthly_expenses: String,
    pub emergency_reserve: String,
    pub transactions: Vec<TransactionRow>,
    pub projects: Vec<ProjectSummary>,
    pub members: Vec<(String, String, BadgeVariant)>,
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
        let search_state = cx.new(|cx| {
            InputState::new(window, cx).placeholder(text(language, "dashboard.search_placeholder"))
        });

        let default_members = vec![
            (
                "Bruno Mugabo".to_string(),
                "Family Head / Admin".to_string(),
                BadgeVariant::Primary,
            ),
            (
                "Claudine Mugabo".to_string(),
                "Treasurer".to_string(),
                BadgeVariant::Success,
            ),
            (
                "Eric Mugabo".to_string(),
                "Member".to_string(),
                BadgeVariant::Neutral,
            ),
            (
                "Alice Keza".to_string(),
                "Member".to_string(),
                BadgeVariant::Neutral,
            ),
            (
                "Patrick Mugabo".to_string(),
                "Youth Member".to_string(),
                BadgeVariant::Info,
            ),
        ];

        Self {
            theme,
            language,
            active_section: AppSection::Home,
            member_name: "Bruno Mugabo".to_string(),
            family_name: "Umuryango Workspace".to_string(),
            search_state,
            activity_filter: ActivityFilter::All,
            show_approval_alert: true,
            pending_approval_count: 1,
            pending_approval_tx_id: None,
            pending_approval_tx_title: Some("Medical Emergency Assistance".to_string()),
            pending_approval_tx_amount: Some("85,000 RWF".to_string()),
            total_treasury: "3,450,000 RWF".to_string(),
            monthly_contributions: "780,000 RWF".to_string(),
            monthly_expenses: "240,000 RWF".to_string(),
            emergency_reserve: "1,500,000 RWF".to_string(),
            transactions: TRANSACTIONS.to_vec(),
            projects: PROJECTS.to_vec(),
            members: default_members,
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
            self.active_section = section;
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: ThemeMode, cx: &mut Context<Self>) {
        self.theme = theme;
        apply_theme(theme, cx);
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
                let pending = txs.iter().find(|t| t.status == "PENDING");
                if let Some(p) = pending {
                    self.pending_approval_count =
                        txs.iter().filter(|t| t.status == "PENDING").count();
                    self.pending_approval_tx_id = Some(p.id.clone());
                    self.pending_approval_tx_title = Some(p.title.clone());
                    self.pending_approval_tx_amount = Some(umuryango_shared::format_money(
                        p.amount_minor_units,
                        &p.currency,
                    ));
                    self.show_approval_alert = true;
                } else {
                    self.pending_approval_count = 0;
                    self.pending_approval_tx_id = None;
                    self.pending_approval_tx_title = None;
                    self.pending_approval_tx_amount = None;
                    self.show_approval_alert = false;
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
}

impl Render for DashboardView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = colors(self.theme);
        let view_entity = cx.entity().clone();
        let lang = self.language;
        let viewport_size = window.viewport_size();
        let viewport_width = viewport_size.width;
        let content_width = app_content_width(viewport_width);

        let select_section = {
            let view = view_entity.clone();
            move |section: AppSection, _window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_active_section(section, cx);
                });
            }
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

        let content: AnyElement = match self.active_section {
            AppSection::Home => {
                let sign_out = {
                    let view = view_entity.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::SignOutRequested);
                        });
                    }
                };

                let header =
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px_6()
                        .py_3()
                        .bg(palette.bg_card)
                        .border_b(palette.border_width)
                        .border_color(palette.border_color)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(32.))
                                        .h(px(32.))
                                        .rounded(palette.radius_checkbox)
                                        .bg(palette.accent_primary)
                                        .border(px(2.0))
                                        .border_color(palette.border_color)
                                        .child(
                                            icons::icon(icons::ICON_PEOPLE_ROOF)
                                                .size(px(15.))
                                                .text_color(palette.text_on_accent),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_primary)
                                                .child(text(lang, "dashboard.workspace_title")),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(palette.text_muted)
                                                .child("Umuryango Governance"),
                                        ),
                                )
                                .child(vertical_divider(palette))
                                .child(badge(
                                    text(lang, "dashboard.workspace_badge"),
                                    BadgeVariant::Primary,
                                    palette,
                                ))
                                .child(badge(
                                    text(lang, "dashboard.sync_status_synced"),
                                    BadgeVariant::Success,
                                    palette,
                                )),
                        )
                        .child(div().flex_1().max_w(px(360.)).mx_4().child(
                            stateful_input_container(
                                palette,
                                &self.search_state,
                                search_input(&self.search_state, palette),
                            ),
                        ))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(member_avatar(
                                    &self.member_name,
                                    Some("Admin"),
                                    AvatarSize::Sm,
                                    palette,
                                ))
                                .child(icon_action_button(
                                    "sign-out-btn",
                                    icons::ICON_RIGHT_FROM_BRACKET,
                                    text(lang, "dashboard.sign_out"),
                                    palette,
                                    sign_out,
                                )),
                        );

                let alert_section = if self.show_approval_alert && self.pending_approval_count > 0 {
                    let view = view_entity.clone();
                    Some(div().px_6().pt_4().child(dismissible_alert(
                        "pending-approval-alert",
                        AlertVariant::Warning,
                        format!(
                            "{}: {}",
                            text(lang, "dashboard.pending_approvals"),
                            text(lang, "dashboard.pending_approval_desc")
                        ),
                        palette,
                        move |_event, _window, cx| {
                            view.update(cx, |this, cx| {
                                this.show_approval_alert = false;
                                cx.notify();
                            });
                        },
                    )))
                } else {
                    None
                };

                let stack_stat_cards = content_width < px(600.0);
                let stat_cards_row = div()
                    .flex()
                    .gap_4()
                    .px_6()
                    .pt_4()
                    .when(stack_stat_cards, |this| this.flex_col())
                    .child(div().flex_1().child(stat_card(
                        "stat-treasury",
                        palette,
                        text(lang, "dashboard.total_treasury"),
                        self.total_treasury.clone(),
                        Some(text(lang, "dashboard.treasury_trend")),
                        Some(icons::ICON_WALLET),
                        Some(("+14.2%".into(), true)),
                    )))
                    .child(div().flex_1().child(stat_card(
                        "stat-contributions",
                        palette,
                        text(lang, "dashboard.monthly_contributions"),
                        self.monthly_contributions.clone(),
                        Some(text(lang, "dashboard.contributions_sub")),
                        Some(icons::ICON_CREDIT_CARD),
                        Some(("8/10 paid".into(), true)),
                    )))
                    .child(div().flex_1().child(stat_card(
                        "stat-expenses",
                        palette,
                        text(lang, "dashboard.monthly_expenses"),
                        self.monthly_expenses.clone(),
                        Some(text(lang, "dashboard.expenses_sub")),
                        Some(icons::ICON_ARROW_DOWN),
                        Some(("-8%".into(), true)),
                    )))
                    .child(div().flex_1().child(stat_card(
                        "stat-reserve",
                        palette,
                        text(lang, "dashboard.emergency_reserve"),
                        self.emergency_reserve.clone(),
                        Some(text(lang, "dashboard.reserve_sub")),
                        Some(icons::ICON_SHIELD_HALVED),
                        Some(("100% funded".into(), true)),
                    )));

                let record_contrib = {
                    let view = view_entity.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::RecordContributionRequested);
                        });
                    }
                };

                let add_expense = {
                    let view = view_entity.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::AddExpenseRequested);
                        });
                    }
                };

                let new_project = {
                    let view = view_entity.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::NewProjectRequested);
                        });
                    }
                };

                let sync_now = {
                    let view = view_entity.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |_this, cx| {
                            cx.emit(DashboardEvent::SyncRequested);
                        });
                    }
                };

                let quick_actions_row = div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .flex_wrap()
                    .gap_3()
                    .px_6()
                    .pt_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .flex_wrap()
                            .gap_3()
                            .child(div().child(primary_button(
                                "record-contrib-btn",
                                text(lang, "dashboard.record_contribution"),
                                palette,
                                record_contrib,
                            )))
                            .child(secondary_button(
                                "add-expense-btn",
                                text(lang, "dashboard.add_expense"),
                                palette,
                                add_expense,
                            ))
                            .child(secondary_button(
                                "new-project-btn",
                                text(lang, "dashboard.new_project"),
                                palette,
                                new_project,
                            )),
                    )
                    .child(div().flex().items_center().gap_2().child(secondary_button(
                        "sync-now-btn",
                        text(lang, "dashboard.sync_now"),
                        palette,
                        sync_now,
                    )));

                let current_filter = self.activity_filter;
                let set_filter = |filter: ActivityFilter| {
                    let view = view_entity.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |this, cx| {
                            this.activity_filter = filter;
                            cx.notify();
                        });
                    }
                };

                let filter_tabs = div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .mb_3()
                    .child(
                        div()
                            .id("filter-tab-all")
                            .px_3()
                            .py_1()
                            .rounded(palette.radius_checkbox)
                            .border(px(2.0))
                            .border_color(palette.border_color)
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .cursor_pointer()
                            .when(current_filter == ActivityFilter::All, |this| {
                                this.bg(palette.accent_primary)
                                    .text_color(palette.text_on_accent)
                            })
                            .when(current_filter != ActivityFilter::All, |this| {
                                this.bg(palette.bg_input)
                                    .text_color(palette.text_secondary)
                                    .hover(|s| s.bg(palette.bg_input_focus))
                            })
                            .child("All")
                            .on_click(set_filter(ActivityFilter::All)),
                    )
                    .child(
                        div()
                            .id("filter-tab-contrib")
                            .px_3()
                            .py_1()
                            .rounded(palette.radius_checkbox)
                            .border(px(2.0))
                            .border_color(palette.border_color)
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .cursor_pointer()
                            .when(current_filter == ActivityFilter::Contributions, |this| {
                                this.bg(palette.accent_primary)
                                    .text_color(palette.text_on_accent)
                            })
                            .when(current_filter != ActivityFilter::Contributions, |this| {
                                this.bg(palette.bg_input)
                                    .text_color(palette.text_secondary)
                                    .hover(|s| s.bg(palette.bg_input_focus))
                            })
                            .child("Contributions")
                            .on_click(set_filter(ActivityFilter::Contributions)),
                    )
                    .child(
                        div()
                            .id("filter-tab-expenses")
                            .px_3()
                            .py_1()
                            .rounded(palette.radius_checkbox)
                            .border(px(2.0))
                            .border_color(palette.border_color)
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .cursor_pointer()
                            .when(current_filter == ActivityFilter::Expenses, |this| {
                                this.bg(palette.accent_primary)
                                    .text_color(palette.text_on_accent)
                            })
                            .when(current_filter != ActivityFilter::Expenses, |this| {
                                this.bg(palette.bg_input)
                                    .text_color(palette.text_secondary)
                                    .hover(|s| s.bg(palette.bg_input_focus))
                            })
                            .child("Expenses")
                            .on_click(set_filter(ActivityFilter::Expenses)),
                    );

                let filtered_txs =
                    self.transactions
                        .iter()
                        .filter(|transaction| match current_filter {
                            ActivityFilter::All => true,
                            ActivityFilter::Contributions => transaction.is_income,
                            ActivityFilter::Expenses => !transaction.is_income,
                        });

                let tx_list = div().flex().flex_col().gap_2().children(
                    filtered_txs
                        .enumerate()
                        .map(|(idx, transaction)| transaction_row(idx, transaction, palette)),
                );

                let projects_section_el = div().flex().flex_col().gap_3().children(
                    self.projects
                        .iter()
                        .map(|project| project_card(project, palette)),
                );

                let approve_action = {
                    let view = view_entity.clone();
                    let tx_id = self.pending_approval_tx_id.clone();
                    move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                        view.update(cx, |this, cx| {
                            if let Some(id) = tx_id.clone() {
                                if let Some(fid) = crate::auth::client::get_current_family_id() {
                                    let _ = crate::auth::client::api_approve_transaction(&fid, &id);
                                }
                                this.refresh_from_backend(cx);
                            } else {
                                this.pending_approval_count =
                                    this.pending_approval_count.saturating_sub(1);
                                cx.notify();
                            }
                        });
                    }
                };

                let pending_approval_card = card_surface(palette)
                    .child(card_header(
                        text(lang, "dashboard.pending_approvals"),
                        Some("Actions requiring family admin decision"),
                        palette,
                    ))
                    .child(if self.pending_approval_count > 0 {
                        div()
                            .p_3()
                            .rounded(palette.radius_input)
                            .bg(palette.bg_input)
                            .border(palette.border_width_input)
                            .border_color(palette.accent_gold)
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(palette.text_primary)
                                            .child(
                                                self.pending_approval_tx_title
                                                    .clone()
                                                    .unwrap_or_else(|| {
                                                        "Medical Emergency Assistance".to_string()
                                                    }),
                                            ),
                                    )
                                    .child(badge(
                                        self.pending_approval_tx_amount
                                            .clone()
                                            .unwrap_or_else(|| "85,000 RWF".to_string()),
                                        BadgeVariant::Warning,
                                        palette,
                                    )),
                            )
                            .child(
                                div().text_xs().text_color(palette.text_muted).child(
                                    "Submitted by Family Member • Needs 1 admin confirmation",
                                ),
                            )
                            .child(div().flex().items_center().gap_2().mt_1().child(
                                primary_button(
                                    "approve-tx-btn",
                                    text(lang, "dashboard.review_approve"),
                                    palette,
                                    approve_action,
                                ),
                            ))
                    } else {
                        div()
                            .p_3()
                            .rounded_lg()
                            .bg(palette.card)
                            .text_xs()
                            .text_color(palette.text_muted)
                            .child("No pending approvals. All family requests are up to date.")
                    });

                let members_card = card_surface(palette)
                    .child(card_header(
                        text(lang, "dashboard.household_members"),
                        Some(format!(
                            "{} active members in this workspace",
                            self.members.len()
                        )),
                        palette,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .children(self.members.iter().map(|(name, role, _variant)| {
                                member_avatar(name, Some(role.as_str()), AvatarSize::Md, palette)
                            })),
                    );

                let storage_card =
                    card_surface(palette)
                        .child(card_header(
                            text(lang, "dashboard.storage_health"),
                            Some("Offline replica & sync state"),
                            palette,
                        ))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            icons::icon(icons::ICON_CIRCLE_CHECK)
                                                .size(px(14.))
                                                .text_color(palette.success),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(palette.text_primary)
                                                .child(self.storage_sync_status.clone()),
                                        ),
                                )
                                .child(div().text_xs().text_color(palette.text_muted).child(
                                    "Authoritative database: PostgreSQL • Local store: SQLite",
                                )),
                        );

                let stack_columns = content_width < px(900.0);
                let main_grid = div()
                    .flex()
                    .gap_6()
                    .px_6()
                    .py_4()
                    .when(stack_columns, |this| this.flex_col())
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                card_surface(palette)
                                    .child(card_header(
                                        text(lang, "dashboard.recent_activity"),
                                        Some("Latest treasury transactions and contributions"),
                                        palette,
                                    ))
                                    .child(filter_tabs)
                                    .child(tx_list),
                            )
                            .child(
                                card_surface(palette)
                                    .child(card_header(
                                        text(lang, "dashboard.active_projects"),
                                        Some("Communal family goals and fundraising progress"),
                                        palette,
                                    ))
                                    .child(projects_section_el),
                            ),
                    )
                    .child(
                        div()
                            .when(stack_columns, |this| this.w_full())
                            .when(!stack_columns, |this| this.w(px(320.)))
                            .flex_none()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(pending_approval_card)
                            .child(members_card)
                            .child(storage_card),
                    );

                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .bg(palette.bg)
                    .text_color(palette.text_primary)
                    .child(header)
                    .child(
                        div().flex_1().min_h_0().overflow_hidden().child(
                            touch_scrollable_fill("dashboard-scroll-body")
                                .when_some(alert_section, |this, alert| this.child(alert))
                                .child(stat_cards_row)
                                .child(quick_actions_row)
                                .child(main_grid),
                        ),
                    )
                    .into_any_element()
            }
            AppSection::Money => {
                money_section(viewport_width, self.theme, self.language).into_any_element()
            }
            AppSection::Projects => projects_section(self.theme, self.language).into_any_element(),
            AppSection::Members => members_section(self.theme, self.language).into_any_element(),
            AppSection::Profile => profile_section(
                self.theme,
                self.language,
                &self.member_name,
                change_theme.clone(),
                change_lang.clone(),
                sign_out_action.clone(),
            )
            .into_any_element(),
        };

        app_layout(
            AppLayoutProps {
                viewport_width,
                theme: self.theme,
                language: self.language,
                active_section: self.active_section,
                member_name: &self.member_name,
            },
            select_section,
            change_theme,
            change_lang,
            sign_out_action,
            content,
        )
    }
}
