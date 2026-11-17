use gpui::{div, prelude::*, App, Application, Context, Render, Window, WindowOptions};

struct Dashboard {
    local_database: String,
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_6()
            .child("Umuryango")
            .child(format!("Local database: {}", self.local_database))
    }
}

fn main() {
    let database_path = "umuryango.db";
    let _connection = umuryango_app::db::open_local(database_path)
        .expect("local SQLite database must be available");

    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| Dashboard {
                local_database: database_path.to_string(),
            })
        })
        .expect("window must open");
    });
}
