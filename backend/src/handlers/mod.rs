pub mod auth;
pub mod families;
pub mod loans;
pub mod members;
pub mod projects;
pub mod sync;
pub mod transactions;

use crate::auth::{verify_token, ServiceError};
use actix_web::{HttpRequest, HttpResponse};
use umuryango_shared::ErrorResponse;

pub fn error_to_response(error: ServiceError, http_req: &HttpRequest) -> HttpResponse {
    let lang = crate::i18n::extract_language(http_req);
    let message = error.localized_message(lang);
    let body = ErrorResponse { error: message };
    match error {
        ServiceError::BadRequest(_) => HttpResponse::BadRequest().json(body),
        ServiceError::Unauthorized(_) => HttpResponse::Unauthorized().json(body),
        ServiceError::Forbidden(_) => HttpResponse::Forbidden().json(body),
        ServiceError::NotFound(_) => HttpResponse::NotFound().json(body),
        ServiceError::Conflict(_) => HttpResponse::Conflict().json(body),
        ServiceError::Internal(_) => HttpResponse::InternalServerError().json(body),
    }
}

pub fn extract_user_id(http_req: &HttpRequest, secret: &str) -> Result<String, ServiceError> {
    let auth_header = http_req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return Err(ServiceError::Unauthorized(
                "Missing or invalid Authorization header".into(),
            ));
        }
    };

    let claims = verify_token(token, secret)
        .map_err(|_| ServiceError::Unauthorized("Invalid or expired token".into()))?;

    Ok(claims.sub)
}
