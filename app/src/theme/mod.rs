#![allow(dead_code)]

use crate::brand::ThemeMode;
use gpui_kit::component::theme::{Theme, ThemeMode as ComponentThemeMode};
use gpui_kit::*;
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::{Once, OnceLock, RwLock};

pub const COZY_KAWAII_FONT_FAMILY: &str = "Fredoka";
pub const DEFAULT_UI_FONT_FAMILY: &str = ".SystemUIFont";

static FREDOKA_FONT_TTF: &[u8] = include_bytes!("../../../assets/fonts/Fredoka.ttf");

pub fn ensure_fonts_registered(cx: &App) {
    static REGISTER_FONTS: Once = Once::new();
    REGISTER_FONTS.call_once(|| {
        if let Err(err) = cx
            .text_system()
            .add_fonts(vec![Cow::Borrowed(FREDOKA_FONT_TTF)])
        {
            eprintln!("failed to register Fredoka font: {err}");
        }
    });
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum UiStyle {
    #[default]
    CozyKawaii,
    NeoBrutalism,
    Claymorphism3D,
}

impl UiStyle {
    pub const ALL: [Self; 3] = [Self::CozyKawaii, Self::NeoBrutalism, Self::Claymorphism3D];

    pub fn label(self) -> &'static str {
        match self {
            Self::CozyKawaii => "Cozy Kawaii",
            Self::NeoBrutalism => "Neo-Brutalism",
            Self::Claymorphism3D => "3D Claymorphism",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CozyMood {
    #[default]
    Matcha,
    Peach,
    Berry,
}

impl CozyMood {
    pub const ALL: [Self; 3] = [Self::Matcha, Self::Peach, Self::Berry];

    pub fn next(self) -> Self {
        match self {
            Self::Matcha => Self::Peach,
            Self::Peach => Self::Berry,
            Self::Berry => Self::Matcha,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Matcha => "Matcha",
            Self::Peach => "Peach",
            Self::Berry => "Berry",
        }
    }

    pub fn swatch_color(self) -> Rgba {
        match self {
            Self::Matcha => rgb(0x9BE0B8),
            Self::Peach => rgb(0xFFAD99),
            Self::Berry => rgb(0xF7A8C4),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StickerShadow {
    pub color: Hsla,
    pub offset: Point<Pixels>,
    pub blur_radius: Pixels,
    pub spread_radius: Pixels,
    pub inset: bool,
}

impl StickerShadow {
    pub fn to_box_shadow(self) -> BoxShadow {
        BoxShadow {
            color: self.color,
            offset: self.offset,
            blur_radius: self.blur_radius,
            spread_radius: self.spread_radius,
            inset: self.inset,
        }
    }
}

impl From<StickerShadow> for BoxShadow {
    fn from(value: StickerShadow) -> Self {
        value.to_box_shadow()
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

#[derive(Clone, Copy, Debug)]
pub struct StyleTokens {
    pub style: UiStyle,
    pub mode: ThemeMode,
    pub mood: CozyMood,
    pub font_family: &'static str,

    // Surfaces
    pub bg_canvas: Rgba,
    pub bg_card: Rgba,
    pub bg_elevated: Rgba,
    pub bg_nav: Rgba,
    pub bg_input: Rgba,
    pub bg_input_focus: Rgba,
    pub bg_badge: Rgba,

    // Accents & Text
    pub accent_primary: Rgba,
    pub accent_rim: Rgba,
    pub accent_secondary: Rgba,
    pub accent_link: Rgba,
    pub accent_gold: Rgba,
    pub text_on_accent: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,

    // Semantic Financial Tokens
    pub income_bg: Rgba,
    pub income_text: Rgba,
    pub income_border: Rgba,
    pub expense_bg: Rgba,
    pub expense_text: Rgba,
    pub expense_border: Rgba,
    pub loan_bg: Rgba,
    pub loan_text: Rgba,
    pub loan_border: Rgba,

    // Geometry & Outlines
    pub border_color: Rgba,
    pub border_width: Pixels,
    pub border_width_input: Pixels,
    pub radius_card: Pixels,
    pub radius_input: Pixels,
    pub radius_button: Pixels,
    pub radius_checkbox: Pixels,

    // Tactile Sticker Shadows
    pub shadow_rim: Rgba,
    pub shadow_card: StickerShadow,
    pub shadow_button: StickerShadow,
    pub shadow_button_active: StickerShadow,
    pub shadow_input: StickerShadow,
    pub shadow_input_inset: StickerShadow,
    pub shadow_checkbox: StickerShadow,

    // Legacy compatibility colors accessible via Deref
    pub legacy: ThemeColors,
}

impl Global for StyleTokens {}

impl Deref for StyleTokens {
    type Target = ThemeColors;

    fn deref(&self) -> &Self::Target {
        &self.legacy
    }
}

impl Default for StyleTokens {
    fn default() -> Self {
        Self::new(
            UiStyle::default(),
            ThemeMode::default(),
            CozyMood::default(),
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct ActiveThemeConfig {
    style: UiStyle,
    mode: ThemeMode,
    mood: CozyMood,
}

impl Default for ActiveThemeConfig {
    fn default() -> Self {
        Self {
            style: UiStyle::CozyKawaii,
            mode: ThemeMode::Dark,
            mood: CozyMood::Matcha,
        }
    }
}

fn active_config_store() -> &'static RwLock<ActiveThemeConfig> {
    static STORE: OnceLock<RwLock<ActiveThemeConfig>> = OnceLock::new();
    STORE.get_or_init(|| RwLock::new(ActiveThemeConfig::default()))
}

pub fn active_style() -> UiStyle {
    active_config_store()
        .read()
        .map(|cfg| cfg.style)
        .unwrap_or_default()
}

pub fn active_mood() -> CozyMood {
    active_config_store()
        .read()
        .map(|cfg| cfg.mood)
        .unwrap_or_default()
}

pub fn active_mode() -> ThemeMode {
    active_config_store()
        .read()
        .map(|cfg| cfg.mode)
        .unwrap_or_default()
}

pub fn active_tokens(mode: ThemeMode) -> StyleTokens {
    let (style, mood) = active_config_store()
        .read()
        .map(|cfg| (cfg.style, cfg.mood))
        .unwrap_or((UiStyle::CozyKawaii, CozyMood::Matcha));
    StyleTokens::new(style, mode, mood)
}

pub fn current_tokens(cx: &App) -> StyleTokens {
    cx.try_global::<StyleTokens>()
        .copied()
        .unwrap_or_else(|| active_tokens(active_mode()))
}

pub fn set_style_tokens(
    style: UiStyle,
    mode: ThemeMode,
    mood: CozyMood,
    cx: &mut App,
) -> StyleTokens {
    if let Ok(mut guard) = active_config_store().write() {
        guard.style = style;
        guard.mode = mode;
        guard.mood = mood;
    }

    let tokens = StyleTokens::new(style, mode, mood);
    cx.set_global(tokens);
    sync_kit_themes(&tokens, cx);
    tokens
}

pub fn set_cozy_mood(mood: CozyMood, cx: &mut App) -> StyleTokens {
    let (style, mode) = active_config_store()
        .read()
        .map(|cfg| (cfg.style, cfg.mode))
        .unwrap_or((UiStyle::CozyKawaii, ThemeMode::Dark));
    set_style_tokens(style, mode, mood, cx)
}

pub fn set_ui_style(style: UiStyle, cx: &mut App) -> StyleTokens {
    let (mode, mood) = active_config_store()
        .read()
        .map(|cfg| (cfg.mode, cfg.mood))
        .unwrap_or((ThemeMode::Dark, CozyMood::Matcha));
    set_style_tokens(style, mode, mood, cx)
}

impl StyleTokens {
    pub fn new(style: UiStyle, mode: ThemeMode, mood: CozyMood) -> Self {
        match style {
            UiStyle::CozyKawaii => Self::cozy_kawaii(mode, mood),
            UiStyle::NeoBrutalism => Self::neo_brutalism(mode, mood),
            UiStyle::Claymorphism3D => Self::claymorphism_3d(mode, mood),
        }
    }

    pub fn cozy_kawaii(mode: ThemeMode, mood: CozyMood) -> Self {
        let (accent_primary, accent_rim, accent_secondary, text_on_accent) = match mood {
            CozyMood::Matcha => (rgb(0x9BE0B8), rgb(0x56A67F), rgb(0xFFB4A2), rgb(0x1A2B22)),
            CozyMood::Peach => (rgb(0xFFAD99), rgb(0xD4745D), rgb(0xFFE099), rgb(0x331914)),
            CozyMood::Berry => (rgb(0xF7A8C4), rgb(0xC46A8B), rgb(0xA9C9FF), rgb(0x2E1320)),
        };

        let (
            bg_canvas,
            bg_card,
            bg_elevated,
            bg_nav,
            bg_input,
            bg_input_focus,
            bg_badge,
            border_color,
            shadow_rim,
            text_primary,
            text_secondary,
            text_muted,
            error,
            success,
            accent_gold,
            accent_blue,
            income_text,
            income_border,
            expense_text,
            expense_border,
            loan_text,
            loan_border,
        ) = match mode {
            ThemeMode::Dark => (
                rgb(0x1A151B),
                rgb(0x251E27),
                rgb(0x302734),
                rgb(0x211B24),
                rgb(0x1D1720),
                rgb(0x221B26),
                rgb(0x322836),
                rgb(0x524458),
                rgb(0x110D12),
                rgb(0xFAF2EA),
                rgb(0xD6C8D4),
                rgb(0x8E7F91),
                rgb(0xFF7B89),
                rgb(0x9BE0B8),
                rgb(0xF9D67A),
                rgb(0xA9C9FF),
                rgb(0x8CE0B8),
                rgb(0x489974),
                rgb(0xFFA69E),
                rgb(0xB85D56),
                rgb(0xF9D67A),
                rgb(0xB38E36),
            ),
            ThemeMode::Light => (
                rgb(0xFDF7EE),
                rgb(0xFFFDF9),
                rgb(0xF6EDE0),
                rgb(0xFFFAF2),
                rgb(0xF7EFE4),
                rgb(0xFFFBF5),
                rgb(0xF1E3CF),
                rgb(0x4A3728),
                rgb(0xE6D8C5),
                rgb(0x3B2B20),
                rgb(0x5E4738),
                rgb(0x8C7362),
                rgb(0xD94E4E),
                rgb(0x2F8F5B),
                rgb(0xC97D1B),
                rgb(0x3B76B8),
                rgb(0x277351),
                rgb(0x489974),
                rgb(0xB3433B),
                rgb(0xB85D56),
                rgb(0x8F6814),
                rgb(0xB38E36),
            ),
        };

        let income_bg = rgb(0x8CE0B8).opacity(0.16);
        let expense_bg = rgb(0xFFA69E).opacity(0.16);
        let loan_bg = rgb(0xF9D67A).opacity(0.16);

        let accent_link = accent_primary;
        let rim_hsla: Hsla = shadow_rim.into();

        let shadow_card = StickerShadow {
            color: rim_hsla,
            offset: point(px(0.0), px(10.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: false,
        };
        let shadow_button = StickerShadow {
            color: rim_hsla,
            offset: point(px(0.0), px(6.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: false,
        };
        let shadow_button_active = StickerShadow {
            color: rim_hsla,
            offset: point(px(0.0), px(2.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: false,
        };
        let shadow_input = StickerShadow {
            color: rim_hsla,
            offset: point(px(0.0), px(5.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: false,
        };
        let shadow_input_inset = StickerShadow {
            color: rim_hsla,
            offset: point(px(0.0), px(-3.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: true,
        };
        let shadow_checkbox = StickerShadow {
            color: rim_hsla,
            offset: point(px(0.0), px(3.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: false,
        };

        let legacy = ThemeColors {
            bg: bg_canvas,
            surface: bg_input,
            card: bg_card,
            border: border_color,
            border_hover: accent_primary,
            primary: accent_primary,
            primary_light: accent_primary,
            primary_hover: accent_secondary,
            accent_gold,
            accent_blue,
            text_primary,
            text_body: text_secondary,
            text_muted,
            text_dim: text_muted,
            error,
            success,
        };

        Self {
            style: UiStyle::CozyKawaii,
            mode,
            mood,
            font_family: COZY_KAWAII_FONT_FAMILY,
            bg_canvas,
            bg_card,
            bg_elevated,
            bg_nav,
            bg_input,
            bg_input_focus,
            bg_badge,
            accent_primary,
            accent_rim,
            accent_secondary,
            accent_link,
            accent_gold,
            text_on_accent,
            text_primary,
            text_secondary,
            text_muted,
            income_bg,
            income_text,
            income_border,
            expense_bg,
            expense_text,
            expense_border,
            loan_bg,
            loan_text,
            loan_border,
            border_color,
            border_width: px(3.0),
            border_width_input: px(2.5),
            radius_card: px(32.0),
            radius_input: px(20.0),
            radius_button: px(22.0),
            radius_checkbox: px(10.0),
            shadow_rim,
            shadow_card,
            shadow_button,
            shadow_button_active,
            shadow_input,
            shadow_input_inset,
            shadow_checkbox,
            legacy,
        }
    }

    pub fn neo_brutalism(mode: ThemeMode, mood: CozyMood) -> Self {
        let mut tokens = Self::cozy_kawaii(mode, mood);
        tokens.style = UiStyle::NeoBrutalism;
        tokens.font_family = DEFAULT_UI_FONT_FAMILY;
        tokens.radius_card = px(6.0);
        tokens.radius_input = px(4.0);
        tokens.radius_button = px(4.0);
        tokens.radius_checkbox = px(4.0);
        tokens
    }

    pub fn claymorphism_3d(mode: ThemeMode, mood: CozyMood) -> Self {
        let mut tokens = Self::cozy_kawaii(mode, mood);
        tokens.style = UiStyle::Claymorphism3D;
        tokens.font_family = DEFAULT_UI_FONT_FAMILY;
        tokens.shadow_card.blur_radius = px(14.0);
        tokens.shadow_button.blur_radius = px(8.0);
        tokens
    }

    pub fn card_shadows(&self) -> Vec<BoxShadow> {
        vec![self.shadow_card.into()]
    }

    pub fn button_shadows(&self) -> Vec<BoxShadow> {
        vec![self.shadow_button.into()]
    }

    pub fn button_active_shadows(&self) -> Vec<BoxShadow> {
        vec![self.shadow_button_active.into()]
    }

    pub fn input_shadows(&self) -> Vec<BoxShadow> {
        vec![self.shadow_input.into(), self.shadow_input_inset.into()]
    }

    pub fn input_focus_shadows(&self) -> Vec<BoxShadow> {
        let glow = BoxShadow {
            color: self.accent_primary.opacity(0.32).into(),
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            inset: false,
        };
        vec![
            self.shadow_input.into(),
            self.shadow_input_inset.into(),
            glow,
        ]
    }

    pub fn checkbox_shadows(&self) -> Vec<BoxShadow> {
        vec![self.shadow_checkbox.into()]
    }
}

pub fn sync_kit_themes(tokens: &StyleTokens, cx: &mut App) {
    ensure_fonts_registered(cx);
    let component_mode = match tokens.mode {
        ThemeMode::Dark => ComponentThemeMode::Dark,
        ThemeMode::Light => ComponentThemeMode::Light,
    };
    Theme::change(component_mode, None, cx);
    Theme::update(cx, |theme| {
        theme.font_family = tokens.font_family.into();
        theme.colors.caret = tokens.accent_primary.into();
        theme.colors.foreground = tokens.text_primary.into();
        theme.colors.muted_foreground = tokens.text_muted.into();
        theme.colors.background = tokens.bg_canvas.into();
        theme.colors.input = tokens.border_color.into();
        theme.colors.border = tokens.border_color.into();
        theme.colors.ring = tokens.accent_primary.into();
        theme.colors.primary = tokens.accent_primary.into();
        theme.colors.primary_hover = tokens.accent_secondary.into();
        theme.colors.primary_active = tokens.accent_primary.into();
        theme.colors.primary_foreground = tokens.text_on_accent.into();
        theme.colors.button_primary = tokens.accent_primary.into();
        theme.colors.button_primary_hover = tokens.accent_secondary.into();
        theme.colors.button_primary_active = tokens.accent_primary.into();
        theme.colors.button_primary_foreground = tokens.text_on_accent.into();
        theme.colors.secondary = tokens.bg_input.into();
        theme.colors.secondary_hover = tokens.bg_input_focus.into();
        theme.colors.secondary_active = tokens.bg_card.into();
        theme.colors.secondary_foreground = tokens.text_primary.into();
        theme.colors.accent = tokens.accent_primary.into();
        theme.colors.accent_foreground = tokens.text_on_accent.into();
        theme.colors.link = tokens.accent_link.into();
        theme.colors.link_hover = tokens.accent_secondary.into();
        theme.colors.popover = tokens.bg_card.into();
        theme.colors.popover_foreground = tokens.text_primary.into();
        theme.radius = tokens.radius_input;
        theme.radius_lg = tokens.radius_card;
    });
}

#[cfg(feature = "rune-design")]
pub fn sync_rune_kit_palette(tokens: &StyleTokens, kit: &mut rune_kit::Kit) {
    let to_rune = |rgba: Rgba| -> rune::Color {
        rune::Color {
            r: ((rgba.rgb >> 16) & 0xFF) as f32 / 255.0,
            g: ((rgba.rgb >> 8) & 0xFF) as f32 / 255.0,
            b: (rgba.rgb & 0xFF) as f32 / 255.0,
            a: rgba.a,
        }
    };
    kit.palette.page = to_rune(tokens.bg_canvas);
    kit.palette.bg_base = to_rune(tokens.bg_card);
    kit.palette.bg_deep = to_rune(tokens.bg_input);
    kit.palette.bg_accent = to_rune(tokens.accent_primary);
    kit.palette.submit_fg = to_rune(tokens.text_on_accent);
    kit.palette.text_base = to_rune(tokens.text_primary);
    kit.palette.text_muted = to_rune(tokens.text_secondary);
    kit.palette.text_faint = to_rune(tokens.text_muted);
    kit.palette.text_accent = to_rune(tokens.accent_link);
    kit.palette.border_base = to_rune(tokens.border_color);
    kit.palette.border_focus = to_rune(tokens.accent_primary);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::rust_2021::test;

    #[test]
    fn dark_cozy_matcha_palette_matches_spec() {
        let tokens = StyleTokens::new(UiStyle::CozyKawaii, ThemeMode::Dark, CozyMood::Matcha);
        assert_eq!(tokens.bg_canvas, rgb(0x1A151B));
        assert_eq!(tokens.bg_card, rgb(0x251E27));
        assert_eq!(tokens.bg_input, rgb(0x1D1720));
        assert_eq!(tokens.bg_input_focus, rgb(0x221B26));
        assert_eq!(tokens.border_color, rgb(0x524458));
        assert_eq!(tokens.shadow_rim, rgb(0x110D12));
        assert_eq!(tokens.text_primary, rgb(0xFAF2EA));
        assert_eq!(tokens.text_secondary, rgb(0xD6C8D4));
        assert_eq!(tokens.text_muted, rgb(0x8E7F91));
        assert_eq!(tokens.accent_primary, rgb(0x9BE0B8));
        assert_eq!(tokens.accent_secondary, rgb(0xFFB4A2));
        assert_eq!(tokens.text_on_accent, rgb(0x1A2B22));
        assert_eq!(tokens.radius_card, px(32.0));
        assert_eq!(tokens.radius_input, px(20.0));
        assert_eq!(tokens.radius_button, px(22.0));
        assert_eq!(tokens.radius_checkbox, px(10.0));
        assert_eq!(tokens.border_width, px(3.0));
        assert_eq!(tokens.border_width_input, px(2.5));
        assert_eq!(tokens.shadow_card.offset.y, px(10.0));
        assert_eq!(tokens.shadow_button.offset.y, px(6.0));
        assert_eq!(tokens.shadow_button_active.offset.y, px(2.0));
    }

    #[test]
    fn cozy_moods_and_light_mode_match_spec() {
        let peach = StyleTokens::new(UiStyle::CozyKawaii, ThemeMode::Dark, CozyMood::Peach);
        assert_eq!(peach.accent_primary, rgb(0xFFAD99));
        assert_eq!(peach.accent_secondary, rgb(0xFFE099));
        assert_eq!(peach.text_on_accent, rgb(0x331914));

        let berry = StyleTokens::new(UiStyle::CozyKawaii, ThemeMode::Dark, CozyMood::Berry);
        assert_eq!(berry.accent_primary, rgb(0xF7A8C4));
        assert_eq!(berry.accent_secondary, rgb(0xA9C9FF));
        assert_eq!(berry.text_on_accent, rgb(0x2E1320));

        let light = StyleTokens::new(UiStyle::CozyKawaii, ThemeMode::Light, CozyMood::Matcha);
        assert_eq!(light.bg_canvas, rgb(0xFDF7EE));
        assert_eq!(light.bg_card, rgb(0xFFFDF9));
        assert_eq!(light.bg_input, rgb(0xF7EFE4));
        assert_eq!(light.border_color, rgb(0x4A3728));
        assert_eq!(light.shadow_rim, rgb(0xE6D8C5));
        assert_eq!(light.text_primary, rgb(0x3B2B20));
    }

    #[test]
    fn placeholder_styles_preserve_distinct_variants() {
        let cozy = StyleTokens::new(UiStyle::CozyKawaii, ThemeMode::Dark, CozyMood::Matcha);
        assert_eq!(cozy.font_family, "Fredoka");
        assert!(!FREDOKA_FONT_TTF.is_empty());

        let brutal = StyleTokens::new(UiStyle::NeoBrutalism, ThemeMode::Dark, CozyMood::Matcha);
        assert_eq!(brutal.style, UiStyle::NeoBrutalism);
        assert_eq!(brutal.font_family, DEFAULT_UI_FONT_FAMILY);

        let clay = StyleTokens::new(UiStyle::Claymorphism3D, ThemeMode::Dark, CozyMood::Matcha);
        assert_eq!(clay.style, UiStyle::Claymorphism3D);
        assert_eq!(clay.font_family, DEFAULT_UI_FONT_FAMILY);
    }
}
