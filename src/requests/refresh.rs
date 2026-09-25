use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn request(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(refresh_submit): Json<RefreshSubmit>,
) -> Response {
    log::trace!("[Request] Refresh {refresh_submit:?}");

    match cookies
        .get(&refresh_submit.room_uuid.to_string())
        .and_then(|cookie| state.verify_token(cookie.value()))
    {
        Some((room_uuid, member_uuid)) => {
            log::debug!("Refresh JWT token room {room_uuid} member {member_uuid}");

            let token = state.create_token(&room_uuid, &member_uuid);
            let cookie = AppState::produce_cookie(room_uuid.to_string(), token);
            cookies.add(cookie);

            StatusCode::OK.into_response()
        }
        None => {
            log::debug!("Refresh JWT token failed");
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RefreshSubmit {
    pub room_uuid: Uuid,
}
