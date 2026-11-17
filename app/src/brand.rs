#![allow(dead_code)]

use gpui_kit::{rgb, Rgba};

pub const BRAND_ICON: &[u8] = include_bytes!("../../assets/brand/umuryango-inverted-icon.png");
pub const BRAND_WORDMARK: &[u8] = include_bytes!("../../assets/brand/umuryango-wordmark.png");
pub const BRAND_HORIZONTAL_WORDMARK: &[u8] =
    include_bytes!("../../assets/brand/umuryango-horizontal-wordmark.png");
pub const BRAND_FAMILY_ICON: &[u8] = include_bytes!("../../assets/brand/umuryango-family-icon.png");

/// Slate 950 / Deep Midnight Navy - Window background
pub fn color_bg() -> Rgba {
    rgb(0x0a0f1d)
}

/// Slate 900 - Primary surface
pub fn color_surface() -> Rgba {
    rgb(0x0f172a)
}

/// Slate 800 - Elevated card surface
pub fn color_card() -> Rgba {
    rgb(0x131d31)
}

/// Slate 700 - Subdued card & field border
pub fn color_border() -> Rgba {
    rgb(0x273654)
}

/// Slate 600 - Hover border
pub fn color_border_hover() -> Rgba {
    rgb(0x3b4f74)
}

/// Umuryango Brand Emerald Green - Growth, harmony, family trust
pub fn color_primary() -> Rgba {
    rgb(0x059669)
}

/// Emerald 500 - Light accent
pub fn color_primary_light() -> Rgba {
    rgb(0x10b981)
}

/// Emerald 700 - Hover state
pub fn color_primary_hover() -> Rgba {
    rgb(0x047857)
}

/// Rwandan Sun Amber / Gold - Communal wealth & prosperity
pub fn color_accent_gold() -> Rgba {
    rgb(0xd97706)
}

/// Sky 600 - Secondary accent / Information
pub fn color_accent_blue() -> Rgba {
    rgb(0x0284c7)
}

/// Slate 50 - High contrast text
pub fn color_text_primary() -> Rgba {
    rgb(0xf8fafc)
}

/// Slate 300 - Standard readable text
pub fn color_text_body() -> Rgba {
    rgb(0xe2e8f0)
}

/// Slate 400 - Secondary / label text
pub fn color_text_muted() -> Rgba {
    rgb(0x94a3b8)
}

/// Slate 500 - Placeholder / subtle hints
pub fn color_text_dim() -> Rgba {
    rgb(0x64748b)
}

/// Red 500 - Error / Alert
pub fn color_error() -> Rgba {
    rgb(0xef4444)
}

/// Emerald 400 - Success
pub fn color_success() -> Rgba {
    rgb(0x34d399)
}
