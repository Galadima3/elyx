use crate::features::auth::{dto::UserResponse, jwt::RefreshToken, model::User};
use sqlx::PgPool;

use uuid::Uuid;
use chrono::{DateTime, Utc};

// insert
pub async fn create(pool: &PgPool, email: &str, password_hash: &str) -> Result<UserResponse, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(user.into())
}

// find by email
pub async fn _find_user_by_email(pool: &PgPool, email: &str) -> Result<UserResponse, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(user.into())
}

//Return User & Password for Auth
pub async fn return_user_details(pool: &PgPool, email: &str) -> Result<User, sqlx::Error>{
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
}
// update

// delete
pub async fn _delete_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user.id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}



// ---------- Refresh Tokens ----------
pub async fn insert_refresh_token(
    pool: &PgPool,
    user_id: i32,
    token_hash: &str,
    family_id: Uuid,
    expires_at: DateTime<Utc>,
) -> Result<RefreshToken, sqlx::Error> {
    sqlx::query_as::<_, RefreshToken>(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, family_id, expires_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(family_id)
    .bind(expires_at)
    .fetch_one(pool)
    .await
}

// Fetches regardless of revoked/expired state, needed to distinguish
// "never existed" from "already used" (reuse detection) in the service layer.
pub async fn find_refresh_token_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<RefreshToken>, sqlx::Error> {
    sqlx::query_as::<_, RefreshToken>("SELECT * FROM refresh_tokens WHERE token_hash = $1")
        .bind(token_hash)
        .fetch_optional(pool)
        .await
}

pub async fn revoke_refresh_token(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn revoke_family(pool: &PgPool, family_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE family_id = $1")
        .bind(family_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn _revoke_all_for_user(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}