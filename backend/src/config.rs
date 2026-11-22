use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self::from_lookup(|key| env::var(key).ok())
    }

    pub fn from_lookup<F>(mut get_var: F) -> Self
    where
        F: FnMut(&str) -> Option<String>,
    {
        let raw_db_url = get_var("DATABASE_URL")
            .filter(|val| !val.trim().is_empty())
            .or_else(|| get_var("PRODUCTION_DATABASE_URL").filter(|val| !val.trim().is_empty()))
            .unwrap_or_else(|| "postgres://postgres:postgres@127.0.0.1:5432/umuryango".into());

        let database_url = normalize_database_url(&raw_db_url);

        let bind_address = get_var("BIND_ADDRESS")
            .filter(|val| !val.trim().is_empty())
            .or_else(|| {
                get_var("PORT")
                    .filter(|val| !val.trim().is_empty())
                    .map(|port| format!("0.0.0.0:{port}"))
            })
            .unwrap_or_else(|| "127.0.0.1:4433".into());

        let jwt_secret = get_var("JWT_SECRET")
            .filter(|val| !val.trim().is_empty())
            .unwrap_or_else(|| "umuryango-dev-secret-key-at-least-32-chars-long!".into());

        Self {
            database_url,
            bind_address,
            jwt_secret,
        }
    }
}

/// Normalizes a PostgreSQL connection URL for SQLx:
/// - Replaces `@localhost:` with `@127.0.0.1:` to avoid Windows IPv6 (`::1`) timeout fallback.
pub fn normalize_database_url(raw_db_url: &str) -> String {
    raw_db_url.trim().replace("@localhost:", "@127.0.0.1:")
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgConnectOptions;
    use std::str::FromStr;

    #[test]
    fn uses_port_env_when_bind_address_not_set() {
        let config = Config::from_lookup(|key| match key {
            "PORT" => Some("10000".into()),
            "PRODUCTION_DATABASE_URL" => Some(
                "postgresql://user:pass@ep-test.eu-central-1.aws.neon.tech/neondb?sslmode=require"
                    .into(),
            ),
            _ => None,
        });

        assert_eq!(config.bind_address, "0.0.0.0:10000");
        assert_eq!(
            config.database_url,
            "postgresql://user:pass@ep-test.eu-central-1.aws.neon.tech/neondb?sslmode=require"
        );
    }

    #[test]
    fn neon_url_parses_in_sqlx() {
        let url = normalize_database_url(
            "postgresql://neondb_owner:secret@ep-small-violet-pooler.eu-central-1.aws.neon.tech/neondb?sslmode=require&channel_binding=require",
        );
        let parsed = PgConnectOptions::from_str(&url);
        assert!(
            parsed.is_ok(),
            "SQLx failed to parse Neon URL: {:?}",
            parsed.err()
        );
    }
}
