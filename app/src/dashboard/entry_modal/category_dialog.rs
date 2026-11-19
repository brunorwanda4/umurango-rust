use super::color_picker::render_color_picker;
use super::icon_picker::render_icon_picker;
use super::types::{parse_color_hex, CATEGORY_FONTAWESOME_ICONS};
use crate::brand::ThemeColors;
use crate::components::ui::{
    pill_button_colored, stateful_input_container, stateful_textarea_container, text_input,
    textarea_input,
};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::*;

pub fn render_category_type_switcher(
    palette: ThemeColors,
    cat_is_income: bool,
    view_entity: Entity<DashboardView>,
    lang: Language,
) -> Div {
    let view_toggle_cat_inc = view_entity.clone();
    let view_toggle_cat_exp = view_entity;

    let cat_income_label = if cat_is_income {
        format!("✓ {}", text(lang, "modal.category_income"))
    } else {
        text(lang, "modal.category_income")
    };

    let cat_expense_label = if !cat_is_income {
        format!("✓ {}", text(lang, "modal.category_expense"))
    } else {
        text(lang, "modal.category_expense")
    };

    div()
        .flex()
        .items_center()
        .gap_2()
        .child(pill_button_colored(
            "entry-cat-type-income",
            cat_income_label,
            if cat_is_income {
                palette.income_text
            } else {
                palette.text_secondary
            },
            palette,
            move |_e, _w, cx| {
                view_toggle_cat_inc.update(cx, |this, cx| {
                    this.entry_category_is_income = true;
                    cx.notify();
                });
            },
        ))
        .child(pill_button_colored(
            "entry-cat-type-expense",
            cat_expense_label,
            if !cat_is_income {
                palette.expense_text
            } else {
                palette.text_secondary
            },
            palette,
            move |_e, _w, cx| {
                view_toggle_cat_exp.update(cx, |this, cx| {
                    this.entry_category_is_income = false;
                    cx.notify();
                });
            },
        ))
}

pub fn render_category_dialog(
    dashboard: &DashboardView,
    palette: ThemeColors,
    view_entity: Entity<DashboardView>,
    lang: Language,
    cx: &App,
) -> AnyElement {
    let cat_is_income = dashboard.entry_category_is_income;
    let cat_active_color = parse_color_hex(&dashboard.entry_category_color);
    let selected_color = dashboard.entry_category_color.clone();
    let selected_icon = dashboard.entry_category_icon.clone();

    let icon_query = dashboard
        .entry_category_icon_input
        .read(cx)
        .value()
        .trim()
        .to_lowercase();

    let mut displayed_icons: Vec<String> = if icon_query.is_empty() {
        CATEGORY_FONTAWESOME_ICONS
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        CATEGORY_FONTAWESOME_ICONS
            .iter()
            .filter(|ic| ic.to_lowercase().contains(&icon_query))
            .map(|s| s.to_string())
            .collect()
    };

    if !icon_query.is_empty()
        && icons::has_icon(&icon_query)
        && !displayed_icons
            .iter()
            .any(|s| s.eq_ignore_ascii_case(&icon_query))
    {
        displayed_icons.insert(0, icon_query.clone());
    }

    div()
        .flex()
        .flex_col()
        .gap_2()
        // 1. Type Switcher
        .child(render_category_type_switcher(
            palette,
            cat_is_income,
            view_entity.clone(),
            lang,
        ))
        // 2. Category Name
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
                        .child(text(lang, "modal.category_name")),
                )
                .child(stateful_input_container(
                    palette,
                    &dashboard.entry_title_input,
                    text_input(&dashboard.entry_title_input, palette),
                )),
        )
        // 3. Color Picker Swatches & Custom Color
        .child(render_color_picker(
            palette,
            &selected_color,
            cat_active_color,
            &dashboard.entry_category_color_input,
            view_entity.clone(),
            lang,
        ))
        // 4. FontAwesome Icon Picker
        .child(render_icon_picker(
            palette,
            &selected_icon,
            cat_active_color,
            &dashboard.entry_category_icon_input,
            displayed_icons,
            &icon_query,
            view_entity,
            lang,
        ))
        // 5. Description Textarea
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
                        .child(text(lang, "modal.category_desc")),
                )
                .child(stateful_textarea_container(
                    palette,
                    &dashboard.entry_category_desc_input,
                    textarea_input(&dashboard.entry_category_desc_input, palette),
                )),
        )
        .into_any_element()
}
