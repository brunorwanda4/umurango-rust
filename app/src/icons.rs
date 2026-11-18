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
pub const ICON_PLUS: &str = "plus";
pub const ICON_ARROW_RIGHT: &str = "arrow-right";
pub const ICON_SHIELD_HALVED: &str = "shield-halved";
pub const ICON_PHONE: &str = "phone";
pub const ICON_XMARK: &str = "xmark";
pub const ICON_CIRCLE_CHECK: &str = "circle-check";
pub const ICON_CIRCLE_XMARK: &str = "circle-xmark";
pub const ICON_CIRCLE_INFO: &str = "circle-info";
pub const ICON_TRIANGLE_EXCLAMATION: &str = "triangle-exclamation";
pub const ICON_MAGNIFYING_GLASS: &str = "magnifying-glass";
pub const ICON_WALLET: &str = "wallet";
pub const ICON_CREDIT_CARD: &str = "credit-card";
pub const ICON_ARROW_UP: &str = "arrow-up";
pub const ICON_ARROW_DOWN: &str = "arrow-down";
pub const ICON_SYNC: &str = "sync";
pub const ICON_BELL: &str = "bell";
pub const ICON_GEAR: &str = "gear";
pub const ICON_RIGHT_FROM_BRACKET: &str = "right-from-bracket";
pub const ICON_HAND_HOLDING_DOLLAR: &str = "hand-holding-dollar";
pub const ICON_BULLSEYE: &str = "bullseye";
pub const ICON_PIGGY_BANK: &str = "piggy-bank";
pub const ICON_FILTER: &str = "filter";

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
