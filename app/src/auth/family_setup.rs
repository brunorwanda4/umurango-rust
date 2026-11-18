#![allow(dead_code)]

use crate::auth::client::{api_accept_invitation, api_get_pending_invitations};
use crate::auth::layout::{auth_footer_prompt, auth_layout};
use crate::brand::{apply_theme, colors, ThemeColors, ThemeMode};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use umuryango_shared::PendingFamilyInviteDto;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FamilySetupChoice {
    CreateFamily,
    JoinFamily,
    Skip,
}

#[derive(Clone, Debug)]
pub enum FamilySetupEvent {
    ThemeChanged(ThemeMode),
    LanguageChanged(Language),
    ChoiceSelected(FamilySetupChoice),
    CreateFamilyRequested,
    JoinFamilyRequested,
    SkipRequested,
    LoginRequested,
    FamilyJoined {
        id: String,
        name: String,
        role: String,
    },
}

pub struct FamilySetupView {
    member_name: String,
    selected_choice: Option<FamilySetupChoice>,
    pending_invites: Vec<PendingFamilyInviteDto>,
    error_message: Option<String>,
    success_message: Option<String>,
    theme: ThemeMode,
    language: Language,
}

impl EventEmitter<FamilySetupEvent> for FamilySetupView {}

impl FamilySetupView {
    pub fn new(theme: ThemeMode, language: Language, cx: &mut Context<Self>) -> Self {
        apply_theme(theme, cx);
        Self {
            member_name: String::new(),
            selected_choice: None,
            pending_invites: Vec::new(),
            error_message: None,
            success_message: None,
            theme,
            language,
        }
    }

    pub fn set_member_name(&mut self, member_name: String, cx: &mut Context<Self>) {
        self.member_name = member_name;
        self.selected_choice = None;
        self.refresh_invitations(cx);
        cx.notify();
    }

    pub fn refresh_invitations(&mut self, cx: &mut Context<Self>) {
        if let Ok(invites) = api_get_pending_invitations() {
            self.pending_invites = invites;
            cx.notify();
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

    fn select_choice(&mut self, choice: FamilySetupChoice, cx: &mut Context<Self>) {
        self.selected_choice = Some(choice);
        match choice {
            FamilySetupChoice::CreateFamily => {
                cx.emit(FamilySetupEvent::CreateFamilyRequested);
            }
            FamilySetupChoice::JoinFamily => {
                cx.emit(FamilySetupEvent::JoinFamilyRequested);
            }
            FamilySetupChoice::Skip => {
                cx.emit(FamilySetupEvent::SkipRequested);
            }
        }
        cx.emit(FamilySetupEvent::ChoiceSelected(choice));
        cx.notify();
    }

    fn handle_accept_invite(&mut self, invite_id: &str, cx: &mut Context<Self>) {
        match api_accept_invitation(invite_id) {
            Ok(resp) => {
                self.success_message = Some(format!("Joined {}!", resp.family_name));
                cx.emit(FamilySetupEvent::FamilyJoined {
                    id: resp.family_id,
                    name: resp.family_name,
                    role: resp.role,
                });
                self.refresh_invitations(cx);
            }
            Err(err) => {
                self.error_message = Some(err);
                cx.notify();
            }
        }
    }
}

impl Render for FamilySetupView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_entity = cx.entity().clone();
        let palette = colors(self.theme);

        let language_change = {
            let view = view_entity.clone();
            move |language: Language, _window: &mut Window, cx: &mut App| {
                view.update(cx, |this, cx| {
                    if this.language != language {
                        this.set_language(language, cx);
                        cx.emit(FamilySetupEvent::LanguageChanged(language));
                    }
                });
            }
        };

        let theme_change = {
            let view = view_entity.clone();
            move |theme: ThemeMode, cx: &mut App| {
                view.update(cx, |this, cx| {
                    this.set_theme(theme, cx);
                    cx.emit(FamilySetupEvent::ThemeChanged(theme));
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
                    .child(text(self.language, "family_setup.subtitle")),
            )
            .when(!self.pending_invites.is_empty(), |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(palette.primary)
                                .child(text(self.language, "family_setup.invited_families_title")),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(text(self.language, "family_setup.invited_families_desc")),
                        )
                        .children(
                            self.pending_invites
                                .iter()
                                .enumerate()
                                .map(|(idx, invite)| {
                                    let inv_id = invite.invite_id.clone();
                                    let fam_name = invite.family_name.clone();
                                    let role = invite.role.clone();
                                    let view = view_entity.clone();
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(palette.surface)
                                        .border_1()
                                        .border_color(palette.primary)
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_3()
                                                .child(
                                                    icons::icon(icons::ICON_USERS)
                                                        .size(px(16.))
                                                        .text_color(palette.primary_light),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(palette.text_primary)
                                                                .child(fam_name),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(palette.text_muted)
                                                                .child(format!("Role: {role}")),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            Button::new(("accept-inv", idx))
                                                .primary()
                                                .label(text(
                                                    self.language,
                                                    "family_setup.accept_invite",
                                                ))
                                                .on_click(move |_event, _window, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.handle_accept_invite(&inv_id, cx);
                                                    });
                                                }),
                                        )
                                }),
                        ),
                )
            })
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
                        .child(msg.clone()),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child({
                        let view = view_entity.clone();
                        family_option_card(
                            "family-setup-create",
                            palette,
                            icons::ICON_USERS,
                            text(self.language, "family_setup.create_title"),
                            text(self.language, "family_setup.create_desc"),
                            self.selected_choice == Some(FamilySetupChoice::CreateFamily),
                            move |_event, _window, cx| {
                                view.update(cx, |this, cx| {
                                    this.select_choice(FamilySetupChoice::CreateFamily, cx);
                                });
                            },
                        )
                    })
                    .child({
                        let view = view_entity.clone();
                        family_option_card(
                            "family-setup-join",
                            palette,
                            icons::ICON_USER_PLUS,
                            text(self.language, "family_setup.join_title"),
                            text(self.language, "family_setup.join_desc"),
                            self.selected_choice == Some(FamilySetupChoice::JoinFamily),
                            move |_event, _window, cx| {
                                view.update(cx, |this, cx| {
                                    this.select_choice(FamilySetupChoice::JoinFamily, cx);
                                });
                            },
                        )
                    }),
            )
            .child({
                let view = view_entity.clone();
                Button::new("family-setup-skip")
                    .label(text(self.language, "family_setup.skip"))
                    .on_click(move |_event, _window, cx| {
                        view.update(cx, |this, cx| {
                            this.select_choice(FamilySetupChoice::Skip, cx);
                        });
                    })
            })
            .child({
                let view = view_entity;
                auth_footer_prompt(
                    "family-setup-login-link",
                    palette,
                    text(self.language, "register.have_account"),
                    text(self.language, "register.sign_in"),
                    move |_event, _window, cx| {
                        view.update(cx, |_this, cx| {
                            cx.emit(FamilySetupEvent::LoginRequested);
                        });
                    },
                )
            });

        auth_layout(
            self.theme,
            self.language,
            text(self.language, "family_setup.title"),
            language_change,
            theme_change,
            content,
        )
    }
}

fn family_option_card(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    icon_name: &'static str,
    title: String,
    description: String,
    selected: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .p_4()
        .rounded_lg()
        .border_1()
        .border_color(if selected {
            palette.primary
        } else {
            palette.border
        })
        .bg(palette.surface)
        .cursor_pointer()
        .hover(|style| style.border_color(palette.border_hover))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .w(px(36.))
                        .h(px(36.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_lg()
                        .bg(palette.card)
                        .border_1()
                        .border_color(palette.border)
                        .child(
                            icons::icon(icon_name)
                                .size(px(16.))
                                .text_color(palette.primary_light),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(palette.text_primary)
                                .child(title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_muted)
                                .child(description),
                        ),
                ),
        )
        .child(
            icons::icon(icons::ICON_ARROW_RIGHT)
                .size(px(14.))
                .text_color(palette.text_muted),
        )
        .on_click(on_click)
}
