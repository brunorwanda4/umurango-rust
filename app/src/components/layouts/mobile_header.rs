use super::controls::{language_toggle_button, theme_toggle_button};
use super::nav::AppSection;
use crate::brand::{ThemeColors, ThemeMode};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::*;

pub fn app_mobile_header<T, L>(
    theme: ThemeMode,
    language: Language,
    active_section: AppSection,
    palette: ThemeColors,
    on_theme_change: T,
    on_language_change: L,
) -> impl IntoElement
where
    T: Fn(ThemeMode, &mut App) + 'static,
    L: Fn(Language, &mut Window, &mut App) + 'static,
{
    div()
        .id("app-mobile-header")
        .w_full()
        .flex_none()
        .flex()
        .items_center()
        .justify_between()
        .px_4()
        .py_2()
        .bg(palette.bg_card)
        .border_b(palette.border_width)
        .border_color(palette.border_color)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .min_w_0()
                .child(
                    div()
                        .w(px(30.0))
                        .h(px(30.0))
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(palette.radius_checkbox)
                        .bg(palette.accent_primary)
                        .border(px(2.0))
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                        .child(
                            icons::icon(icons::ICON_PEOPLE_ROOF)
                                .size(px(14.0))
                                .text_color(palette.text_on_accent),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.text_primary)
                                .child(text(language, active_section.label_key())),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(text(language, "dashboard.workspace_title")),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .flex_shrink_0()
                .child(theme_toggle_button(
                    "mobile-theme-toggle",
                    theme,
                    on_theme_change,
                ))
                .child(language_toggle_button(
                    "mobile-lang-toggle",
                    language,
                    on_language_change,
                )),
        )
}
