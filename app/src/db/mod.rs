use rusqlite::{Connection, Result};

pub fn open_local(path: &str) -> Result<Connection> {
    let connection = Connection::open(path)?;
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS sync_outbox (
            idempotency_key TEXT PRIMARY KEY NOT NULL,
            command TEXT NOT NULL,
            payload TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
    )?;
    Ok(connection)
}
