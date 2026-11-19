use gpui_kit::{rgb, Rgba};

#[allow(dead_code)]
pub const CATEGORY_PALETTE_HEX: [&str; 12] = [
    "#8CE0B8", "#5EEAD4", "#38BDF8", "#A9C9FF", "#C4B5FD", "#F472B6", "#F7A8C4", "#FFA69E",
    "#FB923C", "#F9D67A", "#A3E635", "#94A3B8",
];

pub fn category_palette_colors() -> [(&'static str, Rgba); 12] {
    [
        ("#8CE0B8", rgb(0x8CE0B8)), // Matcha green
        ("#5EEAD4", rgb(0x5EEAD4)), // Mint/Teal
        ("#38BDF8", rgb(0x38BDF8)), // Sky blue
        ("#A9C9FF", rgb(0xA9C9FF)), // Soft blue
        ("#C4B5FD", rgb(0xC4B5FD)), // Lavender
        ("#F472B6", rgb(0xF472B6)), // Pink
        ("#F7A8C4", rgb(0xF7A8C4)), // Berry pink
        ("#FFA69E", rgb(0xFFA69E)), // Coral/Peach
        ("#FB923C", rgb(0xFB923C)), // Orange
        ("#F9D67A", rgb(0xF9D67A)), // Gold/Yellow
        ("#A3E635", rgb(0xA3E635)), // Lime
        ("#94A3B8", rgb(0x94A3B8)), // Slate
    ]
}

pub fn parse_color_hex(hex: &str) -> Rgba {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() == 6 {
        if let Ok(num) = u32::from_str_radix(clean, 16) {
            return rgb(num);
        }
    } else if clean.len() == 3 {
        let mut expanded = String::with_capacity(6);
        for c in clean.chars() {
            expanded.push(c);
            expanded.push(c);
        }
        if let Ok(num) = u32::from_str_radix(&expanded, 16) {
            return rgb(num);
        }
    }
    match hex.to_uppercase().as_str() {
        "#8CE0B8" => rgb(0x8CE0B8),
        "#5EEAD4" => rgb(0x5EEAD4),
        "#38BDF8" => rgb(0x38BDF8),
        "#A9C9FF" => rgb(0xA9C9FF),
        "#C4B5FD" => rgb(0xC4B5FD),
        "#F472B6" => rgb(0xF472B6),
        "#F7A8C4" => rgb(0xF7A8C4),
        "#FFA69E" => rgb(0xFFA69E),
        "#FB923C" => rgb(0xFB923C),
        "#F9D67A" => rgb(0xF9D67A),
        "#A3E635" => rgb(0xA3E635),
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
