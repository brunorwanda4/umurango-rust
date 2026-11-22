use super::*;

impl DashboardView {
    pub(super) fn render_people_screen(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let lang = self.language;
        let member_count = self.family_members.len();
        let view_invite = view_entity.clone();

        let hero_card = card_surface(palette).gap_3().child(
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
                                .child(text(lang, "dashboard.household_members")),
                        )
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .text_color(palette.text_primary)
                                .child(format!(
                                    "{} {}",
                                    member_count,
                                    if member_count == 1 {
                                        "Member"
                                    } else {
                                        "Members"
                                    }
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
        );

        let members_grid = div().flex().flex_col().gap_3().children(
            self.family_members
                .iter()
                .enumerate()
                .map(|(idx, member)| member_card(idx, member, palette)),
        );

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(hero_card)
            .child(members_grid)
            .into_any_element()
    }
}
