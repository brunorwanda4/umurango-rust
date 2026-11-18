//! Actix-web HTTP handlers for offline-first mutation outbox synchronization.

use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::{
    ErrorResponse, SyncPullRequest, SyncPullResponse, SyncPushRequest, SyncPushResponse,
};

use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};
use crate::sync::{pull_records as svc_pull, push_mutations as svc_push};

/// Push local SQLite outbox mutations to the authoritative PostgreSQL database.
#[utoipa::path(
    post,
    path = "/api/v1/sync/push",
    request_body = SyncPushRequest,
    responses(
        (status = 200, description = "Mutations processed idempotently", body = SyncPushResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "sync"
)]
#[post("/push")]
pub async fn sync_push(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    req: web::Json<SyncPushRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    match svc_push(pool.get_ref(), &user_id, &req).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Pull remote transaction records since the specified cursor.
#[utoipa::path(
    post,
    path = "/api/v1/sync/pull",
    request_body = SyncPullRequest,
    responses(
        (status = 200, description = "Records returned with updated cursor", body = SyncPullResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "sync"
)]
#[post("/pull")]
pub async fn sync_pull(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    req: web::Json<SyncPullRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    match svc_pull(pool.get_ref(), &user_id, &req).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => error_to_response(err, &http_req),
    }
}
