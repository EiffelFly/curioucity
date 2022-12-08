use crate::db::model::third_party as db_third_party;
use crate::pb_gen::third_party::v1alpha as pb_third_party;
use chrono::{DateTime, Utc};
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

        let utc_now: DateTime<Utc> = Utc::now();

        let payload = db_third_party::discord::CreateDiscordMessagePayload {
            message_id: req_ref.message_id,
            content: req_ref.content.clone(),
            created_timestamp_at_discord: req_ref.created_timestamp_at_discord.clone(),
            created_timestamp_at_curioucity: utc_now.to_string(),
            markdown_content: req_ref.markdown_content.clone(),
            url: req_ref.url.clone(),
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
}
