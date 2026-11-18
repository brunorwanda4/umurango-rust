//! Shared domain models and synchronization contracts for Umuryango.
//!
//! Money is represented in integer minor units. Never use floating-point
//! values for financial data.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An amount in the smallest unit used by the account currency.
pub type Money = i64;

/// Currency-independent transaction direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Income,
    Expense,
}

/// The lifecycle state of a locally or remotely synchronized record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Pending,
    Synced,
    Failed,
    Conflict,
}

/// Commands that can be queued in the local mutation outbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct Member {
    pub id: Uuid,
    pub family_id: Uuid,
    pub display_name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct Contribution {
    pub id: Uuid,
    pub family_id: Uuid,
    pub member_id: Uuid,
    pub amount: Money,
    pub contributed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncMutation {
    pub idempotency_key: Uuid,
    pub family_id: Uuid,
    pub command: CommandType,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncPushRequest {
    pub mutations: Vec<SyncMutation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncMutationResult {
    pub idempotency_key: Uuid,
    pub status: SyncStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncPushResponse {
    pub results: Vec<SyncMutationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncPullRequest {
    pub family_id: Uuid,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncPullResponse {
    pub cursor: Option<String>,
    pub transactions: Vec<Transaction>,
    pub expenses: Vec<Expense>,
    pub contributions: Vec<Contribution>,
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
