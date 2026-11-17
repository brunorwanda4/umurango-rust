use crate::brand::*;
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    checkbox::Checkbox,
    input::{Input, InputState},
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum LoginEvent {
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
    family_code_input: Entity<InputState>,
    remember_me: bool,
    error_message: Option<String>,
    is_authenticating: bool,
}

impl EventEmitter<LoginEvent> for LoginView {}

impl LoginView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let email_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("member@family.umuryango.rw"));
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter your password or PIN")
                .masked(true)
        });
        let family_code_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Optional: e.g. fam_kigali_01"));

        Self {
            email_input,
            password_input,
            family_code_input,
            remember_me: true,
            error_message: None,
            is_authenticating: false,
        }
    }

    fn fill_demo_account(
        &mut self,
        email: &str,
        password: &str,
        family_code: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.error_message = None;
        self.email_input
            .update(cx, |input, cx| input.set_value(email, window, cx));
        self.password_input
            .update(cx, |input, cx| input.set_value(password, window, cx));
        self.family_code_input
            .update(cx, |input, cx| input.set_value(family_code, window, cx));
        cx.notify();
    }

    fn handle_submit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let email = self.email_input.read(cx).value().to_string();
        let password = self.password_input.read(cx).value().to_string();
        let family_code = self.family_code_input.read(cx).value().to_string();

        let email_trimmed = email.trim();
        let password_trimmed = password.trim();

        if email_trimmed.is_empty() {
            self.error_message = Some("Please enter your email or member ID.".to_string());
            cx.notify();
            return;
        }

        if !email_trimmed.contains('@') {
            self.error_message =
                Some("Please enter a valid email address (e.g. name@family.rw).".to_string());
            cx.notify();
            return;
        }

        if password_trimmed.is_empty() {
            self.error_message = Some("Please enter your security password or PIN.".to_string());
            cx.notify();
            return;
        }

        if password_trimmed.len() < 4 {
            self.error_message = Some("Password or PIN must be at least 4 characters.".to_string());
            cx.notify();
            return;
        }

        self.error_message = None;
        self.is_authenticating = true;
        cx.notify();

        // Determine member role and family based on credentials or fallback
        let (display_name, role, family_name) = if email_trimmed.contains("mama") {
            (
                "Mama Keza".to_string(),
                "Family Admin".to_string(),
                "Umuryango wa Keza".to_string(),
            )
        } else if email_trimmed.contains("papa") {
            (
                "Papa Jean".to_string(),
                "Family Member".to_string(),
                "Umuryango wa Keza".to_string(),
            )
        } else {
            let user_part = email_trimmed.split('@').next().unwrap_or("Member");
            let clean_name = user_part
                .replace('.', " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            let fam = if !family_code.trim().is_empty() {
                format!("Family ({})", family_code.trim())
            } else {
                "Umuryango wa Rwanda".to_string()
            };

            (clean_name, "Member".to_string(), fam)
        };

        cx.emit(LoginEvent::SignedIn {
            email: email_trimmed.to_string(),
            display_name,
            family_name,
            role,
        });
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_entity = cx.entity().clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(color_bg())
            .text_color(color_text_primary())
            .items_center()
            .justify_center()
            .p_6()
            // Main card container
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(460.0))
                    .max_w_full()
                    .bg(color_card())
                    .border_1()
                    .border_color(color_border())
                    .rounded_xl()
                    .p_8()
                    .gap_5()
                    // Header with Brand Logo and Title
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .size(px(56.0))
                                    .rounded_full()
                                    .bg(color_surface())
                                    .border_1()
                                    .border_color(color_border())
                                    .child(
                                        img(Arc::new(Image::from_bytes(
                                            ImageFormat::Png,
                                            BRAND_ICON.to_vec(),
                                        )))
                                        .size(px(34.0)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_2xl()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(color_text_primary())
                                                    .child("Umuryango"),
                                            )
                                            .child(
                                                div()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_full()
                                                    .bg(color_primary())
                                                    .text_color(color_text_primary())
                                                    .text_xs()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Governance"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(color_text_muted())
                                            .child("Family Financial Governance Workspace"),
                                    ),
                            ),
                    )
                    // Divider
                    .child(
                        div()
                            .h(px(1.0))
                            .w_full()
                            .bg(color_border()),
                    )
                    // Sign-in Subheading
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(color_text_primary())
                                    .child("Sign In - Umuryango"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(color_text_muted())
                                    .child("Enter your credentials to access your family accounts and records."),
                            ),
                    )
                    // Error message display if present
                    .when_some(self.error_message.as_ref(), |this, error| {
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .p_3()
                                .rounded_lg()
                                .bg(rgb(0x450a0a))
                                .border_1()
                                .border_color(color_error())
                                .text_color(color_text_primary())
                                .text_xs()
                                .child("⚠️")
                                .child(error.clone()),
                        )
                    })
                    // Quick Demo Credentials Pill Bar
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .p_3()
                            .rounded_lg()
                            .bg(color_surface())
                            .border_1()
                            .border_color(color_border())
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(color_primary_light())
                                            .child("Quick Demo Accounts:"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(color_text_dim())
                                            .child("Click to fill"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child({
                                        let v = view_entity.clone();
                                        Button::new("demo-admin")
                                            .label("Mama Keza (Admin)")
                                            .compact()
                                            .on_click(move |_e, window, cx| {
                                                v.update(cx, |this, cx| {
                                                    this.fill_demo_account(
                                                        "mama.keza@umuryango.rw",
                                                        "Umuryango2026!",
                                                        "fam_kigali_01",
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            })
                                    })
                                    .child({
                                        let v = view_entity.clone();
                                        Button::new("demo-member")
                                            .label("Papa Jean (Member)")
                                            .compact()
                                            .on_click(move |_e, window, cx| {
                                                v.update(cx, |this, cx| {
                                                    this.fill_demo_account(
                                                        "papa.jean@umuryango.rw",
                                                        "Umuryango2026!",
                                                        "fam_kigali_01",
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            })
                                    }),
                            ),
                    )
                    // Form Fields
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            // Email Field
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(color_text_body())
                                            .child("Email or Member ID"),
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .rounded_lg()
                                            .bg(color_surface())
                                            .border_1()
                                            .border_color(color_border())
                                            .p_1()
                                            .child(
                                                Input::new(&self.email_input)
                                                    .cleanable(true),
                                            ),
                                    ),
                            )
                            // Password Field
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .items_center()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(color_text_body())
                                                    .child("Password or Security PIN"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(color_primary_light())
                                                    .child("Forgot PIN?"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .rounded_lg()
                                            .bg(color_surface())
                                            .border_1()
                                            .border_color(color_border())
                                            .p_1()
                                            .child(
                                                Input::new(&self.password_input)
                                                    .cleanable(true)
                                                    .mask_toggle(),
                                            ),
                                    ),
                            )
                            // Family Code Field
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(color_text_muted())
                                            .child("Family Code (Optional)"),
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .rounded_lg()
                                            .bg(color_surface())
                                            .border_1()
                                            .border_color(color_border())
                                            .p_1()
                                            .child(
                                                Input::new(&self.family_code_input)
                                                    .cleanable(true),
                                            ),
                                    ),
                            )
                            // Remember Me & Offline Storage Note
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .gap_2()
                                    .child({
                                        let v = view_entity.clone();
                                        Checkbox::new("remember-me")
                                            .label("Keep signed in on this device")
                                            .checked(self.remember_me)
                                            .on_click(move |checked, _window, cx| {
                                                let c = *checked;
                                                v.update(cx, |this, cx| {
                                                    this.remember_me = c;
                                                    cx.notify();
                                                });
                                            })
                                    })
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(color_text_dim())
                                            .child("Encrypted locally"),
                                    ),
                            ),
                    )
                    // Submit Sign In Button
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .pt_2()
                            .child({
                                let v = view_entity.clone();
                                Button::new("submit-sign-in")
                                    .label(if self.is_authenticating {
                                        "Authenticating..."
                                    } else {
                                        "Sign In to Umuryango"
                                    })
                                    .primary()
                                    .loading(self.is_authenticating)
                                    .on_click(move |_e, window, cx| {
                                        v.update(cx, |this, cx| {
                                            this.handle_submit(window, cx);
                                        });
                                    })
                            }),
                    )
                    // Bottom Trust Badge & Info
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .pt_2()
                            .border_t_1()
                            .border_color(color_border())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(color_text_dim())
                                    .child("🛡️ Local-first & offline resilient • Verified by Family ID"),
                            ),
                    ),
            )
    }
}
