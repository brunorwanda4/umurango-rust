use crate::brand::ThemeMode;
use crate::i18n::Language;
use crate::icons;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::{App, Window};

pub fn theme_toggle_button(
    id: &'static str,
    theme: ThemeMode,
    on_theme_change: impl Fn(ThemeMode, &mut App) + 'static,
) -> Button {
    let label = theme.label();
    Button::new(id)
        .icon(icons::theme_button_icon(theme))
        .accessibility_label(label)
        .tooltip(label)
        .text()
        .compact()
        .on_click(move |_event, _window, cx| on_theme_change(theme.toggle(), cx))
}

pub fn language_toggle_button(
    id: &'static str,
    language: Language,
    on_language_change: impl Fn(Language, &mut Window, &mut App) + 'static,
) -> Button {
    let next_language = language.toggle();
    Button::new(id)
        .label(next_language.code())
        .accessibility_label(language.label())
        .tooltip(language.label())
        .text()
        .compact()
        .on_click(move |_event, window, cx| on_language_change(next_language, window, cx))
}
