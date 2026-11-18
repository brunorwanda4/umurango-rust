use actix_web::{get, middleware::from_fn, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, PgPool};
use umuryango_backend::{
    config::Config, handlers, middleware::request_log::request_log, openapi::ApiDoc,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
    let openapi = ApiDoc::openapi();

    println!("Starting Umuryango API server on {bind_address}");
    println!("Swagger documentation available at http://{bind_address}/swagger-ui/");

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(request_log))
            .app_data(web::Data::<PgPool>::new(pool.clone()))
            .app_data(web::Data::<Config>::new(config.clone()))
            .service(health)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .service(handlers::auth::register)
                            .service(handlers::auth::login)
                            .service(handlers::auth::me),
                    )
                    .service(
                        web::scope("/families")
                            .service(handlers::families::list_families)
                            .service(handlers::families::create_family)
                            .service(handlers::families::join_family)
                            .service(handlers::families::get_pending_invitations)
                            .service(handlers::families::accept_invitation),
                    ),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
