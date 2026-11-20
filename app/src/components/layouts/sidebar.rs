use super::nav::{AppSection, SectionSelectHandler};
use super::responsive::SIDEBAR_WIDTH_PX;
use super::AppLayoutProps;
use crate::brand::{ThemeColors, ThemeMode};
use crate::components::dashboard::MoneySubTab;
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{avatar_initials, extract_initials, AvatarSize};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

type SidebarClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;
type SubNavItem = (
    &'static str,
    MoneySubTab,
    String,
    Option<(&'static str, Rgba)>,
);

#[derive(Clone, Copy)]
struct DropdownColors {
    hover_bg: Rgba,
    text_active: Rgba,
    text_inactive: Rgba,
    danger_col: Rgba,
}

pub fn app_sidebar(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    if palette.is_minimalist() {
        app_sidebar_minimalist(props, palette, on_select).into_any_element()
    } else {
        app_sidebar_cozy(props, palette, on_select).into_any_element()
    }
}

/// Sleek shadcn-style minimalist sidebar with "Family" and "Management" sections.
fn app_sidebar_minimalist(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let language = props.language;
    let active_section = props.active_section;
    let money_sub_tab = props.money_sub_tab;
    let money_tab_handler = props.on_select_money_tab.clone();
    let select_money_parent = on_select.clone();

    let (active_bg, hover_bg, text_active, text_inactive, text_muted_col) =
        if palette.mode == ThemeMode::Dark {
            (
                rgb(0x18181b),
                rgb(0x18181b),
                rgb(0xffffff),
                rgb(0xd4d4d8),
                rgb(0x71717a),
            )
        } else {
            (
                rgb(0xf4f4f5),
                rgb(0xf4f4f5),
                rgb(0x09090b),
                rgb(0x3f3f46),
                rgb(0xa1a1aa),
            )
        };

    let render_item = |id: &'static str,
                       icon_name: &'static str,
                       label: String,
                       is_active: bool,
                       has_chevron: bool,
                       on_click_fn: SidebarClickHandler| {
        div()
            .id(id)
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py(px(7.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .when(is_active, |this| this.bg(active_bg).text_color(text_active))
            .when(!is_active, |this| {
                this.text_color(text_inactive)
                    .hover(move |s| s.bg(hover_bg).text_color(text_active))
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        icons::icon(icon_name)
                            .size(px(16.0))
                            .text_color(if is_active {
                                text_active
                            } else {
                                text_inactive
                            }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(if is_active {
                                FontWeight::SEMIBOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(if is_active {
                                text_active
                            } else {
                                text_inactive
                            })
                            .child(label),
                    ),
            )
            .when(has_chevron, |this| {
                this.child(
                    icons::icon(icons::ICON_CHEVRON_RIGHT)
                        .size(px(11.0))
                        .text_color(text_muted_col),
                )
            })
            .on_click(move |ev, window, cx| on_click_fn(ev, window, cx))
    };

    let on_sel_home = on_select.clone();
    let on_sel_money = on_select.clone();
    let on_sel_proj = on_select.clone();
    let on_sel_reports = on_select.clone();
    let on_sel_notif = on_select.clone();
    let on_sel_members = on_select.clone();
    let on_sel_settings = on_select.clone();

    let in_money = active_section == AppSection::Money;

    div()
        .id("app-sidebar")
        .w(px(SIDEBAR_WIDTH_PX))
        .h_full()
        .flex_none()
        .flex()
        .flex_col()
        .justify_between()
        .bg(palette.bg_nav)
        .border_r(px(1.0))
        .border_color(palette.border_color)
        .child(
            div().flex_1().min_h_0().overflow_hidden().child(
                touch_scrollable_fill("sidebar-scroll").child(
                    div()
                        .flex()
                        .flex_col()
                        .px_2()
                        .py_3()
                        .gap_1()
                        // 1. Family Section
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(text_muted_col)
                                .px_3()
                                .py_1p5()
                                .child(text(language, "sidebar.family")),
                        )
                        .child(render_item(
                            "min-nav-dashboard",
                            icons::ICON_HOUSE,
                            text(language, "sidebar.dashboard"),
                            active_section == AppSection::Home,
                            false,
                            Box::new(move |_e, window, cx| {
                                on_sel_home(AppSection::Home, window, cx)
                            }),
                        ))
                        .child(render_item(
                            "min-nav-money",
                            icons::ICON_WALLET,
                            text(language, "nav.money"),
                            in_money,
                            true,
                            Box::new(move |_e, window, cx| {
                                on_sel_money(AppSection::Money, window, cx)
                            }),
                        ))
                        .when(in_money, |this| {
                            this.child(sidebar_minimalist_money_sub_nav(
                                money_sub_tab,
                                language,
                                palette,
                                active_bg,
                                hover_bg,
                                text_active,
                                text_inactive,
                                money_tab_handler.clone(),
                                select_money_parent.clone(),
                            ))
                        })
                        .child(render_item(
                            "min-nav-projects",
                            icons::ICON_FOLDER,
                            text(language, "nav.projects"),
                            active_section == AppSection::Projects,
                            false,
                            Box::new(move |_e, window, cx| {
                                on_sel_proj(AppSection::Projects, window, cx)
                            }),
                        ))
                        .child(render_item(
                            "min-nav-reports",
                            icons::ICON_FILE_LINES,
                            text(language, "sidebar.reports"),
                            false,
                            false,
                            Box::new(move |_e, window, cx| {
                                on_sel_reports(AppSection::Money, window, cx)
                            }),
                        ))
                        .child(render_item(
                            "min-nav-notifications",
                            icons::ICON_BELL,
                            text(language, "sidebar.notifications"),
                            false,
                            false,
                            Box::new(move |_e, window, cx| {
                                on_sel_notif(AppSection::Home, window, cx)
                            }),
                        ))
                        // 2. Management Section
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(text_muted_col)
                                .px_3()
                                .pt_5()
                                .pb_1p5()
                                .child(text(language, "sidebar.management")),
                        )
                        .child(render_item(
                            "min-nav-members",
                            icons::ICON_USERS,
                            text(language, "nav.members"),
                            active_section == AppSection::Members,
                            false,
                            Box::new(move |_e, window, cx| {
                                on_sel_members(AppSection::Members, window, cx)
                            }),
                        ))
                        .child(render_item(
                            "min-nav-settings",
                            icons::ICON_GEAR,
                            text(language, "sidebar.settings"),
                            active_section == AppSection::Profile,
                            true,
                            Box::new(move |_e, window, cx| {
                                on_sel_settings(AppSection::Profile, window, cx)
                            }),
                        )),
                ),
            ),
        )
        .child(sidebar_minimalist_footer(props, palette, on_select.clone()))
}

fn sidebar_dropdown_item(
    id: &'static str,
    icon: Svg,
    label: String,
    is_danger: bool,
    colors: DropdownColors,
    on_click_fn: SidebarClickHandler,
) -> impl IntoElement {
    let text_col = if is_danger {
        colors.danger_col
    } else {
        colors.text_active
    };
    let icon_col = if is_danger {
        colors.danger_col
    } else {
        colors.text_inactive
    };

    div()
        .id(id)
        .w_full()
        .flex()
        .items_center()
        .gap_2p5()
        .px_2()
        .py(px(6.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(move |s| s.bg(colors.hover_bg))
        .child(icon.size(px(14.0)).flex_none().text_color(icon_col))
        .child(
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::NORMAL)
                .text_color(text_col)
                .truncate()
                .child(label),
        )
        .on_click(move |ev, window, cx| on_click_fn(ev, window, cx))
}

fn sidebar_minimalist_footer(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let (hover_bg, dropdown_bg, text_active, text_inactive, text_muted_col, danger_col) =
        if palette.mode == ThemeMode::Dark {
            (
                rgb(0x18181b),
                rgb(0x121214),
                rgb(0xffffff),
                rgb(0xd4d4d8),
                rgb(0x71717a),
                rgb(0xf87171),
            )
        } else {
            (
                rgb(0xf4f4f5),
                rgb(0xffffff),
                rgb(0x09090b),
                rgb(0x3f3f46),
                rgb(0xa1a1aa),
                rgb(0xef4444),
            )
        };

    let colors = DropdownColors {
        hover_bg,
        text_active,
        text_inactive,
        danger_col,
    };

    let member_name = if props.member_name.trim().is_empty() {
        "Rwanda Bruno".to_string()
    } else {
        props.member_name.to_string()
    };

    let member_subtitle = if !props.member_phone.trim().is_empty() {
        props.member_phone.to_string()
    } else if !props.member_contact.trim().is_empty() {
        props.member_contact.to_string()
    } else {
        "brunorwanda4@gmail.com".to_string()
    };

    let family_name = if props.family_name.trim().is_empty() {
        "Ingabire Family".to_string()
    } else {
        props.family_name.to_string()
    };

    let initials = extract_initials(&member_name);
    let is_open = props.user_dropdown_open;
    let on_toggle = props.on_toggle_user_dropdown.clone();
    let on_close = props.on_close_user_dropdown.clone();
    let on_sign_out = props.on_sign_out.clone();

    div()
        .id("sidebar-footer-container")
        .relative()
        .w_full()
        .border_t(px(1.0))
        .border_color(palette.border_color)
        .p_2()
        .when(is_open, |this| {
            let on_sel_family = on_select.clone();
            let on_sel_overview = on_select.clone();
            let on_sel_fams = on_select.clone();
            let on_sel_recent = on_select.clone();
            let on_sel_manage = on_select.clone();
            let on_sel_create = on_select.clone();
            let close_1 = on_close.clone();
            let close_2 = on_close.clone();
            let close_3 = on_close.clone();
            let close_4 = on_close.clone();
            let close_5 = on_close.clone();
            let close_6 = on_close.clone();
            let close_7 = on_close.clone();
            let sign_out_cb = on_sign_out.clone();
            let fam_label_1 = family_name.clone();
            let fam_label_2 = family_name.clone();

            this.child(
                div()
                    .id("sidebar-user-dropdown-popover")
                    .absolute()
                    .bottom(px(56.0))
                    .left(px(8.0))
                    .right(px(8.0))
                    .bg(dropdown_bg)
                    .border(px(1.0))
                    .border_color(palette.border_color)
                    .rounded(px(12.0))
                    .p_1p5()
                    .shadow(palette.card_shadows())
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_muted_col)
                            .px_2()
                            .pt_1()
                            .pb_0p5()
                            .child("Workspaces"),
                    )
                    .child(sidebar_dropdown_item(
                        "drop-item-workspace-family",
                        icons::icon(icons::ICON_USERS),
                        fam_label_1,
                        false,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_1 {
                                close(window, cx);
                            }
                            on_sel_family(AppSection::Home, window, cx);
                        }),
                    ))
                    .child(sidebar_dropdown_item(
                        "drop-item-overview",
                        icons::icon(icons::ICON_INBOX),
                        "Overview".to_string(),
                        false,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_2 {
                                close(window, cx);
                            }
                            on_sel_overview(AppSection::Home, window, cx);
                        }),
                    ))
                    .child(sidebar_dropdown_item(
                        "drop-item-families",
                        icons::icon(icons::ICON_USERS),
                        "Families".to_string(),
                        false,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_3 {
                                close(window, cx);
                            }
                            on_sel_fams(AppSection::Members, window, cx);
                        }),
                    ))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_muted_col)
                            .px_2()
                            .pt_2()
                            .pb_0p5()
                            .child("Recent families"),
                    )
                    .child(sidebar_dropdown_item(
                        "drop-item-recent-family",
                        icons::icon(icons::ICON_USERS),
                        fam_label_2,
                        false,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_4 {
                                close(window, cx);
                            }
                            on_sel_recent(AppSection::Home, window, cx);
                        }),
                    ))
                    .child(sidebar_dropdown_item(
                        "drop-item-manage-access",
                        icons::icon(icons::ICON_CLIPBOARD_LIST),
                        "Manage family access".to_string(),
                        false,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_5 {
                                close(window, cx);
                            }
                            on_sel_manage(AppSection::Members, window, cx);
                        }),
                    ))
                    .child(sidebar_dropdown_item(
                        "drop-item-create-family",
                        icons::icon(icons::ICON_MONEY_BILL_TRANSFER),
                        "Create family".to_string(),
                        false,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_6 {
                                close(window, cx);
                            }
                            on_sel_create(AppSection::Members, window, cx);
                        }),
                    ))
                    .child(div().h(px(1.0)).w_full().bg(palette.border_color).my_1())
                    .child(sidebar_dropdown_item(
                        "drop-item-logout",
                        icons::icon(icons::ICON_RIGHT_FROM_BRACKET),
                        "Log out".to_string(),
                        true,
                        colors,
                        Box::new(move |_e, window, cx| {
                            if let Some(ref close) = close_7 {
                                close(window, cx);
                            }
                            if let Some(ref sign_out) = sign_out_cb {
                                sign_out(window, cx);
                            }
                        }),
                    )),
            )
        })
        .child(
            div()
                .id("sidebar-footer-user-btn")
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .gap_2p5()
                .px_2p5()
                .py(px(6.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2p5()
                        .min_w_0()
                        .flex_1()
                        .child(
                            div()
                                .w(px(32.0))
                                .h(px(32.0))
                                .flex_none()
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .bg(if palette.mode == ThemeMode::Dark {
                                    rgb(0x27272a)
                                } else {
                                    rgb(0xe4e4e7)
                                })
                                .border(px(1.0))
                                .border_color(palette.border_color)
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(text_active)
                                .child(initials),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .min_w_0()
                                .flex_1()
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text_active)
                                        .truncate()
                                        .child(member_name),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::NORMAL)
                                        .text_color(text_muted_col)
                                        .truncate()
                                        .child(member_subtitle),
                                ),
                        ),
                )
                .child(
                    icons::icon(icons::ICON_SORT)
                        .size(px(12.0))
                        .text_color(text_muted_col),
                )
                .on_click(move |_ev, window, cx| {
                    if let Some(ref toggle) = on_toggle {
                        toggle(window, cx);
                    }
                }),
        )
}

#[allow(clippy::too_many_arguments)]
fn sidebar_minimalist_money_sub_nav(
    active_sub_tab: MoneySubTab,
    language: Language,
    palette: ThemeColors,
    active_bg: Rgba,
    hover_bg: Rgba,
    text_active: Rgba,
    text_inactive: Rgba,
    on_select_sub: Option<super::app_layout::MoneyTabSelectHandler>,
    on_select_section: SectionSelectHandler,
) -> impl IntoElement {
    let sub_items: [SubNavItem; 4] = [
        (
            "min-sub-overview",
            MoneySubTab::Overview,
            text(language, "cozy.tab_overview"),
            None,
        ),
        (
            "min-sub-income",
            MoneySubTab::Income,
            text(language, "cozy.tab_income"),
            Some(("+850k", palette.income_text)),
        ),
        (
            "min-sub-expenses",
            MoneySubTab::Expenses,
            text(language, "cozy.tab_expenses"),
            Some(("-410k", palette.expense_text)),
        ),
        (
            "min-sub-loans",
            MoneySubTab::Loans,
            text(language, "cozy.tab_loans"),
            None,
        ),
    ];

    div()
        .pl(px(28.0))
        .pr_1()
        .py_1()
        .flex()
        .flex_col()
        .gap_0p5()
        .children(sub_items.into_iter().map(|(id, tab, label, metric)| {
            let is_active = active_sub_tab == tab;
            let sub_handler = on_select_sub.clone();
            let sec_handler = on_select_section.clone();

            div()
                .id(id)
                .flex()
                .items_center()
                .justify_between()
                .px_2p5()
                .py(px(5.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .when(is_active, |this| this.bg(active_bg).text_color(text_active))
                .when(!is_active, |this| {
                    this.text_color(text_inactive)
                        .hover(move |s| s.bg(hover_bg).text_color(text_active))
                })
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(if is_active {
                            FontWeight::SEMIBOLD
                        } else {
                            FontWeight::NORMAL
                        })
                        .child(label),
                )
                .when_some(metric, |this, (txt, col)| {
                    this.child(
                        div()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::BOLD)
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

/// Tactile pill card sidebar for Cozy Kawaii mode.
fn app_sidebar_cozy(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let language = props.language;
    let active_section = props.active_section;
    let money_sub_tab = props.money_sub_tab;
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
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_muted)
                                .px_2()
                                .child(text(language, "cozy.family_workspace_heading")),
                        )
                        .child(
                            div().flex().flex_col().gap_2().children(
                                AppSection::ALL
                                    .into_iter()
                                    .filter(|&section| section != AppSection::Profile)
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
                                                language,
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
        .child(sidebar_user_profile(props, palette, on_select.clone()))
}

fn sidebar_user_profile(
    props: &AppLayoutProps<'_>,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let language = props.language;
    let is_active = props.active_section == AppSection::Profile;
    let member_name = if props.member_name.trim().is_empty() {
        "User".to_string()
    } else {
        props.member_name.to_string()
    };
    let family_name = if props.family_name.trim().is_empty() {
        text(language, "cozy.family_workspace_heading").to_string()
    } else {
        props.family_name.to_string()
    };
    let initials = extract_initials(&member_name);

    div()
        .p_3()
        .border_t(palette.border_width)
        .border_color(palette.border_color)
        .bg(palette.bg_nav)
        .child(
            div()
                .id("sidebar-user-card")
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .gap_2p5()
                .p_2()
                .rounded(px(16.0))
                .border(palette.border_width_input)
                .cursor_pointer()
                .when(is_active, |this| {
                    this.bg(palette.accent_primary)
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                })
                .when(!is_active, |this| {
                    this.bg(palette.bg_elevated)
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                        .hover(|mut s| {
                            s = s.top(px(-1.0));
                            s.border_color(palette.accent_primary)
                                .bg(palette.bg_input_focus)
                        })
                })
                .active(|s| s.top(px(2.0)))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2p5()
                        .min_w_0()
                        .flex_1()
                        .child(if is_active {
                            div()
                                .w(px(28.0))
                                .h(px(28.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .bg(palette.bg_canvas)
                                .border(palette.border_width_input)
                                .border_color(palette.border_color)
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.accent_primary)
                                .child(initials)
                                .into_any_element()
                        } else {
                            avatar_initials(initials, AvatarSize::Sm, palette).into_any_element()
                        })
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .min_w_0()
                                .flex_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(if is_active {
                                            palette.text_on_accent
                                        } else {
                                            palette.text_primary
                                        })
                                        .truncate()
                                        .child(member_name),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(if is_active {
                                            palette.text_on_accent.opacity(0.8)
                                        } else {
                                            palette.text_muted
                                        })
                                        .truncate()
                                        .child(family_name),
                                ),
                        ),
                )
                .child(
                    icons::icon(icons::ICON_GEAR)
                        .size(px(14.0))
                        .text_color(if is_active {
                            palette.text_on_accent
                        } else {
                            palette.text_muted
                        }),
                )
                .on_click(move |_event, window, cx| {
                    on_select(AppSection::Profile, window, cx);
                }),
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
                .hover(|mut s| {
                    s = s.top(px(-1.0));
                    s.border_color(palette.accent_primary)
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
    language: Language,
    palette: ThemeColors,
    on_select_sub: Option<super::app_layout::MoneyTabSelectHandler>,
    on_select_section: SectionSelectHandler,
) -> impl IntoElement {
    let sub_items: [SubNavItem; 4] = [
        (
            "side-sub-overview",
            MoneySubTab::Overview,
            text(language, "cozy.sub_overview"),
            None,
        ),
        (
            "side-sub-income",
            MoneySubTab::Income,
            text(language, "cozy.sub_income"),
            Some(("+850k", palette.income_text)),
        ),
        (
            "side-sub-expenses",
            MoneySubTab::Expenses,
            text(language, "cozy.sub_expenses"),
            Some(("-410k", palette.expense_text)),
        ),
        (
            "side-sub-loans",
            MoneySubTab::Loans,
            text(language, "cozy.sub_loans"),
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
                .border(palette.border_width_input)
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
