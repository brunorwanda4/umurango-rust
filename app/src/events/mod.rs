use crate::brand::ThemeColors;
use crate::components::ui::{badge, card_surface, compact_primary_button, BadgeVariant};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::*;
use std::rc::Rc;

pub type NewEventHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type RsvpHandler = Rc<dyn Fn(&'static str, &'static str, &mut Window, &mut App)>;

pub struct EventsPageProps {
    pub language: Language,
    pub palette: ThemeColors,
    pub stacked: bool,
    pub on_new_event: NewEventHandler,
    pub on_rsvp: RsvpHandler,
}

pub fn events_page(props: EventsPageProps) -> impl IntoElement {
    let EventsPageProps {
        language: lang,
        palette,
        stacked: _,
        on_new_event,
        on_rsvp,
    } = props;

    let hero_card = card_surface(palette)
        .flex_row()
        .items_center()
        .justify_between()
        .flex_wrap()
        .gap_3()
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
                        .child(text(lang, "section.events_subtitle")),
                )
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.accent_primary)
                        .child("3 Upcoming Gatherings"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_secondary)
                        .child("Next event: Monthly Family Council this Sunday"),
                ),
        )
        .child(compact_primary_button(
            "events-new-btn",
            "+ New Event",
            palette,
            move |_e, w, cx| on_new_event(w, cx),
        ));

    let events = [
        (
            "Monthly Family Council & Feast",
            "Nov 28, 2026 · 14:00 CAT",
            "Kigali Family Estate & Google Meet",
            "16 Confirmed · 3 Tentative",
            "Upcoming",
        ),
        (
            "End of Year Diaspora Assembly",
            "Dec 20, 2026 · 18:00 CAT",
            "Zoom Hybrid",
            "28 Confirmed",
            "Scheduled",
        ),
        (
            "Agaciro Youth Mentorship Session",
            "Jan 10, 2027 · 10:00 CAT",
            "Norrsken House Kigali",
            "12 Confirmed",
            "Planning",
        ),
    ];

    let event_cards = div()
        .flex()
        .flex_col()
        .gap_3()
        .children(events.iter().enumerate().map(
            |(idx, (title, time, location, rsvps, status))| {
                let rsvp_cb = on_rsvp.clone();
                let ev_title = *title;
                card_surface(palette)
                    .id(("event-card", idx))
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w(px(28.0))
                                            .h(px(28.0))
                                            .rounded_full()
                                            .bg(palette.accent_primary.opacity(0.12))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                icons::icon(icons::ICON_CALENDAR)
                                                    .size(px(14.0))
                                                    .text_color(palette.accent_primary),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(palette.text_primary)
                                            .child(*title),
                                    ),
                            )
                            .child(badge(
                                *status,
                                match *status {
                                    "Upcoming" => BadgeVariant::Primary,
                                    "Scheduled" => BadgeVariant::Success,
                                    _ => BadgeVariant::Neutral,
                                },
                                palette,
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1p5()
                                    .child(
                                        icons::icon(icons::ICON_CLOCK)
                                            .size(px(12.0))
                                            .text_color(palette.text_secondary),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(palette.text_secondary)
                                            .child(*time),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1p5()
                                    .child(
                                        icons::icon(icons::ICON_HOUSE)
                                            .size(px(12.0))
                                            .text_color(palette.text_secondary),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(palette.text_secondary)
                                            .child(*location),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .pt_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_primary)
                                    .child(*rsvps),
                            )
                            .child(
                                div()
                                    .id(("event-rsvp-btn", idx))
                                    .cursor_pointer()
                                    .px_3()
                                    .py_1()
                                    .rounded(palette.radius_button)
                                    .bg(palette.accent_primary)
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.text_on_accent)
                                    .child("RSVP Yes")
                                    .on_click(move |_e, w, cx| {
                                        rsvp_cb(ev_title, "Attending", w, cx);
                                    }),
                            ),
                    )
            },
        ));

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(hero_card)
        .child(event_cards)
        .into_any_element()
}
