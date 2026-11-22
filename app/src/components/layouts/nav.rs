use crate::icons;
use gpui_kit::{App, Window};
use std::rc::Rc;

pub type SectionSelectHandler = Rc<dyn Fn(AppSection, &mut Window, &mut App)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppSection {
    #[default]
    Home,
    Money,
    Projects,
    Reports,
    Members,
    Profile,
}

impl AppSection {
    pub const ALL: [Self; 6] = [
        Self::Home,
        Self::Money,
        Self::Projects,
        Self::Reports,
        Self::Members,
        Self::Profile,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Money => "money",
            Self::Projects => "projects",
            Self::Reports => "reports",
            Self::Members => "members",
            Self::Profile => "profile",
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            Self::Home => "nav.home",
            Self::Money => "nav.money",
            Self::Projects => "nav.projects",
            Self::Reports => "nav.reports",
            Self::Members => "nav.members",
            Self::Profile => "nav.profile",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Home => icons::ICON_HOUSE,
            Self::Money => icons::ICON_WALLET,
            Self::Projects => icons::ICON_BULLSEYE,
            Self::Reports => icons::ICON_FILE_LINES,
            Self::Members => icons::ICON_USERS,
            Self::Profile => icons::ICON_USER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{text, Language};

    #[test]
    fn nav_sections_match_expected_order() {
        assert_eq!(
            AppSection::ALL,
            [
                AppSection::Home,
                AppSection::Money,
                AppSection::Projects,
                AppSection::Reports,
                AppSection::Members,
                AppSection::Profile,
            ]
        );
    }

    #[test]
    fn nav_labels_and_icons_exist_for_all_sections() {
        for section in AppSection::ALL {
            let _ = icons::icon(section.icon());
            for language in [Language::English, Language::Kinyarwanda] {
                let translated = text(language, section.label_key());
                assert_ne!(
                    translated,
                    section.label_key(),
                    "missing translation for {} in {:?}",
                    section.label_key(),
                    language
                );
            }
        }
    }
}
