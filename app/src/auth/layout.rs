use crate::brand::{colors, CozyMood, StyleTokens, ThemeColors, ThemeMode};
use crate::components::ui::{card, primary_button_with_state};
use crate::i18n::Language;
use crate::icons;
use crate::theme::{active_mood, set_cozy_mood};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;
use std::sync::Arc;

type LanguageChangeHandler = Rc<dyn Fn(Language, &mut Window, &mut App)>;
type ThemeChangeHandler = Rc<dyn Fn(ThemeMode, &mut App)>;
type MoodChangeHandler = Rc<dyn Fn(CozyMood, &mut App)>;

pub fn kawaii_family_hearth_icon(palette: StyleTokens) -> impl IntoElement {
    let figure = |size_head: Pixels, size_body_w: Pixels, size_body_h: Pixels, tint: Rgba| {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(1.5))
            .child(
                div()
                    .w(size_head)
                    .h(size_head)
                    .rounded_full()
                    .bg(tint)
                    .border(px(1.5))
                    .border_color(palette.border_color),
            )
            .child(
                div()
                    .w(size_body_w)
                    .h(size_body_h)
                    .rounded_full()
                    .bg(tint)
                    .border(px(1.5))
                    .border_color(palette.border_color),
            )
    };

    div()
        .id("kawaii-family-hearth")
        .w(px(54.0))
        .h(px(54.0))
        .flex_shrink_0()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(2.0))
        .rounded(px(18.0))
        .bg(palette.bg_input)
        .border(palette.border_width)
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .child(
            div().flex().items_center().justify_center().child(
                icons::icon(icons::ICON_HEART)
                    .size(px(13.0))
                    .text_color(palette.accent_secondary),
            ),
        )
        .child(
            div()
                .flex()
                .items_end()
                .justify_center()
                .gap(px(4.0))
                .child(figure(px(9.0), px(13.0), px(10.0), palette.accent_primary))
                .child(figure(px(8.0), px(12.0), px(9.0), palette.accent_secondary)),
        )
}

pub fn cozy_header_switcher<T, M>(
    theme: ThemeMode,
    mood: CozyMood,
    palette: StyleTokens,
    on_theme_change: T,
    on_mood_change: M,
) -> impl IntoElement
where
    T: Fn(ThemeMode, &mut App) + 'static,
    M: Fn(CozyMood, &mut App) + 'static,
{
    let on_theme: ThemeChangeHandler = Rc::new(on_theme_change);
    let on_mood: MoodChangeHandler = Rc::new(on_mood_change);

    let mood_dot_color = |m: CozyMood| match m {
        CozyMood::Matcha => rgb(0x9BE0B8),
        CozyMood::Peach => rgb(0xFFAD99),
        CozyMood::Berry => rgb(0xF7A8C4),
    };

    div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .p_1()
                .rounded_full()
                .bg(palette.bg_input)
                .border(palette.border_width_input)
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
                .children(
                    CozyMood::ALL
                        .into_iter()
                        .enumerate()
                        .map(|(idx, candidate)| {
                            let selected = candidate == mood;
                            let on_mood = on_mood.clone();
                            let dot = mood_dot_color(candidate);

                            div()
                                .id(("cozy-mood-pill", idx))
                                .cursor_pointer()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .px_2p5()
                                .py_1()
                                .rounded_full()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .when(selected, |this| {
                                    this.bg(palette.accent_primary)
                                        .text_color(palette.text_on_accent)
                                })
                                .when(!selected, |this| {
                                    this.text_color(palette.text_secondary)
                                        .hover(|s| s.bg(palette.bg_card))
                                })
                                .child(
                                    div()
                                        .w(px(8.0))
                                        .h(px(8.0))
                                        .rounded_full()
                                        .bg(dot)
                                        .border(px(1.5))
                                        .border_color(palette.border_color),
                                )
                                .child(candidate.label())
                                .on_click(move |_event, _window, cx| on_mood(candidate, cx))
                        }),
                ),
        )
        .child({
            let on_theme = on_theme.clone();
            let icon_name = if theme == ThemeMode::Dark {
                icons::ICON_SUN
            } else {
                icons::ICON_MOON
            };
            div()
                .id("auth-theme-toggle")
                .cursor_pointer()
                .flex()
                .items_center()
                .gap_1p5()
                .px_3()
                .py_1p5()
                .rounded_full()
                .bg(palette.bg_input)
                .border(palette.border_width_input)
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_primary)
                .hover(|s| {
                    s.bg(palette.bg_input_focus)
                        .border_color(palette.accent_primary)
                })
                .child(
                    icons::icon(icon_name)
                        .size(px(13.0))
                        .text_color(palette.accent_primary),
                )
                .child(theme.short_label())
                .on_click(move |_event, _window, cx| on_theme(theme.toggle(), cx))
        })
}

pub fn auth_layout<L, T, C>(
    theme: ThemeMode,
    language: Language,
    title: impl Into<SharedString>,
    on_language_change: L,
    on_theme_change: T,
    content: C,
) -> impl IntoElement
where
    L: Fn(Language, &mut Window, &mut App) + 'static,
    T: Fn(ThemeMode, &mut App) + 'static,
    C: IntoElement,
{
    let mood = active_mood();
    let on_theme = Rc::new(on_theme_change);
    let on_theme_for_mood = on_theme.clone();

    auth_layout_with_mood(
        theme,
        mood,
        language,
        title,
        on_language_change,
        move |next_theme, cx| on_theme(next_theme, cx),
        move |next_mood, cx| {
            set_cozy_mood(next_mood, cx);
            on_theme_for_mood(theme, cx);
        },
        content,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn auth_layout_with_mood<L, T, M, C>(
    theme: ThemeMode,
    mood: CozyMood,
    language: Language,
    title: impl Into<SharedString>,
    on_language_change: L,
    on_theme_change: T,
    on_mood_change: M,
    content: C,
) -> impl IntoElement
where
    L: Fn(Language, &mut Window, &mut App) + 'static,
    T: Fn(ThemeMode, &mut App) + 'static,
    M: Fn(CozyMood, &mut App) + 'static,
    C: IntoElement,
{
    let palette = StyleTokens::cozy_kawaii(theme, mood);

    div()
        .flex()
        .size_full()
        .font_family(palette.font_family)
        .items_center()
        .justify_center()
        .p_6()
        .bg(palette.bg_canvas)
        .text_color(palette.text_primary)
        .child(
            div().absolute().top_6().right_6().child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(cozy_header_switcher(
                        theme,
                        mood,
                        palette,
                        on_theme_change,
                        on_mood_change,
                    ))
                    .child(language_switch(language, theme, on_language_change)),
            ),
        )
        .child(
            card(palette)
                .w(px(450.0))
                .max_w_full()
                .gap_5()
                .p_8()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    img(Arc::new(Image::from_bytes(
                                        ImageFormat::Png,
                                        theme.wordmark().to_vec(),
                                    )))
                                    .w(px(190.0))
                                    .h(px(32.0)),
                                )
                                .child(
                                    div()
                                        .text_2xl()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_primary)
                                        .child(title.into()),
                                ),
                        )
                        .child(kawaii_family_hearth_icon(palette)),
                )
                .child(content),
        )
}

pub fn language_switch<F>(language: Language, theme: ThemeMode, on_change: F) -> impl IntoElement
where
    F: Fn(Language, &mut Window, &mut App) + 'static,
{
    let palette = colors(theme);
    let on_change: LanguageChangeHandler = Rc::new(on_change);

    div()
        .flex()
        .items_center()
        .gap_1()
        .p_1()
        .rounded_full()
        .bg(palette.bg_input)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .child(language_option(
            "auth-language-rw",
            "RW",
            language == Language::Kinyarwanda,
            palette,
            on_change.clone(),
            Language::Kinyarwanda,
        ))
        .child(language_option(
            "auth-language-en",
            "EN",
            language == Language::English,
            palette,
            on_change,
            Language::English,
        ))
}

fn language_option(
    id: &'static str,
    label: &'static str,
    selected: bool,
    palette: ThemeColors,
    on_change: LanguageChangeHandler,
    language: Language,
) -> impl IntoElement {
    div()
        .id(id)
        .cursor_pointer()
        .px_2p5()
        .py_1()
        .rounded_full()
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(if selected {
            palette.text_on_accent
        } else {
            palette.text_secondary
        })
        .hover(|style| style.bg(palette.bg_card))
        .when(selected, |this| this.bg(palette.accent_primary))
        .child(label)
        .on_click(move |_event, window, cx| on_change(language, window, cx))
}

pub fn auth_submit_section(
    button_id: impl Into<ElementId>,
    palette: ThemeColors,
    error_message: Option<&String>,
    label: impl Into<SharedString>,
    loading: bool,
    on_submit: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .mt_4()
        .pb(px(8.0))
        .when_some(error_message, |this, error| {
            this.child(
                div()
                    .w_full()
                    .p_3()
                    .rounded(palette.radius_input)
                    .bg(palette.bg_input)
                    .border(palette.border_width_input)
                    .border_color(palette.error)
                    .text_color(palette.error)
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(error.clone()),
            )
        })
        .child(primary_button_with_state(
            button_id, label, palette, loading, loading, on_submit,
        ))
}

pub fn auth_footer_prompt(
    action_id: impl Into<ElementId>,
    palette: ThemeColors,
    prompt: impl Into<SharedString>,
    action_label: impl Into<SharedString>,
    on_action: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .mt_5()
        .gap_1p5()
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(palette.text_secondary)
        .child(prompt.into())
        .child(
            div()
                .id(action_id)
                .cursor_pointer()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.accent_link)
                .hover(|s| s.text_color(palette.accent_secondary))
                .child(action_label.into())
                .on_click(on_action),
        )
}
