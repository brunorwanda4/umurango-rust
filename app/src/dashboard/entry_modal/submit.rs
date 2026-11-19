use crate::dashboard::{
    format_rwf, format_signed_rwf, CustomCategoryItem, DashboardEvent, DashboardView,
    EntryModalKind,
};
use crate::i18n::text;
use crate::icons;
use gpui_kit::*;

impl DashboardView {
    pub fn submit_entry_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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

        if kind == EntryModalKind::Category {
            self.submit_category_modal(title_str, window, cx);
            return;
        }

        let Some(fid) = crate::auth::client::get_current_family_id() else {
            self.toast_notice = Some("No active family workspace selected.".to_string());
            self.active_entry_modal = None;
            cx.notify();
            return;
        };

        match kind {
            EntryModalKind::Income | EntryModalKind::Expense => {
                self.submit_transaction_modal(
                    &fid,
                    kind == EntryModalKind::Income,
                    parsed_amount,
                    title_str,
                    secondary_str,
                    cx,
                );
            }
            EntryModalKind::Project => {
                self.submit_project_modal(&fid, parsed_amount, title_str, secondary_str, cx);
            }
            EntryModalKind::Loan => {
                self.submit_loan_modal(&fid, parsed_amount, title_str, secondary_str, cx);
            }
            EntryModalKind::MemberInvite => {
                self.submit_member_invite_modal(&fid, title_str, secondary_str, amount_raw, cx);
            }
            EntryModalKind::Category => {}
        }
    }

    fn submit_category_modal(
        &mut self,
        title_str: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = if title_str.is_empty() {
            if self.entry_category_is_income {
                "Contribution".to_string()
            } else {
                "General".to_string()
            }
        } else {
            title_str
        };

        let typed_icon = self
            .entry_category_icon_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let final_icon = if !typed_icon.is_empty() && icons::has_icon(&typed_icon) {
            typed_icon
        } else {
            self.entry_category_icon.clone()
        };

        let desc = self
            .entry_category_desc_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let is_inc = self.entry_category_is_income;

        let typed_color = self
            .entry_category_color_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let color = if !typed_color.is_empty() {
            if typed_color.starts_with('#') {
                typed_color
            } else {
                format!("#{typed_color}")
            }
        } else {
            self.entry_category_color.clone()
        };

        let item = CustomCategoryItem {
            name: name.clone(),
            is_income: is_inc,
            icon: final_icon,
            color,
            description: desc,
        };

        if let Some(pos) = self
            .custom_categories
            .iter()
            .position(|c| c.name == name && c.is_income == is_inc)
        {
            self.custom_categories[pos] = item;
        } else {
            self.custom_categories.push(item);
        }

        if let Some(return_kind) = self.entry_category_return_modal.take() {
            self.active_entry_modal = Some(return_kind);
            let title = self.saved_tx_title.clone();
            let amt = self.saved_tx_amount.clone();
            let lang = self.language;
            let (t_ph, s_ph, a_ph) = match return_kind {
                EntryModalKind::Income => (
                    text(lang, "modal.income_desc_ph"),
                    text(lang, "modal.income_cat_ph"),
                    text(lang, "modal.amount_rwf_ph"),
                ),
                _ => (
                    text(lang, "modal.expense_desc_ph"),
                    text(lang, "modal.expense_cat_ph"),
                    text(lang, "modal.amount_rwf_ph"),
                ),
            };
            self.entry_title_input.update(cx, |state, cx| {
                state.set_placeholder(t_ph, window, cx);
                state.set_value(title, window, cx);
            });
            self.entry_secondary_input.update(cx, |state, cx| {
                state.set_placeholder(s_ph, window, cx);
                state.set_value(name.clone(), window, cx);
            });
            self.entry_amount_input.update(cx, |state, cx| {
                state.set_placeholder(a_ph, window, cx);
                state.set_value(amt, window, cx);
            });
            let is_income = return_kind == EntryModalKind::Income;
            let available = self.get_available_categories(is_income);
            self.entry_category_select.update(cx, |sel, cx| {
                sel.set_items(available, window, cx);
                sel.set_selected_value(&name, window, cx);
            });
            self.toast_notice = Some(format!(
                "{} ✓ {name}",
                text(self.language, "modal.category_saved_toast")
            ));
            cx.notify();
        } else {
            self.active_entry_modal = None;
            self.toast_notice = Some(format!(
                "{} ✓ {name}",
                text(self.language, "modal.category_saved_toast")
            ));
            cx.notify();
        }
    }

    fn submit_transaction_modal(
        &mut self,
        fid: &str,
        is_income: bool,
        parsed_amount: i64,
        title_str: String,
        secondary_str: String,
        cx: &mut Context<Self>,
    ) {
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
        let selected_cat = self
            .entry_category_select
            .read(cx)
            .selected_value()
            .cloned();
        let category = match selected_cat {
            Some(cat) if !cat.trim().is_empty() => cat,
            _ if !secondary_str.is_empty() => secondary_str,
            _ if is_income => "Contribution".to_string(),
            _ => "General".to_string(),
        };

        let req = umuryango_shared::CreateTransactionRequest {
            amount_minor_units: amount,
            is_income,
            category,
            description: description.clone(),
            notes: None,
            idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
        };

        match crate::auth::client::api_create_transaction(fid, &req) {
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

    fn submit_project_modal(
        &mut self,
        fid: &str,
        parsed_amount: i64,
        title_str: String,
        secondary_str: String,
        cx: &mut Context<Self>,
    ) {
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
        match crate::auth::client::api_create_project(fid, &req) {
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

    fn submit_loan_modal(
        &mut self,
        fid: &str,
        parsed_amount: i64,
        title_str: String,
        secondary_str: String,
        cx: &mut Context<Self>,
    ) {
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
        match crate::auth::client::api_create_loan(fid, &req) {
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

    fn submit_member_invite_modal(
        &mut self,
        fid: &str,
        title_str: String,
        secondary_str: String,
        amount_raw: String,
        cx: &mut Context<Self>,
    ) {
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
        match crate::auth::client::api_invite_member(fid, &req) {
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
