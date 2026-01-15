use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{require_admin, require_auth};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    AnnouncementListResponse, AnnouncementQuery, AnnouncementResponse, ApiResponse,
    CreateAnnouncementRequest, MessageResponse, PaginationQuery, UpdateAnnouncementRequest,
};

pub async fn create_announcement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateAnnouncementRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let announcement = db::create_announcement(&pool, &body, &auth_user.id).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(AnnouncementResponse::from(announcement))))
}

pub async fn list_announcements(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    pagination: web::Query<PaginationQuery>,
    query: web::Query<AnnouncementQuery>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let (announcements, total) = db::list_announcements(
        &pool,
        pagination.get_offset(),
        pagination.get_per_page(),
        query.status.as_deref(),
        query.priority.as_deref(),
        query.target_audience.as_deref(),
        query.pinned_only.unwrap_or(false),
    )
    .await?;

    let response = AnnouncementListResponse {
        announcements: announcements
            .into_iter()
            .map(AnnouncementResponse::from)
            .collect(),
        total,
        page: pagination.get_page(),
        per_page: pagination.get_per_page(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_announcement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let announcement_id = path.into_inner();
    let announcement = db::get_announcement_by_id(&pool, &announcement_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(AnnouncementResponse::from(announcement))))
}

pub async fn update_announcement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateAnnouncementRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let announcement_id = path.into_inner();
    let announcement = db::update_announcement(&pool, &announcement_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(AnnouncementResponse::from(announcement))))
}

pub async fn delete_announcement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let announcement_id = path.into_inner();
    db::delete_announcement(&pool, &announcement_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Announcement deleted successfully")))
}

pub async fn publish_announcement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let announcement_id = path.into_inner();
    let announcement = db::publish_announcement(&pool, &announcement_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        AnnouncementResponse::from(announcement),
        "Announcement published successfully",
    )))
}

pub async fn archive_announcement(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let announcement_id = path.into_inner();
    let announcement = db::archive_announcement(&pool, &announcement_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        AnnouncementResponse::from(announcement),
        "Announcement archived successfully",
    )))
}

// Public endpoint - get active announcements for members
pub async fn get_active_announcements(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;

    // Determine target audience based on user role
    let target = auth_user.role.to_string();

    let announcements = db::list_active_announcements(&pool, Some(&target)).await?;

    let response: Vec<AnnouncementResponse> = announcements
        .into_iter()
        .map(AnnouncementResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
