use crate::brand::ThemeColors;
use crate::components::ui::{
    badge, card_elevated, card_header, card_surface, compact_primary_button, dialog_backdrop,
    dialog_container, dialog_footer, dialog_header, pill_button, BadgeVariant,
};
use crate::i18n::{text, Language};
use crate::icons;
use crate::updater::{ReleaseUpdate, UpdateState, CURRENT_VERSION};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Formats an asset byte count into a human-readable size string (e.g. `"14.2 MB"`).
pub fn format_asset_size(bytes: u64) -> String {
    if bytes == 0 {
        return String::new();
    }
    let kb = bytes as f64 / 1024.0;
    if kb < 1024.0 {
        format!("{kb:.0} KB")
    } else {
        let mb = kb / 1024.0;
        format!("{mb:.1} MB")
    }
}

/// Renders a top notification banner when a new application update is available.
pub fn update_notification_banner<R, I>(
    update: &ReleaseUpdate,
    is_downloading: bool,
    language: Language,
    palette: ThemeColors,
    on_review: R,
    on_install: I,
) -> Stateful<Div>
where
    R: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    I: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let install_label = if is_downloading {
        text(language, "update.installing_btn")
    } else {
        text(language, "update.install_btn")
    };

    div()
        .id("app-update-notification-banner")
        .w_full()
        .p_3()
        .rounded(palette.radius_button)
        .bg(palette.income_bg)
        .border(palette.border_width_input)
        .border_color(palette.income_border)
        .flex()
        .items_center()
        .justify_between()
        .flex_wrap()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2p5()
                .child(
                    div()
                        .size(px(30.0))
                        .rounded_full()
                        .bg(palette.accent_primary.opacity(0.18))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            icons::icon(icons::ICON_ROCKET)
                                .size(px(14.0))
                                .text_color(palette.income_text),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(format!(
                                            "{} v{}",
                                            text(language, "update.banner_title"),
                                            update.version
                                        )),
                                )
                                .child(badge(
                                    format!("v{CURRENT_VERSION} -> v{}", update.version),
                                    BadgeVariant::Success,
                                    palette,
                                )),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(palette.text_secondary)
                                .child(update.title.clone()),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(pill_button(
                    "banner-review-update-btn",
                    text(language, "update.review_btn"),
                    palette,
                    on_review,
                ))
                .child(compact_primary_button(
                    "banner-install-update-btn",
                    install_label,
                    palette,
                    on_install,
                )),
        )
}

/// Renders the interactive Update & Restart confirmation modal dialog.
pub fn render_update_modal<C, I>(
    update: &ReleaseUpdate,
    is_downloading: bool,
    language: Language,
    palette: ThemeColors,
    on_close: C,
    on_install: I,
) -> Stateful<Div>
where
    C: Fn(&mut Window, &mut App) + Clone + 'static,
    I: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let close_hdr = on_close.clone();
    let close_cancel = on_close.clone();
    let close_backdrop = on_close;

    let install_label = if is_downloading {
        text(language, "update.installing_btn")
    } else {
        text(language, "update.install_btn")
    };

    let size_badge = format_asset_size(update.asset_size);

    let card = dialog_container("app-update-modal-card", palette)
        .gap_3()
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
        .child(
            dialog_header(palette)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .size(px(30.0))
                                .rounded_full()
                                .bg(palette.accent_primary.opacity(0.18))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    icons::icon(icons::ICON_ROCKET)
                                        .size(px(14.0))
                                        .text_color(palette.accent_primary),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::EXTRA_BOLD)
                                        .text_color(palette.text_primary)
                                        .child(text(language, "update.modal_title")),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(palette.text_secondary)
                                        .child(text(language, "update.modal_subtitle")),
                                ),
                        ),
                )
                .child(
                    div()
                        .id("update-modal-close-icon")
                        .cursor_pointer()
                        .p_1()
                        .child(
                            icons::icon(icons::ICON_XMARK)
                                .size(px(14.0))
                                .text_color(palette.text_secondary),
                        )
                        .on_click(move |_e, window, cx| {
                            close_hdr(window, cx);
                        }),
                ),
        )
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
                            div()
                                .text_xs()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(update.title.clone()),
                        )
                        .child(badge(
                            format!("v{CURRENT_VERSION} -> v{}", update.version),
                            BadgeVariant::Success,
                            palette,
                        )),
                )
                .when(!size_badge.is_empty(), |this| {
                    this.child(badge(size_badge, BadgeVariant::Neutral, palette))
                }),
        )
        .child(
            card_elevated(palette)
                .gap_1p5()
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(language, "update.release_notes")),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_primary)
                        .child(update.notes.clone()),
                ),
        )
        .when_some(update.asset_name.clone(), |this, asset_name| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .gap_1p5()
                    .child(
                        icons::icon(icons::ICON_DOWNLOAD)
                            .size(px(12.0))
                            .text_color(palette.text_muted),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(palette.text_muted)
                            .child(asset_name),
                    ),
            )
        })
        .child(
            dialog_footer(palette)
                .child(pill_button(
                    "update-modal-later-btn",
                    text(language, "update.later_btn"),
                    palette,
                    move |_e, window, cx| {
                        close_cancel(window, cx);
                    },
                ))
                .child(compact_primary_button(
                    "update-modal-install-btn",
                    install_label,
                    palette,
                    on_install,
                )),
        );

    dialog_backdrop(card, palette, move |_e, window, cx| {
        close_backdrop(window, cx);
    })
}

/// Renders the Application Updates card in the Profile / Settings screen.
pub fn app_update_card<C, I>(
    update_state: &UpdateState,
    language: Language,
    palette: ThemeColors,
    on_check: C,
    on_install: I,
) -> Div
where
    C: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    I: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let (status_icon, status_color, status_text) = match update_state {
        UpdateState::Idle => (
            icons::ICON_CIRCLE_INFO,
            palette.text_secondary,
            text(language, "update.status_idle"),
        ),
        UpdateState::Checking => (
            icons::ICON_ROTATE,
            palette.accent_primary,
            text(language, "update.status_checking"),
        ),
        UpdateState::UpToDate => (
            icons::ICON_CIRCLE_CHECK,
            palette.income_text,
            text(language, "update.status_up_to_date"),
        ),
        UpdateState::Available(rel) => (
            icons::ICON_ROCKET,
            palette.income_text,
            format!(
                "{} v{}",
                text(language, "update.status_available"),
                rel.version
            ),
        ),
        UpdateState::Downloading(rel) => (
            icons::ICON_DOWNLOAD,
            palette.accent_primary,
            format!(
                "{} (v{})",
                text(language, "update.status_downloading"),
                rel.version
            ),
        ),
        UpdateState::Error(err) => (
            icons::ICON_TRIANGLE_EXCLAMATION,
            palette.expense_text,
            err.clone(),
        ),
    };

    let has_available_update = matches!(update_state, UpdateState::Available(_));
    let is_downloading = matches!(update_state, UpdateState::Downloading(_));

    card_surface(palette)
        .child(card_header(
            text(language, "update.card_title"),
            Some(text(language, "update.card_subtitle")),
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
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(palette.text_secondary)
                                        .child(text(language, "update.current_version")),
                                )
                                .child(badge(
                                    format!("v{CURRENT_VERSION}"),
                                    BadgeVariant::Primary,
                                    palette,
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    icons::icon(status_icon)
                                        .size(px(14.0))
                                        .text_color(status_color),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_primary)
                                        .child(status_text),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(pill_button(
                            "profile-check-update-btn",
                            text(language, "update.check_btn"),
                            palette,
                            on_check,
                        ))
                        .when(has_available_update || is_downloading, |this| {
                            let label = if is_downloading {
                                text(language, "update.installing_btn")
                            } else {
                                text(language, "update.install_btn")
                            };
                            this.child(compact_primary_button(
                                "profile-install-update-btn",
                                label,
                                palette,
                                on_install,
                            ))
                        }),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[core::prelude::v1::test]
    fn test_format_asset_size() {
        assert_eq!(format_asset_size(0), "");
        assert_eq!(format_asset_size(512 * 1024), "512 KB");
        assert_eq!(format_asset_size(15 * 1024 * 1024), "15.0 MB");
    }
}
