//! Persists the last visited screen so the app reopens where the user left off.

use rusqlite::{Connection, Result};
use umuryango_app::db::{get_app_state, set_app_state};

const SCREEN_KEY: &str = "navigation.last_screen";
const SESSION_TOKEN_KEY: &str = "session.token";
const MEMBER_NAME_KEY: &str = "session.member_name";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuthScreen {
    #[default]
    Login,
    Register,
    Terms,
    FamilySetup,
    CreateFamily,
    JoinFamily,
}

impl AuthScreen {
    fn key(self) -> &'static str {
        match self {
            Self::Login => "login",
            Self::Register => "register",
            Self::Terms => "terms",
            Self::FamilySetup => "family_setup",
            Self::CreateFamily => "create_family",
            Self::JoinFamily => "join_family",
        }
    }

    fn from_key(key: &str) -> Option<Self> {
        match key {
            "login" => Some(Self::Login),
            "register" => Some(Self::Register),
            "terms" => Some(Self::Terms),
            "family_setup" => Some(Self::FamilySetup),
            "create_family" => Some(Self::CreateFamily),
            "join_family" => Some(Self::JoinFamily),
            _ => None,
        }
    }

    /// Screens that call authenticated endpoints and need a signed-in session.
    pub fn requires_session(self) -> bool {
        matches!(
            self,
            Self::FamilySetup | Self::CreateFamily | Self::JoinFamily
        )
    }
}

/// Navigation state restored on launch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SavedNavigation {
    pub screen: AuthScreen,
    /// Session token, only kept while the user is on a screen that needs it.
    pub token: Option<String>,
    pub member_name: Option<String>,
}

/// Loads the last visited screen. Falls back to login when the saved screen is
/// unknown or needs a session that is no longer available.
pub fn load(connection: &Connection) -> SavedNavigation {
    let read = |key| get_app_state(connection, key).ok().flatten();
    let screen = read(SCREEN_KEY)
        .and_then(|key| AuthScreen::from_key(&key))
        .unwrap_or_default();

    if !screen.requires_session() {
        return SavedNavigation {
            screen,
            ..SavedNavigation::default()
        };
    }

    match read(SESSION_TOKEN_KEY) {
        Some(token) => SavedNavigation {
            screen,
            token: Some(token),
            member_name: read(MEMBER_NAME_KEY),
        },
        None => SavedNavigation::default(),
    }
}

/// Saves the current screen. Session data is only written for screens that
/// need it and is removed otherwise.
pub fn save(connection: &Connection, state: &SavedNavigation) -> Result<()> {
    let keep_session = state.screen.requires_session();
    let token = state.token.as_deref().filter(|_| keep_session);
    let member_name = state.member_name.as_deref().filter(|_| keep_session);

    let transaction = connection.unchecked_transaction()?;
    set_app_state(&transaction, SCREEN_KEY, Some(state.screen.key()))?;
    set_app_state(&transaction, SESSION_TOKEN_KEY, token)?;
    set_app_state(&transaction, MEMBER_NAME_KEY, member_name)?;
    transaction.commit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use umuryango_app::db::open_local;

    fn database() -> Connection {
        open_local(":memory:").expect("in-memory database opens")
    }

    #[test]
    fn defaults_to_login_when_nothing_saved() {
        assert_eq!(load(&database()), SavedNavigation::default());
    }

    #[test]
    fn restores_public_screen_without_session() {
        let connection = database();
        save(
            &connection,
            &SavedNavigation {
                screen: AuthScreen::Terms,
                token: Some("token".into()),
                member_name: Some("Aline".into()),
            },
        )
        .unwrap();

        let restored = load(&connection);
        assert_eq!(restored.screen, AuthScreen::Terms);
        assert_eq!(restored.token, None);
        assert_eq!(get_app_state(&connection, SESSION_TOKEN_KEY).unwrap(), None);
    }

    #[test]
    fn restores_session_screen_with_token_and_member_name() {
        let connection = database();
        let state = SavedNavigation {
            screen: AuthScreen::JoinFamily,
            token: Some("token".into()),
            member_name: Some("Aline".into()),
        };
        save(&connection, &state).unwrap();

        assert_eq!(load(&connection), state);
    }

    #[test]
    fn session_screen_without_token_falls_back_to_login() {
        let connection = database();
        save(
            &connection,
            &SavedNavigation {
                screen: AuthScreen::CreateFamily,
                token: None,
                member_name: Some("Aline".into()),
            },
        )
        .unwrap();

        assert_eq!(load(&connection), SavedNavigation::default());
    }

    #[test]
    fn unknown_saved_screen_falls_back_to_login() {
        let connection = database();
        set_app_state(&connection, SCREEN_KEY, Some("dashboard")).unwrap();

        assert_eq!(load(&connection).screen, AuthScreen::Login);
    }

    #[test]
    fn screen_keys_round_trip() {
        for screen in [
            AuthScreen::Login,
            AuthScreen::Register,
            AuthScreen::Terms,
            AuthScreen::FamilySetup,
            AuthScreen::CreateFamily,
            AuthScreen::JoinFamily,
        ] {
            assert_eq!(AuthScreen::from_key(screen.key()), Some(screen));
        }
    }
}
