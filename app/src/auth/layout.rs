use crate::brand::{colors, ThemeMode};
use crate::i18n::Language;
use crate::icons;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;
use std::sync::Arc;

pub fn auth_layout<L, T, C>(
    theme: ThemeMode,
    language: Language,
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
            div()
                .flex()
                .flex_col()
                .w(px(440.0))
                .max_w_full()
                .gap_5()
                .p_8()
                .rounded_xl()
                .bg(palette.card)
                .child(
                    div()
                        .flex()
                        .justify_end()
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
                )
                .child(
                    img(Arc::new(Image::from_bytes(
                        ImageFormat::Png,
                        theme.wordmark().to_vec(),
                    )))
                    .w(px(190.0))
                    .h(px(32.0)),
                )
                .child(content),
        )
}

pub fn language_switch<F>(language: Language, theme: ThemeMode, on_change: F) -> impl IntoElement
where
    F: Fn(Language, &mut Window, &mut App) + 'static,
{
    let palette = colors(theme);
    let on_change: Rc<dyn Fn(Language, &mut Window, &mut App)> = Rc::new(on_change);

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
    on_change: Rc<dyn Fn(Language, &mut Window, &mut App)>,
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
