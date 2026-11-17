#![allow(dead_code)]

use crate::auth::layout::auth_layout;
use crate::brand::{colors, ThemeColors, ThemeMode};
use crate::i18n::{text, Language};
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    checkbox::Checkbox,
    input::{Input, InputState},
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Clone, Debug)]
pub enum LoginEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    SignedIn {
        email: String,
        display_name: String,
        family_name: String,
        role: String,
    },
}

pub struct LoginView {
    email_input: Entity<InputState>,
    password_input: Entity<InputState>,
    remember_me: bool,
    error_message: Option<String>,
    is_authenticating: bool,
    theme: ThemeMode,
    language: Language,
}

impl EventEmitter<LoginEvent> for LoginView {}

impl LoginView {
    pub fn new(
        window: &mut Window,
        theme: ThemeMode,
        language: Language,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            email_input: cx.new(|cx| {
                InputState::new(window, cx).placeholder(text(language, "login.phone_placeholder"))
            }),
            password_input: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder(text(language, "login.password_placeholder"))
                    .masked(true)
            }),
            remember_me: true,
            error_message: None,
            is_authenticating: false,
            theme,
            language,
        }
    }

    pub fn set_theme(&mut self, theme: ThemeMode, cx: &mut Context<Self>) {
        self.theme = theme;
        cx.notify();
    }

    pub fn set_language(
        &mut self,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.language = language;
        self.email_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder(text(language, "login.phone_placeholder"))
        });
        self.password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "login.password_placeholder"))
                .masked(true)
        });
        cx.notify();
    }

    fn handle_submit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let email = self.email_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().trim().to_string();

        if email.is_empty() {
            self.error_message = Some(text(self.language, "login.email_required"));
            cx.notify();
            return;
        }
        if password.len() < 4 {
            self.error_message = Some(text(self.language, "login.password_required"));
            cx.notify();
            return;
        }

        self.error_message = None;
        self.is_authenticating = true;
        cx.notify();

        let display_name = if email.to_lowercase().contains("mama") {
            "Mama Keza"
        } else if email.to_lowercase().contains("papa") {
            "Papa Jean"
        } else {
            "Family Member"
        };

        cx.emit(LoginEvent::SignedIn {
            email,
            display_name: display_name.to_string(),
            family_name: "Umuryango wa Keza".to_string(),
            role: if display_name == "Mama Keza" {
                "Family Admin".to_string()
            } else {
                "Family Member".to_string()
            },
        });
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_entity = cx.entity().clone();
        let palette = colors(self.theme);
        let language_change = {
            let view = view_entity.clone();
            move |language: Language, window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    if this.language != language {
                        this.set_language(language, window, cx);
                        cx.emit(LoginEvent::LanguageChanged(language));
                    }
                });
            }
        };

        let content = div()
            .when_some(self.error_message.as_ref(), |this, error| {
                this.child(
                    div()
                        .p_3()
                        .rounded_lg()
                        .bg(palette.bg)
                        .text_color(palette.error)
                        .text_xs()
                        .child(error.clone()),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(login_field(
                        palette,
                        text(self.language, "login.phone_label"),
                        Input::new(&self.email_input)
                            .cleanable(true)
                            .bg(palette.surface)
                            .text_color(palette.text_body),
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(palette.text_body)
                                            .child(text(self.language, "login.password_label")),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(palette.primary_light)
                                            .child(text(self.language, "login.forgot_pin")),
                                    ),
                            )
                            .child(
                                div().w_full().rounded_lg().bg(palette.surface).p_1().child(
                                    Input::new(&self.password_input)
                                        .cleanable(true)
                                        .mask_toggle()
                                        .bg(palette.surface)
                                        .text_color(palette.text_body),
                                ),
                            ),
                    )
                    .child({
                        let view = view_entity.clone();
                        Checkbox::new("remember-me")
                            .checked(self.remember_me)
                            .text_color(palette.text_body)
                            .child(
                                div()
                                    .text_color(palette.text_body)
                                    .child(text(self.language, "login.remember_me")),
                            )
                            .on_click(move |checked, _window, cx| {
                                view.update(cx, |this, cx| {
                                    this.remember_me = *checked;
                                    cx.notify();
                                });
                            })
                    }),
            )
            .child({
                let view = view_entity.clone();
                Button::new("sign-in")
                    .label(if self.is_authenticating {
                        text(self.language, "login.signing_in")
                    } else {
                        text(self.language, "login.sign_in")
                    })
                    .primary()
                    .loading(self.is_authenticating)
                    .on_click(move |_event, window, cx| {
                        view.update(cx, |this, cx| this.handle_submit(window, cx));
                    })
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .text_xs()
                    .text_color(palette.text_muted)
                    .child(text(self.language, "login.no_account"))
                    .child(
                        div()
                            .text_color(palette.primary_light)
                            .child(text(self.language, "login.ask_admin")),
                    ),
            );

        let theme_change = {
            let view = view_entity;
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.theme = theme;
                    cx.emit(LoginEvent::ThemeChanged(theme));
                    cx.notify();
                });
            }
        };

        auth_layout(
            self.theme,
            self.language,
            language_change,
            theme_change,
            content,
        )
    }
}

fn login_field(palette: ThemeColors, label: String, input: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_xs().text_color(palette.text_body).child(label))
        .child(
            div()
                .w_full()
                .rounded_lg()
                .bg(palette.surface)
                .p_1()
                .child(input),
        )
}
