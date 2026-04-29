use crate::features::auth::model::User;
use sqlx::PgPool;

// insert
pub async fn create(pool: &PgPool, email: &str, password_hash: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

// find by email
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
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
