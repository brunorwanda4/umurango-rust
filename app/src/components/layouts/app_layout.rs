use super::footer::app_footer;
use super::mobile_header::app_mobile_header;
use super::nav::{AppSection, SectionSelectHandler};
use super::responsive::is_large_screen;
use super::sidebar::app_sidebar;
use crate::brand::{colors, ThemeMode};
use crate::i18n::Language;
use gpui_kit::*;
use std::rc::Rc;

pub struct AppLayoutProps<'a> {
    pub viewport_width: Pixels,
    pub theme: ThemeMode,
    pub language: Language,
    pub active_section: AppSection,
    pub member_name: &'a str,
}

pub fn app_layout<N, T, L, S, C>(
    props: AppLayoutProps<'_>,
    on_select_section: N,
    on_theme_change: T,
    on_language_change: L,
    on_sign_out: S,
    content: C,
) -> impl IntoElement
where
    N: Fn(AppSection, &mut Window, &mut App) + 'static,
    T: Fn(ThemeMode, &mut App) + 'static,
    L: Fn(Language, &mut Window, &mut App) + 'static,
    S: Fn(&mut Window, &mut App) + 'static,
    C: IntoElement,
{
    let palette = colors(props.theme);
    let on_select: SectionSelectHandler = Rc::new(on_select_section);

    if is_large_screen(props.viewport_width) {
        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(palette.bg)
            .text_color(palette.text_primary)
            .child(app_sidebar(
                &props,
                palette,
                on_select,
                on_theme_change,
                on_language_change,
                on_sign_out,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .h_full()
                    .overflow_hidden()
                    .child(content),
            )
    } else {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(palette.bg)
            .text_color(palette.text_primary)
            .child(app_mobile_header(
                props.theme,
                props.language,
                props.active_section,
                palette,
                on_theme_change,
                on_language_change,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .w_full()
                    .overflow_hidden()
                    .child(content),
            )
            .child(app_footer(
                props.language,
                props.active_section,
                palette,
                on_select,
            ))
    }
}
