#![recursion_limit = "512"]

#[cfg(not(any(target_os = "ios", target_os = "android")))]
fn main() {
    use gpui_kit::*;
    use umuryango_app::{
        app::UmuryangoApp, db, preferences, theme, updater, window::window_options,
    };

    updater::cleanup_old_binary();

    let connection =
        db::open_local("umuryango.db").expect("local SQLite database must be available");

    application().run(move |cx| {
        init(cx);
        let prefs = preferences::load(&connection);
        theme::set_style_tokens(prefs.style, prefs.theme, prefs.mood, cx);
        open_window(window_options(cx), cx, move |window, cx| {
            cx.new(|cx| UmuryangoApp::new(connection, window, cx))
        })
        .expect("window must open");
    });
}

#[cfg(any(target_os = "ios", target_os = "android"))]
fn main() {}
