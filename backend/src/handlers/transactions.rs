//! Actix-web HTTP handlers for transactions and family treasury metrics.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use umuryango_shared::{
    CreateTransactionRequest, ErrorResponse, TransactionItemDto, TreasurySummaryDto,
};

use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};
use crate::transactions::{
    approve_transaction as svc_approve, create_transaction as svc_create,
    get_treasury_summary as svc_summary, list_transactions as svc_list,
    reject_transaction as svc_reject,
};

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub filter: Option<String>,
}

/// List family transactions with optional activity filtering.
#[utoipa::path(
    get,
    path = "/api/v1/families/{family_id}/transactions",
    params(
        ("family_id" = String, Path, description = "Family workspace ID"),
        ("filter" = Option<String>, Query, description = "Activity filter: all, contributions, expenses")
    ),
    responses(
        (status = 200, description = "List of transactions", body = Vec<TransactionItemDto>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden - Not a family member", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
#[get("/{family_id}/transactions")]
pub async fn list_transactions(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<TransactionQuery>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_list(
        pool.get_ref(),
        &user_id,
        &family_id,
        query.filter.as_deref(),
    )
    .await
    {
        Ok(txs) => HttpResponse::Ok().json(txs),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Record a new transaction (income, contribution, or expense).
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/transactions",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    request_body = CreateTransactionRequest,
    responses(
        (status = 201, description = "Transaction created", body = TransactionItemDto),
        (status = 400, description = "Invalid transaction payload", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
#[post("/{family_id}/transactions")]
pub async fn create_transaction(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
    req: web::Json<CreateTransactionRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_create(pool.get_ref(), &user_id, &family_id, &req).await {
        Ok(tx) => HttpResponse::Created().json(tx),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Retrieve authoritative treasury metrics and summary for a family workspace.
#[utoipa::path(
    get,
    path = "/api/v1/families/{family_id}/transactions/summary",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    responses(
        (status = 200, description = "Treasury financial metrics", body = TreasurySummaryDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
#[get("/{family_id}/transactions/summary")]
pub async fn get_treasury_summary(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_summary(pool.get_ref(), &user_id, &family_id).await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Approve a pending family transaction (requires HEAD or ADMIN authority).
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/transactions/{id}/approve",
    params(
        ("family_id" = String, Path, description = "Family workspace ID"),
        ("id" = String, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction approved", body = TransactionItemDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden - Admin access required", body = ErrorResponse),
        (status = 404, description = "Transaction not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
#[post("/{family_id}/transactions/{id}/approve")]
pub async fn approve_transaction(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let (family_id, transaction_id) = path.into_inner();
    match svc_approve(pool.get_ref(), &user_id, &family_id, &transaction_id).await {
        Ok(tx) => HttpResponse::Ok().json(tx),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Reject a pending family transaction (requires HEAD or ADMIN authority).
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/transactions/{id}/reject",
    params(
        ("family_id" = String, Path, description = "Family workspace ID"),
        ("id" = String, Path, description = "Transaction ID")
    ),
    responses(
        (status = 200, description = "Transaction rejected", body = TransactionItemDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden - Admin access required", body = ErrorResponse),
        (status = 404, description = "Transaction not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "transactions"
)]
#[post("/{family_id}/transactions/{id}/reject")]
pub async fn reject_transaction(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let (family_id, transaction_id) = path.into_inner();
    match svc_reject(pool.get_ref(), &user_id, &family_id, &transaction_id).await {
        Ok(tx) => HttpResponse::Ok().json(tx),
        Err(err) => error_to_response(err, &http_req),
    }
}
