//! HTTP API client for backend authentication and family management.

use std::env;
use std::sync::RwLock;
use umuryango_shared::{
    AuthResponse, CreateFamilyRequest, CreateFamilyResponse, ErrorResponse, JoinFamilyRequest,
    JoinFamilyResponse, LoginRequest, PendingFamilyInviteDto, RegisterRequest,
};

static CURRENT_TOKEN: RwLock<Option<String>> = RwLock::new(None);
static CURRENT_LANGUAGE: RwLock<&'static str> = RwLock::new("en");

/// Stores the current user's session token.
pub fn set_current_token(token: Option<String>) {
    if let Ok(mut lock) = CURRENT_TOKEN.write() {
        *lock = token;
    }
}

/// Retrieves the current user's session token.
pub fn get_current_token() -> Option<String> {
    CURRENT_TOKEN.read().ok().and_then(|lock| lock.clone())
}

/// Stores the current app language code ("en", "rw").
pub fn set_current_language(lang: &'static str) {
    if let Ok(mut lock) = CURRENT_LANGUAGE.write() {
        *lock = lang;
    }
}

/// Retrieves the current app language code.
pub fn get_current_language() -> &'static str {
    CURRENT_LANGUAGE.read().map(|lock| *lock).unwrap_or("en")
}

/// Resolves the backend API base URL from environment or defaults to local dev server.
pub fn api_base_url() -> String {
    env::var("API_URL")
        .or_else(|_| env::var("NEXT_PUBLIC_API_URL"))
        .unwrap_or_else(|_| "http://localhost:4433/api/v1".to_string())
}

/// Calls backend `POST /api/v1/auth/login` to authenticate an existing user.
pub fn api_login(identifier: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/login", api_base_url());
    let payload = LoginRequest {
        identifier: identifier.to_string(),
        password: password.to_string(),
    };

    let response = match ureq::post(&url)
        .set("Accept-Language", get_current_language())
        .send_json(&payload)
    {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            if let Ok(err_body) = resp.into_json::<ErrorResponse>() {
                return Err(err_body.error);
            }
            return Err(format!("Server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error: {}",
                transport.message().unwrap_or("cannot connect to server")
            ));
        }
    };

    let auth_resp = response
        .into_json::<AuthResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))?;

    set_current_token(Some(auth_resp.token.clone()));
    Ok(auth_resp)
}

/// Calls backend `POST /api/v1/auth/register` to register a new user account.
pub fn api_register(name: &str, phone: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/register", api_base_url());
    let payload = RegisterRequest {
        name: name.to_string(),
        phone: phone.to_string(),
        email: None,
        password: password.to_string(),
        avatar_url: None,
    };

    let response = match ureq::post(&url)
        .set("Accept-Language", get_current_language())
        .send_json(&payload)
    {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            if let Ok(err_body) = resp.into_json::<ErrorResponse>() {
                return Err(err_body.error);
            }
            return Err(format!("Server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error: {}",
                transport.message().unwrap_or("cannot connect to server")
            ));
        }
    };

    let auth_resp = response
        .into_json::<AuthResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))?;

    set_current_token(Some(auth_resp.token.clone()));
    Ok(auth_resp)
}

/// Calls backend `POST /api/v1/families` to create a new family and invite members by phone.
pub fn api_create_family(
    name: &str,
    member_phones: Vec<String>,
) -> Result<CreateFamilyResponse, String> {
    let url = format!("{}/families", api_base_url());
    let payload = CreateFamilyRequest {
        name: name.to_string(),
        member_phones,
    };

    let mut req = ureq::post(&url).set("Accept-Language", get_current_language());
    if let Some(token) = get_current_token() {
        req = req.set("Authorization", &format!("Bearer {token}"));
    }

    let response = match req.send_json(&payload) {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            if let Ok(err_body) = resp.into_json::<ErrorResponse>() {
                return Err(err_body.error);
            }
            return Err(format!("Server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error: {}",
                transport.message().unwrap_or("cannot connect to server")
            ));
        }
    };

    response
        .into_json::<CreateFamilyResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
}

/// Calls backend `POST /api/v1/families/join` to join a family with a 4-character code.
pub fn api_join_family(code: &str) -> Result<JoinFamilyResponse, String> {
    let url = format!("{}/families/join", api_base_url());
    let payload = JoinFamilyRequest {
        code: code.to_string(),
    };

    let mut req = ureq::post(&url).set("Accept-Language", get_current_language());
    if let Some(token) = get_current_token() {
        req = req.set("Authorization", &format!("Bearer {token}"));
    }

    let response = match req.send_json(&payload) {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            if let Ok(err_body) = resp.into_json::<ErrorResponse>() {
                return Err(err_body.error);
            }
            return Err(format!("Server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error: {}",
                transport.message().unwrap_or("cannot connect to server")
            ));
        }
    };

    response
        .into_json::<JoinFamilyResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
}

/// Calls backend `GET /api/v1/families/invitations` to list pending invitations for the user.
pub fn api_get_pending_invitations() -> Result<Vec<PendingFamilyInviteDto>, String> {
    let url = format!("{}/families/invitations", api_base_url());
    let mut req = ureq::get(&url).set("Accept-Language", get_current_language());
    if let Some(token) = get_current_token() {
        req = req.set("Authorization", &format!("Bearer {token}"));
    }

    let response = match req.call() {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            if let Ok(err_body) = resp.into_json::<ErrorResponse>() {
                return Err(err_body.error);
            }
            return Err(format!("Server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error: {}",
                transport.message().unwrap_or("cannot connect to server")
            ));
        }
    };

    response
        .into_json::<Vec<PendingFamilyInviteDto>>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
}

/// Calls backend `POST /api/v1/families/invitations/{id}/accept` to accept a pending invite.
pub fn api_accept_invitation(invite_id: &str) -> Result<JoinFamilyResponse, String> {
    let url = format!("{}/families/invitations/{invite_id}/accept", api_base_url());
    let mut req = ureq::post(&url).set("Accept-Language", get_current_language());
    if let Some(token) = get_current_token() {
        req = req.set("Authorization", &format!("Bearer {token}"));
    }

    let response = match req.send_string("") {
        Ok(resp) => resp,
        Err(ureq::Error::Status(status, resp)) => {
            if let Ok(err_body) = resp.into_json::<ErrorResponse>() {
                return Err(err_body.error);
            }
            return Err(format!("Server returned HTTP {status}"));
        }
        Err(ureq::Error::Transport(transport)) => {
            return Err(format!(
                "Network error: {}",
                transport.message().unwrap_or("cannot connect to server")
            ));
        }
    };

    response
        .into_json::<JoinFamilyResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
}
