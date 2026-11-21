use super::footer::app_footer;
use super::mobile_header::app_mobile_header;
use super::nav::{AppSection, SectionSelectHandler};
use super::responsive::is_large_screen;
use super::sidebar::app_sidebar;
use crate::brand::{colors, ThemeMode};
use crate::components::dashboard::MoneySubTab;
use crate::i18n::Language;
use crate::theme::CozyMood;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

pub type MoneyTabSelectHandler = Rc<dyn Fn(MoneySubTab, &mut Window, &mut App)>;
pub type QuickActionHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type BackNavHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type MoodSelectHandler = Rc<dyn Fn(CozyMood, &mut App)>;
pub type UserMenuToggleHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type UserMenuCloseHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type SignOutHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub struct AppLayoutProps<'a> {
    pub viewport_width: Pixels,
    pub safe_bottom: Pixels,
    pub theme: ThemeMode,
    pub mood: CozyMood,
    pub language: Language,
    pub active_section: AppSection,
    pub money_sub_tab: MoneySubTab,
    pub can_go_back: bool,
    pub member_name: &'a str,
    pub member_contact: &'a str,
    pub member_phone: &'a str,
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
    pub user_dropdown_open: bool,
    pub on_toggle_user_dropdown: Option<UserMenuToggleHandler>,
    pub on_close_user_dropdown: Option<UserMenuCloseHandler>,
    pub on_sign_out: Option<SignOutHandler>,
}

pub fn app_layout<N, T, L, S, C>(
    mut props: AppLayoutProps<'_>,
    on_select_section: N,
    _on_theme_change: T,
    _on_language_change: L,
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
    let sign_out_handler: SignOutHandler = Rc::new(on_sign_out);
    if props.on_sign_out.is_none() {
        props.on_sign_out = Some(sign_out_handler);
    }

    if is_large_screen(props.viewport_width) {
        let on_select_header = on_select.clone();
        let on_close_dropdown = props.on_close_user_dropdown.clone();

        div()
            .size_full()
            .flex()
            .flex_row()
            .font_family(palette.font_family)
            .bg(palette.bg_canvas)
            .text_color(palette.text_primary)
            .child(app_sidebar(&props, palette, on_select))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .when(props.user_dropdown_open, |this| {
                        if let Some(ref close) = on_close_dropdown {
                            let close = close.clone();
                            this.on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                                close(window, cx);
                            })
                        } else {
                            this
                        }
                    })
                    .child(app_mobile_header(&props, palette, on_select_header))
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
            .font_family(palette.font_family)
            .bg(palette.bg_canvas)
            .text_color(palette.text_primary)
            .child(app_mobile_header(&props, palette, on_select_header))
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
                props.safe_bottom,
            ))
    }
}
