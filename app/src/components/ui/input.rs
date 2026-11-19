use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState, Textarea, TextareaState},
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn text_input(state: &Entity<InputState>, palette: ThemeColors) -> Input {
    Input::new(state)
        .appearance(false)
        .bordered(false)
        .focus_bordered(false)
        .cleanable(true)
        .cursor_text()
        .text_color(palette.text_primary)
}

pub fn textarea_input(state: &Entity<TextareaState>, _palette: ThemeColors) -> Textarea {
    Textarea::new(state)
        .appearance(false)
        .bordered(false)
        .h(px(46.0))
}

pub fn password_input(
    toggle_id: impl Into<ElementId>,
    state: &Entity<InputState>,
    palette: ThemeColors,
    show_password: bool,
    toggle_label: String,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Input {
    let toggle_icon = if show_password {
        icons::ICON_EYE_SLASH
    } else {
        icons::ICON_EYE
    };

    text_input(state, palette).suffix(
        Button::new(toggle_id)
            .icon(icons::button_icon(toggle_icon).text_color(palette.text_secondary))
            .accessibility_label(toggle_label.clone())
            .tooltip(toggle_label)
            .text()
            .compact()
            .on_click(on_toggle),
    )
}

pub fn search_input(state: &Entity<InputState>, palette: ThemeColors) -> Input {
    text_input(state, palette).prefix(
        div().pl_2().flex().items_center().child(
            icons::icon(icons::ICON_MAGNIFYING_GLASS)
                .size(px(14.))
                .text_color(palette.text_muted),
        ),
    )
}

#[derive(IntoElement)]
struct KawaiiInputContainer {
    palette: ThemeColors,
    state: Option<Entity<InputState>>,
    child: AnyElement,
}

impl RenderOnce for KawaiiInputContainer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let palette = self.palette;
        let is_focused = self
            .state
            .as_ref()
            .map(|state| {
                let s = state.read(cx);
                s.focus_handle(cx).is_focused(window) || s.has_selection_focus(window, cx)
            })
            .unwrap_or(false);

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
            .cursor_text()
            .hover(|style| {
                style
                    .bg(palette.bg_input_focus)
                    .border_color(palette.accent_primary)
            })
            .child(self.child)
    }
}

#[derive(IntoElement)]
struct KawaiiTextareaContainer {
    palette: ThemeColors,
    state: Entity<TextareaState>,
    child: AnyElement,
}

impl RenderOnce for KawaiiTextareaContainer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let palette = self.palette;
        let is_focused = {
            let s = self.state.read(cx);
            s.focus_handle(cx).is_focused(window) || s.has_selection_focus(window, cx)
        };

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
            .min_h(px(48.0))
            .rounded(palette.radius_input)
            .bg(bg_color)
            .border(palette.border_width_input)
            .border_color(border_color)
            .shadow(shadows)
            .px_2()
            .py_1p5()
            .cursor_text()
            .hover(|style| {
                style
                    .bg(palette.bg_input_focus)
                    .border_color(palette.accent_primary)
            })
            .child(self.child)
    }
}

pub fn input_container(palette: ThemeColors, input: impl IntoElement) -> impl IntoElement {
    KawaiiInputContainer {
        palette,
        state: None,
        child: input.into_any_element(),
    }
}

pub fn stateful_input_container(
    palette: ThemeColors,
    state: &Entity<InputState>,
    input: impl IntoElement,
) -> impl IntoElement {
    KawaiiInputContainer {
        palette,
        state: Some(state.clone()),
        child: input.into_any_element(),
    }
}

pub fn stateful_textarea_container(
    palette: ThemeColors,
    state: &Entity<TextareaState>,
    textarea: impl IntoElement,
) -> impl IntoElement {
    KawaiiTextareaContainer {
        palette,
        state: state.clone(),
        child: textarea.into_any_element(),
    }
}

pub fn input_field(
    palette: ThemeColors,
    label: impl Into<SharedString>,
    state: &Entity<InputState>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_secondary)
                .child(label.into()),
        )
        .child(stateful_input_container(
            palette,
            state,
            text_input(state, palette),
        ))
}

pub fn input_field_with_error(
    palette: ThemeColors,
    label: impl Into<SharedString>,
    error: Option<impl Into<SharedString>>,
    state: &Entity<InputState>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
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
                .when_some(error, |this, err| {
                    this.child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(palette.error)
                            .child(err.into()),
                    )
                }),
        )
        .child(stateful_input_container(
            palette,
            state,
            text_input(state, palette),
        ))
}

pub fn password_field(
    toggle_id: impl Into<ElementId>,
    palette: ThemeColors,
    label: impl Into<SharedString>,
    header_extra: Option<AnyElement>,
    state: &Entity<InputState>,
    show_password: bool,
    toggle_label: String,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let mut header = div().flex().justify_between().items_center().child(
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(palette.text_secondary)
            .child(label.into()),
    );

    if let Some(extra) = header_extra {
        header = header.child(extra);
    }

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(header)
        .child(stateful_input_container(
            palette,
            state,
            password_input(
                toggle_id,
                state,
                palette,
                show_password,
                toggle_label,
                on_toggle,
            ),
        ))
}
