//! Actix-web HTTP handlers for family members roster.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::{ErrorResponse, FamilyMemberDto, InviteFamilyMemberRequest};

use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};
use crate::members::{invite_member as svc_invite, list_members as svc_list};

/// List all members in a family workspace.
#[utoipa::path(
    get,
    path = "/api/v1/families/{family_id}/members",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    responses(
        (status = 200, description = "List of household members", body = Vec<FamilyMemberDto>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "members"
)]
#[get("/{family_id}/members")]
pub async fn list_members(
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
        Ok(members) => HttpResponse::Ok().json(members),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Invite or add a member to a family workspace by phone number.
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/members/invite",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    request_body = InviteFamilyMemberRequest,
    responses(
        (status = 201, description = "Member invited or added", body = FamilyMemberDto),
        (status = 400, description = "Invalid payload", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "members"
)]
#[post("/{family_id}/members/invite")]
pub async fn invite_member(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
    req: web::Json<InviteFamilyMemberRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_invite(pool.get_ref(), &user_id, &family_id, &req).await {
        Ok(member) => HttpResponse::Created().json(member),
        Err(err) => error_to_response(err, &http_req),
    }
}
