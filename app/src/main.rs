mod brand;
mod icons;
mod login;

use brand::*;
use gpui_kit::component::{
    badge::Badge,
    button::{Button, ButtonVariants},
    progress::Progress,
};
use gpui_kit::*;
use login::{LoginEvent, LoginView};
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
    login_view: Entity<LoginView>,
    authenticated_member: Option<AuthenticatedMember>,
    _login_subscription: Subscription,
}

impl UmuryangoApp {
    pub fn new(local_database: String, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let login_view = cx.new(|cx| LoginView::new(window, cx));

        let login_subscription = cx.subscribe(
            &login_view,
            |this, _view, event: &LoginEvent, cx| match event {
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
            .bg(color_bg())
            .text_color(color_text_primary())
            // Top Bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .rounded_xl()
                    .bg(color_card())
                    .border_1()
                    .border_color(color_border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                img(Arc::new(Image::from_bytes(
                                    ImageFormat::Png,
                                    BRAND_ICON.to_vec(),
                                )))
                                .size(px(44.0)),
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
                                                    .text_color(color_text_primary())
                                                    .child(family_name),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_full()
                                                    .bg(color_primary().opacity(0.15))
                                                    .text_color(color_primary_light())
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(role),
                                            ),
                                    )
                                    .child(div().text_xs().text_color(color_text_muted()).child(
                                        format!(
                                            "Active Member: {} • Encrypted Local Vault",
                                            display_name
                                        ),
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(Badge::new().count(1))
                            .child({
                                let app = app_entity.clone();
                                Button::new("sign-out-btn").label("Sign Out").on_click(
                                    move |_e, _w, cx| {
                                        app.update(cx, |this, cx| {
                                            this.current_screen = CurrentScreen::SignIn;
                                            this.authenticated_member = None;
                                            cx.notify();
                                        });
                                    },
                                )
                            }),
                    ),
            )
            // Summary Cards
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(summary_card(
                        icons::WALLET,
                        "Available Balance",
                        "RWF 1,240,000",
                        "Household Treasury",
                        color_primary_light(),
                    ))
                    .child(summary_card(
                        icons::ROTATE,
                        "Pending Sync",
                        "0 changes",
                        "Local SQLite outbox in sync",
                        color_accent_blue(),
                    ))
                    .child(summary_card(
                        icons::WALLET,
                        "Monthly Budget",
                        "RWF 850,000",
                        "72% utilized (on track)",
                        color_accent_gold(),
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
                    .bg(color_card())
                    .border_1()
                    .border_color(color_border())
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
                                        icons::icon(icons::CIRCLE_CHECK)
                                            .size(px(16.0))
                                            .text_color(color_primary_light()),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(color_text_primary())
                                            .child("Communal Goal: School Fees & Term 1 Savings"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(color_primary_light())
                                    .child("68% funded"),
                            ),
                    )
                    .child(Progress::new("project-progress").value(68.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .text_color(color_text_muted())
                            .child("Target: RWF 1,500,000")
                            .child("Contributed: RWF 1,020,000"),
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
                    .bg(color_card())
                    .border_1()
                    .border_color(color_border())
                    .child(div().text_xs().text_color(color_text_dim()).child(format!(
                        "Local database: {} (SQLite AES-256 Outbox)",
                        self.local_database
                    )))
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(Button::new("record-contribution").label("Record contribution"))
                            .child(Button::new("add-expense").label("Add expense").primary()),
                    ),
            )
    }
}

impl Render for UmuryangoApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_entity = cx.entity().clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(color_bg())
            // Top Navigation Banner with Screen Switcher for Testing/Previewing
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_6()
                    .py_2()
                    .bg(color_surface())
                    .border_b_1()
                    .border_color(color_border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().size(px(8.0)).rounded_full().bg(color_primary_light()))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(color_text_muted())
                                    .child("Umuryango Native App • Local-First Architecture"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child({
                                let app = app_entity.clone();
                                Button::new("nav-signin")
                                    .label("Screen 1: Sign In")
                                    .compact()
                                    .primary()
                                    .on_click(move |_e, _w, cx| {
                                        app.update(cx, |this, cx| {
                                            this.current_screen = CurrentScreen::SignIn;
                                            cx.notify();
                                        });
                                    })
                            })
                            .child({
                                let app = app_entity.clone();
                                Button::new("nav-dashboard")
                                    .label("Screen 2: Dashboard")
                                    .compact()
                                    .on_click(move |_e, _w, cx| {
                                        app.update(cx, |this, cx| {
                                            this.current_screen = CurrentScreen::Dashboard;
                                            cx.notify();
                                        });
                                    })
                            }),
                    ),
            )
            // Screen Content
            .child(div().flex_1().size_full().child(match self.current_screen {
                CurrentScreen::SignIn => self.login_view.clone().into_any_element(),
                CurrentScreen::Dashboard => self.render_dashboard(window, cx).into_any_element(),
            }))
    }
}

fn summary_card(
    icon: &'static [u8],
    title: &'static str,
    value: &'static str,
    subtitle: &'static str,
    accent: Rgba,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .p_5()
        .flex_1()
        .rounded_xl()
        .bg(color_card())
        .border_1()
        .border_color(color_border())
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(icons::icon(icon).size(px(14.0)).text_color(accent))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(color_text_muted())
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
        .child(div().text_xs().text_color(color_text_dim()).child(subtitle))
}

fn main() {
    let database_path = "umuryango.db";
    let _connection = umuryango_app::db::open_local(database_path)
        .expect("local SQLite database must be available");

    application().run(|cx| {
        init(cx);
        open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| UmuryangoApp::new(database_path.to_string(), window, cx))
        })
        .expect("window must open");
    });
}
