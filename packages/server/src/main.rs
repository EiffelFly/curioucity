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
use grpc_handler::curioucity::{GrpcTagServiceImpl, GrpcUrlServiceImpl};
use http::{header::CONTENT_TYPE, Request};
use hyper::Body;
use pb_gen::curioucity::v1alpha as pb_curioucity;
use rest_handler::curioucity::{
    create_tag, create_url, delete_tag, delete_url, get_tag, get_url, list_tag,
};
use tonic::transport::Server;
use tower::{steer::Steer, BoxError, ServiceExt};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry();

    // build our application with a route
    let http = Router::new()
        .route("/", axum::routing::get(hello))
        .route("/urls", axum::routing::post(create_url))
        .route("/urls", axum::routing::delete(delete_url))
        .route("/urls/:url", axum::routing::get(get_url))
        .route("/tags", axum::routing::post(create_tag))
        .route("/tags", axum::routing::delete(delete_tag))
        .route("/tags/:name", axum::routing::get(get_tag))
        .route("/tags", axum::routing::get(list_tag))
        .fallback(fallback)
        .map_response(|r| r.map(axum::body::boxed))
        .map_err(BoxError::from)
        .boxed_clone();

    let url_service =
        pb_curioucity::url_service_server::UrlServiceServer::new(GrpcUrlServiceImpl::default());

    let tag_service =
        pb_curioucity::tag_service_server::TagServiceServer::new(GrpcTagServiceImpl::default());

    let grpc = Server::builder()
        .add_service(url_service)
        .add_service(tag_service)
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
