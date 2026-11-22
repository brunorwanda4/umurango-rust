use crate::components::ui::money_mask_pattern;
use crate::dashboard::{DashboardView, EntryModalKind};
use crate::i18n::{text, Language};
use gpui_kit::component::date_picker::DatePickerState;
use gpui_kit::*;

impl DashboardView {
    pub(in crate::dashboard) fn update_entry_modal_placeholders(
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
            EntryModalKind::Category => (
                text(lang, "modal.category_name_ph"),
                text(lang, "modal.category_desc_ph"),
                "".to_string(),
            ),
        };
        self.entry_title_input.update(cx, |state, cx| {
            state.set_placeholder(title_ph, window, cx);
        });
        self.entry_secondary_input.update(cx, |state, cx| {
            state.set_placeholder(sec_ph, window, cx);
        });
        self.entry_date_input.update(cx, |state, cx| {
            state.set_placeholder(text(lang, "modal.datetime_ph"), window, cx);
        });
        self.entry_notes_input.update(cx, |state, cx| {
            state.set_placeholder(text(lang, "modal.notes_ph"), window, cx);
        });
        if kind != EntryModalKind::Category {
            self.entry_amount_input.update(cx, |state, cx| {
                state.set_placeholder(amt_ph, window, cx);
            });
        } else {
            self.entry_category_desc_input.update(cx, |state, cx| {
                state.set_placeholder(text(lang, "modal.category_desc_ph"), window, cx);
            });
            self.entry_category_icon_input.update(cx, |state, cx| {
                state.set_placeholder(text(lang, "modal.icon_search_ph"), window, cx);
            });
            self.entry_category_color_input.update(cx, |state, cx| {
                state.set_placeholder("#RRGGBB".to_string(), window, cx);
            });
        }
    }

    pub fn get_available_categories(&self, is_income: bool) -> Vec<String> {
        let mut list = if is_income {
            vec![
                "Contribution".to_string(),
                "Business".to_string(),
                "Farming".to_string(),
                "Salary".to_string(),
            ]
        } else {
            vec![
                "Groceries".to_string(),
                "Education".to_string(),
                "Healthcare".to_string(),
                "Farming".to_string(),
                "Transport".to_string(),
            ]
        };

        for tx in &self.transactions {
            if tx.is_income == is_income {
                let cat = tx.category.to_string();
                if !cat.is_empty() && !list.contains(&cat) {
                    list.push(cat);
                }
            }
        }

        for item in &self.custom_categories {
            if item.is_income == is_income && !list.contains(&item.name) {
                list.push(item.name.clone());
            }
        }

        list
    }

    pub fn open_category_modal(
        &mut self,
        is_income: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current_title = self.entry_title_input.read(cx).value().to_string();
        let current_amount = self.entry_amount_input.read(cx).value().to_string();
        self.saved_tx_title = current_title;
        self.saved_tx_amount = current_amount;
        self.entry_category_return_modal = self.active_entry_modal;
        self.entry_category_is_income = is_income;
        self.entry_category_icon = if is_income {
            "wallet".to_string()
        } else {
            "cart-shopping".to_string()
        };
        self.entry_category_color = if is_income {
            "#8CE0B8".to_string()
        } else {
            "#FFA69E".to_string()
        };
        self.active_entry_modal = Some(EntryModalKind::Category);

        let lang = self.language;
        self.entry_title_input.update(cx, |state, cx| {
            state.set_placeholder(text(lang, "modal.category_name_ph"), window, cx);
            state.set_value("".to_string(), window, cx);
            state.focus_handle(cx).focus(window, cx);
        });
        self.entry_category_desc_input.update(cx, |state, cx| {
            state.set_placeholder(text(lang, "modal.category_desc_ph"), window, cx);
            state.set_value("".to_string(), window, cx);
        });
        self.entry_category_icon_input.update(cx, |state, cx| {
            state.set_placeholder(text(lang, "modal.icon_search_ph"), window, cx);
            state.set_value("".to_string(), window, cx);
        });
        self.entry_category_color_input.update(cx, |state, cx| {
            state.set_placeholder("#RRGGBB".to_string(), window, cx);
            state.set_value("".to_string(), window, cx);
        });
        cx.notify();
    }

    pub fn close_entry_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
            });
            self.entry_amount_input.update(cx, |state, cx| {
                state.set_placeholder(a_ph, window, cx);
                state.set_mask_pattern(money_mask_pattern("RWF"), window, cx);
                state.set_value(amt, window, cx);
            });
            cx.notify();
        } else {
            self.active_entry_modal = None;
            cx.notify();
        }
    }

    pub fn open_entry_modal(
        &mut self,
        kind: EntryModalKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if kind == EntryModalKind::Category {
            self.saved_tx_title.clear();
            self.saved_tx_amount.clear();
            self.entry_category_return_modal = None;
            self.open_category_modal(false, window, cx);
            return;
        }

        let lang = self.language;
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
            EntryModalKind::Category => (
                text(lang, "modal.category_name_ph"),
                text(lang, "modal.category_desc_ph"),
                "".to_string(),
            ),
        };

        self.entry_title_input.update(cx, |state, cx| {
            state.set_placeholder(title_ph, window, cx);
            state.set_value("".to_string(), window, cx);
            state.focus_handle(cx).focus(window, cx);
        });
        self.entry_secondary_input.update(cx, |state, cx| {
            state.set_placeholder(sec_ph, window, cx);
            state.set_value("".to_string(), window, cx);
        });

        if kind == EntryModalKind::Income || kind == EntryModalKind::Expense {
            let is_inc = kind == EntryModalKind::Income;
            let avail = self.get_available_categories(is_inc);
            self.entry_category_select.update(cx, |sel, cx| {
                sel.set_items(avail, window, cx);
                sel.set_selected_index(None, window, cx);
            });
            self.entry_date_picker =
                cx.new(|cx| DatePickerState::new(window, cx).date_format("%Y-%m-%d"));
            self.entry_date_input.update(cx, |state, cx| {
                state.set_placeholder(text(lang, "modal.datetime_ph"), window, cx);
                state.set_value("".to_string(), window, cx);
            });
            self.entry_notes_input.update(cx, |state, cx| {
                state.set_placeholder(text(lang, "modal.notes_ph"), window, cx);
                state.set_value("".to_string(), window, cx);
            });
            self.entry_account_select.update(cx, |sel, cx| {
                sel.set_items(
                    vec![
                        "Mobile Money".to_string(),
                        "Cash".to_string(),
                        "Bank Account".to_string(),
                    ],
                    window,
                    cx,
                );
                sel.set_selected_index(None, window, cx);
            });
        }

        if kind != EntryModalKind::Category {
            self.entry_amount_input.update(cx, |state, cx| {
                state.set_placeholder(amt_ph, window, cx);
                if kind == EntryModalKind::MemberInvite {
                    state.set_mask_pattern(
                        gpui_kit::component::input::MaskPattern::None,
                        window,
                        cx,
                    );
                } else {
                    state.set_mask_pattern(money_mask_pattern("RWF"), window, cx);
                }
                state.set_value("".to_string(), window, cx);
            });
        }
        self.active_entry_modal = Some(kind);
        cx.notify();
    }
}
