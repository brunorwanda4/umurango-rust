use crate::brand::ThemeColors;
pub use gpui_kit::component::select::{
    SearchableVec, Select, SelectDelegate, SelectEvent, SelectGroup, SelectItem, SelectListItem,
    SelectState,
};
pub use gpui_kit::component::IndexPath;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn select_input<D>(state: &Entity<SelectState<D>>, palette: ThemeColors) -> Select<D>
where
    D: SelectDelegate + 'static,
    <D::Item as SelectItem>::Value: PartialEq + Clone,
{
    Select::new(state)
        .appearance(false)
        .text_color(palette.text_primary)
}

#[derive(IntoElement)]
pub struct KawaiiSelectContainer {
    palette: ThemeColors,
    child: AnyElement,
    is_focused: bool,
}

impl KawaiiSelectContainer {
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

impl RenderOnce for KawaiiSelectContainer {
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

pub fn select_container(palette: ThemeColors, child: impl IntoElement) -> impl IntoElement {
    KawaiiSelectContainer::new(palette, child)
}

pub fn stateful_select_container<D>(
    palette: ThemeColors,
    state: &Entity<SelectState<D>>,
    child: impl IntoElement,
    window: &Window,
    cx: &App,
) -> impl IntoElement
where
    D: SelectDelegate + 'static,
    <D::Item as SelectItem>::Value: PartialEq + Clone,
{
    let is_focused = state.read(cx).focus_handle(cx).is_focused(window);
    KawaiiSelectContainer::new(palette, child).focused(is_focused)
}

pub fn select_field(
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
        .child(select_container(palette, child))
}

pub fn select_field_with_action(
    label: impl Into<SharedString>,
    action: impl IntoElement,
    palette: ThemeColors,
    child: impl IntoElement,
) -> Div {
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
                        .child(label.into()),
                )
                .child(action),
        )
        .child(select_container(palette, child))
}
