use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    core::{app_state::AppState, error::AppError},
    features::auth::{jwt, model::UserRequest, service},
};

pub async fn register_user(
    State(appstate): State<AppState>,
    Json(payload): Json<UserRequest>,
) -> impl IntoResponse {
    match service::register(&appstate.db_pool, &payload.email, &payload.password).await {
        Ok(user) => {
            let token = jwt::create_token(user.id, &appstate.jwt_secret).unwrap();
            (StatusCode::CREATED, Json(token)).into_response()
        }
        Err(AppError::Conflict) => (
            StatusCode::CONFLICT,
            format!("Email '{}' already exists", &payload.email),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}

pub async fn login_user(
    State(appstate): State<AppState>,
    Json(payload): Json<UserRequest>,
) -> impl IntoResponse {
    match service::verify_user(&appstate.db_pool, &payload.email, &payload.password).await {
        Ok(user) => match jwt::create_token(user.id, &appstate.jwt_secret) {
            Ok(token) => (StatusCode::OK, Json(token)).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate token {}",
            )
                .into_response(),
        },

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn hello(Extension(token_data): Extension<i32>) -> impl IntoResponse {
    let id = token_data;
    (StatusCode::OK, format!("Hello, ID: {}", id)).into_response()
}
