use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::app_state::AppState;

pub async fn request(State(state): State<AppState>) -> Response {
    log::trace!("[Request] List rooms");

    (StatusCode::OK, Json(state.rooms().list().await)).into_response()
}
