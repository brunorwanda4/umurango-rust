//! Family loans and borrowed balances service operations.

use crate::auth::{verify_family_member, ServiceError};
use crate::transactions::create_transaction;
use sqlx::{PgPool, Row};
use umuryango_shared::{
    CreateLoanRequest, CreateTransactionRequest, LoanDto, LoanRepaymentRequest,
};
use uuid::Uuid;

/// Lists all active and completed loans for a family workspace.
pub async fn list_loans(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
) -> Result<Vec<LoanDto>, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let rows = sqlx::query(
        r#"
        SELECT
            l."id",
            l."familyId",
            l."title",
            l."loanType"::TEXT AS loan_type,
            COALESCE(l."borrowerName", l."lenderName", 'Family Contact') AS counterparty,
            l."totalExpectedAmountMinorUnits" AS total_amount,
            l."paidAmountMinorUnits" AS paid_amount,
            l."remainingBalanceMinorUnits" AS remaining_amount,
            l."currency",
            l."status"::TEXT AS status,
            l."dueDate"::TEXT AS due_date
        FROM "FamilyLoan" l
        WHERE l."familyId" = $1 AND l."deletedAt" IS NULL
        ORDER BY l."createdAt" DESC;
        "#,
    )
    .bind(family_id)
    .fetch_all(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to list loans: {e}")))?;

    let loans = rows
        .into_iter()
        .map(|r| {
            let loan_type: String = r.get("loan_type");
            LoanDto {
                id: r.get("id"),
                family_id: r.get("familyId"),
                title: r.get("title"),
                counterparty_name: r.get("counterparty"),
                is_lent: loan_type == "LENT",
                total_amount_minor_units: r.get("total_amount"),
                paid_amount_minor_units: r.get("paid_amount"),
                remaining_amount_minor_units: r.get("remaining_amount"),
                currency: r.get("currency"),
                status: r.get("status"),
                due_date: r.get("due_date"),
            }
        })
        .collect();

    Ok(loans)
}

/// Creates a new family loan (lent or borrowed).
pub async fn create_loan(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    req: &CreateLoanRequest,
) -> Result<LoanDto, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    let clean_title = req.title.trim();
    if clean_title.is_empty() {
        return Err(ServiceError::BadRequest(
            "Loan title cannot be empty".into(),
        ));
    }
    let clean_person = req.counterparty_name.trim();
    if clean_person.is_empty() {
        return Err(ServiceError::BadRequest(
            "Loan counterparty name cannot be empty".into(),
        ));
    }
    if req.total_amount_minor_units <= 0 {
        return Err(ServiceError::BadRequest(
            "Loan amount must be greater than zero".into(),
        ));
    }

    let paid = req
        .paid_amount_minor_units
        .clamp(0, req.total_amount_minor_units);
    let remaining = (req.total_amount_minor_units - paid).max(0);
    let loan_type = if req.is_lent { "LENT" } else { "BORROWED" };
    let status = if remaining == 0 {
        "COMPLETED"
    } else {
        "ACTIVE"
    };
    let borrower_name = if req.is_lent {
        Some(clean_person)
    } else {
        None
    };
    let lender_name = if !req.is_lent {
        Some(clean_person)
    } else {
        None
    };

    let loan_id = Uuid::new_v4().to_string();
    let row = sqlx::query(
        r#"
        INSERT INTO "FamilyLoan" (
            "id", "familyId", "title", "loanType", "principalAmountMinorUnits",
            "totalExpectedAmountMinorUnits", "paidAmountMinorUnits", "remainingBalanceMinorUnits",
            "currency", "borrowerName", "lenderName", "startDate", "dueDate",
            "status", "approvalStatus", "createdById", "createdAt", "updatedAt"
        )
        VALUES (
            $1, $2, $3, $4::"LoanType", $5,
            $5, $6, $7,
            'RWF', $8, $9, NOW(), COALESCE($10::TIMESTAMPTZ, NOW() + INTERVAL '45 days'),
            $11::"LoanStatus", 'APPROVED', $12, NOW(), NOW()
        )
        RETURNING "id", "familyId", "title", "loanType"::TEXT AS loan_type,
                  COALESCE("borrowerName", "lenderName", 'Family Contact') AS counterparty,
                  "totalExpectedAmountMinorUnits" AS total_amount,
                  "paidAmountMinorUnits" AS paid_amount,
                  "remainingBalanceMinorUnits" AS remaining_amount,
                  "currency", "status"::TEXT AS status, "dueDate"::TEXT AS due_date;
        "#,
    )
    .bind(&loan_id)
    .bind(family_id)
    .bind(clean_title)
    .bind(loan_type)
    .bind(req.total_amount_minor_units)
    .bind(paid)
    .bind(remaining)
    .bind(borrower_name)
    .bind(lender_name)
    .bind(req.due_date.as_deref())
    .bind(status)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to create loan: {e}")))?;

    let lt: String = row.get("loan_type");
    Ok(LoanDto {
        id: row.get("id"),
        family_id: row.get("familyId"),
        title: row.get("title"),
        counterparty_name: row.get("counterparty"),
        is_lent: lt == "LENT",
        total_amount_minor_units: row.get("total_amount"),
        paid_amount_minor_units: row.get("paid_amount"),
        remaining_amount_minor_units: row.get("remaining_amount"),
        currency: row.get("currency"),
        status: row.get("status"),
        due_date: row.get("due_date"),
    })
}

/// Records a repayment against a loan and logs the corresponding treasury transaction.
pub async fn repay_loan(
    pool: &PgPool,
    user_id: &str,
    family_id: &str,
    loan_id: &str,
    req: &LoanRepaymentRequest,
) -> Result<LoanDto, ServiceError> {
    verify_family_member(pool, user_id, family_id).await?;

    if req.amount_minor_units <= 0 {
        return Err(ServiceError::BadRequest(
            "Repayment amount must be greater than zero".into(),
        ));
    }

    let repayment_id = Uuid::new_v4().to_string();

    let row = sqlx::query(
        r#"
        WITH updated_loan AS (
            UPDATE "FamilyLoan"
            SET "paidAmountMinorUnits" = LEAST("totalExpectedAmountMinorUnits", "paidAmountMinorUnits" + $1),
                "remainingBalanceMinorUnits" = GREATEST(0, "totalExpectedAmountMinorUnits" - ("paidAmountMinorUnits" + $1)),
                "status" = CASE
                    WHEN ("paidAmountMinorUnits" + $1) >= "totalExpectedAmountMinorUnits" THEN 'COMPLETED'::"LoanStatus"
                    ELSE "status"
                END,
                "updatedById" = $2,
                "updatedAt" = NOW()
            WHERE "id" = $3 AND "familyId" = $4 AND "deletedAt" IS NULL
            RETURNING "id", "familyId", "title", "loanType"::TEXT AS loan_type,
                      COALESCE("borrowerName", "lenderName", 'Family Contact') AS counterparty,
                      "totalExpectedAmountMinorUnits" AS total_amount,
                      "paidAmountMinorUnits" AS paid_amount,
                      "remainingBalanceMinorUnits" AS remaining_amount,
                      "currency", "status"::TEXT AS status, "dueDate"::TEXT AS due_date
        ),
        ins_repayment AS (
            INSERT INTO "FamilyLoanRepayment" (
                "id", "loanId", "familyId", "amountMinorUnits", "paymentDate",
                "paymentMethod", "notes", "createdById", "createdAt", "updatedAt"
            )
            SELECT $5, ul."id", ul."familyId", $1, NOW(), 'MOBILE_MONEY', $6, $2, NOW(), NOW()
            FROM updated_loan ul
        )
        SELECT * FROM updated_loan;
        "#,
    )
    .bind(req.amount_minor_units)
    .bind(user_id)
    .bind(loan_id)
    .bind(family_id)
    .bind(&repayment_id)
    .bind(req.note.as_deref())
    .fetch_optional(pool)
    .await
    .map_err(|e| ServiceError::Internal(format!("Failed to record loan repayment: {e}")))?;

    let row = match row {
        Some(r) => r,
        None => return Err(ServiceError::NotFound("Loan not found".into())),
    };

    let loan_type: String = row.get("loan_type");
    let is_lent = loan_type == "LENT";
    let counterparty: String = row.get("counterparty");

    // Also record a corresponding transaction so the treasury summary and ledger stay accurate.
    let tx_req = CreateTransactionRequest {
        amount_minor_units: req.amount_minor_units,
        is_income: is_lent,
        category: "Loan Repayment".into(),
        description: format!("Loan Repayment · {counterparty}"),
        notes: req.note.clone(),
        idempotency_key: Some(Uuid::new_v4().to_string()),
    };
    let _ = create_transaction(pool, user_id, family_id, &tx_req).await;

    Ok(LoanDto {
        id: row.get("id"),
        family_id: row.get("familyId"),
        title: row.get("title"),
        counterparty_name: counterparty,
        is_lent,
        total_amount_minor_units: row.get("total_amount"),
        paid_amount_minor_units: row.get("paid_amount"),
        remaining_amount_minor_units: row.get("remaining_amount"),
        currency: row.get("currency"),
        status: row.get("status"),
        due_date: row.get("due_date"),
    })
}
