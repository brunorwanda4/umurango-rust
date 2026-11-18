#![allow(dead_code)]

use crate::auth::layout::{auth_footer_prompt, auth_layout, auth_submit_section};
use crate::brand::{apply_theme, colors, ThemeMode};
use crate::components::{
    checkbox::checkbox,
    input::{input_field, password_field},
};
use crate::i18n::{text, Language};
use gpui_kit::component::input::InputState;
use gpui_kit::*;

#[derive(Clone, Debug)]
pub enum LoginEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    RegisterRequested,
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
    show_password: bool,
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
        apply_theme(theme, cx);

        let email_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder(text(language, "login.phone_placeholder"))
        });
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "login.password_placeholder"))
                .masked(true)
        });

        email_input.update(cx, |input, cx| {
            input.focus(window, cx);
        });

        Self {
            email_input,
            password_input,
            show_password: false,
            remember_me: true,
            error_message: None,
            is_authenticating: false,
            theme,
            language,
        }
    }

    pub fn set_theme(&mut self, theme: ThemeMode, cx: &mut Context<Self>) {
        self.theme = theme;
        apply_theme(theme, cx);
        cx.notify();
    }

    pub fn set_language(
        &mut self,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.language = language;
        let current_email = self.email_input.read(cx).value().to_string();
        let current_password = self.password_input.read(cx).value().to_string();
        let masked = !self.show_password;

        self.email_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "login.phone_placeholder"))
                .default_value(current_email)
        });
        self.password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "login.password_placeholder"))
                .default_value(current_password)
                .masked(masked)
        });
        cx.notify();
    }

    pub fn sync_preferences(
        &mut self,
        theme: ThemeMode,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.theme != theme {
            self.set_theme(theme, cx);
        }
        if self.language != language {
            self.set_language(language, window, cx);
        }
    }

    fn toggle_password_visibility(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_password = !self.show_password;
        let masked = !self.show_password;
        self.password_input.update(cx, |input, cx| {
            input.set_masked(masked, window, cx);
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
        let password_toggle_label = if self.show_password {
            text(self.language, "login.hide_password")
        } else {
            text(self.language, "login.show_password")
        };
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

        let forgot_pin = div()
            .text_xs()
            .text_color(palette.primary_light)
            .child(text(self.language, "login.forgot_pin"))
            .into_any_element();

        let content = div()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(input_field(
                        palette,
                        text(self.language, "login.phone_label"),
                        &self.email_input,
                    ))
                    .child({
                        let view = view_entity.clone();
                        password_field(
                            "toggle-password-visibility",
                            palette,
                            text(self.language, "login.password_label"),
                            Some(forgot_pin),
                            &self.password_input,
                            self.show_password,
                            password_toggle_label,
                            move |_event, window, cx| {
                                view.update(cx, |this, cx| {
                                    this.toggle_password_visibility(window, cx);
                                });
                            },
                        )
                    })
                    .child({
                        let view = view_entity.clone();
                        checkbox(
                            "remember-me",
                            palette,
                            self.remember_me,
                            text(self.language, "login.remember_me"),
                            move |_event, _window, cx| {
                                view.update(cx, |this, cx| {
                                    this.remember_me = !this.remember_me;
                                    cx.notify();
                                });
                            },
                        )
                    }),
            )
            .child({
                let view = view_entity.clone();
                let submit_label = if self.is_authenticating {
                    text(self.language, "login.signing_in")
                } else {
                    text(self.language, "login.sign_in")
                };
                auth_submit_section(
                    "sign-in",
                    palette,
                    self.error_message.as_ref(),
                    submit_label,
                    self.is_authenticating,
                    move |_event, window, cx| {
                        view.update(cx, |this, cx| this.handle_submit(window, cx));
                    },
                )
            })
            .child({
                let view = view_entity.clone();
                auth_footer_prompt(
                    "login-register-link",
                    palette,
                    text(self.language, "login.no_account"),
                    text(self.language, "login.create_account"),
                    move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(LoginEvent::RegisterRequested);
                        });
                    },
                )
            });

        let theme_change = {
            let view = view_entity;
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(theme, cx);
                    cx.emit(LoginEvent::ThemeChanged(theme));
                });
            }
        };

        auth_layout(
            self.theme,
            self.language,
            text(self.language, "login.title"),
            language_change,
            theme_change,
            content,
        )
    }
}
