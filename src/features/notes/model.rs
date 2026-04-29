use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Deserialize, Clone)]
pub struct NoteRequest {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, FromRow, Deserialize, Clone)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
}
