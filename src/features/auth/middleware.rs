use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, header::AUTHORIZATION},
    middleware::Next,
};

use crate::{
    core::{app_state::AppState, error::AppError},
    features::auth::jwt::verify_token,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let secret = state.jwt_secret;

    let token_data = verify_token(token, &secret).map_err(|_| AppError::Unauthorized)?;

    req.extensions_mut().insert(token_data.sub);

    Ok(next.run(req).await)
}
