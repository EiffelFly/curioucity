use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;
use axum::{extract::Path, response::Response};
use axum::{http::StatusCode, response::IntoResponse, Json};

// Note: In the future we want to have this kind of structure
//
// #[async_trait]
// pub trait RestUrlService: Send + Sync + 'static {
//     async fn create_url(
//         Json(data): Json<pb_curioucity::CreateUrlRequest>,
//     ) -> Result<impl IntoResponse, Response>;

//     async fn delete_url(
//         Json(data): Json<pb_curioucity::DeleteUrlRequest>,
//     ) -> Result<pb_curioucity::DeleteUrlResponse, Response>;

//     async fn get_url(
//         Json(data): Json<pb_curioucity::GetUrlRequest>,
//     ) -> Result<pb_curioucity::GetUrlResponse, Response>;
// }

// pub struct RestUrlServiceImpl {}

pub async fn create_url(
    payload: Json<pb_curioucity::CreateUrlRequest>,
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

    let create_url_payload = db_curioucity::CreateUrlPayload {
        url: payload.url.clone(),
        resource_type: match pb_curioucity::ResourceType::as_db_type(payload.resource_type.clone())
        {
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

    let url = match db_curioucity::FullUrl::create(&client, &create_url_payload).await {
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

pub async fn delete_url(
    payload: Json<pb_curioucity::DeleteUrlRequest>,
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

    let delete_url_paylolad = db_curioucity::DeleteUrlPayload {
        url: payload.url.clone(),
    };

    match db_curioucity::FullUrl::delete(client, &delete_url_paylolad).await {
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

pub async fn get_url(
    Path(pb_curioucity::GetUrlRequest { url }): Path<pb_curioucity::GetUrlRequest>,
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

    let payload = db_curioucity::GetUrlPayload { url };

    let url = match db_curioucity::FullUrl::get(client, &payload).await {
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

pub async fn list_url(
    payload: Option<Json<pb_curioucity::ListUrlRequest>>,
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

    let mut page_size: i32 = 10;

    if let Some(payload) = payload {
        page_size = match payload.page_size {
            Some(page_size) => page_size,
            None => 10,
        };
    }

    let list_url_payload = db_curioucity::ListUrlPayload { page_size };

    let urls = match db_curioucity::FullUrl::list(client, &list_url_payload).await {
        Ok(urls) => urls,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when list url: {}", error),
            )
                .into_response())
        }
    };

    let mut pb_urls: Vec<pb_curioucity::FullUrl> = Vec::new();

    for i in urls {
        pb_urls.push(i.as_pb_type());
    }

    let size = pb_urls.len() as i64;

    let resp = pb_curioucity::ListUrlResponse {
        urls: pb_urls,
        size,
    };

    Ok((StatusCode::OK, Json(resp)))
}

pub async fn create_tag(
    payload: Json<pb_curioucity::CreateTagRequest>,
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

    let create_tag_payload = db_curioucity::CreateTagPayload {
        name: payload.name.clone(),
    };

    let tag = match db_curioucity::FullTag::create(client, &create_tag_payload).await {
        Ok(tag) => tag,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when create tag: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_curioucity::CreateTagResponse {
        tag: Some(tag.as_pb_type()),
    };

    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn delete_tag(
    payload: Json<pb_curioucity::DeleteTagRequest>,
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

    let delete_tag_payload = db_curioucity::DeleteTagPayload {
        name: payload.name.clone(),
    };

    match db_curioucity::FullTag::delete(client, &delete_tag_payload).await {
        Ok(_) => return Ok((StatusCode::NO_CONTENT, ())),
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when delete tag: {}", error),
            )
                .into_response())
        }
    };
}

pub async fn get_tag(
    Path(pb_curioucity::GetTagRequest { name }): Path<pb_curioucity::GetTagRequest>,
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

    let payload = db_curioucity::GetTagPayload { name };

    let tag = match db_curioucity::FullTag::get(client, &payload).await {
        Ok(tag) => match tag {
            Some(tag) => tag,
            None => return Err((StatusCode::NOT_FOUND, "".to_string()).into_response()),
        },
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when get tag: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_curioucity::GetTagResponse {
        tag: Some(tag.as_pb_type()),
    };

    Ok((StatusCode::OK, Json(resp)))
}

pub async fn list_tag(
    payload: Option<Json<pb_curioucity::ListTagRequest>>,
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

    let mut page_size: i32 = 10;

    if let Some(payload) = payload {
        page_size = match payload.page_size {
            Some(page_size) => page_size,
            None => 10,
        };
    }

    let payload = db_curioucity::ListTagPayload { page_size };

    let tags = match db_curioucity::FullTag::list(client, &payload).await {
        Ok(tags) => tags,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when list tag: {}", error),
            )
                .into_response())
        }
    };

    let mut pb_tags: Vec<pb_curioucity::FullTag> = Vec::new();

    for i in tags {
        pb_tags.push(i.as_pb_type());
    }

    let size = pb_tags.len() as i64;

    let resp = pb_curioucity::ListTagResponse {
        tags: pb_tags,
        size,
    };

    Ok((StatusCode::OK, Json(resp)))
}
