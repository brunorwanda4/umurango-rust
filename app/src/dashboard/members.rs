use super::*;

impl DashboardView {
    pub(super) fn render_people_screen(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let lang = self.language;
        let total_pledge: i64 = self.family_members.iter().map(|m| m.monthly_goal_rwf).sum();
        let total_contributed: i64 = self.family_members.iter().map(|m| m.contributed_rwf).sum();
        let pledge_pct = if total_pledge <= 0 {
            100
        } else {
            ((total_contributed * 100) / total_pledge).clamp(0, 100) as u8
        };

        let view_invite = view_entity.clone();
        let hero_card = card_surface(palette)
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .flex_wrap()
                    .gap_2()
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
                                    .child(text(lang, "cozy.circle_monthly_pledge")),
                            )
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .text_color(palette.text_primary)
                                    .child(format!(
                                        "{} / {} ({}%)",
                                        format_rwf(total_contributed),
                                        format_rwf(total_pledge),
                                        pledge_pct
                                    )),
                            ),
                    )
                    .child(compact_primary_button(
                        "people-invite-member-btn",
                        text(lang, "cozy.invite_btn"),
                        palette,
                        move |_e, w, cx| {
                            view_invite.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::MemberInvite, w, cx);
                            });
                        },
                    )),
            )
            .child(kawaii_progress_bar(
                pledge_pct,
                palette.accent_primary,
                palette,
            ));

        let members_grid =
            div()
                .flex()
                .flex_col()
                .gap_3()
                .children(self.family_members.iter().enumerate().map(|(idx, member)| {
                    let view_contrib = view_entity.clone();
                    let view_remind = view_entity.clone();
                    let member_name = member.name.to_string();
                    member_pledge_card(
                        idx,
                        member,
                        lang,
                        palette,
                        move |_e, _w, cx| {
                            view_contrib.update(cx, |this, cx| {
                                this.log_member_contribution(idx, 25_000, cx);
                            });
                        },
                        move |_e, _w, cx| {
                            let target_name = member_name.clone();
                            view_remind.update(cx, |this, cx| {
                                this.toast_notice = Some(format!(
                                    "{} {target_name}!",
                                    text(this.language, "cozy.toast_reminded")
                                ));
                                cx.notify();
                            });
                        },
                    )
                }));

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(hero_card)
            .child(members_grid)
            .into_any_element()
    }
}
