use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::core::error::AppError;

pub fn create_token(user_id: i32, secret: &str) -> Result<String, AppError> {
    let expiration = Utc::now() + Duration::hours(24);
    let now = Utc::now();

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: expiration.timestamp(),
    };

    let token_data = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::TokenCreation)?;

    Ok(token_data)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::InvalidToken)?;

    Ok(token.claims)
}

// Struct for holding claims data used in JWT tokens
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: i64, // Expiry time of the token
    pub iat: i64, // Issued at time of the token
    pub sub: i32, // subject (user identifier, e.g. email)
}
