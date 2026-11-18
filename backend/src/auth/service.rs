//! High-level authentication and user service operations.

use crate::auth::{
    dto::{validate_login, validate_register},
    jwt::create_token,
    password::{hash_password, verify_password},
};
use sqlx::{PgPool, Row};
use std::fmt;
use umuryango_shared::{
    AuthResponse, CreateFamilyRequest, CreateFamilyResponse, JoinFamilyResponse, LoginRequest,
    PendingFamilyInviteDto, RegisterRequest, UserDto,
};
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
    let join_code = generate_join_code();

    let _ = sqlx::query(
        r#"
        INSERT INTO "Family" (
            "id", "name", "slug", "joinCode", "country", "defaultCurrency", "contributionCadence",
            "type", "createdById", "personalOwnerId", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, $4, 'RW', 'RWF', 'MONTHLY', 'NUCLEAR', $5, $5, NOW(), NOW())
        ON CONFLICT DO NOTHING;
        "#,
    )
    .bind(&family_id)
    .bind(&family_name)
    .bind(&family_slug)
    .bind(&join_code)
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

    // Check if user was added by phone number to other families, and associate them
    let invited_families = sqlx::query_scalar::<_, String>(
        r#"SELECT "familyId" FROM "FamilyInvite" WHERE "phone" = $1 AND "status" = 'PENDING';"#,
    )
    .bind(clean_phone)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    for inv_fam_id in invited_families {
        let inv_member_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            r#"
            INSERT INTO "FamilyMember" (
                "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, 'MEMBER', '{}', NOW(), NOW())
            ON CONFLICT ("familyId", "userId") DO NOTHING;
            "#,
        )
        .bind(&inv_member_id)
        .bind(&inv_fam_id)
        .bind(&user_id)
        .execute(pool)
        .await;
    }

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

/// Generates a 4-character uppercase alphanumeric join code (e.g. "HO92").
pub fn generate_join_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let bytes = Uuid::new_v4();
    let b = bytes.as_bytes();
    (0..4)
        .map(|i| {
            let idx = (b[i] as usize) % CHARSET.len();
            CHARSET[idx] as char
        })
        .collect()
}

/// Creates a new family workspace with member phone numbers.
pub async fn create_family(
    pool: &PgPool,
    user_id: &str,
    req: &CreateFamilyRequest,
) -> Result<CreateFamilyResponse, ServiceError> {
    let clean_name = req.name.trim();
    if clean_name.is_empty() {
        return Err(ServiceError::BadRequest(
            "Family name cannot be empty".into(),
        ));
    }

    let family_id = Uuid::new_v4().to_string();
    let base_slug = clean_name
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    let slug = format!("{}-{}", base_slug, &family_id[..6]);

    // Generate unique 4-character join code
    let mut code = generate_join_code();
    for _ in 0..10 {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS(SELECT 1 FROM "Family" WHERE "joinCode" = $1);"#,
        )
        .bind(&code)
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if !exists {
            break;
        }
        code = generate_join_code();
    }

    sqlx::query(
        r#"
        INSERT INTO "Family" (
            "id", "name", "slug", "joinCode", "country", "defaultCurrency", "contributionCadence",
            "type", "createdById", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, $4, 'RW', 'RWF', 'MONTHLY', 'NUCLEAR', $5, NOW(), NOW());
        "#,
    )
    .bind(&family_id)
    .bind(clean_name)
    .bind(&slug)
    .bind(&code)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to create family: {e}")))?;

    // Add creator as HEAD of the family
    let creator_member_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO "FamilyMember" (
            "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, 'HEAD', '{}', NOW(), NOW())
        ON CONFLICT ("familyId", "userId") DO NOTHING;
        "#,
    )
    .bind(&creator_member_id)
    .bind(&family_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to add creator as member: {e}")))?;

    let mut members_count = 1;

    // Process invited member phone numbers
    for phone in &req.member_phones {
        let clean_phone = phone.trim();
        if clean_phone.is_empty() {
            continue;
        }

        // Look up if a user exists with this phone number
        let existing_user = sqlx::query_scalar::<_, String>(
            r#"SELECT "id" FROM "User" WHERE "phoneNumber" = $1 LIMIT 1;"#,
        )
        .bind(clean_phone)
        .fetch_optional(pool)
        .await
        .unwrap_or(None);

        if let Some(target_user_id) = existing_user {
            let target_member_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                r#"
                INSERT INTO "FamilyMember" (
                    "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
                )
                VALUES ($1, $2, $3, 'MEMBER', '{}', NOW(), NOW())
                ON CONFLICT ("familyId", "userId") DO NOTHING;
                "#,
            )
            .bind(&target_member_id)
            .bind(&family_id)
            .bind(&target_user_id)
            .execute(pool)
            .await;
            members_count += 1;
        }

        // Also record FamilyInvite so it appears under pending invites
        let invite_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            r#"
            INSERT INTO "FamilyInvite" (
                "id", "familyId", "invitedById", "phone", "authority", "status", "tokenHash", "expiresAt", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, $4, 'MEMBER', 'PENDING', '', NOW() + INTERVAL '30 days', NOW(), NOW());
            "#,
        )
        .bind(&invite_id)
        .bind(&family_id)
        .bind(user_id)
        .bind(clean_phone)
        .execute(pool)
        .await;
    }

    Ok(CreateFamilyResponse {
        id: family_id,
        name: clean_name.to_string(),
        code,
        members_count,
    })
}

/// Joins a family using a 4-character code (e.g. "HO92").
pub async fn join_family_by_code(
    pool: &PgPool,
    user_id: &str,
    code: &str,
) -> Result<JoinFamilyResponse, ServiceError> {
    let clean_code = code.trim().to_uppercase();
    if clean_code.is_empty() {
        return Err(ServiceError::BadRequest(
            "Family join code is required".into(),
        ));
    }

    let family_row =
        sqlx::query(r#"SELECT "id", "name" FROM "Family" WHERE UPPER("joinCode") = $1 LIMIT 1;"#)
            .bind(&clean_code)
            .fetch_optional(pool)
            .await
            .map_err(|e| ServiceError::Internal(format!("Database error: {e}")))?;

    let family = match family_row {
        Some(f) => f,
        None => {
            return Err(ServiceError::NotFound(
                "Invalid family code. Please check the code and try again.".into(),
            ));
        }
    };

    let family_id: String = family.get("id");
    let family_name: String = family.get("name");

    let member_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO "FamilyMember" (
            "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, 'MEMBER', '{}', NOW(), NOW())
        ON CONFLICT ("familyId", "userId") DO NOTHING;
        "#,
    )
    .bind(&member_id)
    .bind(&family_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to join family: {e}")))?;

    // Mark pending invites as accepted
    let _ = sqlx::query(
        r#"
        UPDATE "FamilyInvite"
        SET "status" = 'ACCEPTED', "updatedAt" = NOW()
        WHERE "familyId" = $1
          AND ("phone" = (SELECT "phoneNumber" FROM "User" WHERE "id" = $2)
               OR "email" = (SELECT "email" FROM "User" WHERE "id" = $2));
        "#,
    )
    .bind(&family_id)
    .bind(user_id)
    .execute(pool)
    .await;

    Ok(JoinFamilyResponse {
        family_id,
        family_name,
        role: "MEMBER".into(),
    })
}

/// Retrieves pending invitations for families who added this user based on their phone number.
pub async fn get_pending_invitations(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<PendingFamilyInviteDto>, ServiceError> {
    let user_phone = sqlx::query_scalar::<_, Option<String>>(
        r#"SELECT "phoneNumber" FROM "User" WHERE "id" = $1;"#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Database error: {e}")))?
    .flatten();

    let phone = match user_phone {
        Some(p) if !p.is_empty() => p,
        _ => return Ok(vec![]),
    };

    let rows = sqlx::query(
        r#"
        SELECT i."id" as invite_id, i."familyId" as family_id, f."name" as family_name,
               i."authority"::TEXT as role, i."createdAt"::TEXT as created_at
        FROM "FamilyInvite" i
        JOIN "Family" f ON i."familyId" = f."id"
        WHERE i."phone" = $1 AND i."status" = 'PENDING'
        ORDER BY i."createdAt" DESC;
        "#,
    )
    .bind(&phone)
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Database error: {e}")))?;

    let invites = rows
        .into_iter()
        .map(|r| PendingFamilyInviteDto {
            invite_id: r.get("invite_id"),
            family_id: r.get("family_id"),
            family_name: r.get("family_name"),
            role: r.get("role"),
            created_at: r.get("created_at"),
        })
        .collect();

    Ok(invites)
}

/// Accepts a pending invitation to join a family.
pub async fn accept_invitation(
    pool: &PgPool,
    user_id: &str,
    invite_id: &str,
) -> Result<JoinFamilyResponse, ServiceError> {
    let invite_row = sqlx::query(
        r#"
        SELECT i."id", i."familyId", f."name" as "familyName", i."authority"::TEXT as role, i."phone"
        FROM "FamilyInvite" i
        JOIN "Family" f ON i."familyId" = f."id"
        WHERE i."id" = $1 AND i."status" = 'PENDING';
        "#,
    )
    .bind(invite_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Database error: {e}")))?;

    let invite = match invite_row {
        Some(row) => row,
        None => {
            return Err(ServiceError::NotFound(
                "Invitation not found or expired".into(),
            ))
        }
    };

    let family_id: String = invite.get("familyId");
    let family_name: String = invite.get("familyName");
    let role: String = invite.get("role");

    let member_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO "FamilyMember" (
            "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, $4::"FamilyAuthority", '{}', NOW(), NOW())
        ON CONFLICT ("familyId", "userId") DO NOTHING;
        "#,
    )
    .bind(&member_id)
    .bind(&family_id)
    .bind(user_id)
    .bind(&role)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to join family: {e}")))?;

    let _ = sqlx::query(
        r#"UPDATE "FamilyInvite" SET "status" = 'ACCEPTED', "updatedAt" = NOW() WHERE "id" = $1;"#,
    )
    .bind(invite_id)
    .execute(pool)
    .await;

    Ok(JoinFamilyResponse {
        family_id,
        family_name,
        role,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_join_code() {
        for _ in 0..50 {
            let code = generate_join_code();
            assert_eq!(code.len(), 4);
            assert!(code
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));
        }
    }
}
