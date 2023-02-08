use crate::pb_gen::third_party::v1alpha as pb_third_party;
use crate::{
    db::model::third_party as db_third_party, helper::time::get_edgedb_timestamp_from_2000_micros,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GrpcDiscordServiceImpl {}

#[tonic::async_trait]
impl pb_third_party::discord_service_server::DiscordService for GrpcDiscordServiceImpl {
    async fn create_discord_message(
        &self,
        req: Request<pb_third_party::CreateDiscordMessageRequest>,
    ) -> Result<Response<pb_third_party::CreateDiscordMessageResponse>, Status> {
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

        // Discord store their timestamp in seconds, but we need to convert it to micros seconds
        let edgedb_datetime = match get_edgedb_timestamp_from_2000_micros(
            req_ref.created_timestamp_at_discord * 1000000,
        ) {
            Ok(datetime) => datetime,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when parse discord int timestamp: {}",
                    error
                )))
            }
        };

        let payload = db_third_party::discord::CreateDiscordMessagePayload {
            message_id: req_ref.message_id.clone(),
            content: req_ref.content.clone(),
            created_timestamp_at_discord: edgedb_datetime,
            markdown_content: req_ref.markdown_content.clone(),
            url: req_ref.url.clone(),
            order_in_thread: req_ref.order_in_thread.clone(),
        };

        let discord_message =
            match db_third_party::discord::DiscordMessage::create(client, &payload).await {
                Ok(discord_message) => discord_message,
                Err(error) => {
                    return Err(Status::internal(format!(
                        "Something went wrong when create discord message: {}",
                        error
                    )))
                }
            };

        let resp = pb_third_party::CreateDiscordMessageResponse {
            discord_message: Some(discord_message.as_pb_type()),
        };

        Ok(Response::new(resp))
    }

    async fn delete_discord_message(
        &self,
        req: Request<pb_third_party::DeleteDiscordMessageRequest>,
    ) -> Result<Response<pb_third_party::DeleteDiscordMessageResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let payload = db_third_party::discord::DeleteDiscordMessagePayload {
            message_id: req.get_ref().message_id.clone(),
        };

        match db_third_party::discord::DiscordMessage::delete(client, &payload).await {
            Ok(_) => {
                let resp = pb_third_party::DeleteDiscordMessageResponse {};
                return Ok(Response::new(resp));
            }
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when delete discord message: {}",
                    error
                )))
            }
        }
    }

    async fn get_discord_message(
        &self,
        req: Request<pb_third_party::GetDiscordMessageRequest>,
    ) -> Result<Response<pb_third_party::GetDiscordMessageResponse>, Status> {
        let client = match edgedb_tokio::create_client().await {
            Ok(client) => client,
            Err(error) => {
                return Err(Status::internal(format!(
                    "Something went wrong when access database: {}",
                    error
                )))
            }
        };

        let payload = db_third_party::discord::GetDiscordMessagePayload {
            message_id: req.get_ref().message_id.clone(),
        };

        let discord_message =
            match db_third_party::discord::DiscordMessage::get(client, &payload).await {
                Ok(discord_message) => match discord_message {
                    Some(discord_message) => discord_message,
                    None => return Err(Status::not_found("".to_string())),
                },
                Err(error) => {
                    return Err(Status::internal(format!(
                        "Something went wrong when get discord message: {}",
                        error
                    )))
                }
            };

        let resp = pb_third_party::GetDiscordMessageResponse {
            discord_message: Some(discord_message.as_pb_type()),
        };

        Ok(Response::new(resp))
    }

    async fn list_discord_message(
        &self,
        req: Request<pb_third_party::ListDiscordMessageRequest>,
    ) -> Result<Response<pb_third_party::ListDiscordMessageResponse>, Status> {
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

        let payload = db_third_party::discord::ListDiscordMessagePayload { page_size };

        let discord_messages =
            match db_third_party::discord::DiscordMessage::list(client, &payload).await {
                Ok(discord_messages) => discord_messages,
                Err(error) => {
                    return Err(Status::internal(format!(
                        "Something went wrong when list discord message: {}",
                        error
                    )))
                }
            };

        let mut pb_discord_messages: Vec<pb_third_party::DiscordMessage> = Vec::new();

        for i in discord_messages {
            pb_discord_messages.push(i.as_pb_type());
        }

        let size = pb_discord_messages.len() as i32;

        let resp = pb_third_party::ListDiscordMessageResponse {
            discord_messages: pb_discord_messages,
            size,
        };

        Ok(Response::new(resp))
    }
}
