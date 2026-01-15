use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use validator::Validate;

use crate::auth::{require_admin, require_auth};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, AssignMembershipRequest, BookingListResponse, BookingResponse,
    MemberListResponse, MemberResponse, MemberRole, MessageResponse, PaginationQuery,
    PlanDuration, UpdateMemberRequest,
};

#[derive(Debug, serde::Deserialize)]
pub struct MemberQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub role: Option<String>,
}

pub async fn list_members(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    query: web::Query<MemberQuery>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (members, total) = db::list_members(
        &pool,
        offset,
        per_page,
        query.status.as_deref(),
        query.role.as_deref(),
    )
    .await?;

    let response = MemberListResponse {
        members: members.into_iter().map(MemberResponse::from).collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_member(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    let member_id = path.into_inner();

    // Users can only view their own profile unless they're admin
    if auth_user.id != member_id && auth_user.role != MemberRole::Admin {
        return Err(AppError::AuthorizationError(
            "Not authorized to view this member".to_string(),
        ));
    }

    let member = db::get_member_by_id(&pool, &member_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(MemberResponse::from(member))))
}

pub async fn update_member(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateMemberRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    let member_id = path.into_inner();

    // Users can only update their own profile unless they're admin
    if auth_user.id != member_id && auth_user.role != MemberRole::Admin {
        return Err(AppError::AuthorizationError(
            "Not authorized to update this member".to_string(),
        ));
    }

    // Only admins can change member status
    if body.status.is_some() && auth_user.role != MemberRole::Admin {
        return Err(AppError::AuthorizationError(
            "Only admins can change member status".to_string(),
        ));
    }

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let member = db::update_member(&pool, &member_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(MemberResponse::from(member))))
}

pub async fn delete_member(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let member_id = path.into_inner();
    db::delete_member(&pool, &member_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Member deleted successfully")))
}

pub async fn assign_membership(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<AssignMembershipRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let member_id = path.into_inner();

    // Get the membership plan to calculate end date
    let plan = db::get_membership_plan_by_id(&pool, &body.membership_plan_id).await?;

    let start_date = Utc::now();
    let duration = PlanDuration::from(plan.duration);
    let end_date = start_date + Duration::days(duration.days() as i64);

    let member = db::assign_membership(
        &pool,
        &member_id,
        &body.membership_plan_id,
        &start_date.format("%Y-%m-%d").to_string(),
        &end_date.format("%Y-%m-%d").to_string(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        MemberResponse::from(member),
        "Membership assigned successfully",
    )))
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateRoleRequest {
    pub role: MemberRole,
}

pub async fn update_role(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let member_id = path.into_inner();
    let member = db::update_member_role(&pool, &member_id, &body.role).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(MemberResponse::from(member))))
}

pub async fn get_member_bookings(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    let member_id = path.into_inner();

    // Users can only view their own bookings unless they're admin
    if auth_user.id != member_id && auth_user.role != MemberRole::Admin {
        return Err(AppError::AuthorizationError(
            "Not authorized to view this member's bookings".to_string(),
        ));
    }

    let page = query.get_page();
    let per_page = query.get_per_page();
    let offset = query.get_offset();

    let (bookings, total) = db::get_member_bookings(&pool, &member_id, offset, per_page).await?;

    let response = BookingListResponse {
        bookings: bookings.into_iter().map(BookingResponse::from).collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
