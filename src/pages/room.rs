use axum::{
    extract::{Path, State},
    response::Html,
};
use tera::Context;
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn page(State(state): State<AppState>, Path(room_uuid): Path<Uuid>) -> Html<String> {
    let room_opt = &state.rooms().get(&room_uuid).await;

    if let Some(room) = room_opt {
        let mut ctx = Context::new();
        ctx.insert("room", &room.read().await.as_view(&room_uuid));

        state.render_page("room", &ctx)
    } else {
        state.render_page("no_room", &Context::new())
    }
}
