#![allow(dead_code)]

use std::sync::Arc;

use crate::theme::{active_mood, active_style, active_tokens, set_style_tokens};
pub use crate::theme::{CozyMood, StyleTokens, UiStyle};
use gpui_kit::{img, App, Image, ImageFormat, IntoElement, Pixels, Styled};

pub const BRAND_WORDMARK: &[u8] = include_bytes!("../../assets/brand/umuryango-wordmark.png");
pub const BRAND_WORDMARK_INVERTED: &[u8] =
    include_bytes!("../../assets/brand/umuryango-wordmark-inverted.png");
pub const BRAND_ICON: &[u8] = include_bytes!("../../assets/brand/umuryango-family-icon.png");
pub const BRAND_ICON_INVERTED: &[u8] =
    include_bytes!("../../assets/brand/umuryango-family-icon-inverted.png");
pub const BRAND_ICON_SOLID_DARK: &[u8] =
    include_bytes!("../../assets/brand/umuryango-inverted-icon.png");

pub type ThemeColors = StyleTokens;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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
            Self::Dark => "Light Cozy",
            Self::Light => "Dark Cozy",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }

    pub fn wordmark(self) -> &'static [u8] {
        if self == Self::Dark {
            BRAND_WORDMARK_INVERTED
        } else {
            BRAND_WORDMARK
        }
    }

    pub fn icon(self) -> &'static [u8] {
        if self == Self::Dark {
            BRAND_ICON_INVERTED
        } else {
            BRAND_ICON
        }
    }
}

pub fn brand_logo(theme: ThemeMode, size: Pixels) -> impl IntoElement {
    img(Arc::new(Image::from_bytes(
        ImageFormat::Png,
        theme.icon().to_vec(),
    )))
    .size(size)
}

pub fn apply_theme(mode: ThemeMode, cx: &mut App) {
    let style = active_style();
    let mood = active_mood();
    set_style_tokens(style, mode, mood, cx);
}

pub fn apply_style_and_mood(style: UiStyle, mode: ThemeMode, mood: CozyMood, cx: &mut App) {
    set_style_tokens(style, mode, mood, cx);
}

pub fn colors(mode: ThemeMode) -> StyleTokens {
    active_tokens(mode)
}
