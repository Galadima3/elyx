use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

// Struct for holding claims data used in JWT tokens
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: i64, // Expiry time of the token
    pub iat: i64, // Issued at time of the token
    pub sub: i32, // subject (user identifier, e.g. email)
}
