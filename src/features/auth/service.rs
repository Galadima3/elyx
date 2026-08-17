use bcrypt::{DEFAULT_COST, hash};
use chrono::{Duration, Utc};
use rand::RngExt;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    core::{app_state::AppState, error::AppError}, features::auth::{dto::{TokenResponse, UserResponse}, model::User, repository},
};

pub async fn register(pool: &PgPool, email: &str, password: &str) -> Result<UserResponse, AppError> {
    let password_hash = hash(password, DEFAULT_COST).map_err(|_| AppError::HashFailure)?;

    repository::create(pool, email, &password_hash)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint().is_some() => AppError::Conflict,
            _ => AppError::Database,
        })
}

pub async fn verify_user(pool: &PgPool, email: &str, password: &str) -> Result<User, AppError> {
    let user = repository::return_user_details(pool, email)
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


// ---------- Refresh + Access Tokens ----------

fn generate_refresh_token() -> String {
    let random_bytes: [u8; 32] = rand::rng().random();
    hex::encode(random_bytes)
}

fn hash_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

// Fresh login: starts a new token family
pub async fn issue_token_pair(state: &AppState, user_id: i32) -> Result<TokenResponse, AppError> {
    issue_token_pair_in_family(state, user_id, Uuid::new_v4()).await
}

// Rotation: keeps the caller's family_id so reuse detection can trace it
async fn issue_token_pair_in_family(
    state: &AppState,
    user_id: i32,
    family_id: Uuid,
) -> Result<TokenResponse, AppError> {
    let access_token = crate::features::auth::jwt::create_token(user_id, &state.jwt_secret)?;

    let raw_refresh = generate_refresh_token();
    let hashed = hash_refresh_token(&raw_refresh);
    let expires_at = Utc::now() + Duration::days(14);
    

    repository::insert_refresh_token(&state.db_pool, user_id, &hashed, family_id, expires_at)
        .await
        .map_err(|_| AppError::Database)?;

    Ok(TokenResponse {
        access_token,
        refresh_token: raw_refresh,
    })
}

pub async fn refresh_access_token(state: &AppState, incoming_token: &str) -> Result<TokenResponse, AppError> {
    let hashed = hash_refresh_token(incoming_token);

    let existing = repository::find_refresh_token_by_hash(&state.db_pool, &hashed)
        .await
        .map_err(|_| AppError::Database)?
        .ok_or(AppError::InvalidToken)?;

    if existing.revoked {
        repository::revoke_family(&state.db_pool, existing.family_id)
            .await
            .map_err(|_| AppError::Database)?;
        return Err(AppError::TokenReused);
    }

    if existing.expires_at < Utc::now() {
        return Err(AppError::InvalidToken);
    }

    repository::revoke_refresh_token(&state.db_pool, existing.id)
        .await
        .map_err(|_| AppError::Database)?;

    issue_token_pair_in_family(state, existing.user_id, existing.family_id).await
}