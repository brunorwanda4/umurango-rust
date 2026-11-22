use crate::brand::ThemeColors;
use crate::components::dashboard::{format_rwf, project_goal_card, ProjectFilter, ProjectSummary};
use crate::components::ui::{card_surface, compact_primary_button, filter_pill_group};
use crate::i18n::{text, Language};
use gpui_kit::*;
use std::rc::Rc;

pub type FilterChangeHandler = Rc<dyn Fn(ProjectFilter, &mut Window, &mut App)>;
pub type ProjectActionHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;
pub type BoostActionHandler = Rc<dyn Fn(usize, i64, &mut Window, &mut App)>;
pub type NewProjectHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub struct ProjectsPageProps<'a> {
    pub language: Language,
    pub palette: ThemeColors,
    pub projects: &'a [ProjectSummary],
    pub project_filter: ProjectFilter,
    pub expanded_project_index: Option<usize>,
    pub on_change_filter: FilterChangeHandler,
    pub on_new_project: NewProjectHandler,
    pub on_boost_project: BoostActionHandler,
    pub on_toggle_expand: ProjectActionHandler,
}

pub fn projects_page(props: ProjectsPageProps<'_>) -> impl IntoElement {
    let ProjectsPageProps {
        language: lang,
        palette,
        projects,
        project_filter,
        expanded_project_index,
        on_change_filter,
        on_new_project,
        on_boost_project,
        on_toggle_expand,
    } = props;

    let total_saved: i64 = projects.iter().map(|p| p.saved_rwf).sum();
    let active_count = projects.iter().filter(|p| !p.is_completed).count();
    let completed_count = projects.iter().filter(|p| p.is_completed).count();

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
                    project_filter,
                    palette,
                    {
                        let on_change = on_change_filter.clone();
                        move |next_filter, w, cx| {
                            on_change(next_filter, w, cx);
                        }
                    },
                ))
                .child(compact_primary_button(
                    "projects-new-btn",
                    text(lang, "cozy.new_project_btn"),
                    palette,
                    move |_e, w, cx| on_new_project(w, cx),
                )),
        );

    let projects_grid = div().flex().flex_col().gap_3().children(
        projects
            .iter()
            .enumerate()
            .filter(|(_, p)| match project_filter {
                ProjectFilter::Active => !p.is_completed,
                ProjectFilter::Completed => p.is_completed,
            })
            .map(|(idx, proj)| {
                let boost_cb = on_boost_project.clone();
                let toggle_cb = on_toggle_expand.clone();
                let is_expanded = expanded_project_index == Some(idx);
                project_goal_card(
                    idx,
                    proj,
                    is_expanded,
                    lang,
                    palette,
                    move |_e, w, cx| {
                        boost_cb(idx, 50_000, w, cx);
                    },
                    move |_e, w, cx| {
                        toggle_cb(idx, w, cx);
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
