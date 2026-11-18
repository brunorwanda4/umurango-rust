//! High-level authentication and user service operations.

use crate::auth::{
    dto::{validate_login, validate_register},
    jwt::create_token,
    password::{hash_password, verify_password},
};
use sqlx::{PgPool, Row};
use std::fmt;
use umuryango_shared::{AuthResponse, LoginRequest, RegisterRequest, UserDto};
use uuid::Uuid;

#[derive(Debug)]
pub enum ServiceError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::BadRequest(msg)
            | ServiceError::Unauthorized(msg)
            | ServiceError::Forbidden(msg)
            | ServiceError::NotFound(msg)
            | ServiceError::Conflict(msg)
            | ServiceError::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ServiceError {}

/// Registers a new user, hashes password with Argon2id, and provisions initial workspace.
pub async fn register_user(
    pool: &PgPool,
    req: &RegisterRequest,
    jwt_secret: &str,
) -> Result<AuthResponse, ServiceError> {
    validate_register(req).map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let clean_phone = req.phone.trim();
    let clean_name = req.name.trim();
    let clean_email = req.email.as_ref().map(|e| e.trim().to_lowercase());

    // 1. Check for duplicate phone or email
    let duplicate_check = sqlx::query(
        r#"
        SELECT "id" FROM "User"
        WHERE ("phoneNumber" = $1 AND $1 IS NOT NULL)
           OR ("email" = $2 AND $2 IS NOT NULL)
        LIMIT 1;
        "#,
    )
    .bind(clean_phone)
    .bind(clean_email.as_deref())
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(e.to_string()))?;

    if duplicate_check.is_some() {
        return Err(ServiceError::Conflict(
            "An account with this phone number or email already exists".into(),
        ));
    }

    // 2. Hash password using Argon2id
    let password_hash = hash_password(&req.password)
        .map_err(|e| ServiceError::Internal(format!("Password hashing error: {e}")))?;

    // 3. Insert User
    let user_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO "User" (
            "id", "fullName", "phoneNumber", "email", "passwordHash", "status", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, $4, $5, 'ACTIVE', NOW(), NOW());
        "#,
    )
    .bind(&user_id)
    .bind(clean_name)
    .bind(clean_phone)
    .bind(clean_email.as_deref())
    .bind(&password_hash)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to create user: {e}")))?;

    // 4. Provision primary family workspace for the new member
    let family_id = Uuid::new_v4().to_string();
    let family_slug = format!(
        "{}-{}",
        clean_name.to_lowercase().replace(' ', "-"),
        &user_id[..6]
    );
    let family_name = format!("Umuryango wa {}", clean_name);

    let _ = sqlx::query(
        r#"
        INSERT INTO "Family" (
            "id", "name", "slug", "country", "defaultCurrency", "contributionCadence",
            "type", "createdById", "personalOwnerId", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, 'RW', 'RWF', 'MONTHLY', 'NUCLEAR', $4, $4, NOW(), NOW())
        ON CONFLICT DO NOTHING;
        "#,
    )
    .bind(&family_id)
    .bind(&family_name)
    .bind(&family_slug)
    .bind(&user_id)
    .execute(pool)
    .await;

    let member_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO "FamilyMember" (
            "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, 'HEAD', '{}', NOW(), NOW())
        ON CONFLICT DO NOTHING;
        "#,
    )
    .bind(&member_id)
    .bind(&family_id)
    .bind(&user_id)
    .execute(pool)
    .await;

    // 5. Generate token
    let token = create_token(&user_id, clean_name, jwt_secret, 30 * 86400)
        .map_err(|e| ServiceError::Internal(format!("Token creation error: {e}")))?;

    // 6. Record audit event
    let audit_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO "AuthAuditEvent" ("id", "userId", "emailOrMobile", "action", "result", "createdAt")
        VALUES ($1, $2, $3, 'REGISTER', 'SUCCESS', NOW());
        "#,
    )
    .bind(&audit_id)
    .bind(&user_id)
    .bind(clean_phone)
    .execute(pool)
    .await;

    Ok(AuthResponse {
        user: UserDto {
            id: user_id,
            full_name: clean_name.to_string(),
            phone_number: Some(clean_phone.to_string()),
            email: clean_email,
            status: "ACTIVE".into(),
        },
        token,
        family_name: Some(family_name),
        role: Some("HEAD".into()),
    })
}

/// Authenticates an existing user by phone or email, validating password via Argon2id.
pub async fn login_user(
    pool: &PgPool,
    req: &LoginRequest,
    jwt_secret: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<AuthResponse, ServiceError> {
    validate_login(req).map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    let identifier = req.identifier.trim();

    // 1. Fetch user by phoneNumber OR email
    let row = sqlx::query(
        r#"
        SELECT "id", "fullName", "phoneNumber", "email", "passwordHash", "status"::text AS "status"
        FROM "User"
        WHERE "phoneNumber" = $1 OR LOWER("email") = LOWER($1)
        LIMIT 1;
        "#,
    )
    .bind(identifier)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(e.to_string()))?;

    let row = match row {
        Some(r) => r,
        None => {
            record_auth_audit(
                pool, None, identifier, "LOGIN", "FAILURE", ip_address, user_agent,
            )
            .await;
            return Err(ServiceError::Unauthorized(
                "Invalid phone number or password".into(),
            ));
        }
    };

    let user_id: String = row.get("id");
    let full_name: String = row.get("fullName");
    let phone_number: Option<String> = row.get("phoneNumber");
    let email: Option<String> = row.get("email");
    let password_hash: Option<String> = row.get("passwordHash");
    let status: String = row.get("status");

    if status.eq_ignore_ascii_case("DISABLED") {
        return Err(ServiceError::Forbidden("Account has been disabled".into()));
    }

    let hash = match password_hash {
        Some(h) => h,
        None => {
            record_auth_audit(
                pool,
                Some(&user_id),
                identifier,
                "LOGIN",
                "FAILURE",
                ip_address,
                user_agent,
            )
            .await;
            return Err(ServiceError::Unauthorized(
                "Invalid phone number or password".into(),
            ));
        }
    };

    // 2. Constant-time Argon2id password verification
    if !verify_password(&hash, &req.password) {
        record_auth_audit(
            pool,
            Some(&user_id),
            identifier,
            "LOGIN",
            "FAILURE",
            ip_address,
            user_agent,
        )
        .await;
        return Err(ServiceError::Unauthorized(
            "Invalid phone number or password".into(),
        ));
    }

    // 3. Load user's primary family membership
    let family_row = sqlx::query(
        r#"
        SELECT f."name" AS "familyName", m."authority"::text AS "role"
        FROM "FamilyMember" m
        JOIN "Family" f ON f."id" = m."familyId"
        WHERE m."userId" = $1 AND m."removedAt" IS NULL
        ORDER BY m."createdAt" ASC
        LIMIT 1;
        "#,
    )
    .bind(&user_id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None);

    let (family_name, role) = match family_row {
        Some(fr) => (
            Some(fr.get::<String, _>("familyName")),
            Some(fr.get::<String, _>("role")),
        ),
        None => (None, None),
    };

    // 4. Generate JWT token
    let token = create_token(&user_id, &full_name, jwt_secret, 30 * 86400)
        .map_err(|e| ServiceError::Internal(format!("Token error: {e}")))?;

    // 5. Record session & audit event
    let session_id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO "AuthSession" (
            "id", "userId", "refreshTokenHash", "userAgent", "ipAddress", "expiresAt", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, $4, $5, NOW() + INTERVAL '30 days', NOW(), NOW());
        "#,
    )
    .bind(&session_id)
    .bind(&user_id)
    .bind(&token[..32.min(token.len())])
    .bind(user_agent)
    .bind(ip_address)
    .execute(pool)
    .await;

    record_auth_audit(
        pool,
        Some(&user_id),
        identifier,
        "LOGIN",
        "SUCCESS",
        ip_address,
        user_agent,
    )
    .await;

    Ok(AuthResponse {
        user: UserDto {
            id: user_id,
            full_name,
            phone_number,
            email,
            status,
        },
        token,
        family_name,
        role,
    })
}

/// Fetches user profile by ID.
pub async fn get_user_profile(pool: &PgPool, user_id: &str) -> Result<UserDto, ServiceError> {
    let row = sqlx::query(
        r#"
        SELECT "id", "fullName", "phoneNumber", "email", "status"::text AS "status"
        FROM "User"
        WHERE "id" = $1
        LIMIT 1;
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(e.to_string()))?
    .ok_or_else(|| ServiceError::NotFound("User not found".into()))?;

    Ok(UserDto {
        id: row.get("id"),
        full_name: row.get("fullName"),
        phone_number: row.get("phoneNumber"),
        email: row.get("email"),
        status: row.get("status"),
    })
}

async fn record_auth_audit(
    pool: &PgPool,
    user_id: Option<&str>,
    identifier: &str,
    action: &str,
    result: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) {
    let id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO "AuthAuditEvent" (
            "id", "userId", "emailOrMobile", "action", "result", "ipAddress", "userAgent", "createdAt"
        )
        VALUES ($1, $2, $3, $4::"AuthAuditAction", $5::"AuthAuditResult", $6, $7, NOW());
        "#,
    )
    .bind(&id)
    .bind(user_id)
    .bind(identifier)
    .bind(action)
    .bind(result)
    .bind(ip_address)
    .bind(user_agent)
    .execute(pool)
    .await;
}
