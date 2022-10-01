mod helper;

use axum::handler::Handler;
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", axum::routing::get(hello))
        .route("/discord/thread", post(create_discord_thread));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 8010));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(helper::graceful_shutdown::shutdown_signal())
        .await
        .unwrap();
}

async fn hello() -> String {
    "Hello, World!".into()
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("Not found {}", uri),
    )
}

async fn create_discord_thread(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> impl IntoResponse {
    println!("Create thread payload: {:?}", data);
    (StatusCode::CREATED, "")
}

#[derive(Debug, Deserialize)]
struct CreateDiscordThread {}
