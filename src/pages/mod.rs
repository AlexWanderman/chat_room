mod about;
mod index;
mod room;

use axum::{Router, routing};

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(index::page))
        .route("/about", routing::get(about::page))
        .route("/room/{uuid}", routing::get(room::page))
}
