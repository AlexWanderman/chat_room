use axum::{extract::State, response::Html};
use tera::Context;

use crate::app_state::AppState;

pub async fn page(State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("rooms", &state.rooms().list().await);

    state.render_page("index", &ctx)
}
