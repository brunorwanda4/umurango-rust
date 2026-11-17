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
