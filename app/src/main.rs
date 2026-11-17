mod auth;
mod brand;
mod i18n;
mod icons;

use auth::login::{LoginEvent, LoginView};
use brand::*;
use gpui_kit::component::{
    badge::Badge,
    button::{Button, ButtonVariants},
    progress::Progress,
};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use i18n::{text, Language};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AuthenticatedMember {
    pub email: String,
    pub display_name: String,
    pub family_name: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurrentScreen {
    SignIn,
    Dashboard,
}

pub struct UmuryangoApp {
    local_database: String,
    current_screen: CurrentScreen,
    theme: ThemeMode,
    language: Language,
    login_view: Entity<LoginView>,
    authenticated_member: Option<AuthenticatedMember>,
    _login_subscription: Subscription,
}

impl UmuryangoApp {
    pub fn new(local_database: String, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let theme = ThemeMode::Dark;
        let language = Language::default();
        let login_view = cx.new(|cx| LoginView::new(window, theme, language, cx));

        let login_subscription = cx.subscribe(
            &login_view,
            |this, _view, event: &LoginEvent, cx| match event {
                LoginEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                LoginEvent::LanguageChanged(language) => {
                    this.language = *language;
                    cx.notify();
                }
                LoginEvent::SignedIn {
                    email,
                    display_name,
                    family_name,
                    role,
                } => {
                    this.authenticated_member = Some(AuthenticatedMember {
                        email: email.clone(),
                        display_name: display_name.clone(),
                        family_name: family_name.clone(),
                        role: role.clone(),
                    });
                    this.current_screen = CurrentScreen::Dashboard;
                    cx.notify();
                }
            },
        );

        Self {
            local_database,
            current_screen: CurrentScreen::SignIn,
            theme,
            language,
            login_view,
            authenticated_member: None,
            _login_subscription: login_subscription,
        }
    }

    fn render_dashboard(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let app_entity = cx.entity().clone();
        let palette = colors(self.theme);
        let theme_label = self.theme.label();
        let display_name = self
            .authenticated_member
            .as_ref()
            .map(|m| m.display_name.clone())
            .unwrap_or_else(|| "Mama Keza".to_string());
        let role = self
            .authenticated_member
            .as_ref()
            .map(|m| m.role.clone())
            .unwrap_or_else(|| "Family Admin".to_string());
        let family_name = self
            .authenticated_member
            .as_ref()
            .map(|m| m.family_name.clone())
            .unwrap_or_else(|| "Umuryango wa Keza".to_string());

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .p_8()
            .bg(palette.bg)
            .text_color(palette.text_primary)
            // Top Bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .rounded_xl()
                    .bg(palette.card)
                    .border_1()
                    .border_color(palette.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                img(Arc::new(Image::from_bytes(
                                    ImageFormat::Png,
                                    self.theme.wordmark().to_vec(),
                                )))
                                .w(px(150.0))
                                .h(px(26.0)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(palette.text_primary)
                                                    .child(family_name),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_full()
                                                    .bg(palette.primary.opacity(0.15))
                                                    .text_color(palette.primary_light)
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(role),
                                            ),
                                    )
                                    .child(
                                        div().text_xs().text_color(palette.text_muted).child(
                                            text(self.language, "dashboard.active_member")
                                                .replace("{name}", &display_name),
                                        ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child({
                                let app = app_entity.clone();
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .p_1()
                                    .rounded_full()
                                    .bg(palette.surface)
                                    .child({
                                        let app = app.clone();
                                        let selected = self.language == Language::Kinyarwanda;
                                        Button::new("dashboard-language-rw")
                                            .label("RW")
                                            .compact()
                                            .text()
                                            .when(selected, |this| {
                                                this.bg(palette.card)
                                                    .rounded_full()
                                                    .text_color(palette.text_primary)
                                            })
                                            .when(!selected, |this| {
                                                this.text_color(palette.text_muted)
                                            })
                                            .on_click(move |_e, window, cx| {
                                                app.update(cx, |this, cx| {
                                                    if this.language != Language::Kinyarwanda {
                                                        this.language = Language::Kinyarwanda;
                                                        this.login_view.update(cx, |view, cx| {
                                                            view.set_language(
                                                                Language::Kinyarwanda,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                        cx.notify();
                                                    }
                                                });
                                            })
                                    })
                                    .child({
                                        let app = app.clone();
                                        let selected = self.language == Language::English;
                                        Button::new("dashboard-language-en")
                                            .label("EN")
                                            .compact()
                                            .text()
                                            .when(selected, |this| {
                                                this.bg(palette.card)
                                                    .rounded_full()
                                                    .text_color(palette.text_primary)
                                            })
                                            .when(!selected, |this| {
                                                this.text_color(palette.text_muted)
                                            })
                                            .on_click(move |_e, window, cx| {
                                                app.update(cx, |this, cx| {
                                                    if this.language != Language::English {
                                                        this.language = Language::English;
                                                        this.login_view.update(cx, |view, cx| {
                                                            view.set_language(
                                                                Language::English,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                        cx.notify();
                                                    }
                                                });
                                            })
                                    })
                            })
                            .child({
                                let app = app_entity.clone();
                                Button::new("theme-toggle")
                                    .icon(icons::theme_button_icon(self.theme))
                                    .accessibility_label(theme_label)
                                    .tooltip(theme_label)
                                    .text()
                                    .compact()
                                    .on_click(move |_e, _w, cx| {
                                        app.update(cx, |this, cx| {
                                            let theme = this.theme.toggle();
                                            this.theme = theme;
                                            this.login_view.update(cx, |view, cx| {
                                                view.set_theme(theme, cx);
                                            });
                                            cx.notify();
                                        });
                                    })
                            })
                            .child(Badge::new().count(1))
                            .child({
                                let app = app_entity.clone();
                                Button::new("sign-out-btn")
                                    .label(text(self.language, "dashboard.sign_out"))
                                    .on_click(move |_e, _w, cx| {
                                        app.update(cx, |this, cx| {
                                            this.current_screen = CurrentScreen::SignIn;
                                            this.authenticated_member = None;
                                            cx.notify();
                                        });
                                    })
                            }),
                    ),
            )
            // Summary Cards
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(summary_card(
                        "wallet",
                        text(self.language, "dashboard.available_balance"),
                        "RWF 1,240,000",
                        text(self.language, "dashboard.household_treasury"),
                        palette.primary_light,
                        palette,
                    ))
                    .child(summary_card(
                        "rotate",
                        text(self.language, "dashboard.pending_sync"),
                        "0 changes",
                        text(self.language, "dashboard.local_sync"),
                        palette.accent_blue,
                        palette,
                    ))
                    .child(summary_card(
                        "wallet",
                        text(self.language, "dashboard.monthly_budget"),
                        "RWF 850,000",
                        text(self.language, "dashboard.budget_status"),
                        palette.accent_gold,
                        palette,
                    )),
            )
            // Communal Project Progress Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_5()
                    .rounded_xl()
                    .bg(palette.card)
                    .border_1()
                    .border_color(palette.border)
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
                                    .child(icons::icon_with(
                                        "circle-check",
                                        palette.primary_light,
                                        px(16.0),
                                    ))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(palette.text_primary)
                                            .child(text(self.language, "dashboard.communal_goal")),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(palette.primary_light)
                                    .child(text(self.language, "dashboard.funded")),
                            ),
                    )
                    .child(Progress::new("project-progress").value(68.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .text_color(palette.text_muted)
                            .child(text(self.language, "dashboard.target"))
                            .child(text(self.language, "dashboard.contributed")),
                    ),
            )
            // Footer with Action Buttons and Local Database Info
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .rounded_xl()
                    .bg(palette.card)
                    .border_1()
                    .border_color(palette.border)
                    .child(
                        div().text_xs().text_color(palette.text_dim).child(
                            text(self.language, "dashboard.local_database")
                                .replace("{database}", &self.local_database),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(
                                Button::new("record-contribution")
                                    .label(text(self.language, "dashboard.record_contribution")),
                            )
                            .child(
                                Button::new("add-expense")
                                    .label(text(self.language, "dashboard.add_expense"))
                                    .primary(),
                            ),
                    ),
            )
    }
}

impl Render for UmuryangoApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(colors(self.theme).bg)
            .child(match self.current_screen {
                CurrentScreen::SignIn => self.login_view.clone().into_any_element(),
                CurrentScreen::Dashboard => self.render_dashboard(window, cx).into_any_element(),
            })
    }
}

fn summary_card(
    icon: &'static str,
    title: String,
    value: &'static str,
    subtitle: String,
    accent: Rgba,
    palette: ThemeColors,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .p_5()
        .flex_1()
        .rounded_xl()
        .bg(palette.card)
        .border_1()
        .border_color(palette.border)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(icons::icon_with(icon, accent, px(14.0)))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette.text_muted)
                        .child(title),
                ),
        )
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::BOLD)
                .text_color(accent)
                .child(value),
        )
        .child(div().text_xs().text_color(palette.text_dim).child(subtitle))
}

fn window_options(cx: &App) -> WindowOptions {
    const DEFAULT_WIDTH: f32 = 1100.0;
    const DEFAULT_HEIGHT: f32 = 750.0;
    const MIN_WIDTH: f32 = 360.0;
    const MIN_HEIGHT: f32 = 560.0;
    const SCREEN_FRACTION: f32 = 0.8;
    const SCREEN_MARGIN: f32 = 32.0;

    let display_bounds = cx
        .primary_display()
        .or_else(|| cx.displays().into_iter().next())
        .map(|display| display.bounds());
    let (width, height) = display_bounds
        .map(|bounds| {
            (
                DEFAULT_WIDTH.min(bounds.size.width.as_f32() * SCREEN_FRACTION),
                DEFAULT_HEIGHT.min(bounds.size.height.as_f32() * SCREEN_FRACTION),
            )
        })
        .unwrap_or((DEFAULT_WIDTH, DEFAULT_HEIGHT));
    let (min_width, min_height) = display_bounds
        .map(|bounds| {
            (
                MIN_WIDTH.min((bounds.size.width.as_f32() - SCREEN_MARGIN).max(0.0)),
                MIN_HEIGHT.min((bounds.size.height.as_f32() - SCREEN_MARGIN).max(0.0)),
            )
        })
        .unwrap_or((MIN_WIDTH, MIN_HEIGHT));

    WindowOptions {
        window_bounds: Some(WindowBounds::centered(size(px(width), px(height)), cx)),
        window_min_size: Some(size(px(min_width), px(min_height))),
        ..Default::default()
    }
}

fn main() {
    let database_path = "umuryango.db";
    let _connection = umuryango_app::db::open_local(database_path)
        .expect("local SQLite database must be available");

    application().run(|cx| {
        init(cx);
        open_window(window_options(cx), cx, |window, cx| {
            cx.new(|cx| UmuryangoApp::new(database_path.to_string(), window, cx))
        })
        .expect("window must open");
    });
}
