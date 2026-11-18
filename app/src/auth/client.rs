//! HTTP API client for backend authentication.

use std::env;
use umuryango_shared::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest};

/// Resolves the backend API base URL from environment or defaults to local dev server.
pub fn api_base_url() -> String {
    env::var("API_URL")
        .or_else(|_| env::var("NEXT_PUBLIC_API_URL"))
        .unwrap_or_else(|_| "http://localhost:8080/api/v1".to_string())
}

/// Calls backend `POST /api/v1/auth/login` to authenticate an existing user.
pub fn api_login(identifier: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/login", api_base_url());
    let payload = LoginRequest {
        identifier: identifier.to_string(),
        password: password.to_string(),
    };

    let response = match ureq::post(&url).send_json(&payload) {
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
        .into_json::<AuthResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
}

/// Calls backend `POST /api/v1/auth/register` to register a new user account.
pub fn api_register(name: &str, phone: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/register", api_base_url());
    let payload = RegisterRequest {
        name: name.to_string(),
        phone: phone.to_string(),
        email: None,
        password: password.to_string(),
    };

    let response = match ureq::post(&url).send_json(&payload) {
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
        .into_json::<AuthResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
}
