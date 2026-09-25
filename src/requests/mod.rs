mod auth;
mod create_room;
mod health;
mod list_rooms;
mod refresh;

use axum::{Router, routing};

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/health", routing::get(health::request))
        .route("/api/room", routing::post(create_room::request))
        .route("/api/room", routing::get(list_rooms::request))
        .route("/api/auth", routing::post(auth::request))
        .route("/api/refresh", routing::post(refresh::request))
}
