//! PostgreSQL schema management using tokio-postgres.

use std::error::Error;
use tokio_postgres::{Client, NoTls};

/// The complete PostgreSQL schema DDL derived from schema.prisma.
pub const SCHEMA_SQL: &str = include_str!("../schema.sql");

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

/// Connects to PostgreSQL via tokio-postgres and initializes all types, tables, and indexes.
pub async fn init_db(database_url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (client, _handle) = connect(database_url).await?;
    apply_schema(&client).await?;
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
