#![allow(dead_code)]

use crate::auth::layout::{auth_footer_prompt, auth_layout};
use crate::brand::{apply_theme, colors, ThemeColors, ThemeMode};
use crate::components::scroll::touch_scrollable;
use crate::components::ui::primary_button;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::*;

#[derive(Clone, Debug)]
pub enum TermsEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    Accepted,
    BackRequested,
}

pub struct TermsView {
    theme: ThemeMode,
    language: Language,
}

impl EventEmitter<TermsEvent> for TermsView {}

impl TermsView {
    pub fn new(theme: ThemeMode, language: Language, cx: &mut Context<Self>) -> Self {
        apply_theme(theme, cx);
        Self { theme, language }
    }

    pub fn set_theme(&mut self, theme: ThemeMode, cx: &mut Context<Self>) {
        self.theme = theme;
        apply_theme(theme, cx);
        cx.notify();
    }

    pub fn set_language(&mut self, language: Language, cx: &mut Context<Self>) {
        self.language = language;
        cx.notify();
    }

    pub fn sync_preferences(
        &mut self,
        theme: ThemeMode,
        language: Language,
        cx: &mut Context<Self>,
    ) {
        if self.theme != theme {
            self.set_theme(theme, cx);
        }
        if self.language != language {
            self.set_language(language, cx);
        }
    }
}

impl Render for TermsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_entity = cx.entity().clone();
        let palette = colors(self.theme);

        let language_change = {
            let view = view_entity.clone();
            move |language: Language, _window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    if this.language != language {
                        this.set_language(language, cx);
                        cx.emit(TermsEvent::LanguageChanged(language));
                    }
                });
            }
        };

        let theme_change = {
            let view = view_entity.clone();
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(theme, cx);
                    cx.emit(TermsEvent::ThemeChanged(theme));
                });
            }
        };

        let content = div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_xs()
                    .text_color(palette.text_muted)
                    .child(
                        icons::icon(icons::ICON_SHIELD_HALVED)
                            .size(px(14.))
                            .text_color(palette.primary_light),
                    )
                    .child(text(self.language, "terms.subtitle")),
            )
            .child(
                touch_scrollable("terms-charter-scroll", px(240.))
                    .child(charter_section(
                        palette,
                        text(self.language, "terms.section1_title"),
                        text(self.language, "terms.section1_body"),
                    ))
                    .child(charter_section(
                        palette,
                        text(self.language, "terms.section2_title"),
                        text(self.language, "terms.section2_body"),
                    ))
                    .child(charter_section(
                        palette,
                        text(self.language, "terms.section3_title"),
                        text(self.language, "terms.section3_body"),
                    ))
                    .child(charter_section(
                        palette,
                        text(self.language, "terms.section4_title"),
                        text(self.language, "terms.section4_body"),
                    )),
            )
            .child({
                let view = view_entity.clone();
                div().mt_2().child(primary_button(
                    "terms-accept",
                    text(self.language, "terms.accept"),
                    palette,
                    move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(TermsEvent::Accepted);
                        });
                    },
                ))
            })
            .child({
                let view = view_entity;
                auth_footer_prompt(
                    "terms-back-link",
                    palette,
                    text(self.language, "terms.back_prompt"),
                    text(self.language, "terms.back_action"),
                    move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(TermsEvent::BackRequested);
                        });
                    },
                )
            });

        auth_layout(
            self.theme,
            self.language,
            text(self.language, "terms.title"),
            language_change,
            theme_change,
            content,
        )
    }
}

fn charter_section(palette: ThemeColors, title: String, body: String) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .p_3()
        .rounded_lg()
        .border_1()
        .border_color(palette.border)
        .bg(palette.surface)
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(palette.text_primary)
                .child(title),
        )
        .child(div().text_xs().text_color(palette.text_muted).child(body))
}
