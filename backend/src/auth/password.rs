//! Password hashing and verification using Argon2id.

use argon2::{
    password_hash::{
        phc::PasswordHash, Error as PasswordHashError, PasswordHasher, PasswordVerifier,
    },
    Argon2,
};

/// Hashes a plaintext password using Argon2id with a cryptographically secure random salt.
pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes())?;
    Ok(password_hash.to_string())
}

/// Verifies a plaintext password against an Argon2id password hash in constant time.
pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2id_hashing_and_verification() {
        let password = "SuperSecretPassword123!";
        let hash = hash_password(password).expect("hashing should succeed");

        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_password(&hash, password));
        assert!(!verify_password(&hash, "WrongPassword"));
    }
}
