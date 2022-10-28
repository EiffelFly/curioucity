use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    db::model::curioucity::{CreateUrlPayload, Url},
    helper::error::CurioucityError,
};

pub async fn post_url_handler(
    Json(data): Json<CreateUrlPayload>,
) -> Result<impl IntoResponse, CurioucityError> {
    let client = edgedb_tokio::create_client().await?;

    let payload = CreateUrlPayload {
        url: data.url,
        resource_type: data.resource_type,
    };

    let url = Url::create(client, &payload).await?;

    println!("Url: {:?}", url);

    Ok((StatusCode::CREATED, Json(url)))
}
