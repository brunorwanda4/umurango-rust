#![allow(dead_code)]

use crate::auth::client::api_create_family;
use crate::auth::layout::auth_layout;
use crate::brand::{apply_theme, colors, ThemeMode};
use crate::components::input::{input_field, text_input};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::base::Disableable;
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    input::InputState,
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

#[derive(Clone, Debug)]
pub enum CreateFamilyEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    BackRequested,
    FamilyCreated {
        id: String,
        name: String,
        code: String,
    },
}

pub struct CreateFamilyView {
    name_input: Entity<InputState>,
    phone_input: Entity<InputState>,
    member_phones: Vec<String>,
    error_message: Option<String>,
    success_message: Option<String>,
    is_creating: bool,
    theme: ThemeMode,
    language: Language,
}

impl EventEmitter<CreateFamilyEvent> for CreateFamilyView {}

impl CreateFamilyView {
    pub fn new(
        window: &mut Window,
        theme: ThemeMode,
        language: Language,
        cx: &mut Context<Self>,
    ) -> Self {
        apply_theme(theme, cx);

        let name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "create_family.name_placeholder"))
        });

        let phone_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder(text(language, "create_family.phone_placeholder"))
        });

        Self {
            name_input,
            phone_input,
            member_phones: Vec::new(),
            error_message: None,
            success_message: None,
            is_creating: false,
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
        let name_placeholder = text(language, "create_family.name_placeholder");
        let phone_placeholder = text(language, "create_family.phone_placeholder");

        self.name_input.update(cx, |input, cx| {
            input.set_placeholder(name_placeholder, window, cx);
        });
        self.phone_input.update(cx, |input, cx| {
            input.set_placeholder(phone_placeholder, window, cx);
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

    fn add_phone(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let phone = self
            .phone_input
            .read(cx)
            .text()
            .to_string()
            .trim()
            .to_string();

        if phone.is_empty() {
            return;
        }

        if !self.member_phones.contains(&phone) {
            self.member_phones.push(phone);
        }

        self.phone_input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });

        self.error_message = None;
        cx.notify();
    }

    fn remove_phone(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.member_phones.len() {
            self.member_phones.remove(index);
            cx.notify();
        }
    }

    fn handle_submit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let name = self
            .name_input
            .read(cx)
            .text()
            .to_string()
            .trim()
            .to_string();

        if name.is_empty() {
            self.error_message = Some(text(self.language, "create_family.name_required"));
            cx.notify();
            return;
        }

        self.is_creating = true;
        self.error_message = None;
        cx.notify();

        let phones = self.member_phones.clone();

        match api_create_family(&name, phones) {
            Ok(resp) => {
                self.is_creating = false;
                let code = resp.code.clone();
                let success_msg =
                    format!("{}{}", text(self.language, "create_family.success"), code);
                self.success_message = Some(success_msg);
                cx.notify();

                cx.emit(CreateFamilyEvent::FamilyCreated {
                    id: resp.id,
                    name: resp.name,
                    code: resp.code,
                });
            }
            Err(err) => {
                self.is_creating = false;
                self.error_message = Some(err);
                cx.notify();
            }
        }
    }
}

impl Render for CreateFamilyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_entity = cx.entity().clone();
        let palette = colors(self.theme);

        let language_change = {
            let view = view_entity.clone();
            move |language: Language, window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    if this.language != language {
                        this.set_language(language, window, cx);
                        cx.emit(CreateFamilyEvent::LanguageChanged(language));
                    }
                });
            }
        };

        let theme_change = {
            let view = view_entity.clone();
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(theme, cx);
                    cx.emit(CreateFamilyEvent::ThemeChanged(theme));
                });
            }
        };

        let content = div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_xs()
                    .text_color(palette.text_muted)
                    .child(text(self.language, "create_family.subtitle")),
            )
            .child(input_field(
                palette,
                text(self.language, "create_family.name_label"),
                &self.name_input,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_body)
                            .child(text(self.language, "create_family.members_label")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .flex_1()
                                    .rounded_lg()
                                    .bg(palette.surface)
                                    .p_1()
                                    .child(text_input(&self.phone_input, palette)),
                            )
                            .child({
                                let view = view_entity.clone();
                                Button::new("add-member-btn")
                                    .label(text(self.language, "create_family.add_member"))
                                    .on_click(move |_event, window, cx| {
                                        view.update(cx, |this, cx| {
                                            this.add_phone(window, cx);
                                        });
                                    })
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .when(self.member_phones.is_empty(), |this| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .italic()
                                .child(text(self.language, "create_family.no_members")),
                        )
                    })
                    .children(self.member_phones.iter().enumerate().map(|(idx, phone)| {
                        let phone_str = phone.clone();
                        let view = view_entity.clone();
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_3()
                            .py_1p5()
                            .rounded_md()
                            .bg(palette.surface)
                            .border_1()
                            .border_color(palette.border)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        icons::icon(icons::ICON_PHONE)
                                            .size(px(12.))
                                            .text_color(palette.text_muted),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(palette.text_primary)
                                            .child(phone_str),
                                    ),
                            )
                            .child(
                                div()
                                    .id(("remove-phone", idx))
                                    .cursor_pointer()
                                    .p_1()
                                    .child(
                                        icons::icon(icons::ICON_XMARK)
                                            .size(px(12.))
                                            .text_color(palette.error),
                                    )
                                    .on_click(move |_event, _window, cx| {
                                        view.update(cx, |this, cx| {
                                            this.remove_phone(idx, cx);
                                        });
                                    }),
                            )
                    })),
            )
            .when_some(self.error_message.as_ref(), |this, msg| {
                this.child(
                    div()
                        .w_full()
                        .p_3()
                        .rounded_lg()
                        .border_1()
                        .border_color(palette.error)
                        .bg(palette.surface)
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
                        .rounded_lg()
                        .border_1()
                        .border_color(palette.success)
                        .bg(palette.surface)
                        .text_color(palette.success)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .child(msg.clone()),
                )
            })
            .child({
                let view = view_entity.clone();
                let submit_label = if self.is_creating {
                    text(self.language, "create_family.submitting")
                } else {
                    text(self.language, "create_family.submit")
                };

                Button::new("submit-create-family")
                    .primary()
                    .label(submit_label)
                    .disabled(self.is_creating)
                    .on_click(move |_event, window, cx| {
                        view.update(cx, |this, cx| {
                            this.handle_submit(window, cx);
                        });
                    })
            })
            .child({
                let view = view_entity;
                Button::new("back-family-setup")
                    .label(text(self.language, "create_family.back"))
                    .on_click(move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(CreateFamilyEvent::BackRequested);
                        });
                    })
            });

        auth_layout(
            self.theme,
            self.language,
            text(self.language, "create_family.title"),
            language_change,
            theme_change,
            content,
        )
    }
}
