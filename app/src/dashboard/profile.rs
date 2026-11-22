use super::*;
use crate::theme::UiStyle;

impl DashboardView {
    pub(super) fn render_profile_screen(
        &self,
        stacked: bool,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let lang = self.language;
        let next_lang = self.language.toggle();
        let view_lang = view_entity.clone();
        let view_sign_out = view_entity.clone();

        let role_subtitle = if self.member_role.is_empty() {
            text(lang, "cozy.coadmin_planner").to_string()
        } else {
            self.member_role.clone()
        };

        let identity_card =
            card_surface(palette)
                .gap_4()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_2()
                        .child(member_avatar(
                            &self.member_name,
                            Some(&role_subtitle),
                            AvatarSize::Xl,
                            palette,
                        ))
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded_full()
                                .bg(palette.income_bg)
                                .border(px(1.5))
                                .border_color(palette.income_border)
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.income_text)
                                .child(role_subtitle.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette.text_secondary)
                                .child(self.member_contact.clone()),
                        ),
                )
                .child(
                    card_elevated(palette)
                        .gap_3()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(text(lang, "cozy.theme_studio_title")),
                        )
                        // 1. Style Selection (Cozy Kawaii vs Minimalist)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1p5()
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child("Style Design System"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .children([UiStyle::CozyKawaii, UiStyle::Minimalist].into_iter().map(|st| {
                                            let is_selected = palette.style == st;
                                            let view_st = view_entity.clone();
                                            let (badge_icon, style_name) = match st {
                                                UiStyle::CozyKawaii => ("🧸", "Cozy Kawaii"),
                                                UiStyle::Minimalist => ("✨", "Minimalist"),
                                                _ => ("🎨", "Style"),
                                            };
                                            div()
                                                .id(match st {
                                                    UiStyle::CozyKawaii => "profile-style-cozy",
                                                    UiStyle::Minimalist => "profile-style-minimalist",
                                                    _ => "profile-style-other",
                                                })
                                                .flex_1()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .gap_1p5()
                                                .py_2()
                                                .px_2p5()
                                                .rounded(palette.radius_button)
                                                .border(palette.border_width_input)
                                                .cursor_pointer()
                                                .when(is_selected, |this| {
                                                    this.bg(palette.bg_input_focus)
                                                        .border_color(palette.accent_primary)
                                                        .shadow(palette.checkbox_shadows())
                                                        .text_color(palette.text_primary)
                                                        .font_weight(FontWeight::EXTRA_BOLD)
                                                })
                                                .when(!is_selected, |this| {
                                                    this.bg(palette.bg_card)
                                                        .border_color(palette.border_color)
                                                        .text_color(palette.text_secondary)
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .hover(|s| {
                                                            s.border_color(palette.accent_primary)
                                                                .text_color(palette.text_primary)
                                                        })
                                                })
                                                .child(div().text_size(px(13.0)).child(badge_icon))
                                                .child(div().text_xs().child(style_name))
                                                .when(is_selected, |this| {
                                                    this.child(div().text_xs().text_color(palette.accent_primary).child("✓"))
                                                })
                                                .on_click(move |_e, _w, cx| {
                                                    crate::theme::set_ui_style(st, cx);
                                                    view_st.update(cx, |this, cx| {
                                                        cx.emit(DashboardEvent::ThemeChanged(this.theme));
                                                        cx.notify();
                                                    });
                                                })
                                        }))
                                )
                        )
                        // 2. Appearance Theme (Light vs Dark)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1p5()
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_secondary)
                                        .child("Appearance Mode"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .children([ThemeMode::Light, ThemeMode::Dark].into_iter().map(|mode| {
                                            let is_selected = self.theme == mode;
                                            let view_th = view_entity.clone();
                                            let (mode_icon, mode_name) = match mode {
                                                ThemeMode::Light => ("☀️", "Light"),
                                                ThemeMode::Dark => ("🌙", "Dark"),
                                            };
                                            div()
                                                .id(match mode {
                                                    ThemeMode::Light => "profile-theme-light",
                                                    ThemeMode::Dark => "profile-theme-dark",
                                                })
                                                .flex_1()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .gap_1p5()
                                                .py_2()
                                                .px_2p5()
                                                .rounded(palette.radius_button)
                                                .border(palette.border_width_input)
                                                .cursor_pointer()
                                                .when(is_selected, |this| {
                                                    this.bg(palette.bg_input_focus)
                                                        .border_color(palette.accent_primary)
                                                        .shadow(palette.checkbox_shadows())
                                                        .text_color(palette.text_primary)
                                                        .font_weight(FontWeight::EXTRA_BOLD)
                                                })
                                                .when(!is_selected, |this| {
                                                    this.bg(palette.bg_card)
                                                        .border_color(palette.border_color)
                                                        .text_color(palette.text_secondary)
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .hover(|s| {
                                                            s.border_color(palette.accent_primary)
                                                                .text_color(palette.text_primary)
                                                        })
                                                })
                                                .child(div().text_size(px(13.0)).child(mode_icon))
                                                .child(div().text_xs().child(mode_name))
                                                .when(is_selected, |this| {
                                                    this.child(div().text_xs().text_color(palette.accent_primary).child("✓"))
                                                })
                                                .on_click(move |_e, _w, cx| {
                                                    view_th.update(cx, |this, cx| {
                                                        this.set_theme(mode, cx);
                                                        cx.emit(DashboardEvent::ThemeChanged(mode));
                                                    });
                                                })
                                        }))
                                )
                        )
                        // 3. Cozy Mood Swatches (only if style uses mood)
                        .when(palette.style.uses_mood(), |this| {
                            this.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(palette.text_secondary)
                                            .child("Cozy Mood Accent"),
                                    )
                                    .child(div().flex().items_center().gap_2().children(
                                        CozyMood::ALL.into_iter().map(|mood| {
                                            let is_active = self.mood == mood;
                                            let view_m = view_entity.clone();
                                            let btn_id = match mood {
                                                CozyMood::Matcha => "profile-mood-matcha",
                                                CozyMood::Peach => "profile-mood-peach",
                                                CozyMood::Berry => "profile-mood-berry",
                                            };
                                            div()
                                                .id(btn_id)
                                                .flex_1()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .gap_1p5()
                                                .py_1p5()
                                                .rounded(palette.radius_button)
                                                .border(palette.border_width_input)
                                                .border_color(if is_active {
                                                    palette.accent_primary
                                                } else {
                                                    palette.border_color
                                                })
                                                .bg(if is_active {
                                                    palette.bg_input_focus
                                                } else {
                                                    palette.bg_card
                                                })
                                                .text_xs()
                                                .font_weight(FontWeight::EXTRA_BOLD)
                                                .text_color(palette.text_primary)
                                                .cursor_pointer()
                                                .child(
                                                    div()
                                                        .w(px(10.0))
                                                        .h(px(10.0))
                                                        .rounded_full()
                                                        .bg(mood.swatch_color()),
                                                )
                                                .child(mood.label())
                                                .on_click(move |_e, _w, cx| {
                                                    view_m.update(cx, |this, cx| {
                                                        this.set_mood(mood, cx);
                                                    });
                                                })
                                        }),
                                    ))
                            )
                        })
                        .when(palette.is_minimalist(), |this| {
                            this.child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(palette.text_muted)
                                    .child("Default gpui-kit style active: clean shadcn neutral palette with Inter font and 6px rounded edges."),
                            )
                        })
                        .when(!palette.is_minimalist(), |this| {
                            this.child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(palette.text_muted)
                                    .child("Cozy Kawaii style active: friendly tactile pill buttons and playful palette accents."),
                            )
                        })
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_2()
                                .pt_1()
                                .child(secondary_button(
                                    "profile-switch-lang-btn",
                                    format!(
                                        "{}: {}",
                                        text(lang, "section.language_label"),
                                        self.language.label()
                                    ),
                                    palette,
                                    move |_e, w, cx| {
                                        view_lang.update(cx, |this, cx| {
                                            this.set_language(next_lang, w, cx);
                                            cx.emit(DashboardEvent::LanguageChanged(next_lang));
                                        });
                                    },
                                ))
                                .child(destructive_button(
                                    "profile-signout-btn",
                                    text(lang, "dashboard.sign_out"),
                                    palette,
                                    move |_e, _w, cx| {
                                        view_sign_out.update(cx, |_this, cx| {
                                            cx.emit(DashboardEvent::SignOutRequested);
                                        });
                                    },
                                )),
                        ),
                );

        let view_r1 = view_entity.clone();
        let view_r2 = view_entity.clone();
        let view_r3 = view_entity.clone();
        let r1 = self.rule_multi_sig_over_30k;
        let r2 = self.rule_sunday_digest;
        let r3 = self.rule_auto_roundup_momo;

        let governance_card = card_surface(palette)
            .gap_2p5()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(palette.text_primary)
                    .child(text(lang, "cozy.governance_rules_title")),
            )
            .child(card_elevated(palette).child(toggle_field(
                "rule-multisig-30k",
                text(lang, "cozy.rule_multisig_30k"),
                None::<&str>,
                r1,
                palette,
                move |_e, _w, cx| {
                    view_r1.update(cx, |this, cx| {
                        this.rule_multi_sig_over_30k = !this.rule_multi_sig_over_30k;
                        cx.notify();
                    });
                },
            )))
            .child(card_elevated(palette).child(toggle_field(
                "rule-sunday-digest",
                text(lang, "cozy.rule_sunday_digest"),
                None::<&str>,
                r2,
                palette,
                move |_e, _w, cx| {
                    view_r2.update(cx, |this, cx| {
                        this.rule_sunday_digest = !this.rule_sunday_digest;
                        cx.notify();
                    });
                },
            )))
            .child(card_elevated(palette).child(toggle_field(
                "rule-auto-roundup",
                text(lang, "cozy.rule_auto_roundup"),
                None::<&str>,
                r3,
                palette,
                move |_e, _w, cx| {
                    view_r3.update(cx, |this, cx| {
                        this.rule_auto_roundup_momo = !this.rule_auto_roundup_momo;
                        cx.notify();
                    });
                },
            )));

        let channels_card = card_surface(palette)
            .gap_2p5()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(palette.text_primary)
                    .child(text(lang, "cozy.connected_channels_title")),
            )
            .child(
                div()
                    .flex()
                    .gap_2p5()
                    .when(stacked, |this| this.flex_col())
                    .child(
                        card_elevated(palette)
                            .flex_1()
                            .items_center()
                            .gap_1()
                            .child(
                                icons::icon(icons::ICON_PHONE)
                                    .size(px(16.0))
                                    .text_color(palette.accent_gold),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child("MTN MoMo"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.income_text)
                                    .child(text(lang, "cozy.channel_linked")),
                            ),
                    )
                    .child(
                        card_elevated(palette)
                            .flex_1()
                            .items_center()
                            .gap_1()
                            .child(
                                icons::icon(icons::ICON_CREDIT_CARD)
                                    .size(px(16.0))
                                    .text_color(palette.expense_text),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child("Airtel Money"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.income_text)
                                    .child(text(lang, "cozy.channel_linked")),
                            ),
                    )
                    .child(
                        card_elevated(palette)
                            .flex_1()
                            .items_center()
                            .gap_1()
                            .child(
                                icons::icon(icons::ICON_BUILDING_COLUMNS)
                                    .size(px(16.0))
                                    .text_color(palette.accent_primary),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child("I&M Bank"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.income_text)
                                    .child(text(lang, "cozy.channel_treasury")),
                            ),
                    ),
            );

        let view_sync = view_entity.clone();
        let storage_card = card_surface(palette)
            .child(card_header(
                text(lang, "dashboard.storage_health"),
                Some(text(lang, "cozy.storage_sub")),
                palette,
            ))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                icons::icon(icons::ICON_CIRCLE_CHECK)
                                    .size(px(14.0))
                                    .text_color(palette.income_text),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_primary)
                                    .child(self.storage_sync_status.clone()),
                            ),
                    )
                    .child(pill_button(
                        "profile-sync-now-btn",
                        text(lang, "dashboard.sync_now"),
                        palette,
                        move |_e, _w, cx| {
                            view_sync.update(cx, |_this, cx| {
                                cx.emit(DashboardEvent::SyncRequested);
                            });
                        },
                    )),
            );

        let view_check_update = view_entity.clone();
        let view_install_update = view_entity;
        let update_card = app_update_card(
            &self.update_state,
            lang,
            palette,
            move |_e, _w, cx| {
                view_check_update.update(cx, |this, cx| {
                    this.check_for_app_update(true, cx);
                });
            },
            move |_e, _w, cx| {
                view_install_update.update(cx, |this, cx| {
                    this.install_app_update(cx);
                });
            },
        );

        div()
            .flex()
            .gap_4()
            .when(stacked, |this| this.flex_col())
            .child(div().flex_1().min_w_0().child(identity_card))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(governance_card)
                    .child(channels_card)
                    .child(storage_card)
                    .child(update_card),
            )
            .into_any_element()
    }
}
