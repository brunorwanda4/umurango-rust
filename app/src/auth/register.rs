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
pub enum RegisterEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    LoginRequested,
    Registered {
        name: String,
        phone: String,
        display_name: String,
        family_name: String,
        role: String,
    },
}

pub struct RegisterView {
    name_input: Entity<InputState>,
    phone_input: Entity<InputState>,
    password_input: Entity<InputState>,
    show_password: bool,
    remember_me: bool,
    error_message: Option<String>,
    is_registering: bool,
    theme: ThemeMode,
    language: Language,
}

impl EventEmitter<RegisterEvent> for RegisterView {}

impl RegisterView {
    pub fn new(
        window: &mut Window,
        theme: ThemeMode,
        language: Language,
        cx: &mut Context<Self>,
    ) -> Self {
        apply_theme(theme, cx);

        let name_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder(text(language, "register.name_placeholder"))
        });
        let phone_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder(text(language, "register.phone_placeholder"))
        });
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "register.password_placeholder"))
                .masked(true)
        });

        name_input.update(cx, |input, cx| {
            input.focus(window, cx);
        });

        Self {
            name_input,
            phone_input,
            password_input,
            show_password: false,
            remember_me: true,
            error_message: None,
            is_registering: false,
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
        let current_name = self.name_input.read(cx).value().to_string();
        let current_phone = self.phone_input.read(cx).value().to_string();
        let current_password = self.password_input.read(cx).value().to_string();
        let masked = !self.show_password;

        self.name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "register.name_placeholder"))
                .default_value(current_name)
        });
        self.phone_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "register.phone_placeholder"))
                .default_value(current_phone)
        });
        self.password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "register.password_placeholder"))
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
        let name = self.name_input.read(cx).value().trim().to_string();
        let phone = self.phone_input.read(cx).value().trim().to_string();
        let password = self.password_input.read(cx).value().trim().to_string();

        if let Err(error_key) = validate_registration_input(&name, &phone, &password) {
            self.error_message = Some(text(self.language, error_key));
            cx.notify();
            return;
        }

        self.error_message = None;
        self.is_registering = true;
        cx.notify();

        cx.emit(RegisterEvent::Registered {
            display_name: name.clone(),
            name,
            phone,
            family_name: "Umuryango wa Keza".to_string(),
            role: "Family Member".to_string(),
        });
    }
}

pub fn validate_registration_input(
    name: &str,
    phone: &str,
    password: &str,
) -> Result<(), &'static str> {
    if name.trim().is_empty() {
        return Err("register.name_required");
    }
    if phone.trim().is_empty() {
        return Err("register.phone_required");
    }
    if password.trim().len() < 4 {
        return Err("register.password_required");
    }
    Ok(())
}

impl Render for RegisterView {
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
                        cx.emit(RegisterEvent::LanguageChanged(language));
                    }
                });
            }
        };

        let content = div()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(input_field(
                        palette,
                        text(self.language, "register.name_label"),
                        &self.name_input,
                    ))
                    .child(input_field(
                        palette,
                        text(self.language, "register.phone_label"),
                        &self.phone_input,
                    ))
                    .child({
                        let view = view_entity.clone();
                        password_field(
                            "register-toggle-password-visibility",
                            palette,
                            text(self.language, "register.password_label"),
                            None,
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
                            "register-remember-me",
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
                let submit_label = if self.is_registering {
                    text(self.language, "register.submitting")
                } else {
                    text(self.language, "register.submit")
                };
                auth_submit_section(
                    "register-submit",
                    palette,
                    self.error_message.as_ref(),
                    submit_label,
                    self.is_registering,
                    move |_event, window, cx| {
                        view.update(cx, |this, cx| this.handle_submit(window, cx));
                    },
                )
            })
            .child({
                let view = view_entity.clone();
                auth_footer_prompt(
                    "register-login-link",
                    palette,
                    text(self.language, "register.have_account"),
                    text(self.language, "register.sign_in"),
                    move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(RegisterEvent::LoginRequested);
                        });
                    },
                )
            });

        let theme_change = {
            let view = view_entity;
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(theme, cx);
                    cx.emit(RegisterEvent::ThemeChanged(theme));
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

#[cfg(test)]
mod tests {
    use super::validate_registration_input;
    use crate::i18n::{text, Language};

    #[test]
    fn registration_validation_checks_name_phone_and_password() {
        assert_eq!(
            validate_registration_input("", "+250788123456", "1234"),
            Err("register.name_required")
        );
        assert_eq!(
            validate_registration_input("Keza", "   ", "1234"),
            Err("register.phone_required")
        );
        assert_eq!(
            validate_registration_input("Keza", "+250788123456", "123"),
            Err("register.password_required")
        );
        assert_eq!(
            validate_registration_input("Keza", "+250788123456", "1234"),
            Ok(())
        );
    }

    #[test]
    fn registration_locale_keys_exist_in_both_languages() {
        for key in [
            "register.name_label",
            "register.name_placeholder",
            "register.phone_label",
            "register.phone_placeholder",
            "register.password_label",
            "register.password_placeholder",
            "register.submit",
            "register.submitting",
            "register.have_account",
            "register.sign_in",
            "register.name_required",
            "register.phone_required",
            "register.password_required",
        ] {
            assert_ne!(text(Language::English, key), key);
            assert_ne!(text(Language::Kinyarwanda, key), key);
        }
    }
}
