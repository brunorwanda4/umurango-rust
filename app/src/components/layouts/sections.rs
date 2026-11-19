#![allow(dead_code)]

use crate::brand::{colors, ThemeColors, ThemeMode};
use crate::components::dashboard::{
    expense_category_card, format_rwf, income_stream_card, loan_card, member_pledge_card,
    project_goal_card, transaction_row, EXPENSE_CATEGORIES, FAMILY_MEMBERS, INCOME_STREAMS, LOANS,
    PROJECTS, TRANSACTIONS,
};
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{
    badge, card_elevated, card_header, card_surface, compact_primary_button, destructive_button,
    kawaii_progress_bar, member_avatar, pill_button, secondary_button, stat_card, AvatarSize,
    BadgeVariant,
};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

fn section_header(
    title: impl Into<SharedString>,
    subtitle: impl Into<SharedString>,
    palette: ThemeColors,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(palette.text_primary)
                .child(title.into()),
        )
        .child(
            div()
                .text_sm()
                .text_color(palette.text_secondary)
                .child(subtitle.into()),
        )
}

pub fn money_section(
    _viewport_width: Pixels,
    theme: ThemeMode,
    language: Language,
) -> impl IntoElement {
    let palette = colors(theme);

    let streams = div().flex().flex_col().gap_2p5().children(
        INCOME_STREAMS
            .iter()
            .enumerate()
            .map(|(idx, s)| income_stream_card(idx, s, palette)),
    );

    let categories = div().flex().flex_col().gap_2p5().children(
        EXPENSE_CATEGORIES
            .iter()
            .enumerate()
            .map(|(idx, c)| expense_category_card(idx, c, palette)),
    );

    let loans = div().flex().flex_col().gap_2p5().children(
        LOANS
            .iter()
            .enumerate()
            .map(|(idx, l)| loan_card(idx, l, language, palette, |_e, _w, _cx| {})),
    );

    let transactions = card_surface(palette)
        .child(card_header(
            text(language, "dashboard.recent_activity"),
            Some(text(language, "cozy.recent_activity_sub")),
            palette,
        ))
        .child(
            div().flex().flex_col().gap_2().children(
                TRANSACTIONS
                    .iter()
                    .enumerate()
                    .map(|(index, transaction)| transaction_row(index, transaction, palette)),
            ),
        );

    div().size_full().child(
        touch_scrollable_fill("money-section-scroll").child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .p_6()
                .child(section_header(
                    text(language, "nav.money"),
                    text(language, "section.money_subtitle"),
                    palette,
                ))
                .child(streams)
                .child(categories)
                .child(loans)
                .child(transactions),
        ),
    )
}

pub fn projects_section(theme: ThemeMode, language: Language) -> impl IntoElement {
    let palette = colors(theme);
    let total_saved: i64 = PROJECTS.iter().map(|p| p.saved_rwf).sum();

    div().size_full().child(
        touch_scrollable_fill("projects-section-scroll").child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .p_6()
                .child(section_header(
                    text(language, "nav.projects"),
                    text(language, "section.projects_subtitle"),
                    palette,
                ))
                .child(
                    card_surface(palette)
                        .flex_row()
                        .items_center()
                        .justify_between()
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
                                        .child(text(language, "cozy.total_saved_plans")),
                                )
                                .child(
                                    div()
                                        .text_xl()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.accent_primary)
                                        .child(format_rwf(total_saved)),
                                ),
                        )
                        .child(compact_primary_button(
                            "projects-sec-new-btn",
                            text(language, "cozy.new_project_btn"),
                            palette,
                            |_e, _w, _cx| {},
                        )),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .children(PROJECTS.iter().enumerate().map(|(idx, project)| {
                            project_goal_card(
                                idx,
                                project,
                                false,
                                language,
                                palette,
                                |_e, _w, _cx| {},
                                |_e, _w, _cx| {},
                            )
                        })),
                ),
        ),
    )
}

pub fn members_section(theme: ThemeMode, language: Language) -> impl IntoElement {
    let palette = colors(theme);
    let total_pledge: i64 = FAMILY_MEMBERS.iter().map(|m| m.monthly_goal_rwf).sum();
    let total_contributed: i64 = FAMILY_MEMBERS.iter().map(|m| m.contributed_rwf).sum();
    let pct = if total_pledge <= 0 {
        100
    } else {
        ((total_contributed * 100) / total_pledge).clamp(0, 100) as u8
    };

    div().size_full().child(
        touch_scrollable_fill("members-section-scroll").child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .p_6()
                .child(section_header(
                    text(language, "nav.members"),
                    text(language, "section.members_subtitle"),
                    palette,
                ))
                .child(
                    card_surface(palette)
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
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_secondary)
                                                .child(text(
                                                    language,
                                                    "cozy.circle_monthly_pledge",
                                                )),
                                        )
                                        .child(
                                            div()
                                                .text_lg()
                                                .font_weight(FontWeight::EXTRA_BOLD)
                                                .text_color(palette.text_primary)
                                                .child(format!(
                                                    "{} / {} ({}%)",
                                                    format_rwf(total_contributed),
                                                    format_rwf(total_pledge),
                                                    pct
                                                )),
                                        ),
                                )
                                .child(compact_primary_button(
                                    "members-sec-invite-btn",
                                    text(language, "cozy.invite_btn"),
                                    palette,
                                    |_e, _w, _cx| {},
                                )),
                        )
                        .child(kawaii_progress_bar(pct, palette.accent_primary, palette)),
                )
                .child(div().flex().flex_col().gap_3().children(
                    FAMILY_MEMBERS.iter().enumerate().map(|(idx, member)| {
                        member_pledge_card(
                            idx,
                            member,
                            language,
                            palette,
                            |_e, _w, _cx| {},
                            |_e, _w, _cx| {},
                        )
                    }),
                )),
        ),
    )
}

pub fn profile_section<T, L, S>(
    theme: ThemeMode,
    language: Language,
    member_name: &str,
    on_theme_change: T,
    on_language_change: L,
    on_sign_out: S,
) -> impl IntoElement
where
    T: Fn(ThemeMode, &mut App) + 'static,
    L: Fn(Language, &mut Window, &mut App) + 'static,
    S: Fn(&mut Window, &mut App) + 'static,
{
    let palette = colors(theme);
    let next_theme = theme.toggle();
    let next_language = language.toggle();

    div().size_full().child(
        touch_scrollable_fill("profile-section-scroll").child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .p_6()
                .child(section_header(
                    text(language, "nav.profile"),
                    text(language, "section.profile_subtitle"),
                    palette,
                ))
                .child(
                    card_surface(palette).child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .flex_wrap()
                            .gap_3()
                            .child(member_avatar(
                                member_name,
                                Some(&text(language, "cozy.coadmin_planner")),
                                AvatarSize::Xl,
                                palette,
                            ))
                            .child(badge(
                                text(language, "dashboard.workspace_badge"),
                                BadgeVariant::Primary,
                                palette,
                            )),
                    ),
                )
                .child(
                    card_surface(palette)
                        .child(card_header(
                            text(language, "section.preferences"),
                            Some(text(language, "dashboard.workspace_title")),
                            palette,
                        ))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_3()
                                .child(
                                    card_elevated(palette)
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_primary)
                                                .child(text(language, "section.theme_label")),
                                        )
                                        .child(pill_button(
                                            "profile-theme-btn",
                                            theme.label(),
                                            palette,
                                            move |_event, _window, cx| {
                                                on_theme_change(next_theme, cx)
                                            },
                                        )),
                                )
                                .child(
                                    card_elevated(palette)
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_primary)
                                                .child(text(language, "section.language_label")),
                                        )
                                        .child(secondary_button(
                                            "profile-lang-btn",
                                            language.label(),
                                            palette,
                                            move |_event, window, cx| {
                                                on_language_change(next_language, window, cx)
                                            },
                                        )),
                                ),
                        ),
                )
                .child(
                    card_surface(palette)
                        .child(card_header(
                            text(language, "section.account_actions"),
                            None::<&str>,
                            palette,
                        ))
                        .child(div().flex().child(destructive_button(
                            "profile-sign-out-btn",
                            text(language, "dashboard.sign_out"),
                            palette,
                            move |_event, window, cx| on_sign_out(window, cx),
                        ))),
                ),
        ),
    )
}
