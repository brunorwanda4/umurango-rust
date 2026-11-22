#![allow(dead_code)]

use crate::brand::ThemeMode;
use gpui_kit::{component::Icon, svg, Pixels, Rgba, Styled, Svg};
use include_dir::{include_dir, Dir};
use std::path::Path;
use std::sync::LazyLock;

static FONT_AWESOME: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../assets/fontawesome");

static ALL_ICON_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut names = Vec::new();
    for dir in FONT_AWESOME.dirs() {
        for file in dir.files() {
            if file.path().extension().is_some_and(|ext| ext == "svg") {
                if let Some(stem) = file.path().file_stem().and_then(|s| s.to_str()) {
                    names.push(stem);
                }
            }
        }
    }
    names.sort_unstable();
    names.dedup();
    names
});

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
pub const ICON_DOWNLOAD: &str = "download";
pub const ICON_ROTATE: &str = "rotate";
pub const ICON_ROCKET: &str = "rocket";

/// Returns all unique FontAwesome icon names available in assets/fontawesome.
pub fn all_icons() -> &'static [&'static str] {
    &ALL_ICON_NAMES
}

/// Searches all available FontAwesome icons by substring/prefix matching.
/// Exact matches appear first, followed by prefix matches, then substring matches.
pub fn search_icons(query: &str) -> Vec<&'static str> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }

    let mut exact = Vec::new();
    let mut starts = Vec::new();
    let mut contains = Vec::new();

    for &name in all_icons() {
        let lower = name.to_lowercase();
        if lower == q {
            exact.push(name);
        } else if lower.starts_with(&q) {
            starts.push(name);
        } else if lower.contains(&q) {
            contains.push(name);
        }
    }

    let mut results = exact;
    results.extend(starts);
    results.extend(contains);
    results
}

/// Locates icon SVG data across solid, regular, and brands folders.
pub fn find_icon_data(name: &str) -> Option<(&'static str, &'static [u8])> {
    let clean = name.trim();
    if clean.is_empty() {
        return None;
    }
    const SETS: [&str; 3] = [
        "free-solid-svg-icons",
        "free-regular-svg-icons",
        "free-brands-svg-icons",
    ];
    for set in SETS {
        let path = format!("{set}/{clean}.svg");
        if let Some(file) = FONT_AWESOME.get_file(Path::new(&path)) {
            return Some((set, file.contents()));
        }
    }
    None
}

pub fn has_icon(name: &str) -> bool {
    find_icon_data(name).is_some()
}

pub fn safe_icon(name: &str) -> Svg {
    let clean = name.trim();
    if let Some((_, data)) = find_icon_data(clean) {
        svg().data(data)
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
    if let Some((_, data)) = find_icon_data(name) {
        svg().data(data)
    } else {
        icon_set("free-solid-svg-icons", name)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_icons_and_search() {
        let total = all_icons().len();
        assert!(total > 2000, "Expected over 2000 icons, found {total}");
        assert!(has_icon("wallet"));
        assert!(has_icon("house"));
        assert!(has_icon("car"));
        assert!(has_icon("cart-shopping"));
        assert!(has_icon(ICON_DOWNLOAD));
        assert!(has_icon(ICON_ROTATE));
        assert!(has_icon(ICON_ROCKET));
        assert!(!has_icon("nonexistent_fake_icon_xyz"));

        let car_matches = search_icons("car");
        assert!(!car_matches.is_empty());
        assert_eq!(car_matches[0], "car");
        assert!(car_matches.contains(&"car-battery"));

        let shop_matches = search_icons("shop");
        assert!(!shop_matches.is_empty());
        assert!(shop_matches.contains(&"cart-shopping"));
    }
}
