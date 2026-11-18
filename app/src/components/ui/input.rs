use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::component::{
    button::{Button, ButtonVariants},
    input::{Input, InputState},
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn text_input(state: &Entity<InputState>, palette: ThemeColors) -> Input {
    Input::new(state)
        .cleanable(true)
        .cursor_text()
        .bg(palette.surface)
        .text_color(palette.text_body)
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
            .icon(icons::button_icon(toggle_icon).text_color(palette.text_muted))
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

pub fn input_container(palette: ThemeColors, input: impl IntoElement) -> impl IntoElement {
    div()
        .w_full()
        .rounded_lg()
        .bg(palette.surface)
        .p_1()
        .cursor_text()
        .child(input)
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
                .text_color(palette.text_body)
                .child(label.into()),
        )
        .child(input_container(palette, text_input(state, palette)))
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
                        .text_color(palette.text_body)
                        .child(label.into()),
                )
                .when_some(error, |this, err| {
                    this.child(div().text_xs().text_color(palette.error).child(err.into()))
                }),
        )
        .child(input_container(palette, text_input(state, palette)))
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
            .text_color(palette.text_body)
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
        .child(input_container(
            palette,
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
