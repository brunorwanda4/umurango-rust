use crate::brand::{colors, ThemeMode};
use crate::i18n::Language;
use crate::icons;
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    scroll::ScrollableElement,
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;
use std::sync::Arc;

type LanguageChangeHandler = Rc<dyn Fn(Language, &mut Window, &mut App)>;

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
    let palette = colors(theme);
    let theme_label = theme.label();

    div()
        .flex()
        .size_full()
        .items_center()
        .justify_center()
        .p_6()
        .bg(palette.bg)
        .text_color(palette.text_primary)
        .child(
            div().absolute().top_6().right_6().child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(language_switch(language, theme, on_language_change))
                    .child(
                        Button::new("auth-theme-toggle")
                            .icon(icons::theme_button_icon(theme))
                            .accessibility_label(theme_label)
                            .tooltip(theme_label)
                            .text()
                            .compact()
                            .on_click(move |_event, _window, cx| {
                                on_theme_change(theme.toggle(), cx)
                            }),
                    ),
            ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .w(px(440.0))
                .max_w_full()
                .max_h_full()
                .gap_5()
                .p_8()
                .rounded_xl()
                .bg(palette.card)
                .overflow_y_scrollbar()
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
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(palette.text_primary)
                        .child(title.into()),
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
        .bg(palette.surface)
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
    palette: crate::brand::ThemeColors,
    on_change: LanguageChangeHandler,
    language: Language,
) -> impl IntoElement {
    div()
        .id(id)
        .cursor_pointer()
        .px_2()
        .py_1()
        .rounded_full()
        .text_xs()
        .text_color(if selected {
            palette.text_primary
        } else {
            palette.text_muted
        })
        .hover(|style| style.bg(palette.border_hover))
        .when(selected, |this| this.bg(palette.card))
        .child(label)
        .on_click(move |_event, window, cx| on_change(language, window, cx))
}

pub fn auth_submit_section(
    button_id: impl Into<ElementId>,
    palette: crate::brand::ThemeColors,
    error_message: Option<&String>,
    label: impl Into<SharedString>,
    loading: bool,
    on_submit: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .when_some(error_message, |this, error| {
            this.child(
                div()
                    .w_full()
                    .p_3()
                    .rounded_lg()
                    .bg(palette.bg)
                    .text_color(palette.error)
                    .text_xs()
                    .child(error.clone()),
            )
        })
        .child(
            Button::new(button_id)
                .mt_4()
                .label(label)
                .primary()
                .loading(loading)
                .on_click(on_submit),
        )
}

pub fn auth_footer_prompt(
    action_id: impl Into<ElementId>,
    palette: crate::brand::ThemeColors,
    prompt: impl Into<SharedString>,
    action_label: impl Into<SharedString>,
    on_action: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .my_4()
        .gap_1()
        .text_xs()
        .text_color(palette.text_muted)
        .child(prompt.into())
        .child(
            div()
                .id(action_id)
                .cursor_pointer()
                .text_color(palette.primary_light)
                .child(action_label.into())
                .on_click(on_action),
        )
}
