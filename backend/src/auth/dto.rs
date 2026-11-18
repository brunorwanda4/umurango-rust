//! DTOs and validation helpers for authentication.

use serde::{Deserialize, Serialize};
use umuryango_shared::{LoginRequest, RegisterRequest, UserDto};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MeResponse {
    pub user: UserDto,
}

/// Validates registration payload fields.
pub fn validate_register(req: &RegisterRequest) -> Result<(), &'static str> {
    if req.name.trim().is_empty() {
        return Err("Name is required");
    }
    if req.phone.trim().is_empty() {
        return Err("Phone number is required");
    }
    if req.password.len() < 4 {
        return Err("Password must be at least 4 characters");
    }
    Ok(())
}

/// Validates login payload fields.
pub fn validate_login(req: &LoginRequest) -> Result<(), &'static str> {
    if req.identifier.trim().is_empty() {
        return Err("Phone number or email is required");
    }
    if req.password.is_empty() {
        return Err("Password is required");
    }
    Ok(())
}
