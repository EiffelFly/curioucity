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

        let url = match db_curioucity::FullUrl::create(client, &payload).await {
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

        match db_curioucity::FullUrl::delete(client, &payload).await {
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

        let url = match db_curioucity::FullUrl::get(client, &payload).await {
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
    async fn list_url(
        &self,
        req: Request<pb_curioucity::ListUrlRequest>,
    ) -> Result<Response<pb_curioucity::ListUrlResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let page_size = match req.get_ref().page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let payload = db_curioucity::ListUrlPayload { page_size };

        let urls = match db_curioucity::FullUrl::list(client, &payload).await {
            Ok(urls) => urls,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when list url: {}",
                    error
                )))
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

    async fn list_tag(
        &self,
        req: Request<pb_curioucity::ListTagRequest>,
    ) -> Result<Response<pb_curioucity::ListTagResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let page_size = match req.get_ref().page_size {
            Some(page_size) => page_size,
            None => 10,
        };

        let payload = db_curioucity::ListTagPayload { page_size };

        let tags = match db_curioucity::FullTag::list(client, &payload).await {
            Ok(tags) => tags,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when list tag: {}",
                    error
                )))
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

        Ok(Response::new(resp))
    }
}
