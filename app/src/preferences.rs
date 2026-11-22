//! Persists user theme, style, mood, and language preferences to local SQLite storage.

use crate::brand::ThemeMode;
use crate::db::{get_app_state, open_local, set_app_state};
use crate::i18n::Language;
use crate::theme::{CozyMood, UiStyle};
use rusqlite::{Connection, Result};

pub const PREF_THEME_MODE: &str = "preference.theme_mode";
pub const PREF_UI_STYLE: &str = "preference.ui_style";
pub const PREF_COZY_MOOD: &str = "preference.cozy_mood";
pub const PREF_LANGUAGE: &str = "preference.language";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppPreferences {
    pub theme: ThemeMode,
    pub style: UiStyle,
    pub mood: CozyMood,
    pub language: Language,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            style: UiStyle::CozyKawaii,
            mood: CozyMood::Matcha,
            language: Language::English,
        }
    }
}

pub fn load(connection: &Connection) -> AppPreferences {
    let read = |key| get_app_state(connection, key).ok().flatten();

    let theme = match read(PREF_THEME_MODE).as_deref() {
        Some("light") => ThemeMode::Light,
        Some("dark") => ThemeMode::Dark,
        _ => ThemeMode::Dark,
    };

    let style = match read(PREF_UI_STYLE).as_deref() {
        Some("minimalist") => UiStyle::Minimalist,
        Some("cozy") => UiStyle::CozyKawaii,
        _ => UiStyle::CozyKawaii,
    };

    let mood = match read(PREF_COZY_MOOD).as_deref() {
        Some("peach") => CozyMood::Peach,
        Some("berry") => CozyMood::Berry,
        Some("matcha") => CozyMood::Matcha,
        _ => CozyMood::Matcha,
    };

    let language = match read(PREF_LANGUAGE).as_deref() {
        Some("rw") => Language::Kinyarwanda,
        Some("en") => Language::English,
        _ => Language::English,
    };

    AppPreferences {
        theme,
        style,
        mood,
        language,
    }
}

pub fn save(connection: &Connection, prefs: &AppPreferences) -> Result<()> {
    let theme_str = match prefs.theme {
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    let style_str = match prefs.style {
        UiStyle::Minimalist => "minimalist",
        UiStyle::CozyKawaii => "cozy",
        UiStyle::NeoBrutalism => "brutal",
        UiStyle::Claymorphism3D => "clay",
    };
    let mood_str = match prefs.mood {
        CozyMood::Matcha => "matcha",
        CozyMood::Peach => "peach",
        CozyMood::Berry => "berry",
    };
    let lang_str = prefs.language.locale_code();

    let transaction = connection.unchecked_transaction()?;
    set_app_state(&transaction, PREF_THEME_MODE, Some(theme_str))?;
    set_app_state(&transaction, PREF_UI_STYLE, Some(style_str))?;
    set_app_state(&transaction, PREF_COZY_MOOD, Some(mood_str))?;
    set_app_state(&transaction, PREF_LANGUAGE, Some(lang_str))?;
    transaction.commit()
}

/// Standalone fallback helper that opens `umuryango.db` directly to persist theme changes.
pub fn save_theme_to_db(style: UiStyle, mode: ThemeMode, mood: CozyMood) {
    if let Ok(connection) = open_local("umuryango.db") {
        let theme_str = match mode {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        let style_str = match style {
            UiStyle::Minimalist => "minimalist",
            _ => "cozy",
        };
        let mood_str = match mood {
            CozyMood::Matcha => "matcha",
            CozyMood::Peach => "peach",
            CozyMood::Berry => "berry",
        };
        let _ = set_app_state(&connection, PREF_THEME_MODE, Some(theme_str));
        let _ = set_app_state(&connection, PREF_UI_STYLE, Some(style_str));
        let _ = set_app_state(&connection, PREF_COZY_MOOD, Some(mood_str));
    }
}

#[allow(dead_code)]
pub fn save_language_to_db(language: Language) {
    if let Ok(connection) = open_local("umuryango.db") {
        let _ = set_app_state(&connection, PREF_LANGUAGE, Some(language.locale_code()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn database() -> Connection {
        open_local(":memory:").expect("in-memory database must open")
    }

    #[test]
    fn defaults_when_nothing_saved() {
        let connection = database();
        assert_eq!(load(&connection), AppPreferences::default());
    }

    #[test]
    fn round_trips_custom_preferences() {
        let connection = database();
        let custom = AppPreferences {
            theme: ThemeMode::Light,
            style: UiStyle::Minimalist,
            mood: CozyMood::Peach,
            language: Language::Kinyarwanda,
        };
        save(&connection, &custom).expect("saving preferences should succeed");
        assert_eq!(load(&connection), custom);
    }

    #[test]
    fn round_trips_dark_cozy_matcha() {
        let connection = database();
        let custom = AppPreferences {
            theme: ThemeMode::Dark,
            style: UiStyle::CozyKawaii,
            mood: CozyMood::Matcha,
            language: Language::English,
        };
        save(&connection, &custom).expect("saving preferences should succeed");
        assert_eq!(load(&connection), custom);
    }
}
