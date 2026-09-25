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
    Json(auth_submit): Json<AuthSubmit>,
) -> Result<Response, Response> {
    log::trace!("[Request] Auth {auth_submit:?}");

    // Add new member
    let member_uuid = match state
        .rooms()
        .get(&auth_submit.room_uuid)
        .await
        .ok_or((StatusCode::BAD_REQUEST, "Room does not exist").into_response())?
        .write()
        .await
        .add_member(auth_submit.member_name, auth_submit.room_password)
        .await
    {
        Ok(member_uuid) => member_uuid,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string()).into_response()),
    };

    // Add auth cookie
    let token = state.create_token(&auth_submit.room_uuid, &member_uuid);
    let cookie = AppState::produce_cookie(auth_submit.room_uuid.to_string(), token);

    log::debug!("Giving token");
    cookies.add(cookie);

    Ok(StatusCode::OK.into_response())
}

#[derive(Debug, Deserialize)]
pub struct AuthSubmit {
    pub room_uuid: Uuid,
    pub room_password: Option<String>,
    pub member_name: String,
}
