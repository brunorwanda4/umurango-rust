//! Household members roster service operations.

use crate::auth::{verify_family_member, ServiceError};
use sqlx::{PgPool, Row};
use umuryango_shared::{generate_default_avatar_url, FamilyMemberDto};

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
