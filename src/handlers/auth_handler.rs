use actix_web::{web, HttpRequest, HttpResponse};
use validator::Validate;

use crate::auth::{create_token, require_auth};
use crate::config::Config;
use crate::db::{self, DbPool};
use crate::error::AppError;
use crate::models::{
    ApiResponse, CreateMemberRequest, LoginRequest, LoginResponse, MemberResponse, MemberRole,
};

pub async fn register(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<CreateMemberRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if email already exists
    if db::email_exists(&pool, &body.email).await? {
        return Err(AppError::ConflictError(
            "Email already registered".to_string(),
        ));
    }

    // Hash password
    let password_hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)?;

    // Create member
    let member = db::create_member(&pool, &body, &password_hash).await?;

    // Generate token
    let token = create_token(
        &member.id,
        &member.email,
        &MemberRole::Member,
        &config,
    )?;

    let response = LoginResponse {
        token,
        member: MemberResponse::from(member),
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Find member by email
    let member = db::get_member_by_email(&pool, &body.email)
        .await
        .map_err(|_| AppError::AuthenticationError("Invalid credentials".to_string()))?;

    // Verify password
    let valid = bcrypt::verify(&body.password, &member.password_hash)?;
    if !valid {
        return Err(AppError::AuthenticationError(
            "Invalid credentials".to_string(),
        ));
    }

    // Generate token
    let token = create_token(
        &member.id,
        &member.email,
        &MemberRole::from(member.role.clone()),
        &config,
    )?;

    let response = LoginResponse {
        token,
        member: MemberResponse::from(member),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

pub async fn get_current_user(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    let auth_user = require_auth(&req, &config)?;

    let member = db::get_member_by_id(&pool, &auth_user.id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(MemberResponse::from(member))))
}
