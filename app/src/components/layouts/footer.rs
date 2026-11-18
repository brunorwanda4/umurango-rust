use super::nav::{AppSection, SectionSelectHandler};
use crate::brand::ThemeColors;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn app_footer(
    language: Language,
    active_section: AppSection,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    div()
        .id("app-footer")
        .w_full()
        .flex_none()
        .flex()
        .items_center()
        .justify_around()
        .gap_1()
        .px_2()
        .pt_1()
        .pb_2()
        .bg(palette.surface)
        .border_t_1()
        .border_color(palette.border)
        .children(
            AppSection::ALL
                .into_iter()
                .enumerate()
                .map(|(idx, section)| {
                    footer_nav_item(
                        idx,
                        section,
                        section == active_section,
                        language,
                        palette,
                        on_select.clone(),
                    )
                }),
        )
}

fn footer_nav_item(
    idx: usize,
    section: AppSection,
    active: bool,
    language: Language,
    palette: ThemeColors,
    on_select: SectionSelectHandler,
) -> impl IntoElement {
    let label = text(language, section.label_key());

    div()
        .id(("app-footer-nav", idx))
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_1()
        .px_1()
        .py_1()
        .rounded_lg()
        .cursor_pointer()
        .when(active, |this| this.bg(palette.primary.opacity(0.12)))
        .when(!active, |this| {
            this.hover(|s| s.bg(palette.border.opacity(0.4)))
        })
        .child(
            div()
                .w(px(22.0))
                .h(px(3.0))
                .rounded_full()
                .when(active, |this| this.bg(palette.primary_light)),
        )
        .child(
            icons::icon(section.icon())
                .size(px(16.0))
                .text_color(if active {
                    palette.primary_light
                } else {
                    palette.text_muted
                }),
        )
        .child(
            div()
                .w_full()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_center()
                .text_size(px(11.0))
                .font_weight(if active {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::MEDIUM
                })
                .text_color(if active {
                    palette.primary_light
                } else {
                    palette.text_muted
                })
                .child(label),
        )
        .on_click(move |_event, window, cx| on_select(section, window, cx))
}
