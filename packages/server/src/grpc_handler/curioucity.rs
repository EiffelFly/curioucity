use tonic::{Request, Response, Status};

use crate::db::model::curioucity as db_curioucity;
use crate::helper::error::CurioucityTonicError;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

#[derive(Default)]
pub struct UrlServiceImpl {}

#[tonic::async_trait]
impl pb_curioucity::url_service_server::UrlService for UrlServiceImpl {
    async fn create_url(
        &self,
        req: Request<pb_curioucity::CreateUrlRequest>,
    ) -> Result<Response<pb_curioucity::CreateUrlResponse>, Status> {
        let resp = create_url_helper(req.get_ref()).await?;
        Ok(Response::new(resp))
    }

    async fn delete_url(
        &self,
        req: Request<pb_curioucity::DeleteUrlRequest>,
    ) -> Result<Response<pb_curioucity::DeleteUrlResponse>, Status> {
        let resp = delete_url_helper(req.get_ref()).await?;
        Ok(Response::new(resp))
    }
}

pub async fn create_url_helper(
    req: &pb_curioucity::CreateUrlRequest,
) -> Result<pb_curioucity::CreateUrlResponse, CurioucityTonicError> {
    let client = edgedb_tokio::create_client().await?;

    let payload = db_curioucity::CreateUrlPayload {
        url: req.url.clone(),
        resource_type: pb_curioucity::ResourceType::as_db_type(req.resource_type)?,
    };

    let url = db_curioucity::Url::create(client, &payload).await?;

    let resp = pb_curioucity::CreateUrlResponse {
        url: Some(url.as_pb_type()),
    };

    Ok(resp)
}

pub async fn delete_url_helper(
    req: &pb_curioucity::DeleteUrlRequest,
) -> Result<pb_curioucity::DeleteUrlResponse, CurioucityTonicError> {
    let client = edgedb_tokio::create_client().await?;
    let payload = db_curioucity::DeleteUrlPayload {
        url: req.url.clone(),
    };

    db_curioucity::Url::delete(client, &payload).await?;

    let resp = pb_curioucity::DeleteUrlResponse {};

    Ok(resp)
}
