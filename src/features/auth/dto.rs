use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::features::auth::model::User;

#[derive(Deserialize, Serialize, Validate)]
pub struct RegistrationRequest {
    #[validate(email(message = "Invalid Email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be more than 6 characters"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid Email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be more than 6 characters"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserResponse {
    pub email: String,
    pub id: i32,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            email: user.email,
            id: user.id,
        }
    }
}



#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}