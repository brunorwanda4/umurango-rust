use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::Family;

#[get("")]
pub async fn list_families(_pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().json(Vec::<Family>::new())
}
