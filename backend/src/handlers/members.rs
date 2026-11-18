//! Actix-web HTTP handlers for family members roster.

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::{ErrorResponse, FamilyMemberDto};

use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};
use crate::members::list_members as svc_list;

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
