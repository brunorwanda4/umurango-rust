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
}
