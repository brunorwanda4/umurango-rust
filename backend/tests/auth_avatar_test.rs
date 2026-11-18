use sqlx::Row;
use umuryango_backend::{
    auth::{get_user_profile, login_user, register_user},
    schema::{apply_schema, connect},
};
use umuryango_shared::{generate_default_avatar_url, LoginRequest, RegisterRequest};

#[tokio::test]
async fn test_new_user_registration_generates_and_saves_dicebear_weave_avatar() {
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

    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("sqlx pool connection succeeds");

    let unique_suffix = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let name = format!("Aline Uwase {}", unique_suffix);
    let phone = format!("+250788{}", &unique_suffix[..6]);
    let password = "SecretPassword123";

    let expected_avatar = generate_default_avatar_url(&name);
    assert!(expected_avatar.starts_with("https://api.dicebear.com/10.x/weave/svg?seed="));
    assert!(expected_avatar.contains("Aline%20Uwase"));

    // 1. Register a new user
    let register_req = RegisterRequest {
        name: name.clone(),
        phone: phone.clone(),
        email: None,
        password: password.into(),
        avatar_url: None,
    };

    let auth_res = register_user(&pool, &register_req, "jwt_test_secret_32_bytes_long!!")
        .await
        .expect("User registration should succeed");

    // 2. Returned user DTO should contain the default avatar URL
    assert_eq!(auth_res.user.avatar_url, Some(expected_avatar.clone()));

    // 3. The avatar URL must be authoritatively persisted in the PostgreSQL "User" table
    let db_row = sqlx::query(r#"SELECT "avatarUrl" FROM "User" WHERE "id" = $1"#)
        .bind(&auth_res.user.id)
        .fetch_one(&pool)
        .await
        .expect("Fetching registered user from database must succeed");

    let saved_avatar: Option<String> = db_row.get("avatarUrl");
    assert_eq!(
        saved_avatar,
        Some(expected_avatar.clone()),
        "Avatar URL in database must match generated DiceBear weave URL"
    );

    // 4. Authenticating the user via login should return the saved avatar URL
    let login_req = LoginRequest {
        identifier: phone,
        password: password.into(),
    };

    let login_res = login_user(
        &pool,
        &login_req,
        "jwt_test_secret_32_bytes_long!!",
        None,
        None,
    )
    .await
    .expect("User login should succeed");

    assert_eq!(login_res.user.avatar_url, Some(expected_avatar.clone()));

    // 5. Fetching user profile via get_user_profile should return the saved avatar URL
    let profile = get_user_profile(&pool, &auth_res.user.id)
        .await
        .expect("Profile fetch should succeed");

    assert_eq!(profile.avatar_url, Some(expected_avatar));
}
