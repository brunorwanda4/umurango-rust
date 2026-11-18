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
            crate::auth::MeResponse,
        )
    ),

    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication and registration endpoints"),
        (name = "families", description = "Family workspace management")
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
