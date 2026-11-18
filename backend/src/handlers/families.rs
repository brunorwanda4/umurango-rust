use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::Family;

/// List family workspaces.
#[utoipa::path(
    get,
    path = "/api/v1/families",
    responses(
        (status = 200, description = "List of families", body = Vec<Family>)
    ),
    tag = "families"
)]
#[get("")]
pub async fn list_families(_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().json(Vec::<Family>::new())
}
