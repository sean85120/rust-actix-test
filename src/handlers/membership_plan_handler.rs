use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{require_admin, require_auth};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, CreateMembershipPlanRequest, MembershipPlanListResponse, MembershipPlanResponse,
    MessageResponse, UpdateMembershipPlanRequest,
};

#[derive(Debug, serde::Deserialize)]
pub struct PlanQuery {
    pub include_inactive: Option<bool>,
}

pub async fn list_plans(
    pool: web::Data<DbPool>,
    query: web::Query<PlanQuery>,
) -> Result<HttpResponse, AppError> {
    let include_inactive = query.include_inactive.unwrap_or(false);

    let plans = db::list_membership_plans(&pool, include_inactive).await?;
    let total = plans.len() as i64;

    let response = MembershipPlanListResponse {
        plans: plans.into_iter().map(MembershipPlanResponse::from).collect(),
        total,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn create_plan(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateMembershipPlanRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let plan = db::create_membership_plan(&pool, &body).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(MembershipPlanResponse::from(plan))))
}

pub async fn get_plan(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let plan_id = path.into_inner();
    let plan = db::get_membership_plan_by_id(&pool, &plan_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(MembershipPlanResponse::from(plan))))
}

pub async fn update_plan(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
    body: web::Json<UpdateMembershipPlanRequest>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let plan_id = path.into_inner();
    let plan = db::update_membership_plan(&pool, &plan_id, &body).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(MembershipPlanResponse::from(plan))))
}

pub async fn delete_plan(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;
    require_admin(&auth_user)?;

    let plan_id = path.into_inner();
    db::delete_membership_plan(&pool, &plan_id).await?;

    Ok(HttpResponse::Ok().json(MessageResponse::new("Membership plan deleted successfully")))
}
