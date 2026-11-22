use crate::brand::{CozyMood, ThemeColors, ThemeMode};
use crate::components::dashboard::app_update_card;
use crate::components::ui::{
    card_elevated, card_header, card_surface, destructive_button, member_avatar, pill_button,
    secondary_button, toggle_field, AvatarSize,
};
use crate::i18n::{text, Language};
use crate::icons;
use crate::theme::UiStyle;
use crate::updater::UpdateState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

pub type LanguageToggleHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type ThemeChangeHandler = Rc<dyn Fn(ThemeMode, &mut App)>;
pub type StyleChangeHandler = Rc<dyn Fn(UiStyle, &mut App)>;
pub type MoodChangeHandler = Rc<dyn Fn(CozyMood, &mut App)>;
pub type RuleToggleHandler = Rc<dyn Fn(&'static str, &mut Window, &mut App)>;
pub type SignOutHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type SyncHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type CheckUpdateHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type InstallUpdateHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub struct SettingsPageProps<'a> {
    pub language: Language,
    pub palette: ThemeColors,
    pub theme: ThemeMode,
    pub mood: CozyMood,
    pub stacked: bool,
    pub member_name: &'a str,
    pub member_role: &'a str,
    pub member_contact: &'a str,
    pub family_name: &'a str,
    pub storage_sync_status: &'a str,
    pub rule_multi_sig_over_30k: bool,
    pub rule_sunday_digest: bool,
    pub rule_auto_roundup_momo: bool,
    pub update_state: &'a UpdateState,
    pub on_toggle_language: LanguageToggleHandler,
    pub on_change_theme: ThemeChangeHandler,
    pub on_change_style: StyleChangeHandler,
    pub on_change_mood: MoodChangeHandler,
    pub on_toggle_rule: RuleToggleHandler,
    pub on_sign_out: SignOutHandler,
    pub on_sync: SyncHandler,
    pub on_check_update: CheckUpdateHandler,
    pub on_install_update: InstallUpdateHandler,
}

pub fn settings_page(props: SettingsPageProps<'_>) -> impl IntoElement {
    let SettingsPageProps {
        language: lang,
        palette,
        theme,
        mood: current_mood,
        stacked,
        member_name,
        member_role,
        member_contact,
        family_name: _,
        storage_sync_status,
        rule_multi_sig_over_30k: r1,
        rule_sunday_digest: r2,
        rule_auto_roundup_momo: r3,
        update_state,
        on_toggle_language,
        on_change_theme,
        on_change_style,
        on_change_mood,
        on_toggle_rule,
        on_sign_out,
        on_sync,
        on_check_update,
        on_install_update,
    } = props;

    let role_subtitle = if member_role.is_empty() {
        text(lang, "cozy.coadmin_planner").to_string()
    } else {
        member_role.to_string()
    };

    let identity_card = card_surface(palette)
        .gap_4()
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .child(member_avatar(
                    member_name,
                    Some(&role_subtitle),
                    AvatarSize::Xl,
                    palette,
                ))
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded_full()
                        .bg(palette.income_bg)
                        .border(px(1.5))
                        .border_color(palette.income_border)
                        .text_xs()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.income_text)
                        .child(role_subtitle.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_secondary)
                        .child(member_contact.to_string()),
                ),
        )
        .child(
            card_elevated(palette)
                .gap_3()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.text_primary)
                        .child(text(lang, "cozy.theme_studio_title")),
                )
                // 1. Style Selection (Cozy Kawaii vs Minimalist)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_secondary)
                                .child("Style Design System"),
                        )
                        .child(
                            div().flex().items_center().gap_2().children(
                                [UiStyle::CozyKawaii, UiStyle::Minimalist]
                                    .into_iter()
                                    .map(|st| {
                                        let is_selected = palette.style == st;
                                        let change_st = on_change_style.clone();
                                        let (badge_icon, style_name) = match st {
                                            UiStyle::CozyKawaii => ("🧸", "Cozy Kawaii"),
                                            UiStyle::Minimalist => ("✨", "Minimalist"),
                                            _ => ("🎨", "Style"),
                                        };
                                        div()
                                            .id(match st {
                                                UiStyle::CozyKawaii => "profile-style-cozy",
                                                UiStyle::Minimalist => "profile-style-minimalist",
                                                _ => "profile-style-other",
                                            })
                                            .flex_1()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .gap_1p5()
                                            .py_2()
                                            .px_2p5()
                                            .rounded(palette.radius_button)
                                            .border(palette.border_width_input)
                                            .cursor_pointer()
                                            .when(is_selected, |this| {
                                                this.bg(palette.bg_input_focus)
                                                    .border_color(palette.accent_primary)
                                                    .shadow(palette.checkbox_shadows())
                                                    .text_color(palette.text_primary)
                                                    .font_weight(FontWeight::EXTRA_BOLD)
                                            })
                                            .when(!is_selected, |this| {
                                                this.bg(palette.bg_card)
                                                    .border_color(palette.border_color)
                                                    .text_color(palette.text_secondary)
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .hover(|s| {
                                                        s.border_color(palette.accent_primary)
                                                            .text_color(palette.text_primary)
                                                    })
                                            })
                                            .child(div().text_size(px(13.0)).child(badge_icon))
                                            .child(div().text_xs().child(style_name))
                                            .when(is_selected, |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(palette.accent_primary)
                                                        .child("✓"),
                                                )
                                            })
                                            .on_click(move |_e, _w, cx| {
                                                change_st(st, cx);
                                            })
                                    }),
                            ),
                        ),
                )
                // 2. Appearance Theme (Light vs Dark)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_secondary)
                                .child("Appearance Mode"),
                        )
                        .child(div().flex().items_center().gap_2().children(
                            [ThemeMode::Light, ThemeMode::Dark].into_iter().map(|mode| {
                                let is_selected = theme == mode;
                                let change_th = on_change_theme.clone();
                                let (mode_icon, mode_name) = match mode {
                                    ThemeMode::Light => ("☀️", "Light"),
                                    ThemeMode::Dark => ("🌙", "Dark"),
                                };
                                div()
                                    .id(match mode {
                                        ThemeMode::Light => "profile-theme-light",
                                        ThemeMode::Dark => "profile-theme-dark",
                                    })
                                    .flex_1()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .gap_1p5()
                                    .py_2()
                                    .px_2p5()
                                    .rounded(palette.radius_button)
                                    .border(palette.border_width_input)
                                    .cursor_pointer()
                                    .when(is_selected, |this| {
                                        this.bg(palette.bg_input_focus)
                                            .border_color(palette.accent_primary)
                                            .shadow(palette.checkbox_shadows())
                                            .text_color(palette.text_primary)
                                            .font_weight(FontWeight::EXTRA_BOLD)
                                    })
                                    .when(!is_selected, |this| {
                                        this.bg(palette.bg_card)
                                            .border_color(palette.border_color)
                                            .text_color(palette.text_secondary)
                                            .font_weight(FontWeight::MEDIUM)
                                            .hover(|s| {
                                                s.border_color(palette.accent_primary)
                                                    .text_color(palette.text_primary)
                                            })
                                    })
                                    .child(div().text_size(px(13.0)).child(mode_icon))
                                    .child(div().text_xs().child(mode_name))
                                    .when(is_selected, |this| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(palette.accent_primary)
                                                .child("✓"),
                                        )
                                    })
                                    .on_click(move |_e, _w, cx| {
                                        change_th(mode, cx);
                                    })
                            }),
                        )),
                )
                // 3. Cozy Mood Swatches
                .when(palette.style.uses_mood(), |this| {
                    this.child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child("Cozy Mood Accent"),
                            )
                            .child(div().flex().items_center().gap_2().children(
                                CozyMood::ALL.into_iter().map(|mood| {
                                    let is_active = current_mood == mood;
                                    let change_m = on_change_mood.clone();
                                    let btn_id = match mood {
                                        CozyMood::Matcha => "profile-mood-matcha",
                                        CozyMood::Peach => "profile-mood-peach",
                                        CozyMood::Berry => "profile-mood-berry",
                                    };
                                    div()
                                        .id(btn_id)
                                        .flex_1()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap_1p5()
                                        .py_1p5()
                                        .rounded(palette.radius_button)
                                        .border(palette.border_width_input)
                                        .border_color(if is_active {
                                            palette.accent_primary
                                        } else {
                                            palette.border_color
                                        })
                                        .bg(if is_active {
                                            palette.bg_input_focus
                                        } else {
                                            palette.bg_card
                                        })
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .cursor_pointer()
                                        .child(
                                            div()
                                                .w(px(10.0))
                                                .h(px(10.0))
                                                .rounded_full()
                                                .bg(mood.swatch_color()),
                                        )
                                        .child(mood.label())
                                        .on_click(move |_e, _w, cx| {
                                            change_m(mood, cx);
                                        })
                                }),
                            )),
                    )
                })
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .pt_1()
                        .child(secondary_button(
                            "profile-switch-lang-btn",
                            format!("{}: {}", text(lang, "section.language_label"), lang.label()),
                            palette,
                            move |_e, w, cx| on_toggle_language(w, cx),
                        ))
                        .child(destructive_button(
                            "profile-signout-btn",
                            text(lang, "dashboard.sign_out"),
                            palette,
                            move |_e, w, cx| on_sign_out(w, cx),
                        )),
                ),
        );

    let governance_card = card_surface(palette)
        .gap_2p5()
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(palette.text_primary)
                .child(text(lang, "cozy.governance_rules_title")),
        )
        .child(card_elevated(palette).child(toggle_field(
            "rule-multisig-30k",
            text(lang, "cozy.rule_multisig_30k"),
            None::<&str>,
            r1,
            palette,
            {
                let on_toggle = on_toggle_rule.clone();
                move |_e, w, cx| on_toggle("rule_multi_sig_over_30k", w, cx)
            },
        )))
        .child(card_elevated(palette).child(toggle_field(
            "rule-sunday-digest",
            text(lang, "cozy.rule_sunday_digest"),
            None::<&str>,
            r2,
            palette,
            {
                let on_toggle = on_toggle_rule.clone();
                move |_e, w, cx| on_toggle("rule_sunday_digest", w, cx)
            },
        )))
        .child(card_elevated(palette).child(toggle_field(
            "rule-auto-roundup",
            text(lang, "cozy.rule_auto_roundup"),
            None::<&str>,
            r3,
            palette,
            {
                let on_toggle = on_toggle_rule.clone();
                move |_e, w, cx| on_toggle("rule_auto_roundup_momo", w, cx)
            },
        )));

    let channels_card = card_surface(palette)
        .gap_2p5()
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(palette.text_primary)
                .child(text(lang, "cozy.connected_channels_title")),
        )
        .child(
            div()
                .flex()
                .gap_2p5()
                .when(stacked, |this| this.flex_col())
                .child(
                    card_elevated(palette)
                        .flex_1()
                        .items_center()
                        .gap_1()
                        .child(
                            icons::icon(icons::ICON_PHONE)
                                .size(px(16.0))
                                .text_color(palette.accent_gold),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child("MTN MoMo"),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.income_text)
                                .child(text(lang, "cozy.channel_linked")),
                        ),
                )
                .child(
                    card_elevated(palette)
                        .flex_1()
                        .items_center()
                        .gap_1()
                        .child(
                            icons::icon(icons::ICON_CREDIT_CARD)
                                .size(px(16.0))
                                .text_color(palette.expense_text),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child("Airtel Money"),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.income_text)
                                .child(text(lang, "cozy.channel_linked")),
                        ),
                )
                .child(
                    card_elevated(palette)
                        .flex_1()
                        .items_center()
                        .gap_1()
                        .child(
                            icons::icon(icons::ICON_BUILDING_COLUMNS)
                                .size(px(16.0))
                                .text_color(palette.accent_primary),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child("I&M Bank"),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.income_text)
                                .child(text(lang, "cozy.channel_treasury")),
                        ),
                ),
        );

    let storage_card = card_surface(palette)
        .child(card_header(
            text(lang, "dashboard.storage_health"),
            Some(text(lang, "cozy.storage_sub")),
            palette,
        ))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .flex_wrap()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            icons::icon(icons::ICON_CIRCLE_CHECK)
                                .size(px(14.0))
                                .text_color(palette.income_text),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_primary)
                                .child(storage_sync_status.to_string()),
                        ),
                )
                .child(pill_button(
                    "profile-sync-now-btn",
                    text(lang, "dashboard.sync_now"),
                    palette,
                    move |_e, w, cx| on_sync(w, cx),
                )),
        );

    let update_card = app_update_card(
        update_state,
        lang,
        palette,
        move |_e, w, cx| on_check_update(w, cx),
        move |_e, w, cx| on_install_update(w, cx),
    );

    div()
        .flex()
        .gap_4()
        .when(stacked, |this| this.flex_col())
        .child(div().flex_1().min_w_0().child(identity_card))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap_4()
                .child(governance_card)
                .child(channels_card)
                .child(storage_card)
                .child(update_card),
        )
        .into_any_element()
}
