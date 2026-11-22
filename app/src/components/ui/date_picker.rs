use crate::brand::ThemeColors;
pub use gpui_kit::component::calendar::{Date, Matcher};
pub use gpui_kit::component::date_picker::{
    DatePicker, DatePickerEvent, DatePickerState, DateRangePreset, DateTime,
};
pub use gpui_kit::component::time_field::{HourCycle, TimePrecision};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn date_picker_input(state: &Entity<DatePickerState>, palette: ThemeColors) -> DatePicker {
    DatePicker::new(state)
        .appearance(false)
        .cleanable(true)
        .text_color(palette.text_primary)
}

pub fn selected_date_string(picker: &DatePickerState) -> Option<String> {
    match picker.date() {
        Date::Single(Some(d)) => Some(d.to_string()),
        Date::Range(Some(s), Some(e)) => Some(format!("{s} to {e}")),
        Date::Range(Some(s), None) => Some(s.to_string()),
        _ => None,
    }
}

#[derive(IntoElement)]
pub struct KawaiiDatePickerContainer {
    palette: ThemeColors,
    child: AnyElement,
    is_focused: bool,
}

impl KawaiiDatePickerContainer {
    pub fn new(palette: ThemeColors, child: impl IntoElement) -> Self {
        Self {
            palette,
            child: child.into_any_element(),
            is_focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }
}

impl RenderOnce for KawaiiDatePickerContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let palette = self.palette;
        let is_focused = self.is_focused;

        let bg_color = if is_focused {
            palette.bg_input_focus
        } else {
            palette.bg_input
        };
        let border_color = if is_focused {
            palette.accent_primary
        } else {
            palette.border_color
        };
        let shadows = if is_focused {
            palette.input_focus_shadows()
        } else {
            palette.input_shadows()
        };

        div()
            .w_full()
            .rounded(palette.radius_input)
            .bg(bg_color)
            .border(palette.border_width_input)
            .border_color(border_color)
            .shadow(shadows)
            .px_2()
            .py_1p5()
            .cursor_pointer()
            .hover(|style| {
                style
                    .bg(palette.bg_input_focus)
                    .border_color(palette.accent_primary)
            })
            .child(self.child)
    }
}

pub fn date_picker_container(palette: ThemeColors, child: impl IntoElement) -> impl IntoElement {
    KawaiiDatePickerContainer::new(palette, child)
}

pub fn date_picker_field(
    label: impl Into<SharedString>,
    palette: ThemeColors,
    child: impl IntoElement,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_secondary)
                .child(label.into()),
        )
        .child(date_picker_container(palette, child))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[core::prelude::v1::test]
    fn test_date_picker_helpers() {
        let colors = crate::brand::colors(crate::brand::ThemeMode::Light);
        let field = date_picker_field("Date", colors, div().child("picker"));
        let _any = field.into_any_element();
    }
}
