use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub struct SegmentedTabItem<'a, T: Copy + PartialEq> {
    pub id: &'static str,
    pub value: T,
    pub label: &'a str,
    pub icon: Option<&'static str>,
    pub dot_color: Option<Rgba>,
    pub badge_text: Option<&'a str>,
}

pub fn segmented_tab_bar<T, F>(
    id: impl Into<ElementId>,
    items: &[SegmentedTabItem<'_, T>],
    active: T,
    palette: ThemeColors,
    on_select: F,
) -> impl IntoElement
where
    T: Copy + PartialEq + 'static,
    F: Fn(T, &mut Window, &mut App) + Clone + 'static,
{
    div()
        .id(id)
        .flex()
        .items_center()
        .flex_wrap()
        .gap_2()
        .p_1p5()
        .rounded(px(22.0))
        .bg(palette.bg_card)
        .border(palette.border_width_input)
        .border_color(palette.border_color)
        .shadow(palette.checkbox_shadows())
        .children(items.iter().map(|item| {
            let is_active = item.value == active;
            let value = item.value;
            let handler = on_select.clone();
            let label = item.label.to_string();
            let badge_opt = item.badge_text.map(|b| b.to_string());
            let icon_opt = item.icon;
            let dot_opt = item.dot_color;

            div()
                .id(item.id)
                .relative()
                .flex_1()
                .min_w(px(110.0))
                .flex()
                .items_center()
                .justify_center()
                .gap_2()
                .px_3()
                .py_2()
                .rounded(px(15.0))
                .border(px(2.0))
                .text_xs()
                .font_weight(FontWeight::EXTRA_BOLD)
                .cursor_pointer()
                .when(is_active, |this| {
                    this.bg(palette.accent_primary)
                        .text_color(palette.text_on_accent)
                        .border_color(palette.border_color)
                        .shadow(palette.checkbox_shadows())
                })
                .when(!is_active, |this| {
                    this.bg(palette.bg_badge)
                        .text_color(palette.text_secondary)
                        .border_color(palette.border_color)
                        .hover(|s| {
                            s.top(px(-1.0))
                                .border_color(palette.accent_primary)
                                .text_color(palette.text_primary)
                        })
                })
                .when_some(dot_opt, |this, color| {
                    this.child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(if is_active {
                        palette.text_on_accent
                    } else {
                        color
                    }))
                })
                .when_some(icon_opt, |this, ic| {
                    this.child(icons::icon(ic).size(px(13.0)).text_color(if is_active {
                        palette.text_on_accent
                    } else {
                        palette.accent_primary
                    }))
                })
                .child(label)
                .when_some(badge_opt, |this, b| {
                    this.child(
                        div()
                            .px_1p5()
                            .py(px(1.0))
                            .rounded_full()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .bg(if is_active {
                                palette.text_on_accent.opacity(0.18)
                            } else {
                                palette.bg_input
                            })
                            .text_color(if is_active {
                                palette.text_on_accent
                            } else {
                                palette.text_secondary
                            })
                            .child(b),
                    )
                })
                .on_click(move |_event, window, cx| handler(value, window, cx))
        }))
}

pub fn filter_pill_group<T, F>(
    items: &[(&'static str, T, String)],
    active: T,
    palette: ThemeColors,
    on_select: F,
) -> impl IntoElement
where
    T: Copy + PartialEq + 'static,
    F: Fn(T, &mut Window, &mut App) + Clone + 'static,
{
    div()
        .flex()
        .items_center()
        .flex_wrap()
        .gap_1p5()
        .children(items.iter().map(|(id, val, label)| {
            let is_active = *val == active;
            let value = *val;
            let handler = on_select.clone();
            let text_label = label.clone();

            div()
                .id(*id)
                .relative()
                .px_3()
                .py_1p5()
                .rounded(px(14.0))
                .border(px(2.0))
                .border_color(palette.border_color)
                .shadow(palette.checkbox_shadows())
                .text_xs()
                .font_weight(FontWeight::EXTRA_BOLD)
                .cursor_pointer()
                .when(is_active, |this| {
                    this.bg(palette.accent_primary)
                        .text_color(palette.text_on_accent)
                })
                .when(!is_active, |this| {
                    this.bg(palette.bg_badge)
                        .text_color(palette.text_secondary)
                        .hover(|s| {
                            s.top(px(-1.0))
                                .border_color(palette.accent_primary)
                                .text_color(palette.text_primary)
                        })
                })
                .active(|s| s.top(px(2.0)))
                .child(text_label)
                .on_click(move |_event, window, cx| handler(value, window, cx))
        }))
}
