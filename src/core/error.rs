use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation Failed")]
    Validation(#[from] validator::ValidationErrors),

    #[error("User not found")]
    NotFound,

    #[error("Email already exists")]
    Conflict,

    #[error("Database error")]
    Database,

    #[error("Hash Failure")]
    HashFailure,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Failed to generate Token")]
    TokenCreation,

    #[error("Invalid or expired Token")]
    InvalidToken,

    #[error("Refresh token reuse detected")]
    TokenReused,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "User not found").into_response(),

            AppError::Conflict => (StatusCode::CONFLICT, "User already exists").into_response(),

            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR.into_response(),

            AppError::HashFailure => StatusCode::INTERNAL_SERVER_ERROR.into_response(),

            AppError::Unauthorized | AppError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }

            AppError::TokenReused => {
                (StatusCode::UNAUTHORIZED, "Session revoked, please log in again").into_response()
            }

            AppError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate token",
            )
                .into_response(),

            AppError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                errors.to_string()
            )
                .into_response(),    
        }
    }
}