mod room;

use axum::{Router, routing};

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ws/room/{uuid}", routing::get(room::request))
        .route("/ws/room/{uuid}", routing::connect(room::request))
}
