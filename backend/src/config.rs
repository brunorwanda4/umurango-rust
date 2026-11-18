use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let raw_db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/umuryango".into());
        // Replace `@localhost:` with `@127.0.0.1:` to avoid Windows IPv6 (`::1`) connection timeout fallback (~500ms-1000ms)
        let database_url = raw_db_url.replace("@localhost:", "@127.0.0.1:");

        Self {
            database_url,
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:4433".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "umuryango-dev-secret-key-at-least-32-chars-long!".into()),
        }
    }
}
