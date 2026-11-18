//! High-level authentication and user service operations.

use crate::auth::{
    dto::{validate_login, validate_register},
    jwt::create_token,
    password::{hash_password, verify_password},
};
use sqlx::{PgPool, Row};
use std::fmt;
use umuryango_shared::{
    generate_default_avatar_url, AuthResponse, CreateFamilyRequest, CreateFamilyResponse,
    JoinFamilyResponse, LoginRequest, PendingFamilyInviteDto, RegisterRequest, UserDto,
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

impl ServiceError {
    /// Returns the error message translated to the requested language.
    pub fn localized_message(&self, lang: crate::i18n::SupportedLanguage) -> String {
        let raw = match self {
            ServiceError::BadRequest(msg)
            | ServiceError::Unauthorized(msg)
            | ServiceError::Forbidden(msg)
            | ServiceError::NotFound(msg)
            | ServiceError::Conflict(msg)
            | ServiceError::Internal(msg) => msg,
        };
        crate::i18n::translate_error(raw, lang)
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
    let avatar_url = req
        .avatar_url
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| generate_default_avatar_url(clean_name));

    // 1. Check for duplicate phone or email concurrently with offloaded Argon2id password hashing
    let password_owned = req.password.clone();
    let hash_task = tokio::task::spawn_blocking(move || hash_password(&password_owned));

    let duplicate_fut = sqlx::query(
        r#"
        SELECT "id" FROM "User"
        WHERE "phoneNumber" = $1
           OR ($2 IS NOT NULL AND "email" = $2)
        LIMIT 1;
        "#,
    )
    .bind(clean_phone)
    .bind(clean_email.as_deref())
    .fetch_optional(pool);

    let (duplicate_res, hash_res) = tokio::join!(duplicate_fut, hash_task);

    let duplicate_check = duplicate_res.map_err(|e| ServiceError::Internal(e.to_string()))?;
    if duplicate_check.is_some() {
        return Err(ServiceError::Conflict(
            "An account with this phone number or email already exists".into(),
        ));
    }

    let password_hash = hash_res
        .map_err(|e| ServiceError::Internal(format!("Password task error: {e}")))?
        .map_err(|e| ServiceError::Internal(format!("Password hashing error: {e}")))?;

    // 2. Provision user, primary family workspace, memberships, and audit log in a single round-trip
    let user_id = Uuid::new_v4().to_string();
    let family_id = Uuid::new_v4().to_string();
    let family_slug = format!(
        "{}-{}",
        clean_name.to_lowercase().replace(' ', "-"),
        &user_id[..6]
    );
    let family_name = format!("Umuryango wa {}", clean_name);
    let join_code = generate_join_code();
    let member_id = Uuid::new_v4().to_string();
    let audit_id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        WITH new_user AS (
            INSERT INTO "User" (
                "id", "fullName", "phoneNumber", "email", "passwordHash", "avatarUrl", "status", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'ACTIVE', NOW(), NOW())
            RETURNING "id"
        ),
        new_family AS (
            INSERT INTO "Family" (
                "id", "name", "slug", "joinCode", "country", "defaultCurrency", "contributionCadence",
                "type", "createdById", "personalOwnerId", "createdAt", "updatedAt"
            )
            SELECT $7, $8, $9, $10, 'RW', 'RWF', 'MONTHLY', 'NUCLEAR', "id", "id", NOW(), NOW()
            FROM new_user
            ON CONFLICT DO NOTHING
        ),
        head_member AS (
            INSERT INTO "FamilyMember" (
                "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
            )
            SELECT $11, $7, "id", 'HEAD', '{}', NOW(), NOW()
            FROM new_user
            ON CONFLICT DO NOTHING
        ),
        invited_members AS (
            INSERT INTO "FamilyMember" (
                "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
            )
            SELECT md5(random()::text || clock_timestamp()::text || fi."id"), fi."familyId", u."id", 'MEMBER', '{}', NOW(), NOW()
            FROM "FamilyInvite" fi
            CROSS JOIN new_user u
            WHERE fi."phone" = $3 AND fi."status" = 'PENDING'
            ON CONFLICT ("familyId", "userId") DO NOTHING
        )
        INSERT INTO "AuthAuditEvent" ("id", "userId", "emailOrMobile", "action", "result", "createdAt")
        SELECT $12, "id", $3, 'REGISTER', 'SUCCESS', NOW()
        FROM new_user;
        "#,
    )
    .bind(&user_id)
    .bind(clean_name)
    .bind(clean_phone)
    .bind(clean_email.as_deref())
    .bind(&password_hash)
    .bind(&avatar_url)
    .bind(&family_id)
    .bind(&family_name)
    .bind(&family_slug)
    .bind(&join_code)
    .bind(&member_id)
    .bind(&audit_id)
    .execute(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.is_unique_violation() {
                return ServiceError::Conflict(
                    "An account with this phone number or email already exists".into(),
                );
            }
        }
        ServiceError::Internal(format!("Failed to create user: {e}"))
    })?;

    // 3. Generate token
    let token = create_token(&user_id, clean_name, jwt_secret, 30 * 86400)
        .map_err(|e| ServiceError::Internal(format!("Token creation error: {e}")))?;

    Ok(AuthResponse {
        user: UserDto {
            id: user_id,
            full_name: clean_name.to_string(),
            phone_number: Some(clean_phone.to_string()),
            email: clean_email,
            avatar_url: Some(avatar_url),
            status: "ACTIVE".into(),
        },
        token,
        family_name: Some(family_name),
        role: Some("HEAD".into()),
        family_id: Some(family_id),
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
        SELECT "id", "fullName", "phoneNumber", "email", "passwordHash", "avatarUrl", "status"::text AS "status"
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
    let avatar_url: Option<String> = row
        .get::<Option<String>, _>("avatarUrl")
        .or_else(|| Some(generate_default_avatar_url(&full_name)));
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

    // 2. Constant-time Argon2id password verification offloaded to blocking pool
    let password_owned = req.password.clone();
    let is_valid = tokio::task::spawn_blocking(move || verify_password(&hash, &password_owned))
        .await
        .unwrap_or(false);

    if !is_valid {
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

    // 3. Generate JWT token
    let token = create_token(&user_id, &full_name, jwt_secret, 30 * 86400)
        .map_err(|e| ServiceError::Internal(format!("Token error: {e}")))?;

    // 4. Load user's primary family membership and record session & audit concurrently
    let session_id = Uuid::new_v4().to_string();
    let audit_id = Uuid::new_v4().to_string();
    let token_prefix = &token[..32.min(token.len())];

    let family_fut = sqlx::query(
        r#"
        SELECT f."id" AS "familyId", f."name" AS "familyName", m."authority"::text AS "role"
        FROM "FamilyMember" m
        JOIN "Family" f ON f."id" = m."familyId"
        WHERE m."userId" = $1 AND m."removedAt" IS NULL
        ORDER BY m."createdAt" ASC
        LIMIT 1;
        "#,
    )
    .bind(&user_id)
    .fetch_optional(pool);

    let session_audit_fut = sqlx::query(
        r#"
        WITH ins_session AS (
            INSERT INTO "AuthSession" (
                "id", "userId", "refreshTokenHash", "userAgent", "ipAddress", "expiresAt", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, $4, $5, NOW() + INTERVAL '30 days', NOW(), NOW())
        )
        INSERT INTO "AuthAuditEvent" (
            "id", "userId", "emailOrMobile", "action", "result", "ipAddress", "userAgent", "createdAt"
        )
        VALUES ($6, $2, $7, 'LOGIN', 'SUCCESS', $5, $4, NOW());
        "#,
    )
    .bind(&session_id)
    .bind(&user_id)
    .bind(token_prefix)
    .bind(user_agent)
    .bind(ip_address)
    .bind(&audit_id)
    .bind(identifier)
    .execute(pool);

    let (family_res, _) = tokio::join!(family_fut, session_audit_fut);

    let (family_id, family_name, role) = match family_res.unwrap_or(None) {
        Some(fr) => (
            Some(fr.get::<String, _>("familyId")),
            Some(fr.get::<String, _>("familyName")),
            Some(fr.get::<String, _>("role")),
        ),
        None => (None, None, None),
    };

    Ok(AuthResponse {
        user: UserDto {
            id: user_id,
            full_name,
            phone_number,
            email,
            avatar_url,
            status,
        },
        token,
        family_name,
        role,
        family_id,
    })
}

/// Fetches user profile by ID.
pub async fn get_user_profile(pool: &PgPool, user_id: &str) -> Result<UserDto, ServiceError> {
    let row = sqlx::query(
        r#"
        SELECT "id", "fullName", "phoneNumber", "email", "avatarUrl", "status"::text AS "status"
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

    let full_name: String = row.get("fullName");
    let avatar_url: Option<String> = row
        .get::<Option<String>, _>("avatarUrl")
        .or_else(|| Some(generate_default_avatar_url(&full_name)));

    Ok(UserDto {
        id: row.get("id"),
        full_name,
        phone_number: row.get("phoneNumber"),
        email: row.get("email"),
        avatar_url,
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

    let creator_member_id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        WITH new_family AS (
            INSERT INTO "Family" (
                "id", "name", "slug", "joinCode", "country", "defaultCurrency", "contributionCadence",
                "type", "createdById", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, $4, 'RW', 'RWF', 'MONTHLY', 'NUCLEAR', $5, NOW(), NOW())
        )
        INSERT INTO "FamilyMember" (
            "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
        )
        VALUES ($6, $1, $5, 'HEAD', '{}', NOW(), NOW())
        ON CONFLICT ("familyId", "userId") DO NOTHING;
        "#,
    )
    .bind(&family_id)
    .bind(clean_name)
    .bind(&slug)
    .bind(&code)
    .bind(user_id)
    .bind(&creator_member_id)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to create family: {e}")))?;

    let mut members_count = 1;

    // Process invited member phone numbers in a single batch query
    let clean_phones: Vec<String> = req
        .member_phones
        .iter()
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();

    if !clean_phones.is_empty() {
        let added = sqlx::query_scalar::<_, i64>(
            r#"
            WITH input_phones AS (
                SELECT DISTINCT unnest($1::text[]) AS phone
            ),
            ins_invites AS (
                INSERT INTO "FamilyInvite" (
                    "id", "familyId", "invitedById", "phone", "authority", "status", "tokenHash", "expiresAt", "createdAt", "updatedAt"
                )
                SELECT md5(random()::text || clock_timestamp()::text || ip.phone), $2, $3, ip.phone, 'MEMBER', 'PENDING', '', NOW() + INTERVAL '30 days', NOW(), NOW()
                FROM input_phones ip
            ),
            ins_members AS (
                INSERT INTO "FamilyMember" (
                    "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
                )
                SELECT md5(random()::text || clock_timestamp()::text || u."id"), $2, u."id", 'MEMBER', '{}', NOW(), NOW()
                FROM "User" u
                JOIN input_phones ip ON u."phoneNumber" = ip.phone
                ON CONFLICT ("familyId", "userId") DO NOTHING
                RETURNING 1
            )
            SELECT COUNT(*) FROM ins_members;
            "#,
        )
        .bind(&clean_phones)
        .bind(&family_id)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        members_count += added as usize;
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

    let member_id = Uuid::new_v4().to_string();
    let row = sqlx::query(
        r#"
        WITH target_family AS (
            SELECT "id", "name"
            FROM "Family"
            WHERE "joinCode" = $1 OR UPPER("joinCode") = $1
            LIMIT 1
        ),
        ins_member AS (
            INSERT INTO "FamilyMember" (
                "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
            )
            SELECT $2, tf."id", $3, 'MEMBER', '{}', NOW(), NOW()
            FROM target_family tf
            ON CONFLICT ("familyId", "userId") DO NOTHING
        ),
        upd_invite AS (
            UPDATE "FamilyInvite" fi
            SET "status" = 'ACCEPTED', "updatedAt" = NOW()
            FROM target_family tf, "User" u
            WHERE fi."familyId" = tf."id"
              AND u."id" = $3
              AND (fi."phone" = u."phoneNumber" OR fi."email" = u."email")
        )
        SELECT "id", "name" FROM target_family;
        "#,
    )
    .bind(&clean_code)
    .bind(&member_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Database error: {e}")))?;

    let family = match row {
        Some(f) => f,
        None => {
            return Err(ServiceError::NotFound(
                "Invalid family code. Please check the code and try again.".into(),
            ));
        }
    };

    Ok(JoinFamilyResponse {
        family_id: family.get("id"),
        family_name: family.get("name"),
        role: "MEMBER".into(),
    })
}

/// Retrieves pending invitations for families who added this user based on their phone number.
pub async fn get_pending_invitations(
    pool: &PgPool,
    user_id: &str,
) -> Result<Vec<PendingFamilyInviteDto>, ServiceError> {
    let rows = sqlx::query(
        r#"
        SELECT i."id" AS invite_id, i."familyId" AS family_id, f."name" AS family_name,
               i."authority"::TEXT AS role, i."createdAt"::TEXT AS created_at
        FROM "User" u
        JOIN "FamilyInvite" i ON i."phone" = u."phoneNumber"
        JOIN "Family" f ON i."familyId" = f."id"
        WHERE u."id" = $1
          AND u."phoneNumber" IS NOT NULL
          AND u."phoneNumber" <> ''
          AND i."status" = 'PENDING'
        ORDER BY i."createdAt" DESC;
        "#,
    )
    .bind(user_id)
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
    let member_id = Uuid::new_v4().to_string();
    let invite_row = sqlx::query(
        r#"
        WITH target_invite AS (
            UPDATE "FamilyInvite"
            SET "status" = 'ACCEPTED', "updatedAt" = NOW()
            WHERE "id" = $1 AND "status" = 'PENDING'
            RETURNING "id", "familyId", "authority"
        ),
        ins_member AS (
            INSERT INTO "FamilyMember" (
                "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
            )
            SELECT $2, ti."familyId", $3, ti."authority", '{}', NOW(), NOW()
            FROM target_invite ti
            ON CONFLICT ("familyId", "userId") DO NOTHING
        )
        SELECT ti."familyId", f."name" AS "familyName", ti."authority"::TEXT AS role
        FROM target_invite ti
        JOIN "Family" f ON ti."familyId" = f."id";
        "#,
    )
    .bind(invite_id)
    .bind(&member_id)
    .bind(user_id)
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

    Ok(JoinFamilyResponse {
        family_id: invite.get("familyId"),
        family_name: invite.get("familyName"),
        role: invite.get("role"),
    })
}

/// Verifies that a user is an active member of a family and returns their (member_id, authority).
pub async fn verify_family_member(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
) -> Result<(String, String), ServiceError> {
    let row = sqlx::query(
        r#"
        SELECT "id", "authority"::TEXT AS authority
        FROM "FamilyMember"
        WHERE "familyId" = $1 AND "userId" = $2 AND "removedAt" IS NULL
        LIMIT 1;
        "#,
    )
    .bind(family_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Database error: {e}")))?;

    match row {
        Some(r) => Ok((r.get("id"), r.get("authority"))),
        None => Err(ServiceError::Forbidden(
            "You are not a member of this family".into(),
        )),
    }
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
