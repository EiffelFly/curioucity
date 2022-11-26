use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GrpcUrlServiceImpl {}

#[tonic::async_trait]
impl pb_curioucity::url_service_server::UrlService for GrpcUrlServiceImpl {
    async fn create_url(
        &self,
        req: Request<pb_curioucity::CreateUrlRequest>,
    ) -> Result<Response<pb_curioucity::CreateUrlResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let req_ref = req.get_ref();

        let payload = db_curioucity::CreateUrlPayload {
            url: req_ref.url.clone(),
            resource_type: match pb_curioucity::ResourceType::as_db_type(req_ref.resource_type) {
                Ok(resource_type) => resource_type,
                Err(error) => {
                    return Err(Status::internal(format!(
                        "Something went wrong when transform resource_type: {}",
                        error
                    )))
                }
            },
        };

        let url = match db_curioucity::Url::create(client, &payload).await {
            Ok(url) => url,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when create url: {}",
                    error
                )))
            }
        };

        let resp = pb_curioucity::CreateUrlResponse {
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
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
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
                return Err(Status::internal(format!(
                    "Something went wrong when delete url: {}",
                    error
                )))
            }
        }
    }

    async fn get_url(
        &self,
        req: Request<pb_curioucity::GetUrlRequest>,
    ) -> Result<Response<pb_curioucity::GetUrlResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
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
                return Err(Status::internal(format!(
                    "Something went wrong when get url: {}",
                    error
                )))
            }
        };

        let resp = pb_curioucity::GetUrlResponse {
            url: Some(url.as_pb_type()),
        };

        Ok(Response::new(resp))
    }
}

#[derive(Default)]
pub struct GrpcTagServiceImpl {}

#[tonic::async_trait]
impl pb_curioucity::tag_service_server::TagService for GrpcTagServiceImpl {
    async fn create_tag(
        &self,
        req: Request<pb_curioucity::CreateTagRequest>,
    ) -> Result<Response<pb_curioucity::CreateTagResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let payload = db_curioucity::CreateTagPayload {
            name: req.get_ref().name.clone(),
        };

        let tag = match db_curioucity::FullTag::create(client, &payload).await {
            Ok(tag) => tag,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when create tag: {}",
                    error
                )))
            }
        };

        let resp = pb_curioucity::CreateTagResponse {
            tag: Some(tag.as_pb_type()),
        };

        Ok(Response::new(resp))
    }

    async fn delete_tag(
        &self,
        req: Request<pb_curioucity::DeleteTagRequest>,
    ) -> Result<Response<pb_curioucity::DeleteTagResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let payload = db_curioucity::DeleteTagPayload {
            name: req.get_ref().name.clone(),
        };

        match db_curioucity::FullTag::delete(client, &payload).await {
            Ok(_) => {
                let resp = pb_curioucity::DeleteTagResponse {};
                return Ok(Response::new(resp));
            }
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when delete tag: {}",
                    error
                )))
            }
        };
    }

    async fn get_tag(
        &self,
        req: Request<pb_curioucity::GetTagRequest>,
    ) -> Result<Response<pb_curioucity::GetTagResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let payload = db_curioucity::GetTagPayload {
            name: req.get_ref().name.clone(),
        };

        let tag = match db_curioucity::FullTag::get(client, &payload).await {
            Ok(tag) => match tag {
                Some(tag) => tag,
                None => return Err(Status::not_found("".to_string())),
            },
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when get tag: {}",
                    error
                )))
            }
        };

        let resp = pb_curioucity::GetTagResponse {
            tag: Some(tag.as_pb_type()),
        };

        Ok(Response::new(resp))
    }
}
