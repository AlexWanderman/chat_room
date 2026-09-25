use axum::{extract::State, response::Html};
use tera::Context;

use crate::app_state::AppState;

pub async fn page(State(state): State<AppState>) -> Html<String> {
    state.render_page("about", &Context::new())
}
