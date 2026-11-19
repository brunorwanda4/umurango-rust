//! Communal family projects service operations.

use crate::auth::{verify_family_member, ServiceError};
use sqlx::{PgPool, Row};
use umuryango_shared::{CreateProjectRequest, ProjectDto};
use uuid::Uuid;

/// Lists all communal projects in a family workspace with progress and contributor counts.
pub async fn list_projects(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
) -> Result<Vec<ProjectDto>, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let rows = sqlx::query(
        r#"
        SELECT 
            p."id",
            p."familyId",
            p."name",
            p."category"::TEXT AS category,
            p."description",
            p."targetAmountMinorUnits" AS target_amount,
            p."currency",
            p."status"::TEXT AS status,
            p."targetDate"::TEXT AS target_date,
            COALESCE(SUM(c."amountMinorUnits"), 0) AS raised_amount,
            COUNT(DISTINCT c."memberId") AS contributors_count
        FROM "FamilyProject" p
        LEFT JOIN "FamilyProjectContribution" c ON p."id" = c."projectId"
        WHERE p."familyId" = $1
        GROUP BY p."id"
        ORDER BY p."createdAt" DESC;
        "#,
    )
    .bind(family_id)
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to list projects: {e}")))?;

    let projects = rows
        .into_iter()
        .map(|r| {
            let target: i64 = r.get("target_amount");
            let raised: i64 = r.get("raised_amount");
            let progress = if target > 0 {
                ((raised * 100) / target).min(100) as u8
            } else {
                0
            };
            let contribs: i64 = r.get("contributors_count");

            ProjectDto {
                id: r.get("id"),
                family_id: r.get("familyId"),
                name: r.get("name"),
                category: r.get("category"),
                description: r.get("description"),
                target_amount_minor_units: target,
                raised_amount_minor_units: raised,
                currency: r.get("currency"),
                progress_percentage: progress,
                contributors_count: contribs as usize,
                status: r.get("status"),
                target_date: r.get("target_date"),
            }
        })
        .collect();

    Ok(projects)
}

/// Creates a new communal project for a family workspace.
pub async fn create_project(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    req: &CreateProjectRequest,
) -> Result<ProjectDto, ServiceError> {
    let (member_id, _) = verify_family_member(pool, user_id, family_id).await?;

    let clean_name = req.name.trim();
    if clean_name.is_empty() {
        return Err(ServiceError::BadRequest(
            "Project name cannot be empty".into(),
        ));
    }

    if req.target_amount_minor_units <= 0 {
        return Err(ServiceError::BadRequest(
            "Project target amount must be greater than zero".into(),
        ));
    }

    let project_id = Uuid::new_v4().to_string();
    let category = match req.category.trim().to_uppercase().as_str() {
        "HOUSE" | "VEHICLE" | "EDUCATION" | "BUSINESS" | "EVENT" => {
            req.category.trim().to_uppercase()
        }
        _ => "OTHER".to_string(),
    };

    let target_date = req.target_date.as_deref();

    let row = sqlx::query(
        r#"
        INSERT INTO "FamilyProject" (
            "id", "familyId", "name", "category", "description", "targetAmountMinorUnits",
            "currency", "targetDate", "status", "ownerMemberId", "contributionStrategy",
            "createdById", "createdAt", "updatedAt"
        )
        VALUES (
            $1, $2, $3, $4::"FamilyProjectCategory", $5, $6,
            'RWF', COALESCE($7::TIMESTAMPTZ, NOW() + INTERVAL '60 days'), 'ACTIVE', $8, 'AS_AVAILABLE',
            $9, NOW(), NOW()
        )
        RETURNING "id", "familyId", "name", "category"::TEXT AS category, "description",
                  "targetAmountMinorUnits" AS target_amount, "currency", "status"::TEXT AS status,
                  "targetDate"::TEXT AS target_date;
        "#,
    )
    .bind(&project_id)
    .bind(family_id)
    .bind(clean_name)
    .bind(&category)
    .bind(req.description.as_deref())
    .bind(req.target_amount_minor_units)
    .bind(target_date)
    .bind(&member_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to create project: {e}")))?;

    Ok(ProjectDto {
        id: row.get("id"),
        family_id: row.get("familyId"),
        name: row.get("name"),
        category: row.get("category"),
        description: row.get("description"),
        target_amount_minor_units: row.get("target_amount"),
        raised_amount_minor_units: 0,
        currency: row.get("currency"),
        progress_percentage: 0,
        contributors_count: 0,
        status: row.get("status"),
        target_date: row.get("target_date"),
    })
}

/// Contributes money toward an active communal project.
pub async fn contribute_to_project(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    project_id: &str,
    amount_minor_units: i64,
    note: Option<&str>,
) -> Result<ProjectDto, ServiceError> {
    let (member_id, _) = verify_family_member(pool, user_id, family_id).await?;

    if amount_minor_units <= 0 {
        return Err(ServiceError::BadRequest(
            "Contribution amount must be greater than zero".into(),
        ));
    }

    let contrib_id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO "FamilyProjectContribution" (
            "id", "familyId", "projectId", "memberId", "amountMinorUnits", "currency",
            "status", "note", "paidAt", "recordedById", "createdAt", "updatedAt"
        )
        VALUES ($1, $2, $3, $4, $5, 'RWF', 'VERIFIED', $6, NOW(), $7, NOW(), NOW());
        "#,
    )
    .bind(&contrib_id)
    .bind(family_id)
    .bind(project_id)
    .bind(&member_id)
    .bind(amount_minor_units)
    .bind(note)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to record project contribution: {e}")))?;

    // Fetch updated project stats
    let projects = list_projects(pool, user_id, family_id).await?;
    projects
        .into_iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| ServiceError::NotFound("Project not found".into()))
}
