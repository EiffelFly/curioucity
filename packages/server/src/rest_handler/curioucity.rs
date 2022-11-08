use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::db::model::curioucity as db_curioucity;
use crate::gen::curioucity::v1alpha as pb_curioucity;
use crate::helper::error::CurioucityError;

pub async fn create_url_handler(
    Json(data): Json<pb_curioucity::CreateUrlRequest>,
) -> Result<impl IntoResponse, CurioucityError> {
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