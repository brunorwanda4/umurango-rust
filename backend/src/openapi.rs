//! Swagger and OpenAPI specification for Umuryango Backend API.

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::me,
        crate::handlers::families::list_families,
        crate::handlers::families::create_family,
        crate::handlers::families::join_family,
        crate::handlers::families::get_pending_invitations,
        crate::handlers::families::accept_invitation,
        crate::handlers::transactions::list_transactions,
        crate::handlers::transactions::create_transaction,
        crate::handlers::transactions::get_treasury_summary,
        crate::handlers::transactions::approve_transaction,
        crate::handlers::transactions::reject_transaction,
        crate::handlers::projects::list_projects,
        crate::handlers::projects::create_project,
        crate::handlers::projects::contribute_project,
        crate::handlers::members::list_members,
        crate::handlers::members::invite_member,
        crate::handlers::loans::list_loans,
        crate::handlers::loans::create_loan,
        crate::handlers::loans::repay_loan,
        crate::handlers::sync::sync_push,
        crate::handlers::sync::sync_pull,
    ),
    components(
        schemas(
            umuryango_shared::RegisterRequest,
            umuryango_shared::LoginRequest,
            umuryango_shared::UserDto,
            umuryango_shared::AuthResponse,
            umuryango_shared::ErrorResponse,
            umuryango_shared::Family,
            umuryango_shared::CreateFamilyRequest,
            umuryango_shared::CreateFamilyResponse,
            umuryango_shared::JoinFamilyRequest,
            umuryango_shared::JoinFamilyResponse,
            umuryango_shared::PendingFamilyInviteDto,
            umuryango_shared::TransactionItemDto,
            umuryango_shared::CreateTransactionRequest,
            umuryango_shared::TreasurySummaryDto,
            umuryango_shared::ProjectDto,
            umuryango_shared::CreateProjectRequest,
            umuryango_shared::ProjectContributionRequest,
            umuryango_shared::FamilyMemberDto,
            umuryango_shared::InviteFamilyMemberRequest,
            umuryango_shared::LoanDto,
            umuryango_shared::CreateLoanRequest,
            umuryango_shared::LoanRepaymentRequest,
            umuryango_shared::SyncPushRequest,
            umuryango_shared::SyncPushResponse,
            umuryango_shared::SyncPullRequest,
            umuryango_shared::SyncPullResponse,
            umuryango_shared::SyncMutation,
            umuryango_shared::SyncMutationResult,
            crate::auth::MeResponse,
        )
    ),

    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication and registration endpoints"),
        (name = "families", description = "Family workspace management"),
        (name = "transactions", description = "Treasury transactions and financial metrics"),
        (name = "projects", description = "Communal family projects and fundraising"),
        (name = "members", description = "Household members roster"),
        (name = "loans", description = "Family loans and borrowed balances"),
        (name = "sync", description = "Offline-first mutation sync")
    ),
    info(
        title = "Umuryango API",
        version = "0.1.0",
        description = "Authoritative REST API for Umuryango family financial governance and collaborative workspace."
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
