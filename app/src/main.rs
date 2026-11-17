use gpui_kit::{
    component::{
        badge::Badge,
        button::{Button, ButtonVariants},
        progress::Progress,
    },
    *,
};

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
            .bg(rgb(0x0f172a))
            .text_color(rgb(0xe2e8f0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child("Umuryango"),
                            )
                            .child("Family finance overview"),
                    )
                    .child(Badge::new().count(1)),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(summary_card("Available balance", "RWF 1,240,000"))
                    .child(summary_card("Pending sync", "3 changes")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_4()
                    .rounded_lg()
                    .bg(rgb(0x1e293b))
                    .child("Community project progress")
                    .child(Progress::new("project-progress").value(68.0))
                    .child("68% funded"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(format!("Local database: {}", self.local_database))
                    .child(Button::new("add-expense").label("Add expense").primary()),
            )
    }
}

fn summary_card(title: &'static str, value: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .p_4()
        .flex_1()
        .rounded_lg()
        .bg(rgb(0x1e293b))
        .child(title)
        .child(div().text_lg().font_weight(FontWeight::BOLD).child(value))
}

fn main() {
    let database_path = "umuryango.db";
    let _connection = umuryango_app::db::open_local(database_path)
        .expect("local SQLite database must be available");

    application().run(|cx| {
        init(cx);
        open_window(WindowOptions::default(), cx, |_window, cx| {
            cx.new(|_cx| Dashboard {
                local_database: database_path.to_string(),
            })
        })
        .expect("window must open");
    });
}
