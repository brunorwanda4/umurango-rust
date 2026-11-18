mod auth;
mod brand;
mod components;
mod i18n;
mod icons;

use auth::{
    login::{LoginEvent, LoginView},
    register::{RegisterEvent, RegisterView},
};
use brand::*;
use gpui_kit::*;
use i18n::Language;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum AuthScreen {
    #[default]
    Login,
    Register,
}

pub struct UmuryangoApp {
    theme: ThemeMode,
    language: Language,
    screen: AuthScreen,
    login_view: Entity<LoginView>,
    register_view: Entity<RegisterView>,
    _login_subscription: Subscription,
    _register_subscription: Subscription,
}

impl UmuryangoApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let theme = ThemeMode::Dark;
        let language = Language::default();
        let login_view = cx.new(|cx| LoginView::new(window, theme, language, cx));
        let register_view = cx.new(|cx| RegisterView::new(window, theme, language, cx));

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
                LoginEvent::RegisterRequested => {
                    this.screen = AuthScreen::Register;
                    cx.notify();
                }
                LoginEvent::SignedIn { .. } => {
                    cx.notify();
                }
            },
        );

        let register_subscription = cx.subscribe(
            &register_view,
            |this, _view, event: &RegisterEvent, cx| match event {
                RegisterEvent::ThemeChanged(theme) => {
                    this.theme = *theme;
                    cx.notify();
                }
                RegisterEvent::LanguageChanged(language) => {
                    this.language = *language;
                    cx.notify();
                }
                RegisterEvent::LoginRequested => {
                    this.screen = AuthScreen::Login;
                    cx.notify();
                }
                RegisterEvent::Registered { .. } => {
                    cx.notify();
                }
            },
        );

        Self {
            theme,
            language,
            screen: AuthScreen::Login,
            login_view,
            register_view,
            _login_subscription: login_subscription,
            _register_subscription: register_subscription,
        }
    }
}

impl Render for UmuryangoApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme;
        let language = self.language;

        let active_view: AnyElement = match self.screen {
            AuthScreen::Login => {
                self.login_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, window, cx);
                });
                self.login_view.clone().into_any_element()
            }
            AuthScreen::Register => {
                self.register_view.update(cx, |view, cx| {
                    view.sync_preferences(theme, language, window, cx);
                });
                self.register_view.clone().into_any_element()
            }
        };

        div()
            .size_full()
            .bg(colors(self.theme).bg)
            .child(active_view)
    }
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
            cx.new(|cx| UmuryangoApp::new(window, cx))
        })
        .expect("window must open");
    });
}
