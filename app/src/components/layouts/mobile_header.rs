use super::controls::language_toggle_button;
use super::nav::{AppSection, SectionSelectHandler};
use super::AppLayoutProps;
use crate::brand::{ThemeColors, ThemeMode};
use crate::components::dashboard::MoneySubTab;
use crate::components::ui::{ghost_icon_button, pill_button};
use crate::i18n::{text, Language};
use crate::icons;
use crate::theme::{set_cozy_mood, CozyMood};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn app_mobile_header<T, L>(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select_section: SectionSelectHandler,
    on_theme_change: T,
    on_language_change: L,
) -> impl IntoElement
where
    T: Fn(ThemeMode, &mut App) + 'static,
    L: Fn(Language, &mut Window, &mut App) + 'static,
{
    let theme = props.theme;
    let active_mood = props.mood;
    let language = props.language;
    let active_section = props.active_section;
    let money_sub_tab = props.money_sub_tab;
    let can_go_back = props.can_go_back || active_section != AppSection::Home;
    let back_handler = props.on_back.clone();
    let select_home = on_select_section.clone();
    let select_money = on_select_section;
    let money_tab_cb = props.on_select_money_tab.clone();
    let mood_cb = props.on_mood_change.clone();

    let (title, badge_label, subtitle) = match active_section {
        AppSection::Home => (
            text(language, active_section.label_key()).to_string(),
            "Umuryango".to_string(),
            "Cozy Family Treasury & Shared Harmony".to_string(),
        ),
        AppSection::Money => (
            money_sub_tab.nav_title().to_string(),
            "Ledger".to_string(),
            money_sub_tab.nav_subtitle().to_string(),
        ),
        AppSection::Projects => (
            text(language, active_section.label_key()).to_string(),
            format!("{} Goals", props.projects_badge_count),
            "Planned family milestones & savings progress".to_string(),
        ),
        AppSection::Members => (
            text(language, active_section.label_key()).to_string(),
            format!("{} Members", props.members_badge_count),
            "Family circle pledges, roles & monthly harmony".to_string(),
        ),
        AppSection::Profile => (
            text(language, active_section.label_key()).to_string(),
            "Co-Admin".to_string(),
            "Preferences, multi-sig governance & connected channels".to_string(),
        ),
    };

    let next_theme = theme.toggle();
    let theme_pill_label = match theme {
        ThemeMode::Dark => "🌙 Dark Cozy",
        ThemeMode::Light => "☀️ Light Cozy",
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
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
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
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(subtitle),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .flex_shrink_0()
                .child(pill_button(
                    "navbar-theme-pill-btn",
                    theme_pill_label,
                    palette,
                    move |_event, _window, cx| on_theme_change(next_theme, cx),
                ))
                .child(
                    // 3 Pastel Mood Dots (Matcha, Peach, Berry)
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .px_2p5()
                        .py_1p5()
                        .rounded(px(15.0))
                        .bg(palette.bg_badge)
                        .border(px(2.0))
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                        .children(CozyMood::ALL.into_iter().map(|mood| {
                            let is_selected = mood == active_mood;
                            let cb = mood_cb.clone();
                            let dot_id = match mood {
                                CozyMood::Matcha => "nav-mood-matcha",
                                CozyMood::Peach => "nav-mood-peach",
                                CozyMood::Berry => "nav-mood-berry",
                            };

                            div()
                                .id(dot_id)
                                .w(px(18.0))
                                .h(px(18.0))
                                .rounded_full()
                                .bg(mood.swatch_color())
                                .border(if is_selected { px(2.5) } else { px(1.5) })
                                .border_color(if is_selected {
                                    palette.text_primary
                                } else {
                                    palette.border_color
                                })
                                .cursor_pointer()
                                .on_click(move |_event, _window, cx| {
                                    if let Some(ref handler) = cb {
                                        handler(mood, cx);
                                    } else {
                                        set_cozy_mood(mood, cx);
                                    }
                                })
                        })),
                )
                .child(language_toggle_button(
                    "mobile-lang-toggle",
                    language,
                    on_language_change,
                ))
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
                ),
        )
}
