use super::*;

impl DashboardView {
    pub(super) fn update_entry_modal_placeholders(
        &mut self,
        lang: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(kind) = self.active_entry_modal else {
            return;
        };
        let (title_ph, sec_ph, amt_ph) = match kind {
            EntryModalKind::Income => (
                text(lang, "modal.income_desc_ph"),
                text(lang, "modal.income_cat_ph"),
                text(lang, "modal.amount_rwf_ph"),
            ),
            EntryModalKind::Expense => (
                text(lang, "modal.expense_desc_ph"),
                text(lang, "modal.expense_cat_ph"),
                text(lang, "modal.amount_rwf_ph"),
            ),
            EntryModalKind::Loan => (
                text(lang, "modal.loan_reason_ph"),
                text(lang, "modal.loan_person_ph"),
                text(lang, "modal.loan_amount_rwf_ph"),
            ),
            EntryModalKind::Project => (
                text(lang, "modal.project_name_ph"),
                text(lang, "modal.project_cat_ph"),
                text(lang, "modal.target_amount_rwf_ph"),
            ),
            EntryModalKind::MemberInvite => (
                text(lang, "modal.member_name_ph"),
                text(lang, "modal.member_phone_ph"),
                text(lang, "modal.member_role_ph"),
            ),
        };
        self.entry_title_input.update(cx, |state, cx| {
            state.set_placeholder(title_ph, window, cx);
        });
        self.entry_secondary_input.update(cx, |state, cx| {
            state.set_placeholder(sec_ph, window, cx);
        });
        self.entry_amount_input.update(cx, |state, cx| {
            state.set_placeholder(amt_ph, window, cx);
        });
    }


    pub fn open_entry_modal(
        &mut self,
        kind: EntryModalKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let lang = self.language;
        let (title_ph, title_val, sec_ph, sec_val, amt_ph, amt_val) = match kind {
            EntryModalKind::Income => (
                text(lang, "modal.income_desc_ph"),
                text(lang, "modal.income_desc_val"),
                text(lang, "modal.income_cat_ph"),
                text(lang, "modal.income_cat_val"),
                text(lang, "modal.amount_rwf_ph"),
                "50000".to_string(),
            ),
            EntryModalKind::Expense => (
                text(lang, "modal.expense_desc_ph"),
                text(lang, "modal.expense_desc_val"),
                text(lang, "modal.expense_cat_ph"),
                text(lang, "modal.expense_cat_val"),
                text(lang, "modal.amount_rwf_ph"),
                "35000".to_string(),
            ),
            EntryModalKind::Loan => (
                text(lang, "modal.loan_reason_ph"),
                text(lang, "modal.loan_reason_val"),
                text(lang, "modal.loan_person_ph"),
                text(lang, "modal.loan_person_val"),
                text(lang, "modal.loan_amount_rwf_ph"),
                "120000".to_string(),
            ),
            EntryModalKind::Project => (
                text(lang, "modal.project_name_ph"),
                text(lang, "modal.project_name_val"),
                text(lang, "modal.project_cat_ph"),
                "HOUSE".to_string(),
                text(lang, "modal.target_amount_rwf_ph"),
                "1200000".to_string(),
            ),
            EntryModalKind::MemberInvite => (
                text(lang, "modal.member_name_ph"),
                "".to_string(),
                text(lang, "modal.member_phone_ph"),
                "+25078".to_string(),
                text(lang, "modal.member_role_ph"),
                "MEMBER".to_string(),
            ),
        };

        self.entry_title_input.update(cx, |state, cx| {
            state.set_placeholder(title_ph, window, cx);
            state.set_value(title_val, window, cx);
        });
        self.entry_secondary_input.update(cx, |state, cx| {
            state.set_placeholder(sec_ph, window, cx);
            state.set_value(sec_val, window, cx);
        });
        self.entry_amount_input.update(cx, |state, cx| {
            state.set_placeholder(amt_ph, window, cx);
            state.set_value(amt_val, window, cx);
        });
        self.active_entry_modal = Some(kind);
        cx.notify();
    }

    pub fn submit_entry_modal(&mut self, cx: &mut Context<Self>) {
        let Some(kind) = self.active_entry_modal else {
            return;
        };
        let title_str = self.entry_title_input.read(cx).value().trim().to_string();
        let secondary_str = self
            .entry_secondary_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let amount_raw = self.entry_amount_input.read(cx).value().trim().to_string();
        let parsed_amount: i64 = amount_raw
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i64>()
            .unwrap_or(0);

        let Some(fid) = crate::auth::client::get_current_family_id() else {
            self.toast_notice = Some("No active family workspace selected.".to_string());
            self.active_entry_modal = None;
            cx.notify();
            return;
        };

        match kind {
            EntryModalKind::Income | EntryModalKind::Expense => {
                let is_income = kind == EntryModalKind::Income;
                let amount = if parsed_amount > 0 {
                    parsed_amount
                } else {
                    50_000
                };
                let description = if title_str.is_empty() {
                    if is_income {
                        "Family Contribution".to_string()
                    } else {
                        "Family Expense".to_string()
                    }
                } else {
                    title_str
                };
                let category = if secondary_str.is_empty() {
                    if is_income {
                        "Contribution".to_string()
                    } else {
                        "General".to_string()
                    }
                } else {
                    secondary_str
                };

                let req = umuryango_shared::CreateTransactionRequest {
                    amount_minor_units: amount,
                    is_income,
                    category,
                    description: description.clone(),
                    notes: None,
                    idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
                };

                match crate::auth::client::api_create_transaction(&fid, &req) {
                    Ok(_) => {
                        self.toast_notice = Some(format!(
                            "{} ({})",
                            description,
                            format_signed_rwf(amount, is_income)
                        ));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        if let Ok(payload) = serde_json::to_string(&req) {
                            let command = if is_income {
                                "CREATE_CONTRIBUTION".to_string()
                            } else {
                                "CREATE_EXPENSE".to_string()
                            };
                            cx.emit(DashboardEvent::QueueOutboxMutation { command, payload });
                        }
                        self.toast_notice = Some(format!("Queued offline ({err})"));
                        self.active_entry_modal = None;
                        cx.notify();
                    }
                }
            }
            EntryModalKind::Project => {
                let target = if parsed_amount > 0 {
                    parsed_amount
                } else {
                    1_000_000
                };
                let name = if title_str.is_empty() {
                    "Family Project".to_string()
                } else {
                    title_str
                };
                let category = if secondary_str.is_empty() {
                    "OTHER".to_string()
                } else {
                    secondary_str
                };
                let req = umuryango_shared::CreateProjectRequest {
                    name: name.clone(),
                    category,
                    target_amount_minor_units: target,
                    description: Some(self.member_name.clone()),
                    target_date: None,
                };
                match crate::auth::client::api_create_project(&fid, &req) {
                    Ok(_) => {
                        self.toast_notice = Some(format!("Created project: {name}"));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        self.toast_notice = Some(err);
                        cx.notify();
                    }
                }
            }
            EntryModalKind::Loan => {
                let total = if parsed_amount > 0 {
                    parsed_amount
                } else {
                    100_000
                };
                let title = if title_str.is_empty() {
                    "Family Bridge Loan".to_string()
                } else {
                    title_str
                };
                let counterparty_name = if secondary_str.is_empty() {
                    "Family Contact".to_string()
                } else {
                    secondary_str
                };
                let req = umuryango_shared::CreateLoanRequest {
                    title,
                    counterparty_name: counterparty_name.clone(),
                    is_lent: self.entry_loan_is_lent,
                    total_amount_minor_units: total,
                    paid_amount_minor_units: 0,
                    due_date: None,
                };
                match crate::auth::client::api_create_loan(&fid, &req) {
                    Ok(_) => {
                        self.toast_notice = Some(format!(
                            "{} · {counterparty_name} ({})",
                            text(self.language, "cozy.toast_new_loan"),
                            format_rwf(total)
                        ));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        self.toast_notice = Some(err);
                        cx.notify();
                    }
                }
            }
            EntryModalKind::MemberInvite => {
                if secondary_str.is_empty() {
                    self.toast_notice = Some("Phone number is required.".to_string());
                    cx.notify();
                    return;
                }
                let req = umuryango_shared::InviteFamilyMemberRequest {
                    phone: secondary_str,
                    full_name: if title_str.is_empty() {
                        None
                    } else {
                        Some(title_str)
                    },
                    role: if amount_raw.is_empty() {
                        None
                    } else {
                        Some(amount_raw)
                    },
                };
                match crate::auth::client::api_invite_member(&fid, &req) {
                    Ok(member) => {
                        self.toast_notice = Some(format!(
                            "{} ({})",
                            text(self.language, "cozy.toast_invited"),
                            member.full_name
                        ));
                        self.active_entry_modal = None;
                        self.refresh_from_backend(cx);
                    }
                    Err(err) => {
                        self.toast_notice = Some(err);
                        cx.notify();
                    }
                }
            }
        }
    }


    pub(super) fn render_entry_modal(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> Option<AnyElement> {
        let kind = self.active_entry_modal?;
        let lang = self.language;
        let (modal_icon, modal_title, field1_label, field2_label, field3_label) = match kind {
            EntryModalKind::Income => (
                icons::ICON_HAND_HOLDING_DOLLAR,
                text(lang, "modal.income_title"),
                text(lang, "modal.field_desc_title"),
                text(lang, "modal.field_category_income"),
                text(lang, "modal.field_amount_rwf"),
            ),
            EntryModalKind::Expense => (
                icons::ICON_CREDIT_CARD,
                text(lang, "modal.expense_title"),
                text(lang, "modal.field_desc_purpose"),
                text(lang, "modal.field_category_expense"),
                text(lang, "modal.field_amount_rwf"),
            ),
            EntryModalKind::Loan => (
                icons::ICON_HANDSHAKE,
                text(lang, "modal.loan_title"),
                text(lang, "modal.field_loan_reason"),
                text(lang, "modal.field_counterparty"),
                text(lang, "modal.field_loan_principal"),
            ),
            EntryModalKind::Project => (
                icons::ICON_BULLSEYE,
                text(lang, "modal.project_title"),
                text(lang, "modal.field_project_name"),
                text(lang, "modal.field_category_project"),
                text(lang, "modal.field_target_amount"),
            ),
            EntryModalKind::MemberInvite => (
                icons::ICON_USER_PLUS,
                text(lang, "modal.invite_title"),
                text(lang, "modal.field_full_name"),
                text(lang, "modal.field_phone_number"),
                text(lang, "modal.field_role"),
            ),
        };

        let view_close_bg = view_entity.clone();
        let view_close = view_entity.clone();
        let view_close_footer = view_entity.clone();
        let view_submit = view_entity.clone();
        let view_toggle_lent = view_entity.clone();
        let is_lent = self.entry_loan_is_lent;

        let lent_label = if is_lent {
            format!("✓ {}", text(lang, "modal.loan_lent_out"))
        } else {
            text(lang, "modal.loan_lent_out")
        };

        let borrowed_label = if !is_lent {
            format!("✓ {}", text(lang, "modal.loan_borrowed"))
        } else {
            text(lang, "modal.loan_borrowed")
        };

        let cancel_text = text(lang, "modal.cancel");
        let submit_text = text(lang, "modal.save");

        let card = div()
            .id("entry-modal-card")
            .w(px(540.0))
            .max_w_full()
            .p_6()
            .rounded(palette.radius_card)
            .bg(palette.bg_card)
            .border(px(2.0))
            .border_color(palette.border_color)
            .shadow(palette.card_shadows())
            .text_color(palette.text_primary)
            .flex()
            .flex_col()
            .gap_4()
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .pb_2()
                    .border_b(palette.border_width_input)
                    .border_color(palette.border_color)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                icons::icon(modal_icon)
                                    .size(px(16.0))
                                    .text_color(palette.accent_primary),
                            )
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child(modal_title),
                            ),
                    )
                    .child(
                        div()
                            .id("entry-modal-close-icon")
                            .cursor_pointer()
                            .p_1()
                            .child(
                                icons::icon(icons::ICON_XMARK)
                                    .size(px(14.0))
                                    .text_color(palette.text_secondary),
                            )
                            .on_click(move |_e, _w, cx| {
                                view_close.update(cx, |this, cx| {
                                    this.active_entry_modal = None;
                                    cx.notify();
                                });
                            }),
                    ),
            )
            .when(kind == EntryModalKind::Loan, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(pill_button_colored(
                            "entry-loan-type-lent",
                            lent_label,
                            if is_lent {
                                palette.income_text
                            } else {
                                palette.text_secondary
                            },
                            palette,
                            {
                                let vt = view_toggle_lent.clone();
                                move |_e, _w, cx| {
                                    vt.update(cx, |this, cx| {
                                        this.entry_loan_is_lent = true;
                                        cx.notify();
                                    });
                                }
                            },
                        ))
                        .child(pill_button_colored(
                            "entry-loan-type-borrowed",
                            borrowed_label,
                            if !is_lent {
                                palette.loan_text
                            } else {
                                palette.text_secondary
                            },
                            palette,
                            move |_e, _w, cx| {
                                view_toggle_lent.update(cx, |this, cx| {
                                    this.entry_loan_is_lent = false;
                                    cx.notify();
                                });
                            },
                        )),
                )
            })
            .child(
                div()
                    .flex()
                    .gap_3()
                    .flex_wrap()
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(200.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(field1_label),
                            )
                            .child(stateful_input_container(
                                palette,
                                &self.entry_title_input,
                                text_input(&self.entry_title_input, palette),
                            )),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(180.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(field2_label),
                            )
                            .child(stateful_input_container(
                                palette,
                                &self.entry_secondary_input,
                                text_input(&self.entry_secondary_input, palette),
                            )),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(150.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_secondary)
                                    .child(field3_label),
                            )
                            .child(stateful_input_container(
                                palette,
                                &self.entry_amount_input,
                                text_input(&self.entry_amount_input, palette),
                            )),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_3()
                    .pt_2()
                    .border_t(palette.border_width_input)
                    .border_color(palette.border_color)
                    .child(pill_button(
                        "entry-modal-cancel-btn",
                        cancel_text,
                        palette,
                        move |_e, _w, cx| {
                            view_close_footer.update(cx, |this, cx| {
                                this.active_entry_modal = None;
                                cx.notify();
                            });
                        },
                    ))
                    .child(compact_primary_button(
                        "entry-modal-submit-btn",
                        submit_text,
                        palette,
                        move |_e, _w, cx| {
                            view_submit.update(cx, |this, cx| {
                                this.submit_entry_modal(cx);
                            });
                        },
                    )),
            );

        let backdrop = div()
            .id("entry-modal-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(palette.bg_canvas.opacity(0.85))
            .p_4()
            .on_mouse_down(MouseButton::Left, move |_e, _w, cx| {
                view_close_bg.update(cx, |this, cx| {
                    this.active_entry_modal = None;
                    cx.notify();
                });
            })
            .child(card);

        Some(backdrop.into_any_element())
    }
}
