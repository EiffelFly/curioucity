mod helper;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/discord/thread", post(create_discord_thread));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(helper::graceful_shutdown::shutdown_signal())
        .await
        .unwrap();
}

async fn create_discord_thread(Json(payload): Json<CreateDiscordThread>) -> impl IntoResponse {
    println!("Create thread payload: {:#?}", payload);
    (StatusCode::CREATED, "")
}

#[derive(Debug, Deserialize)]
struct CreateDiscordThread {}
