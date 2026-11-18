use gpui_kit::{px, Pixels};

pub const SIDEBAR_BREAKPOINT_PX: f32 = 768.0;
pub const SIDEBAR_WIDTH_PX: f32 = 232.0;

pub fn is_large_screen(viewport_width: Pixels) -> bool {
    viewport_width >= px(SIDEBAR_BREAKPOINT_PX)
}

pub fn app_content_width(viewport_width: Pixels) -> Pixels {
    if is_large_screen(viewport_width) {
        (viewport_width - px(SIDEBAR_WIDTH_PX)).max(px(0.0))
    } else {
        viewport_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_screen_breakpoint_switches_between_footer_and_sidebar() {
        assert!(!is_large_screen(px(360.0)));
        assert!(!is_large_screen(px(767.9)));
        assert!(is_large_screen(px(768.0)));
        assert!(is_large_screen(px(1100.0)));
    }

    #[test]
    fn content_width_accounts_for_sidebar_only_on_large_screens() {
        assert_eq!(app_content_width(px(480.0)), px(480.0));
        assert_eq!(app_content_width(px(1100.0)), px(1100.0 - SIDEBAR_WIDTH_PX));
    }
}
