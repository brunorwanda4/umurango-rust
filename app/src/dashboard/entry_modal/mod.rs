pub mod actions;
pub mod category_dialog;
pub mod color_picker;
pub mod icon_picker;
pub mod loan_form;
pub mod member_form;
pub mod project_form;
pub mod shell;
pub mod submit;
pub mod transaction_form;
pub mod types;

use self::category_dialog::render_category_dialog;
use self::loan_form::render_loan_form;
use self::member_form::render_member_form;
use self::project_form::render_project_form;
use self::shell::{
    render_modal_backdrop, render_modal_card, render_modal_footer, render_modal_header,
};
use self::transaction_form::render_transaction_form;
use self::types::parse_color_hex;

use crate::brand::ThemeColors;
use crate::dashboard::{DashboardView, EntryModalKind};
use crate::i18n::text;
use crate::icons;
use gpui_kit::*;

impl DashboardView {
    pub(super) fn render_entry_modal(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
        cx: &App,
    ) -> Option<AnyElement> {
        let kind = self.active_entry_modal?;
        let lang = self.language;

        let (modal_icon, modal_title) = match kind {
            EntryModalKind::Income => (
                icons::ICON_HAND_HOLDING_DOLLAR,
                text(lang, "modal.income_title"),
            ),
            EntryModalKind::Expense => (icons::ICON_CREDIT_CARD, text(lang, "modal.expense_title")),
            EntryModalKind::Loan => (icons::ICON_HANDSHAKE, text(lang, "modal.loan_title")),
            EntryModalKind::Project => (icons::ICON_BULLSEYE, text(lang, "modal.project_title")),
            EntryModalKind::MemberInvite => {
                (icons::ICON_USER_PLUS, text(lang, "modal.invite_title"))
            }
            EntryModalKind::Category => (icons::ICON_SLIDERS, text(lang, "modal.category_title")),
        };

        let cat_active_color = parse_color_hex(&self.entry_category_color);

        let form_body: AnyElement = match kind {
            EntryModalKind::Category => {
                render_category_dialog(self, palette, view_entity.clone(), lang, cx)
            }
            EntryModalKind::Income => {
                render_transaction_form(self, palette, view_entity.clone(), lang, true)
            }
            EntryModalKind::Expense => {
                render_transaction_form(self, palette, view_entity.clone(), lang, false)
            }
            EntryModalKind::Loan => render_loan_form(self, palette, view_entity.clone(), lang),
            EntryModalKind::Project => {
                render_project_form(self, palette, view_entity.clone(), lang)
            }
            EntryModalKind::MemberInvite => {
                render_member_form(self, palette, view_entity.clone(), lang)
            }
        };

        let view_close_header = view_entity.clone();
        let view_close_footer = view_entity.clone();
        let view_close_bg = view_entity.clone();
        let view_submit = view_entity;

        let header = render_modal_header(
            modal_icon,
            modal_title,
            kind == EntryModalKind::Category,
            &self.entry_category_icon,
            cat_active_color,
            palette,
            move |_e, window, cx| {
                view_close_header.update(cx, |this, cx| {
                    this.close_entry_modal(window, cx);
                });
            },
        );

        let footer = render_modal_footer(
            text(lang, "modal.cancel"),
            text(lang, "modal.save"),
            palette,
            move |_e, window, cx| {
                view_close_footer.update(cx, |this, cx| {
                    this.close_entry_modal(window, cx);
                });
            },
            move |_e, window, cx| {
                view_submit.update(cx, |this, cx| {
                    this.submit_entry_modal(window, cx);
                });
            },
        );

        let card = render_modal_card(header, form_body, footer, palette);
        let backdrop = render_modal_backdrop(card, palette, move |_e, window, cx| {
            view_close_bg.update(cx, |this, cx| {
                this.close_entry_modal(window, cx);
            });
        });

        Some(backdrop.into_any_element())
    }
}
