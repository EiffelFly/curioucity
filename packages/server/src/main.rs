mod db;
mod gen;
mod helper;
mod proto_ext;
mod rest_handler;

use axum::handler::Handler;
use axum::Router;
use rest_handler::curioucity::create_url_handler;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", axum::routing::get(hello))
        .route("/url", axum::routing::post(create_url_handler));

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
async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("Not found {}", uri),
    )
}
