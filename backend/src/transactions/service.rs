//! Authoritative transactions and financial treasury service operations.

use crate::auth::{verify_family_member, ServiceError};
use sqlx::{PgPool, Row};
use umuryango_shared::{CreateTransactionRequest, TransactionItemDto, TreasurySummaryDto};
use uuid::Uuid;

/// Retrieves the transactions list for a family workspace.
pub async fn list_transactions(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    filter: Option<&str>,
) -> Result<Vec<TransactionItemDto>, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let filter_type = match filter {
        Some(f) if f.eq_ignore_ascii_case("contributions") || f.eq_ignore_ascii_case("income") => {
            Some("INCOME")
        }
        Some(f) if f.eq_ignore_ascii_case("expenses") || f.eq_ignore_ascii_case("expense") => {
            Some("EXPENSE")
        }
        _ => None,
    };

    let rows = sqlx::query(
        r#"
        SELECT 
            t."id",
            t."familyId",
            COALESCE(u."fullName", 'Family Member') AS actor_name,
            COALESCE(t."description", 'Transaction') AS title,
            t."amountMinorUnits" AS amount,
            t."currency",
            t."type"::TEXT AS tx_type,
            COALESCE(c."name", 'General') AS category_name,
            t."status"::TEXT AS tx_status,
            t."date"::TEXT AS occurred_at,
            t."notes"
        FROM "FamilyTransaction" t
        LEFT JOIN "User" u ON t."createdById" = u."id"
        LEFT JOIN "FamilyTransactionCategory" c ON t."categoryId" = c."id"
        WHERE t."familyId" = $1
          AND ($2::TEXT IS NULL OR t."type"::TEXT = $2)
        ORDER BY t."date" DESC, t."createdAt" DESC
        LIMIT 100;
        "#,
    )
    .bind(family_id)
    .bind(filter_type)
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to list transactions: {e}")))?;

    let items = rows
        .into_iter()
        .map(|r| {
            let tx_type: String = r.get("tx_type");
            TransactionItemDto {
                id: r.get("id"),
                family_id: r.get("familyId"),
                actor_name: r.get("actor_name"),
                title: r.get("title"),
                amount_minor_units: r.get("amount"),
                currency: r.get("currency"),
                is_income: tx_type == "INCOME",
                category: r.get("category_name"),
                status: r.get("tx_status"),
                occurred_at: r.get("occurred_at"),
                notes: r.get("notes"),
            }
        })
        .collect();

    Ok(items)
}

/// Records a new financial transaction (contribution, income, or expense).
pub async fn create_transaction(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    req: &CreateTransactionRequest,
) -> Result<TransactionItemDto, ServiceError> {
    let (_, authority) = verify_family_member(pool, user_id, family_id).await?;

    if req.amount_minor_units <= 0 {
        return Err(ServiceError::BadRequest(
            "Transaction amount must be greater than zero".into(),
        ));
    }

    let clean_desc = req.description.trim();
    if clean_desc.is_empty() {
        return Err(ServiceError::BadRequest(
            "Transaction description cannot be empty".into(),
        ));
    }

    let tx_type = if req.is_income { "INCOME" } else { "EXPENSE" };

    // Income is automatically approved. Expenses by HEAD/ADMIN are approved, others are pending.
    let status = if req.is_income || authority == "HEAD" || authority == "ADMIN" {
        "APPROVED"
    } else {
        "PENDING"
    };

    let category_name = if req.category.trim().is_empty() {
        if req.is_income {
            "Contribution"
        } else {
            "General"
        }
    } else {
        req.category.trim()
    };

    // Ensure category exists
    let cat_id = Uuid::new_v4().to_string();
    let category_row = sqlx::query(
        r#"
        WITH existing_cat AS (
            SELECT "id" FROM "FamilyTransactionCategory"
            WHERE "familyId" = $1 AND "type"::TEXT = $2 AND "name" = $3
            LIMIT 1
        ),
        inserted_cat AS (
            INSERT INTO "FamilyTransactionCategory" (
                "id", "familyId", "name", "type", "scope", "status", "isDefault", "createdAt", "updatedAt"
            )
            SELECT $4, $1, $3, $2::"TransactionType", 'FAMILY', 'ACTIVE', false, NOW(), NOW()
            WHERE NOT EXISTS (SELECT 1 FROM existing_cat)
            RETURNING "id"
        )
        SELECT "id" FROM existing_cat
        UNION ALL
        SELECT "id" FROM inserted_cat
        LIMIT 1;
        "#,
    )
    .bind(family_id)
    .bind(tx_type)
    .bind(category_name)
    .bind(&cat_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Category resolution error: {e}")))?;

    let resolved_cat_id: String = category_row.get("id");

    let tx_id = Uuid::new_v4().to_string();
    let idempotency = req
        .idempotency_key
        .clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let row = sqlx::query(
        r#"
        INSERT INTO "FamilyTransaction" (
            "id", "familyId", "categoryId", "amountMinorUnits", "currency", "type", "scope",
            "status", "date", "description", "idempotencyKey", "createdById", "notes", "createdAt", "updatedAt"
        )
        VALUES (
            $1, $2, $3, $4, 'RWF', $5::"TransactionType", 'FAMILY',
            $6::"TransactionStatus", NOW(), $7, $8, $9, $10, NOW(), NOW()
        )
        RETURNING "id", "familyId", "amountMinorUnits", "currency", "type"::TEXT AS tx_type,
                  "status"::TEXT AS tx_status, "date"::TEXT AS occurred_at, "description", "notes";
        "#,
    )
    .bind(&tx_id)
    .bind(family_id)
    .bind(&resolved_cat_id)
    .bind(req.amount_minor_units)
    .bind(tx_type)
    .bind(status)
    .bind(clean_desc)
    .bind(&idempotency)
    .bind(user_id)
    .bind(req.notes.as_deref())
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to record transaction: {e}")))?;

    // Fetch user full name
    let user_name = sqlx::query_scalar::<_, String>(
        r#"SELECT "fullName" FROM "User" WHERE "id" = $1 LIMIT 1;"#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
    .unwrap_or_else(|| "Family Member".into());

    Ok(TransactionItemDto {
        id: row.get("id"),
        family_id: row.get("familyId"),
        actor_name: user_name,
        title: row.get("description"),
        amount_minor_units: row.get("amountMinorUnits"),
        currency: row.get("currency"),
        is_income: row.get::<String, _>("tx_type") == "INCOME",
        category: category_name.to_string(),
        status: row.get("tx_status"),
        occurred_at: row.get("occurred_at"),
        notes: row.get("notes"),
    })
}

/// Retrieves financial metrics for a family workspace.
pub async fn get_treasury_summary(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
) -> Result<TreasurySummaryDto, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN "type" = 'INCOME' AND "status" = 'APPROVED' THEN "amountMinorUnits"
                              WHEN "type" = 'EXPENSE' AND "status" = 'APPROVED' THEN -"amountMinorUnits"
                              ELSE 0 END), 0) AS total_treasury,
            COALESCE(SUM(CASE WHEN "type" = 'INCOME' AND "status" = 'APPROVED' AND "date" >= date_trunc('month', NOW())
                              THEN "amountMinorUnits" ELSE 0 END), 0) AS monthly_contributions,
            COALESCE(SUM(CASE WHEN "type" = 'EXPENSE' AND "status" = 'APPROVED' AND "date" >= date_trunc('month', NOW())
                              THEN "amountMinorUnits" ELSE 0 END), 0) AS monthly_expenses,
            COUNT(CASE WHEN "status" = 'PENDING' THEN 1 END) AS pending_approvals
        FROM "FamilyTransaction"
        WHERE "familyId" = $1;
        "#,
    )
    .bind(family_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to calculate treasury summary: {e}")))?;

    let total_treasury: i64 = row.get("total_treasury");
    let monthly_contributions: i64 = row.get("monthly_contributions");
    let monthly_expenses: i64 = row.get("monthly_expenses");
    let pending_approval_count: i64 = row.get("pending_approvals");

    // Emergency reserve is calculated from active projects or 40% of treasury
    let reserve = (total_treasury * 40 / 100).max(0);

    Ok(TreasurySummaryDto {
        total_treasury_minor_units: total_treasury,
        monthly_contributions_minor_units: monthly_contributions,
        monthly_expenses_minor_units: monthly_expenses,
        emergency_reserve_minor_units: reserve,
        currency: "RWF".into(),
        pending_approval_count: pending_approval_count as usize,
    })
}

/// Approves a pending transaction. Only family HEAD or ADMIN can approve.
pub async fn approve_transaction(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    transaction_id: &str,
) -> Result<TransactionItemDto, ServiceError> {
    let (_, authority) = verify_family_member(pool, user_id, family_id).await?;

    if authority != "HEAD" && authority != "ADMIN" {
        return Err(ServiceError::Forbidden(
            "Only family administrators can approve transactions".into(),
        ));
    }

    let row = sqlx::query(
        r#"
        UPDATE "FamilyTransaction"
        SET "status" = 'APPROVED', "approvedById" = $1, "approvedAt" = NOW(), "updatedAt" = NOW()
        WHERE "id" = $2 AND "familyId" = $3
        RETURNING "id", "familyId", "amountMinorUnits", "currency", "type"::TEXT AS tx_type,
                  "status"::TEXT AS tx_status, "date"::TEXT AS occurred_at, "description", "notes";
        "#,
    )
    .bind(user_id)
    .bind(transaction_id)
    .bind(family_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to approve transaction: {e}")))?;

    let row = match row {
        Some(r) => r,
        None => return Err(ServiceError::NotFound("Transaction not found".into())),
    };

    Ok(TransactionItemDto {
        id: row.get("id"),
        family_id: row.get("familyId"),
        actor_name: "Family Member".into(),
        title: row.get("description"),
        amount_minor_units: row.get("amountMinorUnits"),
        currency: row.get("currency"),
        is_income: row.get::<String, _>("tx_type") == "INCOME",
        category: "General".into(),
        status: row.get("tx_status"),
        occurred_at: row.get("occurred_at"),
        notes: row.get("notes"),
    })
}

/// Rejects a pending transaction. Only family HEAD or ADMIN can reject.
pub async fn reject_transaction(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    transaction_id: &str,
) -> Result<TransactionItemDto, ServiceError> {
    let (_, authority) = verify_family_member(pool, user_id, family_id).await?;

    if authority != "HEAD" && authority != "ADMIN" {
        return Err(ServiceError::Forbidden(
            "Only family administrators can reject transactions".into(),
        ));
    }

    let row = sqlx::query(
        r#"
        UPDATE "FamilyTransaction"
        SET "status" = 'REJECTED', "approvedById" = $1, "approvedAt" = NOW(), "updatedAt" = NOW()
        WHERE "id" = $2 AND "familyId" = $3
        RETURNING "id", "familyId", "amountMinorUnits", "currency", "type"::TEXT AS tx_type,
                  "status"::TEXT AS tx_status, "date"::TEXT AS occurred_at, "description", "notes";
        "#,
    )
    .bind(user_id)
    .bind(transaction_id)
    .bind(family_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to reject transaction: {e}")))?;

    let row = match row {
        Some(r) => r,
        None => return Err(ServiceError::NotFound("Transaction not found".into())),
    };

    Ok(TransactionItemDto {
        id: row.get("id"),
        family_id: row.get("familyId"),
        actor_name: "Family Member".into(),
        title: row.get("description"),
        amount_minor_units: row.get("amountMinorUnits"),
        currency: row.get("currency"),
        is_income: row.get::<String, _>("tx_type") == "INCOME",
        category: "General".into(),
        status: row.get("tx_status"),
        occurred_at: row.get("occurred_at"),
        notes: row.get("notes"),
    })
}
