use crate::brand::ThemeColors;
use crate::components::ui::{
    badge, card_surface, compact_primary_button, kawaii_progress_bar, BadgeVariant,
};
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

pub type NewCampaignHandler = Rc<dyn Fn(&mut Window, &mut App)>;
pub type ContributeHandler = Rc<dyn Fn(&'static str, i64, &mut Window, &mut App)>;

pub struct CampaignsPageProps {
    pub language: Language,
    pub palette: ThemeColors,
    pub stacked: bool,
    pub on_new_campaign: NewCampaignHandler,
    pub on_contribute: ContributeHandler,
}

pub fn campaigns_page(props: CampaignsPageProps) -> impl IntoElement {
    let CampaignsPageProps {
        language: lang,
        palette,
        stacked: _,
        on_new_campaign,
        on_contribute,
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
                        .child(text(lang, "section.campaigns_subtitle")),
                )
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::EXTRA_BOLD)
                        .text_color(palette.accent_primary)
                        .child("4,850,000 RWF"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette.text_secondary)
                        .child("4 active campaigns · 14 member pledges"),
                ),
        )
        .child(compact_primary_button(
            "campaigns-new-btn",
            "+ New Campaign",
            palette,
            move |_e, w, cx| on_new_campaign(w, cx),
        ));

    let campaigns = [
        (
            "Monthly Ikimina Pool",
            "450,000 RWF",
            "600,000 RWF",
            75u8,
            "12 / 16 contributed",
            "Active",
        ),
        (
            "Medical Emergency Fund",
            "1,200,000 RWF",
            "1,200,000 RWF",
            100u8,
            "Fully funded",
            "Completed",
        ),
        (
            "Kigali Wedding Ceremony",
            "1,800,000 RWF",
            "2,500,000 RWF",
            72u8,
            "18 contributors",
            "Active",
        ),
        (
            "Gisozi Land Acquisition",
            "1,400,000 RWF",
            "5,000,000 RWF",
            28u8,
            "8 family pledges",
            "Active",
        ),
    ];

    let campaign_cards =
        div()
            .flex()
            .flex_col()
            .gap_3()
            .children(campaigns.iter().enumerate().map(
                |(idx, (name, raised, target, pct, desc, status))| {
                    let is_completed = *status == "Completed";
                    let contrib_cb = on_contribute.clone();
                    let campaign_name = *name;
                    card_surface(palette)
                        .id(("campaign-card", idx))
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_2p5()
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
                                                    icons::icon(icons::ICON_HAND_HOLDING_DOLLAR)
                                                        .size(px(14.0))
                                                        .text_color(palette.accent_primary),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(palette.text_primary)
                                                .child(*name),
                                        ),
                                )
                                .child(badge(
                                    *status,
                                    if is_completed {
                                        BadgeVariant::Success
                                    } else {
                                        BadgeVariant::Primary
                                    },
                                    palette,
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_primary)
                                        .child(format!("{} / {}", raised, target)),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(palette.text_secondary)
                                        .child(*desc),
                                ),
                        )
                        .child(kawaii_progress_bar(*pct, palette.accent_primary, palette))
                        .when(!is_completed, |this| {
                            this.child(
                                div().flex().justify_end().pt_1().child(
                                    div()
                                        .id(("campaign-contrib-btn", idx))
                                        .cursor_pointer()
                                        .px_3()
                                        .py_1()
                                        .rounded(palette.radius_button)
                                        .bg(palette.accent_primary)
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(palette.text_on_accent)
                                        .child("Contribute")
                                        .on_click(move |_e, w, cx| {
                                            contrib_cb(campaign_name, 25_000, w, cx);
                                        }),
                                ),
                            )
                        })
                },
            ));

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(hero_card)
        .child(campaign_cards)
        .into_any_element()
}
