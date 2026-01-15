use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{require_admin, require_auth, require_instructor_or_admin};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, CourseListResponse, CourseResponse, CreateCourseRequest, MessageResponse,
    ScheduleListResponse, CourseScheduleResponse, UpdateCourseRequest,
};

#[derive(Debug, serde::Deserialize)]
pub struct CourseQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub course_type: Option<String>,
    pub difficulty: Option<String>,
    pub status: Option<String>,
}

pub async fn list_courses(
    pool: web::Data<DbPool>,
    query: web::Query<CourseQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (courses, total) = db::list_courses(
        &pool,
        offset,
        per_page,
        query.course_type.as_deref(),
        query.difficulty.as_deref(),
        query.status.as_deref(),
    )
    .await?;

    let response = CourseListResponse {
        courses: courses.into_iter().map(CourseResponse::from).collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn create_course(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let course = db::create_course(&pool, &body).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(CourseResponse::from(course))))
}

pub async fn get_course(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();
    let course = db::get_course_by_id(&pool, &course_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(CourseResponse::from(course))))
}

pub async fn update_course(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_instructor_or_admin(&auth_user)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let course_id = path.into_inner();
    let course = db::update_course(&pool, &course_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(CourseResponse::from(course))))
}

pub async fn delete_course(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let course_id = path.into_inner();
    db::delete_course(&pool, &course_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Course deleted successfully")))
}

pub async fn get_course_schedules(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    query: web::Query<ScheduleQuery>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let (schedules, total) = db::list_schedules(
        &pool,
        offset,
        per_page,
        Some(&course_id),
        None,
        query.date_from.as_deref(),
        query.date_to.as_deref(),
        query.status.as_deref(),
    )
    .await?;

    let response = ScheduleListResponse {
        schedules: schedules.into_iter().map(CourseScheduleResponse::from).collect(),
        total,
        page,
        per_page,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[derive(Debug, serde::Deserialize)]
pub struct ScheduleQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub status: Option<String>,
}
