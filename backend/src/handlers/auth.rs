//! Actix-web HTTP handlers for authentication and user sessions.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest};

use crate::auth::{get_user_profile, login_user, register_user, MeResponse};
use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};

/// Register a new user account.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User successfully registered", body = AuthResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 409, description = "Phone number or email already in use", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "auth"
)]
#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    match register_user(pool.get_ref(), &req, &config.jwt_secret).await {
        Ok(auth_resp) => HttpResponse::Created().json(auth_resp),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Log in with phone number or email and password.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User successfully authenticated", body = AuthResponse),
        (status = 400, description = "Missing or invalid payload", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 403, description = "Account disabled", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "auth"
)]
#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let client_ip = http_req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());
    let user_agent = http_req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok());

    match login_user(
        pool.get_ref(),
        &req,
        &config.jwt_secret,
        client_ip.as_deref(),
        user_agent,
    )
    .await
    {
        Ok(auth_resp) => HttpResponse::Ok().json(auth_resp),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Get the profile of the currently authenticated user.
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Current user profile", body = MeResponse),
        (status = 401, description = "Missing or invalid bearer token", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "auth"
)]
#[get("/me")]
pub async fn me(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    match get_user_profile(pool.get_ref(), &user_id).await {
        Ok(user) => HttpResponse::Ok().json(MeResponse { user }),
        Err(err) => error_to_response(err, &http_req),
    }
}
