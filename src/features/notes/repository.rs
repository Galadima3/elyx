use sqlx::PgPool;

use crate::features::notes::model::Note;

// Create Note
pub async fn create(
    pool: &PgPool,
    user_id: i32,
    title: &str,
    content: &str,
) -> Result<Note, sqlx::Error> {
    let note = sqlx::query_as::<_, Note>(
        r#"
        INSERT INTO notes (user_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(title)
    .bind(content)
    .fetch_one(pool)
    .await?;

    Ok(note)
}

// List Notes for a user
pub async fn list_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<Note>, sqlx::Error> {
    sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

// Find Note by ID
pub async fn find_by_id(pool: &PgPool, note_id: i32) -> Result<Note, sqlx::Error> {
    sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(note_id)
        .fetch_one(pool)
        .await
}

// Update Note
pub async fn update(
    pool: &PgPool,
    note_id: i32,
    title: &str,
    content: &str,
) -> Result<Note, sqlx::Error> {
    sqlx::query_as::<_, Note>(
        r#"
        UPDATE notes
        SET title = $1, content = $2, updated_at = NOW()
        WHERE id = $3
        RETURNING *
        "#,
    )
    .bind(title)
    .bind(content)
    .bind(note_id)
    .fetch_one(pool)
    .await
}

// Delete Note
pub async fn delete(pool: &PgPool, note_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(note_id)
        .execute(pool)
        .await?;

    Ok(())
}
