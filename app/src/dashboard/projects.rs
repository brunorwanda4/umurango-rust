use super::*;

impl DashboardView {
    pub(super) fn render_projects_screen(
        &self,
        palette: ThemeColors,
        view_entity: Entity<Self>,
    ) -> AnyElement {
        let lang = self.language;
        let total_saved: i64 = self.projects.iter().map(|p| p.saved_rwf).sum();
        let active_count = self.projects.iter().filter(|p| !p.is_completed).count();
        let completed_count = self.projects.iter().filter(|p| p.is_completed).count();

        let view_new_proj = view_entity.clone();
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
                            .child(text(lang, "cozy.total_saved_plans")),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .text_color(palette.accent_primary)
                            .child(format_rwf(total_saved)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.text_secondary)
                            .child(format!(
                                "{} {} · {} {}",
                                active_count,
                                text(lang, "cozy.active_goals_word"),
                                completed_count,
                                text(lang, "cozy.completed_milestone_word")
                            )),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(filter_pill_group(
                        &[
                            (
                                "proj-filter-active",
                                ProjectFilter::Active,
                                format!(
                                    "{} ({active_count})",
                                    text(lang, "cozy.filter_active_plans")
                                ),
                            ),
                            (
                                "proj-filter-done",
                                ProjectFilter::Completed,
                                format!(
                                    "{} ({completed_count})",
                                    text(lang, "cozy.filter_completed")
                                ),
                            ),
                        ],
                        self.project_filter,
                        palette,
                        {
                            let view_pf = view_entity.clone();
                            move |next_filter, _w, cx| {
                                view_pf.update(cx, |this, cx| {
                                    this.project_filter = next_filter;
                                    cx.notify();
                                });
                            }
                        },
                    ))
                    .child(compact_primary_button(
                        "projects-new-btn",
                        text(lang, "cozy.new_project_btn"),
                        palette,
                        move |_e, w, cx| {
                            view_new_proj.update(cx, |this, cx| {
                                this.open_entry_modal(EntryModalKind::Project, w, cx);
                            });
                        },
                    )),
            );

        let filter = self.project_filter;
        let expanded = self.expanded_project_index;
        let projects_grid = div().flex().flex_col().gap_3().children(
            self.projects
                .iter()
                .enumerate()
                .filter(|(_, p)| match filter {
                    ProjectFilter::Active => !p.is_completed,
                    ProjectFilter::Completed => p.is_completed,
                })
                .map(|(idx, proj)| {
                    let view_boost = view_entity.clone();
                    let view_details = view_entity.clone();
                    let is_expanded = expanded == Some(idx);
                    project_goal_card(
                        idx,
                        proj,
                        is_expanded,
                        lang,
                        palette,
                        move |_e, _w, cx| {
                            view_boost.update(cx, |this, cx| {
                                this.boost_project(idx, 50_000, cx);
                            });
                        },
                        move |_e, _w, cx| {
                            view_details.update(cx, |this, cx| {
                                if this.expanded_project_index == Some(idx) {
                                    this.expanded_project_index = None;
                                } else {
                                    this.expanded_project_index = Some(idx);
                                }
                                cx.notify();
                            });
                        },
                    )
                }),
        );

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(hero_card)
            .child(projects_grid)
            .into_any_element()
    }
}
