use crate::auth::jwt::{extract_token_from_header, validate_token};
use crate::config::Config;
use crate::error::AppError;
use crate::models::MemberRole;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    #[allow(dead_code)]
    pub email: String,
    pub role: MemberRole,
}

pub fn require_auth(
    req: &actix_web::HttpRequest,
    config: &Config,
) -> Result<AuthenticatedUser, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::AuthenticationError("Missing authorization header".to_string()))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| AppError::AuthenticationError("Invalid authorization header format".to_string()))?;

    let token_data = validate_token(token, config)?;

    Ok(AuthenticatedUser {
        id: token_data.claims.sub,
        email: token_data.claims.email,
        role: MemberRole::from(token_data.claims.role),
    })
}

pub fn require_admin(user: &AuthenticatedUser) -> Result<(), AppError> {
    if user.role != MemberRole::Admin {
        return Err(AppError::AuthorizationError(
            "Admin access required".to_string(),
        ));
    }
    Ok(())
}

pub fn require_instructor_or_admin(user: &AuthenticatedUser) -> Result<(), AppError> {
    if user.role != MemberRole::Admin && user.role != MemberRole::Instructor {
        return Err(AppError::AuthorizationError(
            "Instructor or admin access required".to_string(),
        ));
    }
    Ok(())
}
