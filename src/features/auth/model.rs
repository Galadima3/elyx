use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

