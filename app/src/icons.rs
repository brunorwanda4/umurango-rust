#![allow(dead_code)]

use crate::brand::ThemeMode;
use gpui_kit::{component::Icon, svg, Pixels, Rgba, Styled, Svg};
use include_dir::{include_dir, Dir};
use std::path::Path;

static FONT_AWESOME: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../assets/fontawesome");

pub const ICON_CHECK: &str = "check";
pub const ICON_EYE: &str = "eye";
pub const ICON_EYE_SLASH: &str = "eye-slash";
pub const ICON_USERS: &str = "users";
pub const ICON_USER_PLUS: &str = "user-plus";
pub const ICON_ARROW_RIGHT: &str = "arrow-right";
pub const ICON_SHIELD_HALVED: &str = "shield-halved";

fn icon_data(set: &str, name: &str) -> &'static [u8] {
    let path = format!("{set}/{name}.svg");
    FONT_AWESOME
        .get_file(Path::new(&path))
        .unwrap_or_else(|| panic!("Font Awesome icon not found: {path}"))
        .contents()
}

pub fn icon(name: &str) -> Svg {
    icon_set("free-solid-svg-icons", name)
}

pub fn icon_set(set: &str, name: &str) -> Svg {
    svg().data(icon_data(set, name))
}

pub fn icon_with(name: &str, color: Rgba, size: Pixels) -> Svg {
    icon(name).size(size).text_color(color)
}

pub fn button_icon(name: &str) -> Icon {
    button_icon_set("free-solid-svg-icons", name)
}

pub fn button_icon_set(set: &str, name: &str) -> Icon {
    Icon::default().data(icon_data(set, name))
}

pub fn theme_button_icon(theme: ThemeMode) -> Icon {
    button_icon(if theme == ThemeMode::Dark {
        "sun"
    } else {
        "moon"
    })
    .text_color(crate::brand::colors(theme).text_primary)
}
