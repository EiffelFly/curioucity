use crate::db::model::third_party as db_third_party;
use crate::pb_gen::third_party::v1alpha as pb_third_party;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn create_discord_message(
    payload: Json<pb_third_party::CreateDiscordMessageRequest>,
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

    let payload = db_third_party::discord::CreateDiscordMessagePayload {
        message_id: payload.message_id as i64,
        content: payload.content.clone(),
        created_timestamp_at_discord: payload.created_timestamp_at_discord.clone(),
        markdown_content: payload.markdown_content.clone(),
        url: payload.url.clone(),
        order_in_thread: payload.order_in_thread.clone(),
    };

    let discord_message =
        match db_third_party::discord::DiscordMessage::create(client, &payload).await {
            Ok(discord_message) => discord_message,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Something went wrong when create discord_message: {}",
                        error
                    ),
                )
                    .into_response())
            }
        };

    let resp = pb_third_party::CreateDiscordMessageResponse {
        discord_message: Some(discord_message.as_pb_type()),
    };

    Ok((StatusCode::CREATED, Json(resp)))
}
