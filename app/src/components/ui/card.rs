use crate::brand::ThemeColors;
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub fn card(palette: ThemeColors) -> Div {
    div()
        .flex()
        .flex_col()
        .w_full()
        .p_6()
        .rounded(palette.radius_card)
        .bg(palette.bg_card)
        .border(palette.border_width)
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .text_color(palette.text_primary)
}

pub fn card_surface(palette: ThemeColors) -> Div {
    div()
        .flex()
        .flex_col()
        .w_full()
        .p_5()
        .rounded(palette.radius_card)
        .bg(palette.bg_card)
        .border(palette.border_width)
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .text_color(palette.text_primary)
}

pub fn interactive_card(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(id)
        .relative()
        .flex()
        .flex_col()
        .w_full()
        .p_5()
        .rounded(palette.radius_card)
        .bg(palette.bg_card)
        .border(palette.border_width)
        .border_color(palette.border_color)
        .shadow(palette.card_shadows())
        .cursor_pointer()
        .hover(|style| {
            style
                .top(px(-2.0))
                .border_color(palette.accent_primary)
                .bg(palette.bg_input_focus)
        })
        .active(|style| style.top(px(4.0)).shadow(palette.button_active_shadows()))
        .on_click(on_click)
}

pub fn card_header(
    title: impl Into<SharedString>,
    subtitle: Option<impl Into<SharedString>>,
    palette: ThemeColors,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .mb_3()
        .child(
            div()
                .text_base()
                .font_weight(FontWeight::BOLD)
                .text_color(palette.text_primary)
                .child(title.into()),
        )
        .when_some(subtitle, |this, sub| {
            this.child(
                div()
                    .text_xs()
                    .text_color(palette.text_secondary)
                    .child(sub.into()),
            )
        })
}

pub fn card_footer(palette: ThemeColors) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .mt_4()
        .pt_3()
        .border_t(palette.border_width_input)
        .border_color(palette.border_color)
}

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
struct OptionCard {
    id: ElementId,
    palette: ThemeColors,
    icon_name: &'static str,
    title: SharedString,
    description: SharedString,
    selected: bool,
    on_click: ClickHandler,
}

impl RenderOnce for OptionCard {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let hovered_state =
            window.use_keyed_state((self.id.clone(), "hover-state"), cx, |_, _| false);
        let is_hovered = *hovered_state.read(cx);
        let active = self.selected || is_hovered;
        let target_phase = if active { 1.0 } else { 0.0 };
        let spring = SpringConfig::new(280.0, 24.0, 1.0);

        let palette = self.palette;
        let icon_name = self.icon_name;
        let title = self.title;
        let description = self.description;
        let on_click = self.on_click;
        let spring_id = (self.id.clone(), "hover-spring");

        div()
            .id(self.id)
            .w_full()
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_between()
            .gap_3()
            .p_4()
            .rounded(palette.radius_input)
            .border(palette.border_width_input)
            .shadow(palette.button_shadows())
            .cursor_pointer()
            .on_hover(move |hovered, _window, cx| {
                hovered_state.update(cx, |state, cx| {
                    if *state != *hovered {
                        *state = *hovered;
                        cx.notify();
                    }
                });
            })
            .on_click(move |event, window, cx| (on_click)(event, window, cx))
            .with_spring(
                spring_id,
                SpringAnimation::new(spring)
                    .to(AnimationPhase(target_phase))
                    .with_epsilon(0.005),
                move |card, phase| {
                    let border_color =
                        phase.interpolate_clamped(palette.border_color, palette.accent_primary);
                    let icon_border_color =
                        phase.interpolate_clamped(palette.border_color, palette.accent_primary);
                    let arrow_color =
                        phase.interpolate_clamped(palette.text_muted, palette.accent_primary);
                    let arrow_offset_left = phase.interpolate_clamped(px(0.), px(6.));
                    let arrow_offset_right = phase.interpolate_clamped(px(6.), px(0.));

                    card.border_color(border_color)
                        .bg(if active {
                            palette.bg_input_focus
                        } else {
                            palette.bg_input
                        })
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .w(px(38.))
                                        .h(px(38.))
                                        .flex_shrink_0()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded(palette.radius_checkbox)
                                        .bg(palette.bg_card)
                                        .border(palette.border_width_input)
                                        .border_color(icon_border_color)
                                        .child(
                                            icons::icon(icon_name)
                                                .size(px(16.))
                                                .text_color(palette.accent_primary),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .overflow_hidden()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(
                                            div()
                                                .w_full()
                                                .text_sm()
                                                .whitespace_normal()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_primary)
                                                .child(title.clone()),
                                        )
                                        .child(
                                            div()
                                                .w_full()
                                                .text_xs()
                                                .whitespace_normal()
                                                .text_color(palette.text_secondary)
                                                .child(description.clone()),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .pl(arrow_offset_left)
                                .pr(arrow_offset_right)
                                .child(
                                    icons::icon(icons::ICON_ARROW_RIGHT)
                                        .size(px(14.))
                                        .text_color(arrow_color),
                                ),
                        )
                },
            )
    }
}

pub fn option_card(
    id: impl Into<ElementId>,
    palette: ThemeColors,
    icon_name: &'static str,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    selected: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    OptionCard {
        id: id.into(),
        palette,
        icon_name,
        title: title.into(),
        description: description.into(),
        selected,
        on_click: Box::new(on_click),
    }
}
