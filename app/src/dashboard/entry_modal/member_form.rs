use crate::brand::ThemeColors;
use crate::components::ui::{stateful_input_container, text_input};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use gpui_kit::*;

pub fn render_member_form(
    dashboard: &DashboardView,
    palette: ThemeColors,
    _view_entity: Entity<DashboardView>,
    lang: Language,
) -> AnyElement {
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
                        .child(text(lang, "modal.field_full_name")),
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
                        .child(text(lang, "modal.field_phone_number")),
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
                        .child(text(lang, "modal.field_role")),
                )
                .child(stateful_input_container(
                    palette,
                    &dashboard.entry_amount_input,
                    text_input(&dashboard.entry_amount_input, palette),
                )),
        )
        .into_any_element()
}
