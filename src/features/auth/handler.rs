use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    core::{app_state::AppState, error::AppError},
    features::auth::{dto::RegistrationRequest, jwt, service},
};

pub async fn register_user(
    State(appstate): State<AppState>,
    Json(payload): Json<RegistrationRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let user = service::register(&appstate.db_pool, &payload.email, &payload.password).await?;

    let token = jwt::create_token(user.id, &appstate.jwt_secret)?;

    Ok((StatusCode::CREATED, Json(token)).into_response())
}

pub async fn login_user(
    State(appstate): State<AppState>,
    Json(payload): Json<RegistrationRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let user = service::verify_user(&appstate.db_pool, &payload.email, &payload.password).await?;

    let token = jwt::create_token(user.id, &appstate.jwt_secret)?;

    Ok((StatusCode::OK, Json(token)).into_response())

}

pub async fn hello(Extension(token_data): Extension<i32>) -> impl IntoResponse {
    let id = token_data;
    (StatusCode::OK, format!("Hello, ID: {}", id)).into_response()
}
