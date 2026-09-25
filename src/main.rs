mod app_state;
mod forms;
mod messages;
mod pages;
mod requests;
mod sockets;

use std::{env, net::SocketAddr};

use axum::Router;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("chat_rooms=trace,tower_http=info"))
                .unwrap(),
        )
        .init();

    let address = env::var("AXUM_ADDRESS").unwrap();
    let listener = TcpListener::bind(&address).await.unwrap();
    let state = app_state::AppState::new();
    let app = router(state).into_make_service_with_connect_info::<SocketAddr>();

    log::info!("Starting server on {address}");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            log::info!("SIGTERM received");
        })
        .await
        .unwrap();
}

fn router(state: app_state::AppState) -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .merge(pages::router())
        .merge(requests::router())
        .merge(sockets::router())
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
