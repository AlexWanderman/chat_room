use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn request() -> Response {
    log::trace!("[Request] Health ");

    StatusCode::OK.into_response()
}
