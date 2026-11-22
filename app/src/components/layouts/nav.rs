use crate::icons;
use gpui_kit::{App, Window};
use std::rc::Rc;

pub type SectionSelectHandler = Rc<dyn Fn(AppSection, &mut Window, &mut App)>;
pub type WorkspaceModeSelectHandler = Rc<dyn Fn(WorkspaceMode, &mut Window, &mut App)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WorkspaceMode {
    #[default]
    Family,
    Personal,
}

impl WorkspaceMode {
    pub const ALL: [Self; 2] = [Self::Family, Self::Personal];

    pub fn id(self) -> &'static str {
        match self {
            Self::Family => "family",
            Self::Personal => "personal",
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            Self::Family => "workspace.family_mode",
            Self::Personal => "workspace.personal_mode",
        }
    }

    pub fn title_key(self) -> &'static str {
        match self {
            Self::Family => "workspace.family",
            Self::Personal => "workspace.personal",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Family => icons::ICON_PEOPLE_ROOF,
            Self::Personal => icons::ICON_USER,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppSection {
    #[default]
    Home,
    Money,
    Campaigns,
    Projects,
    Events,
    Vault,
    Reports,
    Members,
    Profile,
}

impl AppSection {
    pub const ALL: [Self; 9] = [
        Self::Home,
        Self::Money,
        Self::Campaigns,
        Self::Projects,
        Self::Events,
        Self::Vault,
        Self::Reports,
        Self::Members,
        Self::Profile,
    ];

    pub const FAMILY_SECTIONS: [Self; 8] = [
        Self::Home,
        Self::Money,
        Self::Campaigns,
        Self::Projects,
        Self::Events,
        Self::Vault,
        Self::Reports,
        Self::Members,
    ];

    pub const PERSONAL_SECTIONS: [Self; 4] = [Self::Home, Self::Money, Self::Vault, Self::Profile];

    pub fn sections_for(mode: WorkspaceMode) -> &'static [Self] {
        match mode {
            WorkspaceMode::Family => &Self::FAMILY_SECTIONS,
            WorkspaceMode::Personal => &Self::PERSONAL_SECTIONS,
        }
    }

    pub fn mobile_sections_for(mode: WorkspaceMode) -> &'static [Self] {
        match mode {
            WorkspaceMode::Family => &[
                Self::Home,
                Self::Money,
                Self::Campaigns,
                Self::Projects,
                Self::Members,
            ],
            WorkspaceMode::Personal => &Self::PERSONAL_SECTIONS,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Money => "money",
            Self::Campaigns => "campaigns",
            Self::Projects => "projects",
            Self::Events => "events",
            Self::Vault => "vault",
            Self::Reports => "reports",
            Self::Members => "members",
            Self::Profile => "profile",
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            Self::Home => "nav.home",
            Self::Money => "nav.money",
            Self::Campaigns => "nav.campaigns",
            Self::Projects => "nav.projects",
            Self::Events => "nav.events",
            Self::Vault => "nav.vault",
            Self::Reports => "nav.reports",
            Self::Members => "nav.members",
            Self::Profile => "nav.profile",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Home => icons::ICON_HOUSE,
            Self::Money => icons::ICON_WALLET,
            Self::Campaigns => icons::ICON_HAND_HOLDING_DOLLAR,
            Self::Projects => icons::ICON_BULLSEYE,
            Self::Events => icons::ICON_CALENDAR,
            Self::Vault => icons::ICON_SHIELD_HALVED,
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
                AppSection::Campaigns,
                AppSection::Projects,
                AppSection::Events,
                AppSection::Vault,
                AppSection::Reports,
                AppSection::Members,
                AppSection::Profile,
            ]
        );
    }

    #[test]
    fn workspace_modes_match_expected() {
        assert_eq!(
            WorkspaceMode::ALL,
            [WorkspaceMode::Family, WorkspaceMode::Personal]
        );
        assert_eq!(
            AppSection::sections_for(WorkspaceMode::Family),
            AppSection::FAMILY_SECTIONS
        );
        assert_eq!(
            AppSection::sections_for(WorkspaceMode::Personal),
            AppSection::PERSONAL_SECTIONS
        );
        assert_eq!(
            AppSection::mobile_sections_for(WorkspaceMode::Personal),
            AppSection::PERSONAL_SECTIONS
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

    #[test]
    fn workspace_labels_and_icons_exist() {
        for mode in WorkspaceMode::ALL {
            let _ = icons::icon(mode.icon());
            for language in [Language::English, Language::Kinyarwanda] {
                let translated = text(language, mode.label_key());
                assert_ne!(
                    translated,
                    mode.label_key(),
                    "missing translation for {} in {:?}",
                    mode.label_key(),
                    language
                );
            }
        }
    }
}
