use crate::brand::ThemeColors;
use crate::components::ui::{stateful_input_container, text_input};
use crate::dashboard::DashboardView;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::component::input::InputState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn render_icon_picker(
    palette: ThemeColors,
    selected_icon: &str,
    cat_active_color: Rgba,
    icon_input: &Entity<InputState>,
    displayed_icons: Vec<String>,
    icon_query: &str,
    view_entity: Entity<DashboardView>,
    lang: Language,
) -> Div {
    let view_icon = view_entity;

    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(palette.text_secondary)
                        .child(text(lang, "modal.category_icon")),
                )
                .child(div().w(px(130.0)).child(stateful_input_container(
                    palette,
                    icon_input,
                    text_input(icon_input, palette),
                ))),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_1()
                .p_1()
                .rounded(palette.radius_input)
                .bg(palette.bg_elevated)
                .border(palette.border_width_input)
                .border_color(palette.border_color)
                .when(displayed_icons.is_empty(), |this| {
                    this.child(
                        div()
                            .w_full()
                            .py_2()
                            .px_3()
                            .text_xs()
                            .text_color(palette.text_secondary)
                            .child(format!("No icon found matching \"{icon_query}\"")),
                    )
                })
                .children(displayed_icons.into_iter().map(|icon_name| {
                    let is_selected = selected_icon.eq_ignore_ascii_case(&icon_name);
                    let vi = view_icon.clone();
                    let ic_name = icon_name.clone();
                    div()
                        .id(format!("fa-icon-choice-{icon_name}"))
                        .cursor_pointer()
                        .size(px(25.0))
                        .rounded(px(5.0))
                        .bg(if is_selected {
                            cat_active_color.opacity(0.20)
                        } else {
                            palette.bg_card
                        })
                        .border(if is_selected { px(1.5) } else { px(1.0) })
                        .border_color(if is_selected {
                            cat_active_color
                        } else {
                            palette.border_color
                        })
                        .hover(|s| s.border_color(cat_active_color))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(icons::safe_icon(&icon_name).size(px(11.0)).text_color(
                            if is_selected {
                                cat_active_color
                            } else {
                                palette.text_secondary
                            },
                        ))
                        .on_click(move |_e, _window, cx| {
                            let name = ic_name.clone();
                            vi.update(cx, |this, cx| {
                                this.entry_category_icon = name;
                                cx.notify();
                            });
                        })
                })),
        )
}
