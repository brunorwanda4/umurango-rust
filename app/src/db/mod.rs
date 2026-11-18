use rusqlite::{params, Connection, OptionalExtension, Result};

pub fn open_local(path: &str) -> Result<Connection> {
    let connection = Connection::open(path)?;
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS sync_outbox (
            idempotency_key TEXT PRIMARY KEY NOT NULL,
            command TEXT NOT NULL,
            payload TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS app_state (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );",
    )?;
    Ok(connection)
}

/// Reads a locally persisted app state value, such as the last visited screen.
pub fn get_app_state(connection: &Connection, key: &str) -> Result<Option<String>> {
    connection
        .query_row(
            "SELECT value FROM app_state WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()
}

/// Stores a local app state value. Passing `None` removes the key.
pub fn set_app_state(connection: &Connection, key: &str, value: Option<&str>) -> Result<()> {
    match value {
        Some(value) => connection.execute(
            "INSERT INTO app_state (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value,
                 updated_at = CURRENT_TIMESTAMP",
            params![key, value],
        )?,
        None => connection.execute("DELETE FROM app_state WHERE key = ?1", params![key])?,
    };
    Ok(())
}

/// Queues a new mutation into the local SQLite sync outbox.
pub fn queue_outbox_mutation(
    connection: &Connection,
    idempotency_key: &str,
    command: &str,
    payload: &str,
) -> Result<()> {
    connection.execute(
        "INSERT INTO sync_outbox (idempotency_key, command, payload)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(idempotency_key) DO NOTHING",
        params![idempotency_key, command, payload],
    )?;
    Ok(())
}

/// Retrieves all pending mutations from the local outbox.
pub fn get_pending_outbox_mutations(
    connection: &Connection,
) -> Result<Vec<(String, String, String)>> {
    let mut stmt = connection.prepare(
        "SELECT idempotency_key, command, payload FROM sync_outbox ORDER BY created_at ASC",
    )?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    let mut result = Vec::new();
    for r in rows {
        result.push(r?);
    }
    Ok(result)
}

/// Deletes an acknowledged mutation from the local outbox.
pub fn delete_outbox_mutation(connection: &Connection, idempotency_key: &str) -> Result<()> {
    connection.execute(
        "DELETE FROM sync_outbox WHERE idempotency_key = ?1",
        params![idempotency_key],
    )?;
    Ok(())
}

/// Counts total pending mutations in the local outbox.
pub fn count_pending_outbox(connection: &Connection) -> Result<usize> {
    let count: i64 =
        connection.query_row("SELECT COUNT(*) FROM sync_outbox", [], |row| row.get(0))?;
    Ok(count as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_state_round_trips_updates_and_deletes() {
        let connection = open_local(":memory:").expect("in-memory database opens");

        assert_eq!(get_app_state(&connection, "screen").unwrap(), None);

        set_app_state(&connection, "screen", Some("register")).unwrap();
        assert_eq!(
            get_app_state(&connection, "screen").unwrap().as_deref(),
            Some("register")
        );

        set_app_state(&connection, "screen", Some("terms")).unwrap();
        assert_eq!(
            get_app_state(&connection, "screen").unwrap().as_deref(),
            Some("terms")
        );

        set_app_state(&connection, "screen", None).unwrap();
        assert_eq!(get_app_state(&connection, "screen").unwrap(), None);
    }

    #[test]
    fn outbox_queues_retrieves_and_deletes() {
        let connection = open_local(":memory:").expect("in-memory database opens");
        assert_eq!(count_pending_outbox(&connection).unwrap(), 0);

        queue_outbox_mutation(
            &connection,
            "k1",
            "CREATE_CONTRIBUTION",
            "{\"amount\": 50000}",
        )
        .unwrap();
        assert_eq!(count_pending_outbox(&connection).unwrap(), 1);

        let items = get_pending_outbox_mutations(&connection).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0, "k1");
        assert_eq!(items[0].1, "CREATE_CONTRIBUTION");

        delete_outbox_mutation(&connection, "k1").unwrap();
        assert_eq!(count_pending_outbox(&connection).unwrap(), 0);
    }
}
