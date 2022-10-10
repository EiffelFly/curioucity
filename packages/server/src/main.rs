mod helper;

use axum::handler::Handler;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", axum::routing::get(hello))
        .route("/ping", axum::routing::get(pong))
        .route("/discord/thread", post(create_discord_thread));

    let port = std::env::var("BACKEND_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8010);

    // run our app with hyper
    println!("Http Server started on 0.0.0.0:{:?}", port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(helper::graceful_shutdown::shutdown_signal())
        .await
        .unwrap();
}

async fn hello() -> String {
    "Hello, World!".into()
}

struct AppError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

async fn pong() -> Result<(), AppError> {
    test_db().await?;
    Ok(())
}

async fn test_db() -> Result<(), anyhow::Error> {
    let conn = edgedb_tokio::create_client().await?;
    let val = conn
        .query_required_single::<i64, _>("SELECT 7*8", &())
        .await?;
    println!("7*8 is: {}", val);
    Ok(())
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
