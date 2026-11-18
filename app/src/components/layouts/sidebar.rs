use super::controls::{language_toggle_button, theme_toggle_button};
use super::nav::{AppSection, SectionSelectHandler};
use super::responsive::SIDEBAR_WIDTH_PX;
use super::AppLayoutProps;
use crate::brand::{ThemeColors, ThemeMode};
use crate::components::ui::{icon_action_button, member_avatar, AvatarSize};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::sync::Arc;

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
    let open_profile = on_select.clone();

    div()
        .id("app-sidebar")
        .w(px(SIDEBAR_WIDTH_PX))
        .h_full()
        .flex_none()
        .flex()
        .flex_col()
        .justify_between()
        .bg(palette.surface)
        .border_r_1()
        .border_color(palette.border)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_5()
                .p_4()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .pb_3()
                        .border_b_1()
                        .border_color(palette.border)
                        .child(
                            img(Arc::new(Image::from_bytes(
                                ImageFormat::Png,
                                theme.wordmark().to_vec(),
                            )))
                            .w(px(142.0))
                            .h(px(24.0)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(text(language, "dashboard.workspace_title")),
                        ),
                )
                .child(
                    div().flex().flex_col().gap_1().children(
                        AppSection::ALL
                            .into_iter()
                            .enumerate()
                            .map(|(idx, section)| {
                                sidebar_nav_item(
                                    idx,
                                    section,
                                    section == active_section,
                                    language,
                                    palette,
                                    on_select.clone(),
                                )
                            }),
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .p_4()
                .border_t_1()
                .border_color(palette.border)
                .child(
                    div()
                        .id("sidebar-member-profile")
                        .p_2()
                        .rounded_lg()
                        .cursor_pointer()
                        .hover(|s| s.bg(palette.border.opacity(0.45)))
                        .on_click(move |_event, window, cx| {
                            open_profile(AppSection::Profile, window, cx)
                        })
                        .child(member_avatar(
                            props.member_name,
                            Some("Admin"),
                            AvatarSize::Sm,
                            palette,
                        )),
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
    language: Language,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let label = text(language, section.label_key());

    div()
        .id(("sidebar-nav", idx))
        .w_full()
        .flex()
        .items_center()
        .gap_3()
        .px_3()
        .py_2()
        .rounded_lg()
        .border_1()
        .cursor_pointer()
        .when(active, |this| {
            this.bg(palette.primary.opacity(0.14))
                .border_color(palette.primary.opacity(0.35))
        })
        .when(!active, |this| {
            this.border_color(palette.surface)
                .hover(|s| s.bg(palette.border.opacity(0.45)))
        })
        .child(
            div()
                .w(px(3.0))
                .h(px(16.0))
                .rounded_full()
                .when(active, |this| this.bg(palette.primary_light)),
        )
        .child(
            icons::icon(section.icon())
                .size(px(15.0))
                .text_color(if active {
                    palette.primary_light
                } else {
                    palette.text_muted
                }),
        )
        .child(
            div()
                .text_sm()
                .font_weight(if active {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::MEDIUM
                })
                .text_color(if active {
                    palette.text_primary
                } else {
                    palette.text_muted
                })
                .child(label),
        )
        .on_click(move |_event, window, cx| on_select(section, window, cx))
}
