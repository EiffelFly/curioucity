use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::db::model::curioucity as db_curioucity;
use crate::helper::error::CurioucityAxumError;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

pub async fn create_url_handler(
    Json(data): Json<pb_curioucity::CreateUrlRequest>,
) -> Result<impl IntoResponse, CurioucityAxumError> {
    let client = edgedb_tokio::create_client().await?;

    let payload = db_curioucity::CreateUrlPayload {
        url: data.url,
        resource_type: pb_curioucity::ResourceType::as_db_type(data.resource_type)?,
    };

    let url = db_curioucity::Url::create(client, &payload).await?;

    let resp = pb_curioucity::CreateUrlResponse {
        url: Some(url.as_pb_type()),
    };

    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn delete_url_handler(
    Json(data): Json<pb_curioucity::DeleteUrlRequest>,
) -> Result<impl IntoResponse, CurioucityAxumError> {
    let client = edgedb_tokio::create_client().await?;

    let payload = db_curioucity::DeleteUrlPayload { url: data.url };

    db_curioucity::Url::delete(client, &payload).await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

pub async fn get_url_handler(
    Json(data): Json<pb_curioucity::GetUrlRequest>,
) -> Result<impl IntoResponse, CurioucityAxumError> {
    let client = edgedb_tokio::create_client().await?;

    let payload = db_curioucity::GetUrlPayload { url: data.url };

    let url = db_curioucity::Url::get(client, &payload).await?;

    let resp = pb_curioucity::GetUrlResponse {
        url: Some(url.as_pb_type()),
    };

    Ok((StatusCode::OK, Json(resp)))
}
