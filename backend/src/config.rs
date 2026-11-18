use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/umuryango".into()),
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "umuryango-dev-secret-key-at-least-32-chars-long!".into()),
        }
    }
}
