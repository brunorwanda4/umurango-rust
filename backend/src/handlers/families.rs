//! Actix-web HTTP handlers for family creation, joining, and invitations.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::{PgPool, Row};
use umuryango_shared::{
    CreateFamilyRequest, CreateFamilyResponse, ErrorResponse, Family, JoinFamilyRequest,
    JoinFamilyResponse, PendingFamilyInviteDto,
};

use crate::auth::{
    accept_invitation as svc_accept_invitation, create_family as svc_create_family,
    get_pending_invitations as svc_get_pending_invitations,
    join_family_by_code as svc_join_family_by_code,
};
use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};

/// Create a new family workspace with member phone numbers.
#[utoipa::path(
    post,
    path = "/api/v1/families",
    request_body = CreateFamilyRequest,
    responses(
        (status = 201, description = "Family created successfully", body = CreateFamilyResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "families"
)]
#[post("")]
pub async fn create_family(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    req: web::Json<CreateFamilyRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err),
    };

    match svc_create_family(pool.get_ref(), &user_id, &req).await {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(err) => error_to_response(err),
    }
}

/// Join an existing family workspace using a 4-character code.
#[utoipa::path(
    post,
    path = "/api/v1/families/join",
    request_body = JoinFamilyRequest,
    responses(
        (status = 200, description = "Successfully joined family", body = JoinFamilyResponse),
        (status = 400, description = "Missing or invalid code", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Family code not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "families"
)]
#[post("/join")]
pub async fn join_family(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    req: web::Json<JoinFamilyRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err),
    };

    match svc_join_family_by_code(pool.get_ref(), &user_id, &req.code).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => error_to_response(err),
    }
}

/// Get pending family invitations for the authenticated user by phone number.
#[utoipa::path(
    get,
    path = "/api/v1/families/invitations",
    responses(
        (status = 200, description = "Pending invitations list", body = Vec<PendingFamilyInviteDto>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "families"
)]
#[get("/invitations")]
pub async fn get_pending_invitations(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err),
    };

    match svc_get_pending_invitations(pool.get_ref(), &user_id).await {
        Ok(invites) => HttpResponse::Ok().json(invites),
        Err(err) => error_to_response(err),
    }
}

/// Accept a pending family invitation.
#[utoipa::path(
    post,
    path = "/api/v1/families/invitations/{id}/accept",
    params(
        ("id" = String, Path, description = "Invitation ID")
    ),
    responses(
        (status = 200, description = "Invitation accepted", body = JoinFamilyResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Invitation not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "families"
)]
#[post("/invitations/{id}/accept")]
pub async fn accept_invitation(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err),
    };

    let invite_id = path.into_inner();
    match svc_accept_invitation(pool.get_ref(), &user_id, &invite_id).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => error_to_response(err),
    }
}

/// List family workspaces the authenticated user belongs to.
#[utoipa::path(
    get,
    path = "/api/v1/families",
    responses(
        (status = 200, description = "List of families", body = Vec<Family>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "families"
)]
#[get("")]
pub async fn list_families(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Ok().json(Vec::<Family>::new()),
    };

    let rows = sqlx::query(
        r#"
        SELECT f."id", f."name", f."slug", f."country", f."defaultCurrency", f."contributionCadence"
        FROM "Family" f
        JOIN "FamilyMember" m ON f."id" = m."familyId"
        WHERE m."userId" = $1 AND m."removedAt" IS NULL
        ORDER BY f."createdAt" DESC;
        "#,
    )
    .bind(&user_id)
    .fetch_all(pool.get_ref())
    .await;

    let families = match rows {
        Ok(r) => r
            .into_iter()
            .filter_map(|row| {
                let id_str: String = row.get("id");
                let uuid = uuid::Uuid::parse_str(&id_str).ok()?;
                Some(Family {
                    id: uuid,
                    name: row.get("name"),
                    currency: row.get("defaultCurrency"),
                })
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    HttpResponse::Ok().json(families)
}
