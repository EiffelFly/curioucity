mod db;
mod pb_gen {
    include!("gen/mod.rs");
}
mod grpc_handler;
mod helper;
mod proto_ext;
mod rest_handler;

use std::net::SocketAddr;

use axum::Router;
use grpc_handler::curioucity::UrlServiceImpl;
use http::{header::CONTENT_TYPE, Request};
use hyper::Body;
use pb_gen::curioucity::v1alpha as pb_curioucity;
use rest_handler::curioucity::create_url_handler;
use tonic::transport::Server;
use tower::{steer::Steer, BoxError, ServiceExt};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry();

    // build our application with a route
    let http = Router::new()
        .route("/", axum::routing::get(hello))
        .route("/url", axum::routing::post(create_url_handler))
        .fallback(fallback)
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .map_err(BoxError::from)
        .boxed_clone();

    let url_service =
        pb_curioucity::url_service_server::UrlServiceServer::new(UrlServiceImpl::default());

    let grpc = Server::builder()
        .add_service(url_service)
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .map_err(BoxError::from)
        .boxed_clone();

    let http_grpc = Steer::new(vec![http, grpc], |req: &Request<Body>, _svcs: &[_]| {
        if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
            0
        } else {
            1
        }
    });

    let port = std::env::var("BACKEND_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8010);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    println!("Http Server started on 0.0.0.0:{:?}", port);

    axum::Server::bind(&addr)
        .serve(tower::make::Shared::new(http_grpc))
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
