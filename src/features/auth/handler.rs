use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    core::{app_state::AppState, error::AppError}, features::{auth::{dto::{LoginRequest, RefreshRequest, RegistrationRequest, TokenResponse}, service}},
};

pub async fn register_user(
    State(appstate): State<AppState>,
    Json(payload): Json<RegistrationRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let user = service::register(&appstate.db_pool, &payload.email, &payload.password).await?;

    //let token = jwt::create_token(user.id, &appstate.jwt_secret)?;
    let pair = service::issue_token_pair(&appstate, user.id).await?;

    Ok((
        StatusCode::CREATED,
        Json(TokenResponse {
            access_token: pair.access_token,
            refresh_token: pair.refresh_token,
        }),
    )
        .into_response())
}

pub async fn login_user(
    State(appstate): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let user = service::verify_user(&appstate.db_pool, &payload.email, &payload.password).await?;

    // let token = jwt::create_token(user.id, &appstate.jwt_secret)?;
    let pair = service::issue_token_pair(&appstate, user.id).await?;

    Ok((StatusCode::OK, Json(TokenResponse{
        access_token: pair.access_token,
        refresh_token: pair.refresh_token
    })).into_response())

}

pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let pair = service::refresh_access_token(&state, &payload.refresh_token).await?;

    Ok(Json(TokenResponse {
        access_token: pair.access_token,
        refresh_token: pair.refresh_token,
    }))
}

pub async fn hello(Extension(token_data): Extension<i32>) -> impl IntoResponse {
    let id = token_data;
    (StatusCode::OK, format!("Hello, ID: {}", id)).into_response()
}
