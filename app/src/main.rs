mod auth;
mod brand;
mod i18n;
mod icons;

use auth::login::{LoginEvent, LoginView};
use brand::*;
use gpui_kit::*;
use i18n::Language;

pub struct UmuryangoApp {
    theme: ThemeMode,
    language: Language,
    login_view: Entity<LoginView>,
    _login_subscription: Subscription,
}

impl UmuryangoApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
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
                LoginEvent::SignedIn { .. } => {
                    cx.notify();
                }
            },
        );

        Self {
            theme,
            language,
            login_view,
            _login_subscription: login_subscription,
        }
    }
}

impl Render for UmuryangoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(colors(self.theme).bg)
            .child(self.login_view.clone())
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
