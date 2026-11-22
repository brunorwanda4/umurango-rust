use crate::brand::ThemeColors;
use crate::components::ui::{badge, card_surface, compact_primary_button, BadgeVariant};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::*;
use std::rc::Rc;

pub type UploadDocHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type AccessDocHandler = Rc<dyn Fn(&'static str, &mut Window, &mut App)>;

pub struct VaultPageProps {
    pub language: Language,
    pub palette: ThemeColors,
    pub stacked: bool,
    pub on_upload_doc: UploadDocHandler,
    pub on_access_doc: AccessDocHandler,
}

pub fn vault_page(props: VaultPageProps) -> impl IntoElement {
    let VaultPageProps {
        language: lang,
        palette,
        stacked: _,
        on_upload_doc,
        on_access_doc,
    } = props;

    let hero_card = card_surface(palette)
        .flex_row()
        .items_center()
        .justify_between()
        .flex_wrap()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "section.vault_subtitle")),
                )
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.accent_primary)
                        .child("Secure Document Vault"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_secondary)
                        .child("4 encrypted family documents · Permission-controlled access"),
                ),
        )
        .child(compact_primary_button(
            "vault-upload-btn",
            "+ Upload Document",
            palette,
            move |_e, w, cx| on_upload_doc(w, cx),
        ));

    let documents = [
        (
            "Gisozi Land Title & Survey UPI 1/03/04",
            "Real Estate & Property",
            "Uploaded Nov 12, 2026 by Custodian",
            "Family Council Only",
            "Encrypted",
        ),
        (
            "Family Constitution & Charter v2.1",
            "Governance",
            "Ratified Oct 04, 2026",
            "All Members",
            "Verified",
        ),
        (
            "Agricultural Co-op Partnership Agreement",
            "Business",
            "Uploaded Sep 19, 2026",
            "Managers & Signatories",
            "Encrypted",
        ),
        (
            "Audited Annual Statement FY 2025-2026",
            "Financial Audit",
            "Approved by Elders Council",
            "Public to Family",
            "Signed",
        ),
    ];

    let document_cards =
        div()
            .flex()
            .flex_col()
            .gap_3()
            .children(documents.iter().enumerate().map(
                |(idx, (title, category, meta, permission, status))| {
                    let access_cb = on_access_doc.clone();
                    let doc_title = *title;
                    card_surface(palette)
                        .id(("vault-doc-card", idx))
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_2p5()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .w(px(28.0))
                                                .h(px(28.0))
                                                .rounded_full()
                                                .bg(palette.accent_primary.opacity(0.12))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    icons::icon(icons::ICON_FILE_LINES)
                                                        .size(px(14.0))
                                                        .text_color(palette.accent_primary),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_primary)
                                                .child(*title),
                                        ),
                                )
                                .child(badge(
                                    *status,
                                    match *status {
                                        "Encrypted" => BadgeVariant::Primary,
                                        "Verified" => BadgeVariant::Success,
                                        _ => BadgeVariant::Neutral,
                                    },
                                    palette,
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(palette.text_secondary)
                                        .child(format!("{} · {}", category, meta)),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.accent_primary)
                                        .child(format!("Access: {}", permission)),
                                ),
                        )
                        .child(
                            div().flex().justify_end().pt_1().child(
                                div()
                                    .id(("vault-access-btn", idx))
                                    .cursor_pointer()
                                    .px_3()
                                    .py_1()
                                    .rounded(palette.radius_button)
                                    .bg(palette.bg_elevated)
                                    .border(palette.border_width_input)
                                    .border_color(palette.border_color)
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_primary)
                                    .child("Decrypt & View")
                                    .on_click(move |_e, w, cx| {
                                        access_cb(doc_title, w, cx);
                                    }),
                            ),
                        )
                },
            ));

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(hero_card)
        .child(document_cards)
        .into_any_element()
}
