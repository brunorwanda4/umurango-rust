use crate::brand::{apply_theme, colors, ThemeMode};
use crate::components::dashboard::{project_card, transaction_row, PROJECTS, TRANSACTIONS};
use crate::components::scroll::touch_scrollable;
use crate::components::ui::{
    badge, card_header, card_surface, dismissible_alert, icon_action_button, input_container,
    member_avatar, search_input, secondary_button, stat_card, vertical_divider, AlertVariant,
    AvatarSize, BadgeVariant,
};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    input::InputState,
};
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
    member_name: String,
    search_state: Entity<InputState>,
    activity_filter: ActivityFilter,
    show_approval_alert: bool,
    pending_approval_count: usize,
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

        Self {
            theme,
            language,
            member_name: "Bruno Mugabo".to_string(),
            search_state,
            activity_filter: ActivityFilter::All,
            show_approval_alert: true,
            pending_approval_count: 1,
        }
    }

    pub fn set_member_name(&mut self, name: String, cx: &mut Context<Self>) {
        if !name.trim().is_empty() {
            self.member_name = name;
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = colors(self.theme);
        let view_entity = cx.entity().clone();
        let lang = self.language;

        let toggle_theme = {
            let view = view_entity.clone();
            let current_theme = self.theme;
            move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                let next_theme = match current_theme {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
                view.update(cx, |this, cx| {
                    this.set_theme(next_theme, cx);
                    cx.emit(DashboardEvent::ThemeChanged(next_theme));
                });
            }
        };

        let toggle_lang = {
            let view = view_entity.clone();
            let current_lang = self.language;
            move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                let next_lang = match current_lang {
                    Language::English => Language::Kinyarwanda,
                    Language::Kinyarwanda => Language::English,
                };
                view.update(cx, |this, cx| {
                    this.set_language(next_lang, window, cx);
                    cx.emit(DashboardEvent::LanguageChanged(next_lang));
                });
            }
        };

        let sign_out = {
            let view = view_entity.clone();
            move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                view.update(cx, |_this, cx| {
                    cx.emit(DashboardEvent::SignOutRequested);
                });
            }
        };

        // Top Navigation Header
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .px_6()
            .py_3()
            .bg(palette.surface)
            .border_b_1()
            .border_color(palette.border)
            .child(
                // Left Brand and Family Info
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
                            .rounded_md()
                            .bg(palette.primary)
                            .child(
                                icons::icon(icons::ICON_SHIELD_HALVED)
                                    .size(px(16.))
                                    .text_color(palette.text_primary),
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
            .child(
                // Center Search Bar
                div().flex_1().max_w(px(360.)).mx_4().child(input_container(
                    palette,
                    search_input(&self.search_state, palette),
                )),
            )
            .child(
                // Right Controls (Theme, Language, User, Sign Out)
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("theme-toggle-btn")
                            .icon(icons::theme_button_icon(self.theme))
                            .tooltip(if self.theme == ThemeMode::Dark {
                                "Switch to light mode"
                            } else {
                                "Switch to dark mode"
                            })
                            .text()
                            .compact()
                            .on_click(toggle_theme),
                    )
                    .child(
                        Button::new("lang-toggle-btn")
                            .label(if self.language == Language::English {
                                "RW"
                            } else {
                                "EN"
                            })
                            .tooltip(if self.language == Language::English {
                                "Kinyarwanda"
                            } else {
                                "English"
                            })
                            .text()
                            .compact()
                            .on_click(toggle_lang),
                    )
                    .child(vertical_divider(palette))
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

        // Optional Pending Approval Alert Banner
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

        // Stat Cards Row
        let stat_cards_row = div()
            .flex()
            .gap_4()
            .px_6()
            .pt_4()
            .child(div().flex_1().child(stat_card(
                "stat-treasury",
                palette,
                text(lang, "dashboard.total_treasury"),
                "3,450,000 RWF",
                Some(text(lang, "dashboard.treasury_trend")),
                Some(icons::ICON_WALLET),
                Some(("+14.2%".into(), true)),
            )))
            .child(div().flex_1().child(stat_card(
                "stat-contributions",
                palette,
                text(lang, "dashboard.monthly_contributions"),
                "780,000 RWF",
                Some(text(lang, "dashboard.contributions_sub")),
                Some(icons::ICON_CREDIT_CARD),
                Some(("8/10 paid".into(), true)),
            )))
            .child(div().flex_1().child(stat_card(
                "stat-expenses",
                palette,
                text(lang, "dashboard.monthly_expenses"),
                "240,000 RWF",
                Some(text(lang, "dashboard.expenses_sub")),
                Some(icons::ICON_ARROW_DOWN),
                Some(("-8%".into(), true)),
            )))
            .child(div().flex_1().child(stat_card(
                "stat-reserve",
                palette,
                text(lang, "dashboard.emergency_reserve"),
                "1,500,000 RWF",
                Some(text(lang, "dashboard.reserve_sub")),
                Some(icons::ICON_SHIELD_HALVED),
                Some(("100% funded".into(), true)),
            )));

        // Quick Actions Row
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
            .px_6()
            .pt_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        Button::new("record-contrib-btn")
                            .primary()
                            .label(text(lang, "dashboard.record_contribution"))
                            .icon(icons::button_icon(icons::ICON_PLUS))
                            .on_click(record_contrib),
                    )
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

        // Filter tabs for Recent Activity
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
                    .rounded_md()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(current_filter == ActivityFilter::All, |this| {
                        this.bg(palette.primary).text_color(palette.text_primary)
                    })
                    .when(current_filter != ActivityFilter::All, |this| {
                        this.bg(palette.card)
                            .text_color(palette.text_muted)
                            .hover(|s| s.bg(palette.border))
                    })
                    .child("All")
                    .on_click(set_filter(ActivityFilter::All)),
            )
            .child(
                div()
                    .id("filter-tab-contrib")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(current_filter == ActivityFilter::Contributions, |this| {
                        this.bg(palette.primary).text_color(palette.text_primary)
                    })
                    .when(current_filter != ActivityFilter::Contributions, |this| {
                        this.bg(palette.card)
                            .text_color(palette.text_muted)
                            .hover(|s| s.bg(palette.border))
                    })
                    .child("Contributions")
                    .on_click(set_filter(ActivityFilter::Contributions)),
            )
            .child(
                div()
                    .id("filter-tab-expenses")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(current_filter == ActivityFilter::Expenses, |this| {
                        this.bg(palette.primary).text_color(palette.text_primary)
                    })
                    .when(current_filter != ActivityFilter::Expenses, |this| {
                        this.bg(palette.card)
                            .text_color(palette.text_muted)
                            .hover(|s| s.bg(palette.border))
                    })
                    .child("Expenses")
                    .on_click(set_filter(ActivityFilter::Expenses)),
            );

        let filtered_txs = TRANSACTIONS
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

        let projects_section = div().flex().flex_col().gap_3().children(
            PROJECTS
                .iter()
                .map(|project| project_card(project, palette)),
        );

        // Sidebar: Pending Approvals Card
        let approve_action = {
            let view = view_entity.clone();
            move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.pending_approval_count = this.pending_approval_count.saturating_sub(1);
                    cx.notify();
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
                    .rounded_lg()
                    .bg(palette.card)
                    .border_1()
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
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(palette.text_primary)
                                    .child("Medical Emergency Assistance"),
                            )
                            .child(badge("85,000 RWF", BadgeVariant::Warning, palette)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_muted)
                            .child("Submitted by Eric Mugabo • Needs 1 admin confirmation"),
                    )
                    .child(
                        div().flex().items_center().gap_2().mt_1().child(
                            Button::new("approve-tx-btn")
                                .primary()
                                .label(text(lang, "dashboard.review_approve"))
                                .compact()
                                .on_click(approve_action),
                        ),
                    )
            } else {
                div()
                    .p_3()
                    .rounded_lg()
                    .bg(palette.card)
                    .text_xs()
                    .text_color(palette.text_muted)
                    .child("No pending approvals. All family requests are up to date.")
            });

        // Sidebar: Family Household Members Card
        let members_card = card_surface(palette)
            .child(card_header(
                text(lang, "dashboard.household_members"),
                Some("5 active members in this workspace"),
                palette,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(member_avatar(
                        "Bruno Mugabo",
                        Some("Family Head / Admin"),
                        AvatarSize::Md,
                        palette,
                    ))
                    .child(member_avatar(
                        "Claudine Mugabo",
                        Some("Treasurer"),
                        AvatarSize::Md,
                        palette,
                    ))
                    .child(member_avatar(
                        "Eric Mugabo",
                        Some("Member"),
                        AvatarSize::Md,
                        palette,
                    ))
                    .child(member_avatar(
                        "Alice Keza",
                        Some("Member"),
                        AvatarSize::Md,
                        palette,
                    ))
                    .child(member_avatar(
                        "Patrick Mugabo",
                        Some("Youth Member"),
                        AvatarSize::Md,
                        palette,
                    )),
            );

        // Sidebar: Local Storage Health Card
        let storage_card = card_surface(palette)
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
                                    .child(text(lang, "dashboard.storage_health_desc")),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_muted)
                            .child("Authoritative database: PostgreSQL • Local store: SQLite"),
                    ),
            );

        // Main Two-Column Layout
        let main_grid = div()
            .flex()
            .gap_6()
            .px_6()
            .py_4()
            .child(
                // Left Column (65%)
                div()
                    .flex_1()
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
                            .child(projects_section),
                    ),
            )
            .child(
                // Right Column (35%)
                div()
                    .w(px(320.))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(pending_approval_card)
                    .child(members_card)
                    .child(storage_card),
            );

        // Outer Page Container with TouchScrollable
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(palette.bg)
            .text_color(palette.text_primary)
            .child(header)
            .child(
                div().flex_1().child(
                    touch_scrollable("dashboard-scroll-body", px(1400.))
                        .when_some(alert_section, |this, alert| this.child(alert))
                        .child(stat_cards_row)
                        .child(quick_actions_row)
                        .child(main_grid),
                ),
            )
    }
}
