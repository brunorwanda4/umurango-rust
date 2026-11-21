use super::nav::{AppSection, SectionSelectHandler};
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

    let (title, badge_label) = match active_section {
        AppSection::Home => (
            text(language, active_section.label_key()),
            text(language, "cozy.nav_home_badge"),
        ),
        AppSection::Money => (
            money_sub_tab.nav_title_for(language),
            text(language, "cozy.nav_money_badge"),
        ),
        AppSection::Projects => (
            text(language, active_section.label_key()),
            format!(
                "{} {}",
                props.projects_badge_count,
                text(language, "cozy.nav_projects_badge_suffix")
            ),
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
            // Bell Notification Button with Pending Approval Badge
            div()
                .id("navbar-bell-approvals-btn")
                .relative()
                .w(px(36.0))
                .h(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(13.0))
                .bg(palette.bg_badge)
                .border(px(2.0))
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
                .cursor_pointer()
                .hover(|s| s.top(px(-1.0)).border_color(palette.accent_primary))
                .on_click(move |_event, window, cx| {
                    if let Some(ref sub_cb) = money_tab_cb {
                        sub_cb(MoneySubTab::Overview, window, cx);
                    } else {
                        select_money(AppSection::Money, window, cx);
                    }
                })
                .child(
                    icons::icon(icons::ICON_BELL)
                        .size(px(14.0))
                        .text_color(palette.text_primary),
                )
                .when(pending_count > 0, |this| {
                    this.child(
                        div()
                            .px_1()
                            .rounded_full()
                            .bg(palette.expense_text)
                            .border(px(1.5))
                            .border_color(palette.bg_nav)
                            .text_size(px(9.0))
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(rgb(0x1E1920))
                            .child(pending_count.to_string()),
                    )
                }),
        )
}
