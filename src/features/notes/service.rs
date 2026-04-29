use crate::{
    core::error::AppError,
    features::notes::{model::Note, repository},
};
use sqlx::PgPool;

// Create Note
pub async fn note_create(
    pool: &PgPool,
    user_id: i32,
    title: &str,
    content: &str,
) -> Result<Note, AppError> {
    repository::create(pool, user_id, title, content)
        .await
        .map_err(|_| AppError::Database)
}

// List Notes for a User
pub async fn note_list_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<Note>, AppError> {
    repository::list_by_user(pool, user_id)
        .await
        .map_err(|_| AppError::Database)
}

// Find Note by ID
pub async fn note_find_by_id(pool: &PgPool, note_id: i32) -> Result<Note, AppError> {
    repository::find_by_id(pool, note_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Database,
        })
}

// Update Note
pub async fn note_update(
    pool: &PgPool,
    note_id: i32,
    title: &str,
    content: &str,
) -> Result<Note, AppError> {
    repository::update(pool, note_id, title, content)
        .await
        .map_err(|_| AppError::Database)
}

// Delete Note
pub async fn note_delete(pool: &PgPool, note_id: i32) -> Result<(), AppError> {
    repository::delete(pool, note_id)
        .await
        .map_err(|_| AppError::Database)
}
