use gpui_kit::{rgb, Rgba};

#[allow(dead_code)]
pub const CATEGORY_PALETTE_HEX: [&str; 8] = [
    "#8CE0B8", "#5EEAD4", "#FFA69E", "#F7A8C4", "#F9D67A", "#A9C9FF", "#C4B5FD", "#94A3B8",
];

pub fn category_palette_colors() -> [(&'static str, Rgba); 8] {
    [
        ("#8CE0B8", rgb(0x8CE0B8)), // Matcha green
        ("#5EEAD4", rgb(0x5EEAD4)), // Mint/Teal
        ("#FFA69E", rgb(0xFFA69E)), // Coral/Peach
        ("#F7A8C4", rgb(0xF7A8C4)), // Berry pink
        ("#F9D67A", rgb(0xF9D67A)), // Gold
        ("#A9C9FF", rgb(0xA9C9FF)), // Sky blue
        ("#C4B5FD", rgb(0xC4B5FD)), // Lavender
        ("#94A3B8", rgb(0x94A3B8)), // Slate
    ]
}

pub fn parse_color_hex(hex: &str) -> Rgba {
    match hex.to_uppercase().as_str() {
        "#8CE0B8" => rgb(0x8CE0B8),
        "#5EEAD4" => rgb(0x5EEAD4),
        "#FFA69E" => rgb(0xFFA69E),
        "#F7A8C4" => rgb(0xF7A8C4),
        "#F9D67A" => rgb(0xF9D67A),
        "#A9C9FF" => rgb(0xA9C9FF),
        "#C4B5FD" => rgb(0xC4B5FD),
        "#94A3B8" => rgb(0x94A3B8),
        _ => rgb(0x8CE0B8),
    }
}

pub const CATEGORY_FONTAWESOME_ICONS: [&str; 24] = [
    "wallet",
    "credit-card",
    "coins",
    "piggy-bank",
    "hand-holding-dollar",
    "building-columns",
    "house",
    "users",
    "heart",
    "shield-halved",
    "bell",
    "graduation-cap",
    "book",
    "briefcase",
    "chart-pie",
    "chart-column",
    "seedling",
    "tractor",
    "cart-shopping",
    "utensils",
    "car",
    "bolt",
    "phone",
    "tag",
];
