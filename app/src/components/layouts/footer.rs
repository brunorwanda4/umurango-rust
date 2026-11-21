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
    safe_bottom: Pixels,
) -> impl IntoElement {
    div()
        .id("app-footer")
        .w_full()
        .flex_none()
        .flex()
        .items_center()
        .justify_around()
        .gap_1p5()
        .px_2p5()
        .pt_2()
        .pb(px(8.0) + safe_bottom)
        .bg(palette.bg_nav)
        .border_t(palette.border_width)
        .border_color(palette.border_color)
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
        .relative()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_1()
        .px_1p5()
        .py_1p5()
        .rounded(px(16.0))
        .border(px(2.0))
        .cursor_pointer()
        .when(active, |this| {
            this.top(px(-2.0))
                .bg(palette.accent_primary)
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
        })
        .when(!active, |this| {
            this.border_color(palette.bg_nav)
                .hover(|s| s.bg(palette.bg_elevated).border_color(palette.border_color))
        })
        .child(
            icons::icon(section.icon())
                .size(px(15.0))
                .text_color(if active {
                    palette.text_on_accent
                } else {
                    palette.text_secondary
                }),
        )
        .child(
            div()
                .w_full()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_center()
                .text_size(px(10.0))
                .font_weight(FontWeight::EXTRA_BOLD)
                .text_color(if active {
                    palette.text_on_accent
                } else {
                    palette.text_secondary
                })
                .child(label),
        )
        .on_click(move |_event, window, cx| on_select(section, window, cx))
}
