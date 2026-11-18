#![allow(dead_code)]

use crate::auth::client::api_join_family;
use crate::auth::layout::auth_layout;
use crate::brand::{apply_theme, colors, ThemeMode};
use crate::components::ui::{primary_button_with_state, secondary_button};
use crate::i18n::{text, Language};
use gpui_kit::component::{
    input::{OtpEvent, OtpInput, OtpState},
    Sizable, Size,
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Clone, Debug)]
pub enum JoinFamilyEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    BackRequested,
    FamilyJoined {
        id: String,
        name: String,
        role: String,
    },
}

pub struct JoinFamilyView {
    otp_state: Entity<OtpState>,
    error_message: Option<String>,
    success_message: Option<String>,
    is_joining: bool,
    theme: ThemeMode,
    language: Language,
    _subscription: Subscription,
}

impl EventEmitter<JoinFamilyEvent> for JoinFamilyView {}

impl JoinFamilyView {
    pub fn new(
        window: &mut Window,
        theme: ThemeMode,
        language: Language,
        cx: &mut Context<Self>,
    ) -> Self {
        apply_theme(theme, cx);

        // 4-character code for family join
        let otp_state = cx.new(|cx| OtpState::new(4, window, cx));

        let subscription = cx.subscribe(&otp_state, |_this, _entity, event: &OtpEvent, cx| {
            if *event == OtpEvent::Complete {
                cx.notify();
            }
        });

        Self {
            otp_state,
            error_message: None,
            success_message: None,
            is_joining: false,
            theme,
            language,
            _subscription: subscription,
        }
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

    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = &event.keystroke.key;
        let mut current = self.otp_state.read(cx).value().to_string();

        if key == "backspace" {
            current.pop();
            self.otp_state.update(cx, |state, cx| {
                state.set_value(current, window, cx);
            });
            self.error_message = None;
            cx.notify();
        } else if current.len() < 4 {
            let ch = key.chars().next().or_else(|| {
                event
                    .keystroke
                    .key_char
                    .as_deref()
                    .and_then(|s| s.chars().next())
            });

            if let Some(c) = ch {
                if c.is_ascii_alphanumeric() {
                    current.push(c.to_ascii_uppercase());
                    self.otp_state.update(cx, |state, cx| {
                        state.set_value(current, window, cx);
                    });
                    self.error_message = None;
                    cx.notify();
                }
            }
        }
    }

    fn handle_submit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let code = self
            .otp_state
            .read(cx)
            .value()
            .to_string()
            .trim()
            .to_uppercase();

        if code.len() < 4 {
            self.error_message = Some(text(self.language, "join_family.code_required"));
            cx.notify();
            return;
        }

        self.is_joining = true;
        self.error_message = None;
        cx.notify();

        match api_join_family(&code) {
            Ok(resp) => {
                self.is_joining = false;
                self.success_message = Some(text(self.language, "join_family.success"));
                cx.notify();

                cx.emit(JoinFamilyEvent::FamilyJoined {
                    id: resp.family_id,
                    name: resp.family_name,
                    role: resp.role,
                });
            }
            Err(err) => {
                self.is_joining = false;
                self.error_message = Some(err);
                cx.notify();
            }
        }
    }
}

impl Render for JoinFamilyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_entity = cx.entity().clone();
        let palette = colors(self.theme);

        let language_change = {
            let view = view_entity.clone();
            move |language: Language, _window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    if this.language != language {
                        this.set_language(language, cx);
                        cx.emit(JoinFamilyEvent::LanguageChanged(language));
                    }
                });
            }
        };

        let theme_change = {
            let view = view_entity.clone();
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(theme, cx);
                    cx.emit(JoinFamilyEvent::ThemeChanged(theme));
                });
            }
        };

        let code_value = self.otp_state.read(cx).value().to_string();

        let content = div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_xs()
                    .text_color(palette.text_muted)
                    .child(text(self.language, "join_family.subtitle")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_3()
                    .p_6()
                    .rounded(palette.radius_input)
                    .bg(palette.bg_input)
                    .border(palette.border_width_input)
                    .border_color(palette.border_color)
                    .shadow(palette.input_shadows())
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(palette.text_secondary)
                            .child(text(self.language, "join_family.code_label")),
                    )
                    .child(
                        div()
                            .id("otp-wrapper")
                            .track_focus(&self.otp_state.read(cx).focus_handle(cx))
                            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                                this.handle_key_down(event, window, cx);
                            }))
                            .child(
                                OtpInput::new(&self.otp_state)
                                    .groups(1)
                                    .with_size(Size::Large),
                            ),
                    )
                    .when(!code_value.is_empty(), |this| {
                        this.child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.accent_primary)
                                .child(code_value),
                        )
                    }),
            )
            .when_some(self.error_message.as_ref(), |this, msg| {
                this.child(
                    div()
                        .w_full()
                        .p_3()
                        .rounded(palette.radius_input)
                        .border(palette.border_width_input)
                        .border_color(palette.error)
                        .bg(palette.bg_input)
                        .text_color(palette.error)
                        .text_xs()
                        .child(msg.clone()),
                )
            })
            .when_some(self.success_message.as_ref(), |this, msg| {
                this.child(
                    div()
                        .w_full()
                        .p_3()
                        .rounded(palette.radius_input)
                        .border(palette.border_width_input)
                        .border_color(palette.success)
                        .bg(palette.bg_input)
                        .text_color(palette.success)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .child(msg.clone()),
                )
            })
            .child({
                let view = view_entity.clone();
                let submit_label = if self.is_joining {
                    text(self.language, "join_family.submitting")
                } else {
                    text(self.language, "join_family.submit")
                };

                primary_button_with_state(
                    "submit-join-family",
                    submit_label,
                    palette,
                    self.is_joining,
                    self.is_joining,
                    move |_event, window, cx| {
                        view.update(cx, |this, cx| {
                            this.handle_submit(window, cx);
                        });
                    },
                )
            })
            .child({
                let view = view_entity;
                secondary_button(
                    "back-family-setup",
                    text(self.language, "join_family.back"),
                    palette,
                    move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(JoinFamilyEvent::BackRequested);
                        });
                    },
                )
            });

        auth_layout(
            self.theme,
            self.language,
            text(self.language, "join_family.title"),
            language_change,
            theme_change,
            content,
        )
    }
}
