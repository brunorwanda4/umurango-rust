use dotenvy::dotenv;
use std::env;
use umuryango_backend::schema::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let use_production = env::args().any(|arg| arg == "--production" || arg == "--prod");

    let database_url = if use_production {
        env::var("PRODUCTION_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .expect("PRODUCTION_DATABASE_URL or DATABASE_URL must be set")
    } else {
        env::var("DATABASE_URL")
            .or_else(|_| env::var("PRODUCTION_DATABASE_URL"))
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/umuryango".into())
    };

    println!("Initializing Umuryango database schema...");
    init_db(&database_url).await?;
    println!("Database schema successfully initialized!");

    Ok(())
}
