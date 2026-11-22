//! HTTP API client for backend authentication and family management.

use std::env;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;
use umuryango_shared::{
    AuthResponse, CreateFamilyRequest, CreateFamilyResponse, CreateLoanRequest,
    CreateProjectRequest, CreateTransactionRequest, ErrorResponse, Family, FamilyMemberDto,
    InviteFamilyMemberRequest, JoinFamilyRequest, JoinFamilyResponse, LoanDto,
    LoanRepaymentRequest, LoginRequest, PendingFamilyInviteDto, ProjectContributionRequest,
    ProjectDto, RegisterRequest, SyncMutation, SyncPullRequest, SyncPullResponse, SyncPushRequest,
    SyncPushResponse, TransactionItemDto, TreasurySummaryDto, UserDto,
};

/// Production backend API base URL deployed on Render.
pub const PRODUCTION_API_BASE_URL: &str = "https://umurango-rust.onrender.com/api/v1";

static CURRENT_TOKEN: RwLock<Option<String>> = RwLock::new(None);
static CURRENT_LANGUAGE: RwLock<&'static str> = RwLock::new("en");
static CURRENT_FAMILY_ID: RwLock<Option<String>> = RwLock::new(None);
static CURRENT_FAMILY_NAME: RwLock<Option<String>> = RwLock::new(None);
static CURRENT_USER: RwLock<Option<UserDto>> = RwLock::new(None);
static CURRENT_ROLE: RwLock<Option<String>> = RwLock::new(None);
static HTTP_AGENT: OnceLock<ureq::Agent> = OnceLock::new();

fn http_agent() -> &'static ureq::Agent {
    HTTP_AGENT.get_or_init(|| {
        ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(30))
            .timeout_read(Duration::from_secs(45))
            .timeout_write(Duration::from_secs(30))
            .build()
    })
}

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

/// Stores the current authenticated user profile and family role.
pub fn set_current_user(user: Option<UserDto>, role: Option<String>) {
    if let Ok(mut lock) = CURRENT_USER.write() {
        *lock = user;
    }
    if let Ok(mut lock) = CURRENT_ROLE.write() {
        *lock = role;
    }
}

/// Retrieves the current authenticated user profile.
pub fn get_current_user() -> Option<UserDto> {
    CURRENT_USER.read().ok().and_then(|lock| lock.clone())
}

/// Retrieves the current user's family role ("HEAD", "ADMIN", "MEMBER").
pub fn get_current_role() -> Option<String> {
    CURRENT_ROLE.read().ok().and_then(|lock| lock.clone())
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

/// Stores the active family ID and name.
pub fn set_current_family(family_id: Option<String>, family_name: Option<String>) {
    if let Ok(mut lock) = CURRENT_FAMILY_ID.write() {
        *lock = family_id;
    }
    if let Ok(mut lock) = CURRENT_FAMILY_NAME.write() {
        *lock = family_name;
    }
}

/// Retrieves the active family ID.
pub fn get_current_family_id() -> Option<String> {
    CURRENT_FAMILY_ID.read().ok().and_then(|lock| lock.clone())
}

/// Retrieves the active family name.
pub fn get_current_family_name() -> Option<String> {
    CURRENT_FAMILY_NAME
        .read()
        .ok()
        .and_then(|lock| lock.clone())
}

/// Normalizes a backend base URL so that it always points to the `/api/v1` prefix
/// without a trailing slash (e.g. `https://umurango-rust.onrender.com` ->
/// `https://umurango-rust.onrender.com/api/v1`).
pub fn normalize_api_base_url(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return PRODUCTION_API_BASE_URL.to_string();
    }
    if trimmed.ends_with("/api/v1") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/api/v1")
    }
}

/// Returns true if a URL points to the local loopback interface (`127.0.0.1` or `localhost`).
pub fn is_loopback_url(url: &str) -> bool {
    let lower = url.trim().to_ascii_lowercase();
    lower.contains("://127.0.0.1") || lower.contains("://localhost") || lower.contains("://[::1]")
}

/// Resolves the backend API base URL.
///
/// Defaults to the Render production backend (`https://umurango-rust.onrender.com/api/v1`).
/// Can be overridden at runtime via `UMURYANGO_API_URL` or `API_URL` (on Android,
/// loopback URLs such as `127.0.0.1` or `localhost` are always ignored so mobile builds
/// never accidentally target the phone's own loopback interface).
pub fn api_base_url() -> String {
    if let Ok(url) = env::var("UMURYANGO_API_URL").or_else(|_| env::var("API_URL")) {
        let trimmed = url.trim();
        if !trimmed.is_empty() && !(cfg!(target_os = "android") && is_loopback_url(trimmed)) {
            return normalize_api_base_url(trimmed);
        }
    }

    PRODUCTION_API_BASE_URL.to_string()
}

fn authorized_request(req: ureq::Request) -> ureq::Request {
    let mut req = req.set("Accept-Language", get_current_language());
    if let Some(token) = get_current_token() {
        req = req.set("Authorization", &format!("Bearer {token}"));
    }
    req
}

/// Calls backend `POST /api/v1/auth/login` to authenticate an existing user.
pub fn api_login(identifier: &str, password: &str) -> Result<AuthResponse, String> {
    let url = format!("{}/auth/login", api_base_url());
    let payload = LoginRequest {
        identifier: identifier.to_string(),
        password: password.to_string(),
    };

    let response = match http_agent()
        .post(&url)
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
    set_current_user(Some(auth_resp.user.clone()), auth_resp.role.clone());
    if auth_resp.family_id.is_some() {
        set_current_family(auth_resp.family_id.clone(), auth_resp.family_name.clone());
    }
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

    let response = match http_agent()
        .post(&url)
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
    set_current_user(Some(auth_resp.user.clone()), auth_resp.role.clone());
    if auth_resp.family_id.is_some() {
        set_current_family(auth_resp.family_id.clone(), auth_resp.family_name.clone());
    }
    Ok(auth_resp)
}

/// Calls backend `GET /api/v1/auth/me` to fetch the currently authenticated user profile.
pub fn api_get_me() -> Result<UserDto, String> {
    let url = format!("{}/auth/me", api_base_url());
    let req = authorized_request(http_agent().get(&url));

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

    let body = response
        .into_json::<serde_json::Value>()
        .map_err(|e| format!("Failed to parse user profile: {e}"))?;
    let user_val = body.get("user").cloned().unwrap_or(body);
    let user: UserDto = serde_json::from_value(user_val)
        .map_err(|e| format!("Failed to decode user profile: {e}"))?;
    let role = get_current_role();
    set_current_user(Some(user.clone()), role);
    Ok(user)
}

/// Calls backend `GET /api/v1/families` to list families the user belongs to.
pub fn api_list_families() -> Result<Vec<Family>, String> {
    let url = format!("{}/families", api_base_url());
    let req = authorized_request(http_agent().get(&url));

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
        .into_json::<Vec<Family>>()
        .map_err(|e| format!("Failed to parse server response: {e}"))
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

    let req = authorized_request(http_agent().post(&url));
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

    let resp = response
        .into_json::<CreateFamilyResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))?;

    set_current_family(Some(resp.id.clone()), Some(resp.name.clone()));
    Ok(resp)
}

/// Calls backend `POST /api/v1/families/join` to join a family with a 4-character code.
pub fn api_join_family(code: &str) -> Result<JoinFamilyResponse, String> {
    let url = format!("{}/families/join", api_base_url());
    let payload = JoinFamilyRequest {
        code: code.to_string(),
    };

    let req = authorized_request(http_agent().post(&url));
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

    let resp = response
        .into_json::<JoinFamilyResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))?;

    set_current_family(Some(resp.family_id.clone()), Some(resp.family_name.clone()));
    Ok(resp)
}

/// Calls backend `GET /api/v1/families/invitations` to list pending invitations for the user.
pub fn api_get_pending_invitations() -> Result<Vec<PendingFamilyInviteDto>, String> {
    let url = format!("{}/families/invitations", api_base_url());
    let req = authorized_request(http_agent().get(&url));

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
    let req = authorized_request(http_agent().post(&url));

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

    let resp = response
        .into_json::<JoinFamilyResponse>()
        .map_err(|e| format!("Failed to parse server response: {e}"))?;

    set_current_family(Some(resp.family_id.clone()), Some(resp.family_name.clone()));
    Ok(resp)
}

/// Calls backend `GET /api/v1/families/{family_id}/transactions/summary` to fetch treasury metrics.
pub fn api_get_treasury_summary(family_id: &str) -> Result<TreasurySummaryDto, String> {
    let url = format!(
        "{}/families/{family_id}/transactions/summary",
        api_base_url()
    );
    let req = authorized_request(http_agent().get(&url));

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
        .into_json::<TreasurySummaryDto>()
        .map_err(|e| format!("Failed to parse treasury summary: {e}"))
}

/// Calls backend `GET /api/v1/families/{family_id}/transactions` to fetch transactions list.
pub fn api_list_transactions(
    family_id: &str,
    filter: Option<&str>,
) -> Result<Vec<TransactionItemDto>, String> {
    let mut url = format!("{}/families/{family_id}/transactions", api_base_url());
    if let Some(f) = filter {
        url = format!("{url}?filter={f}");
    }
    let req = authorized_request(http_agent().get(&url));

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
        .into_json::<Vec<TransactionItemDto>>()
        .map_err(|e| format!("Failed to parse transactions list: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/transactions` to record a transaction.
pub fn api_create_transaction(
    family_id: &str,
    payload: &CreateTransactionRequest,
) -> Result<TransactionItemDto, String> {
    let url = format!("{}/families/{family_id}/transactions", api_base_url());
    let req = authorized_request(http_agent().post(&url));

    let response = match req.send_json(payload) {
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
        .into_json::<TransactionItemDto>()
        .map_err(|e| format!("Failed to parse created transaction: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/transactions/{id}/approve` to approve a transaction.
pub fn api_approve_transaction(
    family_id: &str,
    transaction_id: &str,
) -> Result<TransactionItemDto, String> {
    let url = format!(
        "{}/families/{family_id}/transactions/{transaction_id}/approve",
        api_base_url()
    );
    let req = authorized_request(http_agent().post(&url));

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
        .into_json::<TransactionItemDto>()
        .map_err(|e| format!("Failed to parse approved transaction: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/transactions/{id}/reject` to reject a transaction.
pub fn api_reject_transaction(
    family_id: &str,
    transaction_id: &str,
) -> Result<TransactionItemDto, String> {
    let url = format!(
        "{}/families/{family_id}/transactions/{transaction_id}/reject",
        api_base_url()
    );
    let req = authorized_request(http_agent().post(&url));

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
        .into_json::<TransactionItemDto>()
        .map_err(|e| format!("Failed to parse rejected transaction: {e}"))
}

/// Calls backend `GET /api/v1/families/{family_id}/projects` to fetch communal projects.
pub fn api_list_projects(family_id: &str) -> Result<Vec<ProjectDto>, String> {
    let url = format!("{}/families/{family_id}/projects", api_base_url());
    let req = authorized_request(http_agent().get(&url));

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
        .into_json::<Vec<ProjectDto>>()
        .map_err(|e| format!("Failed to parse projects list: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/projects` to create a communal project.
pub fn api_create_project(
    family_id: &str,
    payload: &CreateProjectRequest,
) -> Result<ProjectDto, String> {
    let url = format!("{}/families/{family_id}/projects", api_base_url());
    let req = authorized_request(http_agent().post(&url));

    let response = match req.send_json(payload) {
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
        .into_json::<ProjectDto>()
        .map_err(|e| format!("Failed to parse created project: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/projects/{id}/contribute` to contribute toward a project.
pub fn api_contribute_project(
    family_id: &str,
    project_id: &str,
    payload: &ProjectContributionRequest,
) -> Result<ProjectDto, String> {
    let url = format!(
        "{}/families/{family_id}/projects/{project_id}/contribute",
        api_base_url()
    );
    let req = authorized_request(http_agent().post(&url));

    let response = match req.send_json(payload) {
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
        .into_json::<ProjectDto>()
        .map_err(|e| format!("Failed to parse updated project: {e}"))
}

/// Calls backend `GET /api/v1/families/{family_id}/members` to list family members.
pub fn api_list_members(family_id: &str) -> Result<Vec<FamilyMemberDto>, String> {
    let url = format!("{}/families/{family_id}/members", api_base_url());
    let req = authorized_request(http_agent().get(&url));

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
        .into_json::<Vec<FamilyMemberDto>>()
        .map_err(|e| format!("Failed to parse members list: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/members/invite` to invite or add a member.
pub fn api_invite_member(
    family_id: &str,
    payload: &InviteFamilyMemberRequest,
) -> Result<FamilyMemberDto, String> {
    let url = format!("{}/families/{family_id}/members/invite", api_base_url());
    let req = authorized_request(http_agent().post(&url));

    let response = match req.send_json(payload) {
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
        .into_json::<FamilyMemberDto>()
        .map_err(|e| format!("Failed to parse invited member: {e}"))
}

/// Calls backend `GET /api/v1/families/{family_id}/loans` to list family loans.
pub fn api_list_loans(family_id: &str) -> Result<Vec<LoanDto>, String> {
    let url = format!("{}/families/{family_id}/loans", api_base_url());
    let req = authorized_request(http_agent().get(&url));

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
        .into_json::<Vec<LoanDto>>()
        .map_err(|e| format!("Failed to parse loans list: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/loans` to create a family loan.
pub fn api_create_loan(family_id: &str, payload: &CreateLoanRequest) -> Result<LoanDto, String> {
    let url = format!("{}/families/{family_id}/loans", api_base_url());
    let req = authorized_request(http_agent().post(&url));

    let response = match req.send_json(payload) {
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
        .into_json::<LoanDto>()
        .map_err(|e| format!("Failed to parse created loan: {e}"))
}

/// Calls backend `POST /api/v1/families/{family_id}/loans/{id}/repay` to record a loan repayment.
pub fn api_repay_loan(
    family_id: &str,
    loan_id: &str,
    payload: &LoanRepaymentRequest,
) -> Result<LoanDto, String> {
    let url = format!(
        "{}/families/{family_id}/loans/{loan_id}/repay",
        api_base_url()
    );
    let req = authorized_request(http_agent().post(&url));

    let response = match req.send_json(payload) {
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
        .into_json::<LoanDto>()
        .map_err(|e| format!("Failed to parse repaid loan: {e}"))
}

/// Calls backend `POST /api/v1/sync/push` to push local mutations outbox.
pub fn api_sync_push(mutations: Vec<SyncMutation>) -> Result<SyncPushResponse, String> {
    let url = format!("{}/sync/push", api_base_url());
    let payload = SyncPushRequest { mutations };
    let req = authorized_request(http_agent().post(&url));

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
        .into_json::<SyncPushResponse>()
        .map_err(|e| format!("Failed to parse sync push response: {e}"))
}

/// Calls backend `POST /api/v1/sync/pull` to pull remote records since cursor.
pub fn api_sync_pull(family_id: &str, cursor: Option<String>) -> Result<SyncPullResponse, String> {
    let fam_uuid =
        uuid::Uuid::parse_str(family_id).map_err(|e| format!("Invalid family ID UUID: {e}"))?;

    let url = format!("{}/sync/pull", api_base_url());
    let payload = SyncPullRequest {
        family_id: fam_uuid,
        cursor,
    };
    let req = authorized_request(http_agent().post(&url));

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
        .into_json::<SyncPullResponse>()
        .map_err(|e| format!("Failed to parse sync pull response: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_api_base_url() {
        assert_eq!(
            normalize_api_base_url("https://umurango-rust.onrender.com"),
            "https://umurango-rust.onrender.com/api/v1"
        );
        assert_eq!(
            normalize_api_base_url("https://umurango-rust.onrender.com/"),
            "https://umurango-rust.onrender.com/api/v1"
        );
        assert_eq!(
            normalize_api_base_url("https://umurango-rust.onrender.com/api/v1"),
            "https://umurango-rust.onrender.com/api/v1"
        );
        assert_eq!(
            normalize_api_base_url("http://localhost:4433"),
            "http://localhost:4433/api/v1"
        );
        assert_eq!(
            normalize_api_base_url("http://localhost:4433/api/v1/"),
            "http://localhost:4433/api/v1"
        );
    }

    #[test]
    fn test_is_loopback_url() {
        assert!(is_loopback_url("http://127.0.0.1:4433/api/v1"));
        assert!(is_loopback_url("http://localhost:4433/api/v1"));
        assert!(is_loopback_url("http://[::1]:4433/api/v1"));
        assert!(!is_loopback_url(
            "https://umurango-rust.onrender.com/api/v1"
        ));
    }
}
