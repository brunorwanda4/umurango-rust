//! Household members roster service operations.

use crate::auth::{verify_family_member, ServiceError};
use sqlx::{PgPool, Row};
use umuryango_shared::{generate_default_avatar_url, FamilyMemberDto, InviteFamilyMemberRequest};
use uuid::Uuid;

/// Lists all active members in a family workspace.
pub async fn list_members(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
) -> Result<Vec<FamilyMemberDto>, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let rows = sqlx::query(
        r#"
        SELECT 
            m."id",
            m."userId",
            u."fullName",
            u."phoneNumber",
            u."email",
            m."authority"::TEXT AS role,
            u."avatarUrl",
            m."createdAt"::TEXT AS joined_at
        FROM "FamilyMember" m
        JOIN "User" u ON m."userId" = u."id"
        WHERE m."familyId" = $1 AND m."removedAt" IS NULL
        ORDER BY m."createdAt" ASC;
        "#,
    )
    .bind(family_id)
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to list family members: {e}")))?;

    let members = rows
        .into_iter()
        .map(|r| {
            let full_name: String = r.get("fullName");
            let avatar = r
                .get::<Option<String>, _>("avatarUrl")
                .or_else(|| Some(generate_default_avatar_url(&full_name)));

            FamilyMemberDto {
                id: r.get("id"),
                user_id: r.get("userId"),
                full_name,
                phone_number: r.get("phoneNumber"),
                email: r.get("email"),
                role: r.get("role"),
                avatar_url: avatar,
                joined_at: r.get("joined_at"),
            }
        })
        .collect();

    Ok(members)
}

/// Invites or adds a member to an existing family workspace by phone number.
pub async fn invite_member(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    req: &InviteFamilyMemberRequest,
) -> Result<FamilyMemberDto, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let clean_phone = req.phone.trim();
    if clean_phone.is_empty() {
        return Err(ServiceError::BadRequest(
            "Member phone number is required".into(),
        ));
    }

    let clean_name = req
        .full_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(clean_phone);
    let authority = match req
        .role
        .as_deref()
        .unwrap_or("MEMBER")
        .trim()
        .to_uppercase()
        .as_str()
    {
        "HEAD" => "HEAD",
        "ADMIN" => "ADMIN",
        _ => "MEMBER",
    };

    let avatar_url = generate_default_avatar_url(clean_name);
    let invite_id = Uuid::new_v4().to_string();
    let new_user_id = Uuid::new_v4().to_string();
    let new_member_id = Uuid::new_v4().to_string();

    let row = sqlx::query(
        r#"
        WITH ins_invite AS (
            INSERT INTO "FamilyInvite" (
                "id", "familyId", "invitedById", "phone", "authority", "status",
                "tokenHash", "expiresAt", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, $4, $5::"FamilyAuthority", 'PENDING', '', NOW() + INTERVAL '30 days', NOW(), NOW())
        ),
        existing_user AS (
            SELECT "id", "fullName", "phoneNumber", "email", "avatarUrl"
            FROM "User"
            WHERE "phoneNumber" = $4
            LIMIT 1
        ),
        created_user AS (
            INSERT INTO "User" (
                "id", "fullName", "phoneNumber", "avatarUrl", "status", "createdAt", "updatedAt"
            )
            SELECT $6, $7, $4, $8, 'ACTIVE', NOW(), NOW()
            WHERE NOT EXISTS (SELECT 1 FROM existing_user)
            RETURNING "id", "fullName", "phoneNumber", "email", "avatarUrl"
        ),
        target_user AS (
            SELECT * FROM existing_user
            UNION ALL
            SELECT * FROM created_user
            LIMIT 1
        ),
        upserted_member AS (
            INSERT INTO "FamilyMember" (
                "id", "familyId", "userId", "authority", "capabilities", "createdAt", "updatedAt"
            )
            SELECT $9, $2, tu."id", $5::"FamilyAuthority", '{}', NOW(), NOW()
            FROM target_user tu
            ON CONFLICT ("familyId", "userId") DO UPDATE
                SET "removedAt" = NULL, "updatedAt" = NOW()
            RETURNING "id", "userId", "authority"::TEXT AS role, "createdAt"::TEXT AS joined_at
        )
        SELECT
            um."id",
            um."userId",
            tu."fullName",
            tu."phoneNumber",
            tu."email",
            um.role,
            tu."avatarUrl",
            um.joined_at
        FROM upserted_member um
        CROSS JOIN target_user tu;
        "#,
    )
    .bind(&invite_id)
    .bind(family_id)
    .bind(user_id)
    .bind(clean_phone)
    .bind(authority)
    .bind(&new_user_id)
    .bind(clean_name)
    .bind(&avatar_url)
    .bind(&new_member_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to invite member: {e}")))?;

    let full_name: String = row.get("fullName");
    let avatar = row
        .get::<Option<String>, _>("avatarUrl")
        .or_else(|| Some(generate_default_avatar_url(&full_name)));

    Ok(FamilyMemberDto {
        id: row.get("id"),
        user_id: row.get("userId"),
        full_name,
        phone_number: row.get("phoneNumber"),
        email: row.get("email"),
        role: row.get("role"),
        avatar_url: avatar,
        joined_at: row.get("joined_at"),
    })
}
