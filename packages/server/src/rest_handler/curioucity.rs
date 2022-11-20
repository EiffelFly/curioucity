use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

pub async fn create_url_handler(
    Json(data): Json<pb_curioucity::CreateUrlRequest>,
) -> Result<impl IntoResponse, Response> {
    let client = match edgedb_tokio::create_client().await {
        Ok(client) => client,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when access database: {}", error),
            )
                .into_response())
        }
    };

    let payload = db_curioucity::CreateUrlPayload {
        url: data.url,
        resource_type: match pb_curioucity::ResourceType::as_db_type(data.resource_type) {
            Ok(resource_type) => resource_type,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Something went wrong when transform resource_type: {}",
                        error
                    ),
                )
                    .into_response())
            }
        },
    };

    let url = match db_curioucity::Url::create(client, &payload).await {
        Ok(url) => url,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when create url: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_curioucity::CreateUrlResponse {
        url: Some(url.as_pb_type()),
    };

    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn delete_url_handler(
    Json(data): Json<pb_curioucity::DeleteUrlRequest>,
) -> Result<impl IntoResponse, Response> {
    let client = match edgedb_tokio::create_client().await {
        Ok(client) => client,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when access database: {}", error),
            )
                .into_response())
        }
    };

    let payload = db_curioucity::DeleteUrlPayload { url: data.url };

    match db_curioucity::Url::delete(client, &payload).await {
        Ok(_) => return Ok((StatusCode::NO_CONTENT, ())),
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when delete url: {}", error),
            )
                .into_response())
        }
    };
}

pub async fn get_url_handler(
    Json(data): Json<pb_curioucity::GetUrlRequest>,
) -> Result<impl IntoResponse, Response> {
    let client = match edgedb_tokio::create_client().await {
        Ok(client) => client,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when access database: {}", error),
            )
                .into_response())
        }
    };

    let payload = db_curioucity::GetUrlPayload { url: data.url };

    let url = match db_curioucity::Url::get(client, &payload).await {
        Ok(url) => match url {
            Some(url) => url,
            None => return Err((StatusCode::NOT_FOUND, "".to_string()).into_response()),
        },
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when get url: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_curioucity::GetUrlResponse {
        url: Some(url.as_pb_type()),
    };

    Ok((StatusCode::OK, Json(resp)))
}
