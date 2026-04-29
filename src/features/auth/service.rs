use bcrypt::{DEFAULT_COST, hash};
use sqlx::PgPool;

use crate::{
    core::error::AppError,
    features::auth::{model::User, repository},
};

pub async fn register(pool: &PgPool, email: &str, password: &str) -> Result<User, AppError> {
    let password_hash = hash(password, DEFAULT_COST).map_err(|_| AppError::HashFailure)?;

    repository::create(pool, email, &password_hash)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint().is_some() => AppError::Conflict,
            _ => AppError::Database,
        })
}

pub async fn verify_user(pool: &PgPool, email: &str, password: &str) -> Result<User, AppError> {
    let user = repository::find_user_by_email(pool, email)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Database,
        })?;

    let is_valid = bcrypt::verify(password, &user.password).map_err(|_| AppError::HashFailure)?;

    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    Ok(user)
}
