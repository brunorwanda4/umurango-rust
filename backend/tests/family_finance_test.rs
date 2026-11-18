use sqlx::PgPool;
use umuryango_backend::{
    auth::register_user,
    members::list_members,
    projects::{contribute_to_project, create_project},
    schema::{apply_schema, connect},
    sync::{pull_records, push_mutations},
    transactions::{create_transaction, get_treasury_summary, list_transactions},
};
use umuryango_shared::{
    CommandType, CreateProjectRequest, CreateTransactionRequest, RegisterRequest, SyncMutation,
    SyncPullRequest, SyncPushRequest, SyncStatus,
};
use uuid::Uuid;

#[tokio::test]
async fn test_full_family_finance_lifecycle() {
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
        .expect("apply_schema should execute successfully");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("sqlx pool connection succeeds");

    let unique_suffix = Uuid::new_v4().to_string()[..8].to_string();
    let name = format!("David Mugisha {}", unique_suffix);
    let phone = format!("+250789{}", &unique_suffix[..6]);

    // 1. Register user (auto-provisions primary family workspace)
    let reg_req = RegisterRequest {
        name: name.clone(),
        phone: phone.clone(),
        email: None,
        password: "ValidPassword123".into(),
        avatar_url: None,
    };
    let auth = register_user(&pool, &reg_req, "test-secret-at-least-32-chars-long!")
        .await
        .expect("Registration succeeds");

    let user_id = auth.user.id;
    let family_id = auth.family_id.expect("Family ID must be returned");

    // 2. Initial treasury summary must be zero
    let initial_summary = get_treasury_summary(&pool, &user_id, &family_id)
        .await
        .expect("Fetch initial summary");
    assert_eq!(initial_summary.total_treasury_minor_units, 0);
    assert_eq!(initial_summary.monthly_contributions_minor_units, 0);
    assert_eq!(initial_summary.monthly_expenses_minor_units, 0);
    assert_eq!(initial_summary.pending_approval_count, 0);

    // 3. Record a contribution: 50,000 RWF (income)
    let contrib_req = CreateTransactionRequest {
        amount_minor_units: 50_000,
        is_income: true,
        category: "Monthly Dues".into(),
        description: "November Household Contribution".into(),
        notes: Some("Via Mobile Money".into()),
        idempotency_key: Some(Uuid::new_v4().to_string()),
    };
    let contrib_tx = create_transaction(&pool, &user_id, &family_id, &contrib_req)
        .await
        .expect("Record contribution");
    assert_eq!(contrib_tx.amount_minor_units, 50_000);
    assert!(contrib_tx.is_income);
    assert_eq!(contrib_tx.status, "APPROVED");

    // 4. Verify summary after contribution
    let summary_after_contrib = get_treasury_summary(&pool, &user_id, &family_id)
        .await
        .expect("Fetch summary after contribution");
    assert_eq!(summary_after_contrib.total_treasury_minor_units, 50_000);
    assert_eq!(
        summary_after_contrib.monthly_contributions_minor_units,
        50_000
    );

    // 5. Record an expense: 15,000 RWF
    let expense_req = CreateTransactionRequest {
        amount_minor_units: 15_000,
        is_income: false,
        category: "Groceries".into(),
        description: "Weekly Food Supplies".into(),
        notes: None,
        idempotency_key: Some(Uuid::new_v4().to_string()),
    };
    let expense_tx = create_transaction(&pool, &user_id, &family_id, &expense_req)
        .await
        .expect("Record expense");
    assert_eq!(expense_tx.amount_minor_units, 15_000);
    assert!(!expense_tx.is_income);

    // 6. Verify summary after expense (50,000 - 15,000 = 35,000)
    let summary_after_expense = get_treasury_summary(&pool, &user_id, &family_id)
        .await
        .expect("Fetch summary after expense");
    assert_eq!(summary_after_expense.total_treasury_minor_units, 35_000);
    assert_eq!(summary_after_expense.monthly_expenses_minor_units, 15_000);

    // 7. Verify list_transactions
    let txs = list_transactions(&pool, &user_id, &family_id, None)
        .await
        .expect("List transactions");
    assert_eq!(txs.len(), 2);

    let contribs_only = list_transactions(&pool, &user_id, &family_id, Some("contributions"))
        .await
        .expect("Filter contributions");
    assert_eq!(contribs_only.len(), 1);
    assert!(contribs_only[0].is_income);

    // 8. Create and contribute to communal project
    let project_req = CreateProjectRequest {
        name: "Bugesera Land Acquisition".into(),
        category: "HOUSE".into(),
        target_amount_minor_units: 200_000,
        description: Some("Collective family plot purchase".into()),
        target_date: None,
    };
    let project = create_project(&pool, &user_id, &family_id, &project_req)
        .await
        .expect("Create project");
    assert_eq!(project.name, "Bugesera Land Acquisition");
    assert_eq!(project.target_amount_minor_units, 200_000);
    assert_eq!(project.raised_amount_minor_units, 0);
    assert_eq!(project.progress_percentage, 0);

    // Contribute 50,000 to project (25% progress)
    let updated_project = contribute_to_project(
        &pool,
        &user_id,
        &family_id,
        &project.id,
        50_000,
        Some("Initial deposit".into()),
    )
    .await
    .expect("Contribute to project");
    assert_eq!(updated_project.raised_amount_minor_units, 50_000);
    assert_eq!(updated_project.progress_percentage, 25);
    assert_eq!(updated_project.contributors_count, 1);

    // 9. Household members roster
    let members = list_members(&pool, &user_id, &family_id)
        .await
        .expect("List members");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].role, "HEAD");
    assert_eq!(members[0].full_name, name);

    // 10. Offline outbox sync push
    let sync_key = Uuid::new_v4();
    let fam_uuid = Uuid::parse_str(&family_id).unwrap();
    let push_req = SyncPushRequest {
        mutations: vec![SyncMutation {
            idempotency_key: sync_key,
            family_id: fam_uuid,
            command: CommandType::CreateContribution,
            payload: serde_json::json!({
                "amount": 10_000,
                "description": "Offline mobile contribution",
                "category": "Contribution"
            }),
        }],
    };
    let push_resp = push_mutations(&pool, &user_id, &push_req)
        .await
        .expect("Push sync mutations");
    assert_eq!(push_resp.results.len(), 1);
    assert_eq!(push_resp.results[0].status, SyncStatus::Synced);

    // 11. Re-push must be idempotent and return existing Synced result
    let push_resp_retry = push_mutations(&pool, &user_id, &push_req)
        .await
        .expect("Re-push sync mutations");
    assert_eq!(push_resp_retry.results.len(), 1);
    assert_eq!(push_resp_retry.results[0].status, SyncStatus::Synced);

    // 12. Offline sync pull
    let pull_req = SyncPullRequest {
        family_id: fam_uuid,
        cursor: None,
    };
    let pull_resp = pull_records(&pool, &user_id, &pull_req)
        .await
        .expect("Pull sync records");
    assert!(pull_resp.transactions.len() >= 3);
    assert!(pull_resp.cursor.is_some());
}
