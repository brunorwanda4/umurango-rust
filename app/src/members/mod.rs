use crate::brand::ThemeColors;
use crate::components::dashboard::{member_card, MemberContributionRow};
use crate::components::ui::{card_surface, compact_primary_button, BadgeVariant};
use crate::i18n::{text, Language};
use gpui_kit::*;
use std::rc::Rc;

pub type InviteMemberHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub struct MembersPageProps<'a> {
    pub language: Language,
    pub palette: ThemeColors,
    pub members: &'a [(String, String, BadgeVariant)],
    pub family_members: &'a [MemberContributionRow],
    pub on_invite_member: InviteMemberHandler,
}

pub fn members_page(props: MembersPageProps<'_>) -> impl IntoElement {
    let MembersPageProps {
        language: lang,
        palette,
        members: _,
        family_members,
        on_invite_member,
    } = props;

    let member_count = family_members.len();
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
                move |_e, w, cx| on_invite_member(w, cx),
            )),
    );

    let members_grid = div().flex().flex_col().gap_3().children(
        family_members
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
