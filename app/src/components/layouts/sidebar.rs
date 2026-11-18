use super::controls::{language_toggle_button, theme_toggle_button};
use super::nav::{AppSection, SectionSelectHandler};
use super::responsive::SIDEBAR_WIDTH_PX;
use super::AppLayoutProps;
use crate::brand::{ThemeColors, ThemeMode};
use crate::components::dashboard::MoneySubTab;
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{compact_primary_button, icon_action_button};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn app_sidebar<T, L, S>(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
    on_theme_change: T,
    on_language_change: L,
    on_sign_out: S,
) -> impl IntoElement
where
    T: Fn(ThemeMode, &mut App) + 'static,
    L: Fn(Language, &mut Window, &mut App) + 'static,
    S: Fn(&mut Window, &mut App) + 'static,
{
    let theme = props.theme;
    let language = props.language;
    let active_section = props.active_section;
    let money_sub_tab = props.money_sub_tab;
    let quick_entry = props.on_quick_entry.clone();
    let select_for_quick = on_select.clone();
    let money_tab_handler = props.on_select_money_tab.clone();
    let select_money_parent = on_select.clone();

    let badge_for = |section: AppSection| -> Option<String> {
        match section {
            AppSection::Home => None,
            AppSection::Money => Some(props.money_badge_count.to_string()),
            AppSection::Projects => Some(props.projects_badge_count.to_string()),
            AppSection::Members => Some(props.members_badge_count.to_string()),
            AppSection::Profile => None,
        }
    };

    div()
        .id("app-sidebar")
        .w(px(SIDEBAR_WIDTH_PX))
        .h_full()
        .flex_none()
        .flex()
        .flex_col()
        .justify_between()
        .bg(palette.bg_nav)
        .border_r(palette.border_width)
        .border_color(palette.border_color)
        .child(
            div().flex_1().min_h_0().overflow_hidden().child(
                touch_scrollable_fill("sidebar-scroll").child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .p_4()
                        .child(
                            // Brand Emblem & Wordmark
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .pb_2()
                                .child(
                                    div()
                                        .w(px(44.0))
                                        .h(px(44.0))
                                        .flex_shrink_0()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded(px(16.0))
                                        .bg(palette.bg_elevated)
                                        .border(palette.border_width_input)
                                        .border_color(palette.border_color)
                                        .shadow(palette.checkbox_shadows())
                                        .child(
                                            icons::icon(icons::ICON_HOUSE_CHIMNEY_USER)
                                                .size(px(20.0))
                                                .text_color(palette.accent_primary),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_0p5()
                                        .child(
                                            div()
                                                .text_base()
                                                .font_weight(FontWeight::EXTRA_BOLD)
                                                .text_color(palette.text_primary)
                                                .child("Umuryango"),
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
                                                .text_color(palette.text_secondary)
                                                .child("FAMILY TREASURY"),
                                        ),
                                ),
                        )
                        .child(compact_primary_button(
                            "sidebar-quick-entry-btn",
                            "+ Quick Entry",
                            palette,
                            move |_event, window, cx| {
                                if let Some(ref handler) = quick_entry {
                                    handler(window, cx);
                                } else {
                                    select_for_quick(AppSection::Money, window, cx);
                                }
                            },
                        ))
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_muted)
                                .px_2()
                                .child("FAMILY WORKSPACE"),
                        )
                        .child(
                            div().flex().flex_col().gap_2().children(
                                AppSection::ALL
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, section)| {
                                        let is_active = section == active_section;
                                        let badge_text = badge_for(section);
                                        let item = sidebar_nav_item(
                                            idx,
                                            section,
                                            is_active,
                                            badge_text,
                                            language,
                                            palette,
                                            on_select.clone(),
                                        );

                                        if section == AppSection::Money {
                                            let sub_nav = sidebar_money_sub_nav(
                                                active_section == AppSection::Money,
                                                money_sub_tab,
                                                palette,
                                                money_tab_handler.clone(),
                                                select_money_parent.clone(),
                                            );
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_1p5()
                                                .child(item)
                                                .child(sub_nav)
                                                .into_any_element()
                                        } else {
                                            item.into_any_element()
                                        }
                                    }),
                            ),
                        ),
                ),
            ),
        )
        .child(
            // Live Family Treasury Card + Controls Footer
            div()
                .flex()
                .flex_col()
                .gap_2p5()
                .p_3()
                .border_t(palette.border_width_input)
                .border_color(palette.border_color)
                .child(
                    div()
                        .p_3()
                        .rounded(px(20.0))
                        .bg(palette.bg_elevated)
                        .border(px(2.0))
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_secondary)
                                        .child("✨ FAMILY TREASURY"),
                                )
                                .child(
                                    div()
                                        .px_1p5()
                                        .py(px(1.0))
                                        .rounded_full()
                                        .bg(palette.income_bg)
                                        .border(px(1.5))
                                        .border_color(palette.income_border)
                                        .text_size(px(9.0))
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.income_text)
                                        .child("2/2 Admins"),
                                ),
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.accent_primary)
                                .child(props.total_treasury.to_string()),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .mt_1()
                                .child(div().flex().items_center().gap_1().children(
                                    ["BN", "AI", "PJ", "GR"].into_iter().map(|initials| {
                                        div()
                                            .w(px(20.0))
                                            .h(px(20.0))
                                            .rounded_full()
                                            .bg(palette.bg_card)
                                            .border(px(1.5))
                                            .border_color(palette.border_color)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(9.0))
                                            .font_weight(FontWeight::EXTRA_BOLD)
                                            .text_color(palette.text_primary)
                                            .child(initials)
                                    }),
                                ))
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child(props.family_name.to_string()),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(theme_toggle_button(
                                    "sidebar-theme-toggle",
                                    theme,
                                    on_theme_change,
                                ))
                                .child(language_toggle_button(
                                    "sidebar-lang-toggle",
                                    language,
                                    on_language_change,
                                )),
                        )
                        .child(icon_action_button(
                            "sidebar-sign-out-btn",
                            icons::ICON_RIGHT_FROM_BRACKET,
                            text(language, "dashboard.sign_out"),
                            palette,
                            move |_event, window, cx| on_sign_out(window, cx),
                        )),
                ),
        )
}

fn sidebar_nav_item(
    idx: usize,
    section: AppSection,
    active: bool,
    badge_text: Option<String>,
    language: Language,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let label = text(language, section.label_key());

    div()
        .id(("sidebar-nav", idx))
        .relative()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .px_3()
        .py_2p5()
        .rounded(px(18.0))
        .border(palette.border_width_input)
        .cursor_pointer()
        .when(active, |this| {
            this.bg(palette.accent_primary)
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
        })
        .when(!active, |this| {
            this.bg(palette.bg_elevated)
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
                .hover(|s| {
                    s.top(px(-1.0))
                        .border_color(palette.accent_primary)
                        .bg(palette.bg_input_focus)
                })
        })
        .active(|s| s.top(px(2.0)))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2p5()
                .child(
                    icons::icon(section.icon())
                        .size(px(15.0))
                        .text_color(if active {
                            palette.text_on_accent
                        } else {
                            palette.accent_primary
                        }),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(if active {
                            palette.text_on_accent
                        } else {
                            palette.text_primary
                        })
                        .child(label),
                ),
        )
        .when_some(badge_text, |this, count_str| {
            this.child(
                div()
                    .px_2()
                    .py(px(1.0))
                    .rounded_full()
                    .text_size(px(10.0))
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .bg(if active {
                        palette.text_on_accent.opacity(0.2)
                    } else {
                        palette.bg_card
                    })
                    .text_color(if active {
                        palette.text_on_accent
                    } else {
                        palette.text_secondary
                    })
                    .child(count_str),
            )
        })
        .on_click(move |_event, window, cx| on_select(section, window, cx))
}

fn sidebar_money_sub_nav(
    in_money_section: bool,
    active_sub_tab: MoneySubTab,
    palette: ThemeColors,
    on_select_sub: Option<super::app_layout::MoneyTabSelectHandler>,
    on_select_section: SectionSelectHandler,
) -> impl IntoElement {
    let sub_items: [(
        &'static str,
        MoneySubTab,
        &'static str,
        Option<(&'static str, Rgba)>,
    ); 4] = [
        (
            "side-sub-overview",
            MoneySubTab::Overview,
            "• Overview",
            None,
        ),
        (
            "side-sub-income",
            MoneySubTab::Income,
            "• Income",
            Some(("+850k", palette.income_text)),
        ),
        (
            "side-sub-expenses",
            MoneySubTab::Expenses,
            "• Expenses",
            Some(("-410k", palette.expense_text)),
        ),
        (
            "side-sub-loans",
            MoneySubTab::Loans,
            "• Loan & Borrowed",
            None,
        ),
    ];

    div()
        .pl_4()
        .flex()
        .flex_col()
        .gap_1()
        .children(sub_items.into_iter().map(|(id, tab, label, metric)| {
            let is_active = in_money_section && active_sub_tab == tab;
            let sub_handler = on_select_sub.clone();
            let sec_handler = on_select_section.clone();

            div()
                .id(id)
                .flex()
                .items_center()
                .justify_between()
                .px_2p5()
                .py_1p5()
                .rounded(px(12.0))
                .border(px(1.5))
                .text_xs()
                .font_weight(FontWeight::EXTRA_BOLD)
                .cursor_pointer()
                .when(is_active, |this| {
                    this.bg(palette.bg_elevated)
                        .border_color(palette.accent_primary)
                        .text_color(palette.text_primary)
                })
                .when(!is_active, |this| {
                    this.bg(palette.bg_card)
                        .border_color(palette.border_color)
                        .text_color(palette.text_secondary)
                        .hover(|s| {
                            s.border_color(palette.accent_primary)
                                .text_color(palette.text_primary)
                        })
                })
                .child(label)
                .when_some(metric, |this, (txt, col)| {
                    this.child(
                        div()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(col)
                            .child(txt),
                    )
                })
                .on_click(move |_event, window, cx| {
                    if let Some(ref handler) = sub_handler {
                        handler(tab, window, cx);
                    } else {
                        sec_handler(AppSection::Money, window, cx);
                    }
                })
        }))
}
