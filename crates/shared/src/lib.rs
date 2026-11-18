//! Shared domain models and synchronization contracts for Umuryango.
//!
//! Money is represented in integer minor units. Never use floating-point
//! values for financial data.

use serde::{Deserialize, Serialize};
pub use uuid;
pub use uuid::Uuid;

/// An amount in the smallest unit used by the account currency.
pub type Money = i64;

/// Currency-independent transaction direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Income,
    Expense,
}

/// The lifecycle state of a locally or remotely synchronized record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Pending,
    Synced,
    Failed,
    Conflict,
}

/// Commands that can be queued in the local mutation outbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandType {
    CreateExpense,
    RecordPayment,
    CreateContribution,
    UpdateFamily,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Member {
    pub id: Uuid,
    pub family_id: Uuid,
    pub display_name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Transaction {
    pub id: Uuid,
    pub family_id: Uuid,
    pub member_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: Money,
    pub description: String,
    pub occurred_at: String,
    pub sync_status: SyncStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Expense {
    pub id: Uuid,
    pub family_id: Uuid,
    pub paid_by: Uuid,
    pub amount: Money,
    pub category: String,
    pub description: String,
    pub occurred_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Contribution {
    pub id: Uuid,
    pub family_id: Uuid,
    pub member_id: Uuid,
    pub amount: Money,
    pub contributed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SyncMutation {
    pub idempotency_key: Uuid,
    pub family_id: Uuid,
    pub command: CommandType,
    #[cfg_attr(feature = "openapi", schema(value_type = Object))]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SyncPushRequest {
    pub mutations: Vec<SyncMutation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SyncMutationResult {
    pub idempotency_key: Uuid,
    pub status: SyncStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SyncPushResponse {
    pub results: Vec<SyncMutationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SyncPullRequest {
    pub family_id: Uuid,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SyncPullResponse {
    pub cursor: Option<String>,
    pub transactions: Vec<Transaction>,
    pub expenses: Vec<Expense>,
    pub contributions: Vec<Contribution>,
}

/// Default DiceBear avatar style used for new user accounts.
/// Based on https://www.dicebear.com/styles/weave/
pub const DEFAULT_AVATAR_STYLE_BASE_URL: &str = "https://api.dicebear.com/10.x/weave/svg";

/// Generates a default avatar URL using DiceBear's "weave" style.
/// Accepts a seed string (such as user name or user id) and returns the full SVG URL.
/// Empty or whitespace-only seeds return the base URL without query parameters.
pub fn generate_default_avatar_url(seed: &str) -> String {
    let clean = seed.trim();
    if clean.is_empty() {
        DEFAULT_AVATAR_STYLE_BASE_URL.to_string()
    } else {
        let encoded =
            percent_encoding::utf8_percent_encode(clean, percent_encoding::NON_ALPHANUMERIC);
        format!("{DEFAULT_AVATAR_STYLE_BASE_URL}?seed={encoded}")
    }
}

/// Request payload to register a new user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RegisterRequest {
    pub name: String,
    pub phone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// Request payload to log in an existing user with phone or email.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

/// User profile details returned upon successful authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UserDto {
    pub id: String,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub status: String,
}

/// Authentication response containing the authenticated user, bearer token, and family context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AuthResponse {
    pub user: UserDto,
    pub token: String,
    pub family_name: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub family_id: Option<String>,
}

/// Standard error response envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ErrorResponse {
    pub error: String,
}

/// Request payload to create a new family workspace with member phone numbers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateFamilyRequest {
    pub name: String,
    #[serde(default)]
    pub member_phones: Vec<String>,
}

/// Response returned after successfully creating a family workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateFamilyResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub members_count: usize,
}

/// Request payload to join an existing family using a 4-character code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct JoinFamilyRequest {
    pub code: String,
}

/// Response returned after successfully joining a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct JoinFamilyResponse {
    pub family_id: String,
    pub family_name: String,
    pub role: String,
}

/// A pending family invitation or membership created by phone number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PendingFamilyInviteDto {
    pub invite_id: String,
    pub family_id: String,
    pub family_name: String,
    pub role: String,
    pub created_at: String,
}

/// Transaction item returned in activity feeds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TransactionItemDto {
    pub id: String,
    pub family_id: String,
    pub actor_name: String,
    pub title: String,
    pub amount_minor_units: Money,
    pub currency: String,
    pub is_income: bool,
    pub category: String,
    pub status: String,
    pub occurred_at: String,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Request payload to record a new transaction (income, contribution, or expense).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateTransactionRequest {
    pub amount_minor_units: Money,
    pub is_income: bool,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

/// Authoritative financial summary for a family workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TreasurySummaryDto {
    pub total_treasury_minor_units: Money,
    pub monthly_contributions_minor_units: Money,
    pub monthly_expenses_minor_units: Money,
    pub emergency_reserve_minor_units: Money,
    pub currency: String,
    pub pending_approval_count: usize,
}

/// Communal project DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ProjectDto {
    pub id: String,
    pub family_id: String,
    pub name: String,
    pub category: String,
    #[serde(default)]
    pub description: Option<String>,
    pub target_amount_minor_units: Money,
    pub raised_amount_minor_units: Money,
    pub currency: String,
    pub progress_percentage: u8,
    pub contributors_count: usize,
    pub status: String,
    pub target_date: String,
}

/// Request payload to create a new communal family project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CreateProjectRequest {
    pub name: String,
    pub category: String,
    pub target_amount_minor_units: Money,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub target_date: Option<String>,
}

/// Request payload to contribute toward a communal family project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ProjectContributionRequest {
    pub amount_minor_units: Money,
    #[serde(default)]
    pub note: Option<String>,
}

/// Family member roster entry DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct FamilyMemberDto {
    pub id: String,
    pub user_id: String,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub role: String,
    #[serde(default)]
    pub avatar_url: Option<String>,
    pub joined_at: String,
}

/// Deterministic financial amount formatter.
/// Formats an integer minor unit into standard comma-separated representation (e.g. 3,450,000 RWF).
pub fn format_money(amount: Money, currency: &str) -> String {
    let is_negative = amount < 0;
    let abs_val = amount.unsigned_abs();
    let s = abs_val.to_string();
    let mut formatted = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(c);
    }
    if is_negative {
        format!("-{formatted} {currency}")
    } else {
        format!("{formatted} {currency}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_default_avatar_url_simple_name() {
        let url = generate_default_avatar_url("Aline");
        assert_eq!(url, "https://api.dicebear.com/10.x/weave/svg?seed=Aline");
    }

    #[test]
    fn test_generate_default_avatar_url_with_spaces() {
        let url = generate_default_avatar_url("Jean Luc");
        assert_eq!(
            url,
            "https://api.dicebear.com/10.x/weave/svg?seed=Jean%20Luc"
        );
    }

    #[test]
    fn test_generate_default_avatar_url_empty_or_whitespace() {
        assert_eq!(
            generate_default_avatar_url(""),
            DEFAULT_AVATAR_STYLE_BASE_URL
        );
        assert_eq!(
            generate_default_avatar_url("   "),
            DEFAULT_AVATAR_STYLE_BASE_URL
        );
    }

    #[test]
    fn test_user_dto_serialization_with_avatar() {
        let user = UserDto {
            id: "user-123".into(),
            full_name: "Alice Mutesi".into(),
            phone_number: Some("+250788111222".into()),
            email: None,
            avatar_url: Some("https://api.dicebear.com/10.x/weave/svg?seed=Alice%20Mutesi".into()),
            status: "ACTIVE".into(),
        };

        let json = serde_json::to_string(&user).expect("serialization succeeds");
        assert!(json.contains("avatar_url"));

        let deserialized: UserDto = serde_json::from_str(&json).expect("deserialization succeeds");
        assert_eq!(deserialized, user);
    }

    #[test]
    fn test_user_dto_backward_compatibility_without_avatar() {
        let json = r#"{"id":"user-1","full_name":"Bob","phone_number":null,"email":null,"status":"ACTIVE"}"#;
        let deserialized: UserDto = serde_json::from_str(json).expect("deserialization succeeds");
        assert_eq!(deserialized.avatar_url, None);
    }

    #[test]
    fn test_format_money() {
        assert_eq!(format_money(3450000, "RWF"), "3,450,000 RWF");
        assert_eq!(format_money(-85000, "RWF"), "-85,000 RWF");
        assert_eq!(format_money(0, "RWF"), "0 RWF");
        assert_eq!(format_money(500, "RWF"), "500 RWF");
    }

    #[test]
    fn test_auth_response_with_and_without_family_id() {
        let json_without = r#"{
            "user": {"id":"u1","full_name":"Test","status":"ACTIVE"},
            "token": "tok123",
            "family_name": "Family",
            "role": "HEAD"
        }"#;
        let auth: AuthResponse = serde_json::from_str(json_without).unwrap();
        assert_eq!(auth.family_id, None);

        let json_with = r#"{
            "user": {"id":"u1","full_name":"Test","status":"ACTIVE"},
            "token": "tok123",
            "family_name": "Family",
            "role": "HEAD",
            "family_id": "fam-456"
        }"#;
        let auth2: AuthResponse = serde_json::from_str(json_with).unwrap();
        assert_eq!(auth2.family_id, Some("fam-456".into()));
    }
}
