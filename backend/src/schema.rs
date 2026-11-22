//! PostgreSQL schema management using SQLx (with TLS support) and tokio-postgres.

use crate::config::normalize_database_url;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::error::Error;
use tokio_postgres::{Client, NoTls};

/// The complete PostgreSQL schema DDL derived from schema.prisma.
pub const SCHEMA_SQL: &str = include_str!("../schema.sql");

/// Applies the complete Umuryango schema using a `sqlx::PgPool` (supports TLS / Neon).
pub async fn apply_schema_pool(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(SCHEMA_SQL).execute(pool).await?;
    Ok(())
}

/// Applies the complete Umuryango schema using an existing `tokio_postgres::Client`.
pub async fn apply_schema(client: &Client) -> Result<(), tokio_postgres::Error> {
    client.batch_execute(SCHEMA_SQL).await
}

/// Connects to PostgreSQL using `tokio_postgres::NoTls` and spawns the connection driver task.
pub async fn connect(
    database_url: &str,
) -> Result<(Client, tokio::task::JoinHandle<()>), tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;
    let handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("tokio-postgres connection error: {e}");
        }
    });
    Ok((client, handle))
}

/// Connects to PostgreSQL via SQLx (supporting both local and TLS/Neon connections)
/// and initializes all types, tables, and indexes.
pub async fn init_db(database_url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let normalized = normalize_database_url(database_url);
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&normalized)
        .await?;
    apply_schema_pool(&pool).await?;
    Ok(())
}

/// Lists all table names currently present in the public schema.
pub async fn list_tables(client: &Client) -> Result<Vec<String>, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name;",
            &[],
        )
        .await?;

    Ok(rows.into_iter().map(|row| row.get(0)).collect())
}
