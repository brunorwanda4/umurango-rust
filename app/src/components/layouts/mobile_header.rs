use super::nav::{AppSection, SectionSelectHandler, WorkspaceMode};
use super::AppLayoutProps;
use crate::brand::ThemeColors;
use crate::components::dashboard::MoneySubTab;
use crate::components::ui::ghost_icon_button;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn app_mobile_header(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select_section: SectionSelectHandler,
) -> impl IntoElement {
    let language = props.language;
    let active_section = props.active_section;
    let money_sub_tab = props.money_sub_tab;
    let can_go_back = props.can_go_back || active_section != AppSection::Home;
    let back_handler = props.on_back.clone();
    let select_home = on_select_section.clone();
    let select_money = on_select_section;
    let money_tab_cb = props.on_select_money_tab.clone();
    let on_toggle_workspace = props.on_select_workspace_mode.clone();
    let current_mode = props.workspace_mode;

    let (title, badge_label) = match active_section {
        AppSection::Home => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_home_badge"),
        ),
        AppSection::Money => (
            money_sub_tab.nav_title_for(language),
            text(language, "cozy.nav_money_badge"),
        ),
        AppSection::Campaigns => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_campaigns_badge"),
        ),
        AppSection::Projects => (
            text(language, active_section.label_key()),
            format!(
                "{} {}",
                props.projects_badge_count,
                text(language, "cozy.nav_projects_badge_suffix")
            ),
        ),
        AppSection::Events => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_events_badge"),
        ),
        AppSection::Vault => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_vault_badge"),
        ),
        AppSection::Reports => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_reports_badge"),
        ),
        AppSection::Members => (
            text(language, active_section.label_key()),
            format!(
                "{} {}",
                props.members_badge_count,
                text(language, "cozy.nav_members_badge_suffix")
            ),
        ),
        AppSection::Profile => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_profile_badge"),
        ),
    };

    let pending_count = props.pending_approvals_count;

    div()
        .id("app-mobile-header")
        .w_full()
        .flex_none()
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .px_5()
        .py_3()
        .bg(palette.bg_nav)
        .border_b(palette.border_width)
        .border_color(palette.border_color)
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .min_w_0()
                .child(ghost_icon_button(
                    "navbar-ghost-back-btn",
                    icons::ICON_CHEVRON_LEFT,
                    can_go_back,
                    palette,
                    move |_event, window, cx| {
                        if let Some(ref back) = back_handler {
                            back(window, cx);
                        } else {
                            select_home(AppSection::Home, window, cx);
                        }
                    },
                ))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .min_w_0()
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(title),
                        )
                        .child(
                            div()
                                .px_2()
                                .py(px(1.0))
                                .rounded_full()
                                .bg(palette.bg_badge)
                                .border(px(1.5))
                                .border_color(palette.border_color)
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.accent_primary)
                                .child(badge_label),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .when_some(on_toggle_workspace, |this, toggle| {
                    let next_mode = match current_mode {
                        WorkspaceMode::Family => WorkspaceMode::Personal,
                        WorkspaceMode::Personal => WorkspaceMode::Family,
                    };
                    this.child(
                        div()
                            .id("mobile-workspace-toggle-btn")
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .px_2()
                            .py(px(3.0))
                            .rounded_full()
                            .bg(palette.bg_elevated)
                            .border(px(1.5))
                            .border_color(palette.border_color)
                            .cursor_pointer()
                            .hover(|s| s.bg(palette.accent_primary.opacity(0.12)))
                            .child(
                                icons::icon(current_mode.icon())
                                    .size(px(12.0))
                                    .text_color(palette.accent_primary),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_primary)
                                    .child(text(language, current_mode.label_key())),
                            )
                            .on_click(move |_e, window, cx| {
                                toggle(next_mode, window, cx);
                            }),
                    )
                })
                .child(
                    div()
                        .id("mobile-bell-btn")
                        .relative()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .bg(palette.bg_elevated)
                        .border(px(1.5))
                        .border_color(palette.border_color)
                        .cursor_pointer()
                        .hover(|s| s.bg(palette.accent_primary.opacity(0.12)))
                        .child(
                            icons::icon(icons::ICON_BELL)
                                .size(px(14.0))
                                .text_color(palette.text_secondary),
                        )
                        .when(pending_count > 0, |this| {
                            this.child(
                                div()
                                    .absolute()
                                    .top(px(-2.0))
                                    .right(px(-2.0))
                                    .min_w(px(16.0))
                                    .h(px(16.0))
                                    .px(px(3.0))
                                    .rounded_full()
                                    .bg(palette.loan_text)
                                    .border(px(1.5))
                                    .border_color(palette.border_color)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_size(px(9.0))
                                            .font_weight(FontWeight::EXTRA_BOLD)
                                            .text_color(palette.text_on_accent)
                                            .child(pending_count.to_string()),
                                    ),
                            )
                        })
                        .on_click(move |_e, window, cx| {
                            select_money(AppSection::Money, window, cx);
                            if let Some(ref cb) = money_tab_cb {
                                cb(MoneySubTab::Overview, window, cx);
                            }
                        }),
                ),
        )
}
