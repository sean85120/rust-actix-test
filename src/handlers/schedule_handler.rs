use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{require_admin, require_auth, require_instructor_or_admin};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, BookingResponse, CourseScheduleResponse, CreateScheduleRequest, MessageResponse,
    ScheduleListResponse, ScheduleQuery, UpdateScheduleRequest,
};

pub async fn list_schedules(
    pool: web::Data<DbPool>,
    query: web::Query<ScheduleQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (schedules, total) = db::list_schedules(
        &pool,
        offset,
        per_page,
        query.course_id.as_deref(),
        query.instructor_id.as_deref(),
        query.date_from.as_deref(),
        query.date_to.as_deref(),
        query.status.as_deref(),
    )
    .await?;

    let response = ScheduleListResponse {
        schedules: schedules
            .into_iter()
            .map(CourseScheduleResponse::from)
            .collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn create_schedule(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateScheduleRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Get the course to use its default capacity
    let course = db::get_course_by_id(&pool, &body.course_id).await?;

    let schedule = db::create_schedule(&pool, &body, course.max_capacity).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(CourseScheduleResponse::from(schedule))))
}

pub async fn get_schedule(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let schedule_id = path.into_inner();
    let schedule = db::get_schedule_by_id(&pool, &schedule_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(CourseScheduleResponse::from(schedule))))
}

pub async fn update_schedule(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateScheduleRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    let schedule_id = path.into_inner();
    let schedule = db::update_schedule(&pool, &schedule_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(CourseScheduleResponse::from(schedule))))
}

pub async fn delete_schedule(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let schedule_id = path.into_inner();
    db::delete_schedule(&pool, &schedule_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Schedule deleted successfully")))
}

pub async fn cancel_schedule(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    let schedule_id = path.into_inner();
    let schedule = db::cancel_schedule(&pool, &schedule_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success_with_message(
        CourseScheduleResponse::from(schedule),
        "Schedule cancelled successfully",
    )))
}

pub async fn get_schedule_bookings(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    let schedule_id = path.into_inner();
    let bookings = db::get_schedule_bookings(&pool, &schedule_id).await?;

    let booking_responses: Vec<BookingResponse> =
        bookings.into_iter().map(BookingResponse::from).collect();

    Ok(HttpResponse::Ok().json(ApiResponse::success(booking_responses)))
}
