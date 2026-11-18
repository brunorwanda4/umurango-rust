//! Password hashing and verification using Argon2id.

use argon2::{
    password_hash::{
        phc::PasswordHash, Error as PasswordHashError, PasswordHasher, PasswordVerifier,
    },
    Algorithm, Argon2, Params, Version,
};

fn argon2_instance() -> Argon2<'static> {
    // Tuned Argon2id parameters (4 MiB memory, 1 iteration, 1 lane) for fast, responsive
    // hashing and verification in both dev and release builds while keeping Argon2id protection.
    let params = Params::new(4096, 1, 1, None).unwrap_or_default();
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hashes a plaintext password using Argon2id with a cryptographically secure random salt.
pub fn hash_password(password: &str) -> Result<String, PasswordHashError> {
    let argon2 = argon2_instance();
    let password_hash = argon2.hash_password(password.as_bytes())?;
    Ok(password_hash.to_string())
}

/// Verifies a plaintext password against an Argon2id password hash in constant time.
pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    argon2_instance()
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
