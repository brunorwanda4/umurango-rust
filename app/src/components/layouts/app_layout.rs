use super::footer::app_footer;
use super::mobile_header::app_mobile_header;
use super::nav::{AppSection, SectionSelectHandler};
use super::responsive::is_large_screen;
use super::sidebar::app_sidebar;
use crate::brand::{colors, ThemeMode};
use crate::components::dashboard::MoneySubTab;
use crate::i18n::Language;
use crate::theme::CozyMood;
use gpui_kit::*;
use std::rc::Rc;

pub type MoneyTabSelectHandler = Rc<dyn Fn(MoneySubTab, &mut Window, &mut App)>;
pub type QuickActionHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type BackNavHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type MoodSelectHandler = Rc<dyn Fn(CozyMood, &mut App)>;

pub struct AppLayoutProps<'a> {
    pub viewport_width: Pixels,
    pub theme: ThemeMode,
    pub mood: CozyMood,
    pub language: Language,
    pub active_section: AppSection,
    pub money_sub_tab: MoneySubTab,
    pub can_go_back: bool,
    pub member_name: &'a str,
    pub family_name: &'a str,
    pub total_treasury: &'a str,
    pub money_badge_count: usize,
    pub projects_badge_count: usize,
    pub members_badge_count: usize,
    pub pending_approvals_count: usize,
    pub on_select_money_tab: Option<MoneyTabSelectHandler>,
    pub on_quick_entry: Option<QuickActionHandler>,
    pub on_back: Option<BackNavHandler>,
    pub on_mood_change: Option<MoodSelectHandler>,
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
    let on_theme = Rc::new(on_theme_change);
    let on_lang = Rc::new(on_language_change);

    if is_large_screen(props.viewport_width) {
        let on_theme_header = on_theme.clone();
        let on_lang_header = on_lang.clone();
        let on_select_header = on_select.clone();

        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(palette.bg_canvas)
            .text_color(palette.text_primary)
            .child(app_sidebar(
                &props,
                palette,
                on_select,
                move |next_theme, cx| on_theme(next_theme, cx),
                move |next_lang, window, cx| on_lang(next_lang, window, cx),
                on_sign_out,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(app_mobile_header(
                        &props,
                        palette,
                        on_select_header,
                        move |next_theme, cx| on_theme_header(next_theme, cx),
                        move |next_lang, window, cx| on_lang_header(next_lang, window, cx),
                    ))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .min_h_0()
                            .w_full()
                            .overflow_hidden()
                            .child(content),
                    ),
            )
    } else {
        let on_select_header = on_select.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(palette.bg_canvas)
            .text_color(palette.text_primary)
            .child(app_mobile_header(
                &props,
                palette,
                on_select_header,
                move |next_theme, cx| on_theme(next_theme, cx),
                move |next_lang, window, cx| on_lang(next_lang, window, cx),
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
