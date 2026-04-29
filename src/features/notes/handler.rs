use crate::{
    core::{app_state::AppState, error::AppError},
    features::notes::{model::NoteRequest, service as note_service},
};
use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

// Create Note Handler
pub async fn create_note_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<NoteRequest>,
) -> impl IntoResponse {
    match note_service::note_create(&state.db_pool, user_id, &payload.title, &payload.content).await
    {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create note").into_response(),
    }
}

// List Notes Handler
pub async fn list_notes_handler(
    State(state): State<AppState>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match note_service::note_list_by_user(&state.db_pool, user_id).await {
        Ok(notes) => Json(notes).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch notes").into_response(),
    }
}

// Update Note Handler
pub async fn update_note_handler(
    State(state): State<AppState>,
    Path(note_id): Path<i32>,
    Json(payload): Json<NoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let note = note_service::note_update(&state.db_pool, note_id, &payload.title, &payload.content)
        .await?;

    Ok(Json(note))
}
// Delete Note Handler
pub async fn delete_note_handler(
    State(state): State<AppState>,
    Path(note_id): Path<i32>,
) -> impl IntoResponse {
    match note_service::note_delete(&state.db_pool, note_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete note").into_response(),
    }
}

// Get Specific Note Handler
pub async fn read_specific_note_handler(
    State(state): State<AppState>,
    Path(note_id): Path<i32>,
) -> impl IntoResponse {
    match note_service::note_find_by_id(&state.db_pool, note_id).await {
        Ok(note) => (StatusCode::OK, Json(note)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Failed to find Note").into_response(),
    }
}
