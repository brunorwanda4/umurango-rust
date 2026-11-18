mod app;
mod auth;
mod brand;
mod components;
mod dashboard;
mod i18n;
mod icons;
mod navigation;
mod theme;
mod window;

use app::UmuryangoApp;
use gpui_kit::*;
use umuryango_app::db;
use window::window_options;

fn main() {
    let connection =
        db::open_local("umuryango.db").expect("local SQLite database must be available");

    application().run(move |cx| {
        init(cx);
        open_window(window_options(cx), cx, move |window, cx| {
            cx.new(|cx| UmuryangoApp::new(connection, window, cx))
        })
        .expect("window must open");
    });
}
