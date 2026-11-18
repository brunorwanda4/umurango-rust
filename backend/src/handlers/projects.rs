//! Actix-web HTTP handlers for communal family projects.

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use umuryango_shared::{
    CreateProjectRequest, ErrorResponse, ProjectContributionRequest, ProjectDto,
};

use crate::config::Config;
use crate::handlers::{error_to_response, extract_user_id};
use crate::projects::{
    contribute_to_project as svc_contribute, create_project as svc_create,
    list_projects as svc_list,
};

/// List all communal projects for a family workspace.
#[utoipa::path(
    get,
    path = "/api/v1/families/{family_id}/projects",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    responses(
        (status = 200, description = "List of family projects", body = Vec<ProjectDto>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "projects"
)]
#[get("/{family_id}/projects")]
pub async fn list_projects(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_list(pool.get_ref(), &user_id, &family_id).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Create a new communal project in a family workspace.
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/projects",
    params(
        ("family_id" = String, Path, description = "Family workspace ID")
    ),
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created", body = ProjectDto),
        (status = 400, description = "Invalid project payload", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "projects"
)]
#[post("/{family_id}/projects")]
pub async fn create_project(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<String>,
    req: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let family_id = path.into_inner();
    match svc_create(pool.get_ref(), &user_id, &family_id, &req).await {
        Ok(project) => HttpResponse::Created().json(project),
        Err(err) => error_to_response(err, &http_req),
    }
}

/// Contribute toward a communal family project.
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/projects/{id}/contribute",
    params(
        ("family_id" = String, Path, description = "Family workspace ID"),
        ("id" = String, Path, description = "Project ID")
    ),
    request_body = ProjectContributionRequest,
    responses(
        (status = 200, description = "Contribution recorded and project updated", body = ProjectDto),
        (status = 400, description = "Invalid contribution payload", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Project not found", body = ErrorResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "projects"
)]
#[post("/{family_id}/projects/{id}/contribute")]
pub async fn contribute_project(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    http_req: HttpRequest,
    path: web::Path<(String, String)>,
    req: web::Json<ProjectContributionRequest>,
) -> impl Responder {
    let user_id = match extract_user_id(&http_req, &config.jwt_secret) {
        Ok(id) => id,
        Err(err) => return error_to_response(err, &http_req),
    };

    let (family_id, project_id) = path.into_inner();
    match svc_contribute(
        pool.get_ref(),
        &user_id,
        &family_id,
        &project_id,
        req.amount_minor_units,
        req.note.as_deref(),
    )
    .await
    {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(err) => error_to_response(err, &http_req),
    }
}
