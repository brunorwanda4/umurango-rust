use gpui_kit::*;

pub fn window_options(cx: &App) -> WindowOptions {
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
        titlebar: Some(TitlebarOptions {
            title: Some("Umuryango".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(14.0), px(14.0))),
        }),
        app_owns_titlebar_drag: true,
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
    }
}
