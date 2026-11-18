use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, PgPool};
use umuryango_backend::{config::Config, handlers};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&config.database_url)
        .map_err(|error| std::io::Error::other(error.to_string()))?;

    let bind_address = config.bind_address.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::<PgPool>::new(pool.clone()))
            .service(health)
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/families").service(handlers::families::list_families)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
