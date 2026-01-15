use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::AppError;
use crate::models::MemberRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // member_id
    pub email: String,
    pub role: String,
    pub exp: i64,          // expiration time
    pub iat: i64,          // issued at
}

pub fn create_token(
    member_id: &str,
    email: &str,
    role: &MemberRole,
    config: &Config,
) -> Result<String, AppError> {
    let now = Utc::now();
    let expiration = now + Duration::hours(config.jwt_expiration_hours);

    let claims = Claims {
        sub: member_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn validate_token(token: &str, config: &Config) -> Result<TokenData<Claims>, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data)
}

pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(&auth_header[7..])
    } else {
        None
    }
}
