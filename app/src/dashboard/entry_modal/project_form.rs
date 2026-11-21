use crate::brand::ThemeColors;
use crate::components::ui::{
    money_input, stateful_input_container, text_input, DialogContent, DialogDescription,
    DialogFooter, DialogHeader, DialogTitle, WindowExt,
};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

pub fn render_project_form(
    dashboard: &DashboardView,
    palette: ThemeColors,
    _view_entity: Entity<DashboardView>,
    lang: Language,
) -> AnyElement {
    DialogContent::new()
        .gap_3()
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
                                .child(text(lang, "modal.field_project_name")),
                        )
                        .child(stateful_input_container(
                            palette,
                            &dashboard.entry_title_input,
                            text_input(&dashboard.entry_title_input, palette),
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
                                .child(text(lang, "modal.field_category_project")),
                        )
                        .child(stateful_input_container(
                            palette,
                            &dashboard.entry_secondary_input,
                            text_input(&dashboard.entry_secondary_input, palette),
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
                                .child(text(lang, "modal.field_target_amount")),
                        )
                        .child(money_input(
                            "entry-project-amount-input",
                            &dashboard.entry_amount_input,
                            "RWF",
                            palette,
                        )),
                ),
        )
        .into_any_element()
}

#[allow(dead_code)]
pub fn render_project_dialog(
    dashboard: &DashboardView,
    palette: ThemeColors,
    view_entity: Entity<DashboardView>,
    lang: Language,
) -> AnyElement {
    render_project_form(dashboard, palette, view_entity, lang)
}

/// Opens a standalone project dialog using GPUI Kit's native declarative dialog API (`window.open_dialog`).
#[allow(dead_code)]
pub fn open_project_dialog(
    window: &mut Window,
    cx: &mut App,
    dashboard_view: Entity<DashboardView>,
    palette: ThemeColors,
    lang: Language,
) {
    window.open_dialog(cx, move |dialog, _window, _cx| {
        let view = dashboard_view.clone();
        dialog
            .w(px(520.0))
            .overlay(true)
            .overlay_closable(true)
            .keyboard(true)
            .content(move |content, _window, cx| {
                let db = view.read(cx);
                content
                    .child(
                        DialogHeader::new()
                            .child(DialogTitle::new().child(text(lang, "modal.project_title")))
                            .child(
                                DialogDescription::new()
                                    .child(text(lang, "cozy.planned_projects_sub")),
                            ),
                    )
                    .child(render_project_form(db, palette, view.clone(), lang))
                    .child(
                        DialogFooter::new()
                            .child(
                                Button::new("project-dialog-cancel")
                                    .outline()
                                    .label(text(lang, "modal.cancel"))
                                    .on_click(|_, window, cx| {
                                        window.close_dialog(cx);
                                    }),
                            )
                            .child(
                                Button::new("project-dialog-save")
                                    .primary()
                                    .label(text(lang, "modal.save"))
                                    .on_click({
                                        let v = view.clone();
                                        move |_, window, cx| {
                                            v.update(cx, |this, cx| {
                                                this.submit_entry_modal(window, cx);
                                            });
                                            window.close_dialog(cx);
                                        }
                                    }),
                            ),
                    )
            })
    });
}
