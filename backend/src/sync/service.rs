//! Idempotent mutation outbox sync service.

use crate::auth::{verify_family_member, ServiceError};
use sqlx::{PgPool, Row};
use umuryango_shared::{
    CommandType, SyncMutationResult, SyncPullRequest, SyncPullResponse, SyncPushRequest,
    SyncPushResponse, SyncStatus, Transaction, TransactionType,
};
use uuid::Uuid;

/// Processes an array of pending mutations pushed from a client's local SQLite outbox.
pub async fn push_mutations(
    pool: &PgPool,
    user_id: &str,
    req: &SyncPushRequest,
) -> Result<SyncPushResponse, ServiceError> {
    let mut results = Vec::new();

    for mutation in &req.mutations {
        let key_str = mutation.idempotency_key.to_string();
        let family_id_str = mutation.family_id.to_string();

        // 1. Check idempotency: if already processed, return existing accepted result
        let existing = sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS(SELECT 1 FROM "FamilySyncMutation" WHERE "familyId" = $1 AND "idempotencyKey" = $2);"#,
        )
        .bind(&family_id_str)
        .bind(&key_str)
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if existing {
            results.push(SyncMutationResult {
                idempotency_key: mutation.idempotency_key,
                status: SyncStatus::Synced,
                error: None,
            });
            continue;
        }

        // 2. Enforce membership
        let member_check = verify_family_member(pool, user_id, &family_id_str).await;
        let (member_id, authority) = match member_check {
            Ok(m) => m,
            Err(e) => {
                results.push(SyncMutationResult {
                    idempotency_key: mutation.idempotency_key,
                    status: SyncStatus::Failed,
                    error: Some(e.to_string()),
                });
                continue;
            }
        };

        // 3. Process mutation by command type
        let mut mutation_error = None;
        match mutation.command {
            CommandType::CreateContribution => {
                let amount = mutation
                    .payload
                    .get("amount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let description = mutation
                    .payload
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contribution");
                let cat_name = mutation
                    .payload
                    .get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Contribution");

                if amount <= 0 {
                    mutation_error = Some("Amount must be greater than zero".to_string());
                } else {
                    let cat_id = Uuid::new_v4().to_string();
                    let tx_id = Uuid::new_v4().to_string();
                    let res = sqlx::query(
                        r#"
                        WITH cat AS (
                            INSERT INTO "FamilyTransactionCategory" (
                                "id", "familyId", "name", "type", "scope", "status", "isDefault", "createdAt", "updatedAt"
                            )
                            VALUES ($1, $2, $3, 'INCOME', 'FAMILY', 'ACTIVE', false, NOW(), NOW())
                            ON CONFLICT ("familyId", "scope", "ownerId", "type", "name") DO UPDATE SET "updatedAt" = NOW()
                            RETURNING "id"
                        )
                        INSERT INTO "FamilyTransaction" (
                            "id", "familyId", "categoryId", "amountMinorUnits", "currency", "type", "scope",
                            "status", "date", "description", "idempotencyKey", "createdById", "createdAt", "updatedAt"
                        )
                        SELECT $4, $2, cat."id", $5, 'RWF', 'INCOME', 'FAMILY', 'APPROVED', NOW(), $6, $7, $8, NOW(), NOW()
                        FROM cat;
                        "#
                    )
                    .bind(&cat_id)
                    .bind(&family_id_str)
                    .bind(cat_name)
                    .bind(&tx_id)
                    .bind(amount)
                    .bind(description)
                    .bind(&key_str)
                    .bind(user_id)
                    .execute(pool)
                    .await;

                    if let Err(e) = res {
                        mutation_error = Some(format!("Database error: {e}"));
                    }
                }
            }
            CommandType::CreateExpense => {
                let amount = mutation
                    .payload
                    .get("amount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let description = mutation
                    .payload
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Expense");
                let cat_name = mutation
                    .payload
                    .get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("General");
                let status = if authority == "HEAD" || authority == "ADMIN" {
                    "APPROVED"
                } else {
                    "PENDING"
                };

                if amount <= 0 {
                    mutation_error = Some("Amount must be greater than zero".to_string());
                } else {
                    let cat_id = Uuid::new_v4().to_string();
                    let tx_id = Uuid::new_v4().to_string();
                    let res = sqlx::query(
                        r#"
                        WITH cat AS (
                            INSERT INTO "FamilyTransactionCategory" (
                                "id", "familyId", "name", "type", "scope", "status", "isDefault", "createdAt", "updatedAt"
                            )
                            VALUES ($1, $2, $3, 'EXPENSE', 'FAMILY', 'ACTIVE', false, NOW(), NOW())
                            ON CONFLICT ("familyId", "scope", "ownerId", "type", "name") DO UPDATE SET "updatedAt" = NOW()
                            RETURNING "id"
                        )
                        INSERT INTO "FamilyTransaction" (
                            "id", "familyId", "categoryId", "amountMinorUnits", "currency", "type", "scope",
                            "status", "date", "description", "idempotencyKey", "createdById", "createdAt", "updatedAt"
                        )
                        SELECT $4, $2, cat."id", $5, 'RWF', 'EXPENSE', 'FAMILY', $9::"TransactionStatus", NOW(), $6, $7, $8, NOW(), NOW()
                        FROM cat;
                        "#
                    )
                    .bind(&cat_id)
                    .bind(&family_id_str)
                    .bind(cat_name)
                    .bind(&tx_id)
                    .bind(amount)
                    .bind(description)
                    .bind(&key_str)
                    .bind(user_id)
                    .bind(status)
                    .execute(pool)
                    .await;

                    if let Err(e) = res {
                        mutation_error = Some(format!("Database error: {e}"));
                    }
                }
            }
            CommandType::RecordPayment => {
                let amount = mutation
                    .payload
                    .get("amount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let description = mutation
                    .payload
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Payment");

                if amount <= 0 {
                    mutation_error = Some("Amount must be greater than zero".to_string());
                } else {
                    let cat_id = Uuid::new_v4().to_string();
                    let tx_id = Uuid::new_v4().to_string();
                    let _ = sqlx::query(
                        r#"
                        WITH cat AS (
                            INSERT INTO "FamilyTransactionCategory" (
                                "id", "familyId", "name", "type", "scope", "status", "isDefault", "createdAt", "updatedAt"
                            )
                            VALUES ($1, $2, 'Payment', 'INCOME', 'FAMILY', 'ACTIVE', false, NOW(), NOW())
                            ON CONFLICT ("familyId", "scope", "ownerId", "type", "name") DO UPDATE SET "updatedAt" = NOW()
                            RETURNING "id"
                        )
                        INSERT INTO "FamilyTransaction" (
                            "id", "familyId", "categoryId", "amountMinorUnits", "currency", "type", "scope",
                            "status", "date", "description", "idempotencyKey", "createdById", "createdAt", "updatedAt"
                        )
                        SELECT $4, $2, cat."id", $5, 'RWF', 'INCOME', 'FAMILY', 'APPROVED', NOW(), $6, $7, $8, NOW(), NOW()
                        FROM cat;
                        "#
                    )
                    .bind(&cat_id)
                    .bind(&family_id_str)
                    .bind(&tx_id)
                    .bind(amount)
                    .bind(description)
                    .bind(&key_str)
                    .bind(user_id)
                    .execute(pool)
                    .await;
                }
            }
            CommandType::UpdateFamily => {
                if let Some(new_name) = mutation.payload.get("name").and_then(|v| v.as_str()) {
                    let _ = sqlx::query(
                        r#"UPDATE "Family" SET "name" = $1, "updatedAt" = NOW() WHERE "id" = $2"#,
                    )
                    .bind(new_name)
                    .bind(&family_id_str)
                    .execute(pool)
                    .await;
                }
            }
        }

        // 4. Record mutation outcome in FamilySyncMutation
        let outcome = if mutation_error.is_none() {
            "ACCEPTED"
        } else {
            "REJECTED"
        };
        let sync_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            r#"
            INSERT INTO "FamilySyncMutation" (
                "id", "familyId", "actorId", "deviceId", "idempotencyKey", "localRecordId",
                "commandType", "payloadHash", "outcome", "responseJson", "createdAt", "updatedAt"
            )
            VALUES ($1, $2, $3, 'native-app', $4, $4, $5, 'hash', $6, '{}'::jsonb, NOW(), NOW())
            ON CONFLICT ("familyId", "idempotencyKey") DO NOTHING;
            "#,
        )
        .bind(&sync_id)
        .bind(&family_id_str)
        .bind(&member_id)
        .bind(&key_str)
        .bind(format!("{:?}", mutation.command))
        .bind(outcome)
        .execute(pool)
        .await;

        if let Some(err) = mutation_error {
            results.push(SyncMutationResult {
                idempotency_key: mutation.idempotency_key,
                status: SyncStatus::Failed,
                error: Some(err),
            });
        } else {
            results.push(SyncMutationResult {
                idempotency_key: mutation.idempotency_key,
                status: SyncStatus::Synced,
                error: None,
            });
        }
    }

    Ok(SyncPushResponse { results })
}

/// Pulls remote transaction records for a family workspace since the given cursor.
pub async fn pull_records(
    pool: &PgPool,
    user_id: &str,
    req: &SyncPullRequest,
) -> Result<SyncPullResponse, ServiceError> {
    let family_id_str = req.family_id.to_string();
    verify_family_member(pool, user_id, &family_id_str).await?;

    let rows = sqlx::query(
        r#"
        SELECT 
            t."id",
            t."familyId",
            t."createdById",
            t."type"::TEXT AS tx_type,
            t."amountMinorUnits" AS amount,
            COALESCE(t."description", '') AS description,
            t."date"::TEXT AS occurred_at,
            t."updatedAt"::TEXT AS updated_at
        FROM "FamilyTransaction" t
        WHERE t."familyId" = $1
          AND ($2::TIMESTAMPTZ IS NULL OR t."updatedAt" > $2::TIMESTAMPTZ)
        ORDER BY t."updatedAt" ASC
        LIMIT 100;
        "#,
    )
    .bind(&family_id_str)
    .bind(req.cursor.as_deref())
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to pull sync records: {e}")))?;

    let mut new_cursor = req.cursor.clone();
    let mut transactions = Vec::new();

    for r in rows {
        let id_str: String = r.get("id");
        let fid_str: String = r.get("familyId");
        let uid_str: String = r.get("createdById");
        let tx_type_str: String = r.get("tx_type");
        let updated_at: String = r.get("updated_at");

        new_cursor = Some(updated_at);

        let tx_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
        let fid = Uuid::parse_str(&fid_str).unwrap_or(req.family_id);
        let uid = Uuid::parse_str(&uid_str).unwrap_or_else(|_| Uuid::new_v4());

        transactions.push(Transaction {
            id: tx_id,
            family_id: fid,
            member_id: uid,
            transaction_type: if tx_type_str == "INCOME" {
                TransactionType::Income
            } else {
                TransactionType::Expense
            },
            amount: r.get("amount"),
            description: r.get("description"),
            occurred_at: r.get("occurred_at"),
            sync_status: SyncStatus::Synced,
        });
    }

    Ok(SyncPullResponse {
        cursor: new_cursor,
        transactions,
        expenses: Vec::new(),
        contributions: Vec::new(),
    })
}
