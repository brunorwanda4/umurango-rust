//! JWT token creation and verification.

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (User ID)
    pub sub: String,
    /// User Full Name
    pub name: String,
    /// Issued at (timestamp)
    pub iat: usize,
    /// Expiration time (timestamp)
    pub exp: usize,
}

/// Creates a signed JWT token for the user with an expiration duration.
pub fn create_token(
    user_id: &str,
    name: &str,
    secret: &str,
    duration_secs: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        name: name.to_string(),
        iat: now as usize,
        exp: (now + duration_secs) as usize,
    };

    let key = EncodingKey::from_secret(secret.as_bytes())?;
    encode(&Header::default(), &claims, &key)
}

/// Validates and decodes a signed JWT token.
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(secret.as_bytes())?;
    let token_data = decode::<Claims>(token, &key, &Validation::default())?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_create_and_verify() {
        let secret = "test-secret-key-that-is-long-enough-32-bytes";
        let token = create_token("user-123", "Jean Doe", secret, 3600)
            .expect("token generation should succeed");
        let claims = verify_token(&token, secret).expect("token verification should succeed");

        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.name, "Jean Doe");
    }
}
