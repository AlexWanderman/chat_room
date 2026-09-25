use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::app_state::{AppState, validation};

pub async fn request(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(room_submit): Json<RoomSubmit>,
) -> Result<Response, Response> {
    log::trace!("[Request] Create room {room_submit:?}");

    // Validate member name to avoid creating room and fail creating member
    if let Err(err) = validation::validate_name(&room_submit.member_name) {
        return Err((StatusCode::BAD_REQUEST, err.to_string()).into_response());
    }

    // Create room
    let room_uuid = match state
        .rooms()
        .create(
            room_submit.name,
            room_submit.description,
            room_submit.is_listed,
            room_submit.password.clone(),
        )
        .await
    {
        Ok(room_uuid) => room_uuid,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string()).into_response()),
    };

    // Add new member
    let member_uuid = match state
        .rooms()
        .get(&room_uuid)
        .await
        .ok_or((StatusCode::BAD_REQUEST, "Room does not exist").into_response())?
        .write()
        .await
        .add_member(room_submit.member_name, room_submit.password)
        .await
    {
        Ok(member_uuid) => member_uuid,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string()).into_response()),
    };

    // Add auth cookie
    let token = state.create_token(&room_uuid, &member_uuid);
    let cookie = AppState::produce_cookie(room_uuid.to_string(), token);

    log::debug!("Giving token");
    cookies.add(cookie);

    Ok((StatusCode::OK, room_uuid.to_string()).into_response())
}

#[derive(Debug, Deserialize)]
pub struct RoomSubmit {
    pub name: String,
    pub description: String,
    pub is_listed: bool,
    pub password: Option<String>,
    pub member_name: String,
}
