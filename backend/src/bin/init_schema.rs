use dotenvy::dotenv;
use std::env;
use umuryango_backend::schema::init_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://antigravity@localhost:5432/umuryango".into());

    println!("Connecting via tokio-postgres to {database_url}...");
    init_db(&database_url).await?;
    println!("Database schema successfully initialized!");

    Ok(())
}
