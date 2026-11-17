#![allow(dead_code)]

use gpui_kit::{rgb, Rgba};

pub const BRAND_WORDMARK: &[u8] = include_bytes!("../../assets/brand/umuryango-wordmark.png");
pub const BRAND_WORDMARK_INVERTED: &[u8] =
    include_bytes!("../../assets/brand/umuryango-wordmark-inverted.png");

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Light mode",
            Self::Light => "Dark mode",
        }
    }

    pub fn wordmark(self) -> &'static [u8] {
        match self {
            Self::Dark => BRAND_WORDMARK_INVERTED,
            Self::Light => BRAND_WORDMARK,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ThemeColors {
    pub bg: Rgba,
    pub surface: Rgba,
    pub card: Rgba,
    pub border: Rgba,
    pub border_hover: Rgba,
    pub primary: Rgba,
    pub primary_light: Rgba,
    pub primary_hover: Rgba,
    pub accent_gold: Rgba,
    pub accent_blue: Rgba,
    pub text_primary: Rgba,
    pub text_body: Rgba,
    pub text_muted: Rgba,
    pub text_dim: Rgba,
    pub error: Rgba,
    pub success: Rgba,
}

pub fn colors(mode: ThemeMode) -> ThemeColors {
    match mode {
        ThemeMode::Dark => ThemeColors {
            bg: rgb(0x000000),
            surface: rgb(0x000000),
            card: rgb(0x000000),
            border: rgb(0x262626),
            border_hover: rgb(0x404040),
            primary: rgb(0x059669),
            primary_light: rgb(0x10b981),
            primary_hover: rgb(0x047857),
            accent_gold: rgb(0xd97706),
            accent_blue: rgb(0x0284c7),
            text_primary: rgb(0xffffff),
            text_body: rgb(0xf5f5f5),
            text_muted: rgb(0xa3a3a3),
            text_dim: rgb(0x737373),
            error: rgb(0xef4444),
            success: rgb(0x34d399),
        },
        ThemeMode::Light => ThemeColors {
            bg: rgb(0xf8fafc),
            surface: rgb(0xffffff),
            card: rgb(0xffffff),
            border: rgb(0xe2e8f0),
            border_hover: rgb(0xcbd5e1),
            primary: rgb(0x047857),
            primary_light: rgb(0x059669),
            primary_hover: rgb(0x065f46),
            accent_gold: rgb(0xb45309),
            accent_blue: rgb(0x0369a1),
            text_primary: rgb(0x0f172a),
            text_body: rgb(0x1e293b),
            text_muted: rgb(0x64748b),
            text_dim: rgb(0x94a3b8),
            error: rgb(0xdc2626),
            success: rgb(0x059669),
        },
    }
}
