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
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(
                    "Something went wrong when create database client".to_string(),
                ))
            }
        };

        let req_ref = req.get_ref();

        let payload = db_curioucity::CreateUrlPayload {
            url: req_ref.url.clone(),
            resource_type: match pb_curioucity::ResourceType::as_db_type(req_ref.resource_type) {
                Ok(resource_type) => resource_type,
                Err(error) => {
                    return Err(Status::internal(
                        "Something went wrong when transform resource_type".to_string(),
                    ))
                }
            },
        };

        let url = match db_curioucity::Url::create(client, &payload).await {
            Ok(url) => url,
            Err(error) => {
                return Err(Status::internal(
                    "Something went wrong when create url".to_string(),
                ))
            }
        };

        let resp = pb_curioucity::CreateUrlResponse {
            url: Some(url.as_pb_type()),
        };

        Ok(Response::new(resp))
    }

    async fn get_url(
        &self,
        req: Request<pb_curioucity::GetUrlRequest>,
    ) -> Result<Response<pb_curioucity::GetUrlResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(
                    "Something went wrong when create database client".to_string(),
                ))
            }
        };

        let payload = db_curioucity::GetUrlPayload {
            url: req.get_ref().url.clone(),
        };

        let url = match db_curioucity::Url::get(client, &payload).await {
            Ok(url) => match url {
                Some(url) => url,
                None => return Err(Status::not_found("".to_string())),
            },
            Err(error) => {
                return Err(Status::internal(
                    "Something went wrong when get url from database".to_string(),
                ))
            }
        };

        let resp = pb_curioucity::GetUrlResponse {
            url: Some(url.as_pb_type()),
        };

        Ok(Response::new(resp))
    }

    async fn delete_url(
        &self,
        req: Request<pb_curioucity::DeleteUrlRequest>,
    ) -> Result<Response<pb_curioucity::DeleteUrlResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(
                    "Something went wrong when create database client".to_string(),
                ))
            }
        };

        let payload = db_curioucity::DeleteUrlPayload {
            url: req.get_ref().url.clone(),
        };

        match db_curioucity::Url::delete(client, &payload).await {
            Ok(_) => {
                let resp = pb_curioucity::DeleteUrlResponse {};
                return Ok(Response::new(resp));
            }
            Err(error) => {
                return Err(Status::internal(
                    "Something went wrong when delete url".to_string(),
                ))
            }
        }
    }
}
