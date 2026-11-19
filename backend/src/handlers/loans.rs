//! Actix-web HTTP handlers for family loans and repayments.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::{CreateLoanRequest, ErrorResponse, LoanDto, LoanRepaymentRequest};

use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};
use crate::loans::{create_loan as svc_create, list_loans as svc_list, repay_loan as svc_repay};

/// List all loans (lent and borrowed) for a family workspace.
#[utoipa::path(
    get,
    path = "/api/v1/families/{family_id}/loans",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    responses(
        (status = 200, description = "List of family loans", body = Vec<LoanDto>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "loans"
)]
#[get("/{family_id}/loans")]
pub async fn list_loans(
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
    match svc_list(pool.get_ref(), &user_id, &family_id).await {
        Ok(loans) => HttpResponse::Ok().json(loans),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Create a new family loan or borrowed record.
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/loans",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    request_body = CreateLoanRequest,
    responses(
        (status = 201, description = "Loan created", body = LoanDto),
        (status = 400, description = "Invalid loan payload", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "loans"
)]
#[post("/{family_id}/loans")]
pub async fn create_loan(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
    req: web::Json<CreateLoanRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_create(pool.get_ref(), &user_id, &family_id, &req).await {
        Ok(loan) => HttpResponse::Created().json(loan),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Record a repayment installment on a family loan.
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/loans/{id}/repay",
    params(
        ("family_id" = String, Path, description = "Family workspace ID"),
        ("id" = String, Path, description = "Loan ID")
    ),
    request_body = LoanRepaymentRequest,
    responses(
        (status = 200, description = "Repayment recorded and loan updated", body = LoanDto),
        (status = 400, description = "Invalid repayment payload", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Loan not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "loans"
)]
#[post("/{family_id}/loans/{id}/repay")]
pub async fn repay_loan(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<(String, String)>,
    req: web::Json<LoanRepaymentRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let (family_id, loan_id) = path.into_inner();
    match svc_repay(pool.get_ref(), &user_id, &family_id, &loan_id, &req).await {
        Ok(loan) => HttpResponse::Ok().json(loan),
        Err(err) => error_to_response(err, &http_req),
    }
}
