use umuryango_backend::schema::{apply_schema, connect, list_tables, SCHEMA_SQL};

#[test]
fn schema_sql_is_valid_and_non_empty() {
    assert!(!SCHEMA_SQL.is_empty());
    assert!(SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS \"User\""));
    assert!(SCHEMA_SQL.contains("\"email\" TEXT"));
    assert!(SCHEMA_SQL.contains("ALTER TABLE \"User\" ALTER COLUMN \"email\" DROP NOT NULL"));
    assert!(SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS \"Family\""));
    assert!(SCHEMA_SQL.contains("\"joinCode\" TEXT"));
    assert!(SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS \"FamilyMember\""));
    assert!(SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS \"FamilyTransaction\""));
    assert!(SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS \"FamilyLoan\""));
    assert!(SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS \"FamilySyncMutation\""));
}

#[tokio::test]
async fn test_apply_schema_with_tokio_postgres() {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://antigravity@localhost:5433/umuryango_test".into());

    let (client, _handle) = match connect(&db_url).await {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Skipping database connection test (server not reachable): {err}");
            return;
        }
    };

    apply_schema(&client)
        .await
        .expect("apply_schema should execute successfully with tokio-postgres");

    let tables = list_tables(&client)
        .await
        .expect("list_tables should return tables");

    // All 31 tables from schema.prisma must exist
    let expected_tables = [
        "AuthAuditEvent",
        "AuthSession",
        "EmailVerification",
        "Family",
        "FamilyAuditEvent",
        "FamilyBudget",
        "FamilyContact",
        "FamilyContactDisplayName",
        "FamilyContributionAssignment",
        "FamilyContributionCycle",
        "FamilyContributionPayment",
        "FamilyContributionPlan",
        "FamilyInvite",
        "FamilyLoan",
        "FamilyLoanAuditLog",
        "FamilyLoanDocument",
        "FamilyLoanRepayment",
        "FamilyMember",
        "FamilyProject",
        "FamilyProjectContribution",
        "FamilyProjectParticipant",
        "FamilyProjectUpdate",
        "FamilyReceipt",
        "FamilySyncMutation",
        "FamilyTransaction",
        "FamilyTransactionCategory",
        "Notification",
        "PasswordReset",
        "PersonPlan",
        "PersonPlanMilestone",
        "User",
    ];

    for table in expected_tables {
        assert!(
            tables.contains(&table.to_string()),
            "Expected table '{table}' to be created in public schema, but got: {:?}",
            tables
        );
    }
}
