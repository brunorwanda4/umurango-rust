#![allow(dead_code)]

use crate::brand::ThemeMode;
use gpui_kit::{component::Icon, svg, Pixels, Rgba, Styled, Svg};
use include_dir::{include_dir, Dir};
use std::path::Path;

static FONT_AWESOME: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../assets/fontawesome");

pub const ICON_CHECK: &str = "check";
pub const ICON_EYE: &str = "eye";
pub const ICON_EYE_SLASH: &str = "eye-slash";
pub const ICON_HOUSE: &str = "house";
pub const ICON_USER: &str = "user";
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
pub const ICON_HEART: &str = "heart";
pub const ICON_PEOPLE_ROOF: &str = "people-roof";
pub const ICON_HOUSE_CHIMNEY_USER: &str = "house-chimney-user";
pub const ICON_SUN: &str = "sun";
pub const ICON_MOON: &str = "moon";
pub const ICON_CHEVRON_LEFT: &str = "chevron-left";
pub const ICON_CHEVRON_RIGHT: &str = "chevron-right";
pub const ICON_CHEVRON_DOWN: &str = "chevron-down";
pub const ICON_ARROW_LEFT: &str = "arrow-left";
pub const ICON_COINS: &str = "coins";
pub const ICON_HANDSHAKE: &str = "handshake";
pub const ICON_CHART_COLUMN: &str = "chart-column";
pub const ICON_CHART_PIE: &str = "chart-pie";
pub const ICON_STAR: &str = "star";
pub const ICON_SEEDLING: &str = "seedling";
pub const ICON_GRADUATION_CAP: &str = "graduation-cap";
pub const ICON_TRACTOR: &str = "tractor";
pub const ICON_BOLT: &str = "bolt";
pub const ICON_CART_SHOPPING: &str = "cart-shopping";
pub const ICON_BUILDING_COLUMNS: &str = "building-columns";
pub const ICON_PAPER_PLANE: &str = "paper-plane";
pub const ICON_CLOCK: &str = "clock";
pub const ICON_CALENDAR: &str = "calendar";
pub const ICON_SLIDERS: &str = "sliders";
pub const ICON_PALETTE: &str = "palette";
pub const ICON_MINUS: &str = "minus";
pub const ICON_WINDOW_MINIMIZE: &str = "window-minimize";
pub const ICON_WINDOW_MAXIMIZE: &str = "window-maximize";
pub const ICON_WINDOW_RESTORE: &str = "window-restore";
pub const ICON_EXPAND: &str = "expand";
pub const ICON_COMPRESS: &str = "compress";
pub const ICON_TAG: &str = "tag";
pub const ICON_FOLDER: &str = "folder";
pub const ICON_FILE_LINES: &str = "file-lines";
pub const ICON_ADDRESS_CARD: &str = "address-card";
pub const ICON_SORT: &str = "sort";
pub const ICON_INBOX: &str = "inbox";
pub const ICON_CLIPBOARD_LIST: &str = "clipboard-list";
pub const ICON_MONEY_BILL_TRANSFER: &str = "money-bill-transfer";

pub fn has_icon(name: &str) -> bool {
    let clean = name.trim();
    if clean.is_empty() {
        return false;
    }
    let path = format!("free-solid-svg-icons/{clean}.svg");
    FONT_AWESOME.get_file(Path::new(&path)).is_some()
}

pub fn safe_icon(name: &str) -> Svg {
    let clean = name.trim();
    if has_icon(clean) {
        icon(clean)
    } else {
        icon("tag")
    }
}

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
    safe_icon(name).size(size).text_color(color)
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
