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
    let auth_header = match req.headers().get(AUTHORIZATION) {
        Some(value) => value,
        None => return Err(AppError::Unauthorized),
    };

    let raw = match auth_header.to_str() {
        Ok(value) => value,
        Err(_) => return Err(AppError::Unauthorized),
    };

    let token = raw.strip_prefix("Bearer ").unwrap_or(raw);

    let secret = state.jwt_secret;

    let token_data = verify_token(token, &secret).map_err(|_| AppError::Unauthorized)?;

    req.extensions_mut().insert(token_data.sub);

    Ok(next.run(req).await)
}
