use crate::brand::{colors, ThemeColors, ThemeMode};
use crate::components::dashboard::{project_card, transaction_row, PROJECTS, TRANSACTIONS};
use crate::components::layouts::app_content_width;
use crate::components::scroll::touch_scrollable_fill;
use crate::components::ui::{
    badge, card_header, card_surface, destructive_button, member_avatar, secondary_button,
    stat_card, AvatarSize, BadgeVariant,
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
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_primary)
                .child(title.into()),
        )
        .child(
            div()
                .text_sm()
                .text_color(palette.text_muted)
                .child(subtitle.into()),
        )
}

pub fn money_section(
    viewport_width: Pixels,
    theme: ThemeMode,
    language: Language,
) -> impl IntoElement {
    let palette = colors(theme);
    let stacked = app_content_width(viewport_width) < px(640.0);

    let summary_cards = div()
        .flex()
        .gap_4()
        .when(stacked, |this| this.flex_col())
        .child(div().flex_1().child(stat_card(
            "money-stat-treasury",
            palette,
            text(language, "dashboard.total_treasury"),
            "3,450,000 RWF",
            Some(text(language, "dashboard.treasury_trend")),
            Some(icons::ICON_WALLET),
            Some(("+14.2%".into(), true)),
        )))
        .child(div().flex_1().child(stat_card(
            "money-stat-contributions",
            palette,
            text(language, "dashboard.monthly_contributions"),
            "780,000 RWF",
            Some(text(language, "dashboard.contributions_sub")),
            Some(icons::ICON_CREDIT_CARD),
            Some(("8/10 paid".into(), true)),
        )))
        .child(div().flex_1().child(stat_card(
            "money-stat-expenses",
            palette,
            text(language, "dashboard.monthly_expenses"),
            "240,000 RWF",
            Some(text(language, "dashboard.expenses_sub")),
            Some(icons::ICON_ARROW_DOWN),
            Some(("-8%".into(), true)),
        )));

    let transactions = card_surface(palette)
        .child(card_header(
            text(language, "dashboard.recent_activity"),
            Some("Latest treasury transactions and contributions"),
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
                .child(summary_cards)
                .child(transactions),
        ),
    )
}

pub fn projects_section(theme: ThemeMode, language: Language) -> impl IntoElement {
    let palette = colors(theme);

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
                        .child(card_header(
                            text(language, "dashboard.active_projects"),
                            Some("Communal family goals and fundraising progress"),
                            palette,
                        ))
                        .child(
                            div().flex().flex_col().gap_3().children(
                                PROJECTS
                                    .iter()
                                    .map(|project| project_card(project, palette)),
                            ),
                        ),
                ),
        ),
    )
}

pub fn members_section(theme: ThemeMode, language: Language) -> impl IntoElement {
    let palette = colors(theme);
    const MEMBERS: [(&str, &str, BadgeVariant); 5] = [
        ("Bruno Mugabo", "Family Head / Admin", BadgeVariant::Primary),
        ("Claudine Mugabo", "Treasurer", BadgeVariant::Success),
        ("Eric Mugabo", "Member", BadgeVariant::Neutral),
        ("Alice Keza", "Member", BadgeVariant::Neutral),
        ("Patrick Mugabo", "Youth Member", BadgeVariant::Info),
    ];

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
                        .child(card_header(
                            text(language, "dashboard.household_members"),
                            Some("5 active members in this workspace"),
                            palette,
                        ))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_3()
                                .children(MEMBERS.into_iter().map(|(name, role, variant)| {
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .flex_wrap()
                                        .gap_2()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(palette.card)
                                        .border_1()
                                        .border_color(palette.border)
                                        .child(member_avatar(
                                            name,
                                            Some(role),
                                            AvatarSize::Md,
                                            palette,
                                        ))
                                        .child(badge(role, variant, palette))
                                })),
                        ),
                ),
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
                                Some("Family Admin"),
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
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .gap_3()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(palette.text_body)
                                                .child(text(language, "section.theme_label")),
                                        )
                                        .child(secondary_button(
                                            "profile-theme-btn",
                                            theme.label(),
                                            palette,
                                            move |_event, _window, cx| {
                                                on_theme_change(next_theme, cx)
                                            },
                                        )),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .gap_3()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(palette.text_body)
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
