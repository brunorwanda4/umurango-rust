#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Language {
    #[default]
    English,
    Kinyarwanda,
}

impl Language {
    pub fn toggle(self) -> Self {
        match self {
            Self::English => Self::Kinyarwanda,
            Self::Kinyarwanda => Self::English,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::English => "EN",
            Self::Kinyarwanda => "RW",
        }
    }

    pub fn locale_code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Kinyarwanda => "rw",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::English => "Kinyarwanda",
            Self::Kinyarwanda => "English",
        }
    }
}

fn dictionary(language: Language) -> &'static HashMap<String, String> {
    static ENGLISH: OnceLock<HashMap<String, String>> = OnceLock::new();
    static KINYARWANDA: OnceLock<HashMap<String, String>> = OnceLock::new();

    match language {
        Language::English => ENGLISH.get_or_init(|| {
            serde_json::from_str(include_str!("../../assets/locales/en.json"))
                .expect("English translations must be valid JSON")
        }),
        Language::Kinyarwanda => KINYARWANDA.get_or_init(|| {
            serde_json::from_str(include_str!("../../assets/locales/rw.json"))
                .expect("Kinyarwanda translations must be valid JSON")
        }),
    }
}

pub fn text(language: Language, key: &str) -> String {
    dictionary(language)
        .get(key)
        .cloned()
        .unwrap_or_else(|| key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_and_kinyarwanda_dictionaries_have_matching_non_empty_keys() {
        let en = dictionary(Language::English);
        let rw = dictionary(Language::Kinyarwanda);

        assert!(!en.is_empty(), "English dictionary must not be empty");
        assert_eq!(
            en.len(),
            rw.len(),
            "English ({}) and Kinyarwanda ({}) key counts must match",
            en.len(),
            rw.len()
        );

        for (key, en_val) in en {
            assert!(
                !en_val.trim().is_empty(),
                "English translation for '{key}' must not be empty"
            );
            let rw_val = rw
                .get(key)
                .unwrap_or_else(|| panic!("Missing Kinyarwanda translation for key '{key}'"));
            assert!(
                !rw_val.trim().is_empty(),
                "Kinyarwanda translation for '{key}' must not be empty"
            );
        }
    }
}
