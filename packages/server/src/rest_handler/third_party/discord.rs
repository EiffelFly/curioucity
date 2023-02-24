use crate::db::model::third_party as db_third_party;
use crate::helper::time::get_edgedb_timestamp_from_2000_micros;
use crate::pb_gen::third_party::v1alpha as pb_third_party;
use axum::extract::Path;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn create_discord_guild(
    payload: Json<pb_third_party::CreateDiscordGuildRequest>,
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

    // Discord store their timestamp in seconds, but we need to convert it to micros seconds
    let edgedb_datetime =
        match get_edgedb_timestamp_from_2000_micros(payload.created_timestamp_at_discord * 1000000)
        {
            Ok(datetime) => datetime,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Something went wrong when parse discord int timestamp: {}",
                        error
                    ),
                )
                    .into_response())
            }
        };

    let payload = db_third_party::discord::CreateDiscordGuildPayload {
        guild_id: payload.guild_id.clone(),
        created_timestamp_at_discord: edgedb_datetime,
        icon: payload.icon.clone(),
        url: payload.url.clone(),
        name: payload.name.clone(),
    };

    let discord_guild = match db_third_party::discord::DiscordGuild::create(client, &payload).await
    {
        Ok(result) => result,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when create discord guild: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_third_party::CreateDiscordGuildResponse {
        discord_guild: Some(discord_guild.as_pb_type()),
    };

    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn delete_discord_guild(
    Path(pb_third_party::DeleteDiscordGuildRequest { guild_id }): Path<
        pb_third_party::DeleteDiscordGuildRequest,
    >,
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

    let delete_discord_guild_payload =
        db_third_party::discord::DeleteDiscordGuildPayload { guild_id };

    match db_third_party::discord::DiscordGuild::delete(client, &delete_discord_guild_payload).await
    {
        Ok(_) => return Ok((StatusCode::NO_CONTENT, ())),
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when delete discord guild: {}", error),
            )
                .into_response())
        }
    };
}

pub async fn get_discord_guild(
    Path(pb_third_party::GetDiscordGuildRequest { guild_id }): Path<
        pb_third_party::GetDiscordGuildRequest,
    >,
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

    let payload = db_third_party::discord::GetDiscordGuildPayload { guild_id };

    let discord_guild = match db_third_party::discord::DiscordGuild::get(client, &payload).await {
        Ok(result) => match result {
            Some(result) => result,
            None => return Err((StatusCode::NOT_FOUND, "".to_string()).into_response()),
        },
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when get discord message: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_third_party::GetDiscordGuildResponse {
        discord_guild: Some(discord_guild.as_pb_type()),
    };

    Ok((StatusCode::OK, Json(resp)))
}

pub async fn create_discord_thread(
    payload: Json<pb_third_party::CreateDiscordThreadRequest>,
) -> Result<impl IntoResponse, Response> {
    let client = match edgedb_tokio::create_client().await {
        Ok(result) => result,
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when access database: {}", error),
            )
                .into_response())
        }
    };

    // Discord store their timestamp in seconds, but we need to convert it to micros seconds
    let edgedb_datetime =
        match get_edgedb_timestamp_from_2000_micros(payload.created_timestamp_at_discord * 1000000)
        {
            Ok(result) => result,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Something went wrong when parse discord int timestamp: {}",
                        error
                    ),
                )
                    .into_response())
            }
        };

    // We need to implement this json object in the future
    let mock_json_value = serde_json::json!({"key": "value"}).to_string();

    let payload = db_third_party::discord::CreateDiscordThreadPayload {
        thread_id: payload.thread_id.clone(),
        created_timestamp_at_discord: edgedb_datetime,
        markdown_content: payload.markdown_content.clone(),
        url: payload.url.clone(),
        full_messages_json: mock_json_value,
    };

    let discord_thread =
        match db_third_party::discord::DiscordThread::create(client, &payload).await {
            Ok(result) => result,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong when create discord thread: {}", error),
                )
                    .into_response())
            }
        };

    let resp = pb_third_party::CreateDiscordThreadResponse {
        discord_thread: Some(discord_thread.as_pb_type()),
    };

    Ok((StatusCode::CREATED, Json(resp)))
}

pub async fn delete_discord_thread(
    Path(pb_third_party::DeleteDiscordThreadRequest { thread_id }): Path<
        pb_third_party::DeleteDiscordThreadRequest,
    >,
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

    let delete_discord_thread_payload =
        db_third_party::discord::DeleteDiscordThreadPayload { thread_id };

    match db_third_party::discord::DiscordThread::delete(client, &delete_discord_thread_payload)
        .await
    {
        Ok(_) => return Ok((StatusCode::NO_CONTENT, ())),
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when delete discord thread: {}", error),
            )
                .into_response())
        }
    };
}

pub async fn get_discord_thread(
    Path(pb_third_party::GetDiscordThreadRequest { thread_id }): Path<
        pb_third_party::GetDiscordThreadRequest,
    >,
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

    let payload = db_third_party::discord::GetDiscordThreadPayload { thread_id };

    let discord_thread = match db_third_party::discord::DiscordThread::get(client, &payload).await {
        Ok(result) => match result {
            Some(result) => result,
            None => return Err((StatusCode::NOT_FOUND, "".to_string()).into_response()),
        },
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when get discord message: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_third_party::GetDiscordThreadResponse {
        discord_thread: Some(discord_thread.as_pb_type()),
    };

    Ok((StatusCode::OK, Json(resp)))
}

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

    // Discord store their timestamp in seconds, but we need to convert it to micros seconds
    let edgedb_datetime =
        match get_edgedb_timestamp_from_2000_micros(payload.created_timestamp_at_discord * 1000000)
        {
            Ok(datetime) => datetime,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!(
                        "Something went wrong when parse discord int timestamp: {}",
                        error
                    ),
                )
                    .into_response())
            }
        };

    let payload = db_third_party::discord::CreateDiscordMessagePayload {
        message_id: payload.message_id.clone(),
        content: payload.content.clone(),
        created_timestamp_at_discord: edgedb_datetime,
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
                        "Something went wrong when create discord message: {}",
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

pub async fn delete_discord_message(
    Path(pb_third_party::DeleteDiscordMessageRequest { message_id }): Path<
        pb_third_party::DeleteDiscordMessageRequest,
    >,
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

    let delete_discord_message_payload =
        db_third_party::discord::DeleteDiscordMessagePayload { message_id };

    match db_third_party::discord::DiscordMessage::delete(client, &delete_discord_message_payload)
        .await
    {
        Ok(_) => return Ok((StatusCode::NO_CONTENT, ())),
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Something went wrong when delete discord message: {}",
                    error
                ),
            )
                .into_response())
        }
    };
}

pub async fn get_discord_message(
    Path(pb_third_party::GetDiscordMessageRequest { message_id }): Path<
        pb_third_party::GetDiscordMessageRequest,
    >,
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

    let payload = db_third_party::discord::GetDiscordMessagePayload { message_id };

    let discord_message = match db_third_party::discord::DiscordMessage::get(client, &payload).await
    {
        Ok(discord_message) => match discord_message {
            Some(discord_message) => discord_message,
            None => return Err((StatusCode::NOT_FOUND, "".to_string()).into_response()),
        },
        Err(error) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong when get discord message: {}", error),
            )
                .into_response())
        }
    };

    let resp = pb_third_party::GetDiscordMessageResponse {
        discord_message: Some(discord_message.as_pb_type()),
    };

    Ok((StatusCode::OK, Json(resp)))
}

pub async fn list_discord_message(
    payload: Option<Json<pb_third_party::ListDiscordMessageRequest>>,
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

    let list_discord_message_payload =
        db_third_party::discord::ListDiscordMessagePayload { page_size };

    let db_discord_messages =
        match db_third_party::discord::DiscordMessage::list(client, &list_discord_message_payload)
            .await
        {
            Ok(discord_messages) => discord_messages,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong when list discord message: {}", error),
                )
                    .into_response())
            }
        };

    let mut pb_discord_messages: Vec<pb_third_party::DiscordMessage> = Vec::new();

    for i in db_discord_messages {
        pb_discord_messages.push(i.as_pb_type());
    }

    let size = pb_discord_messages.len() as i32;

    let resp = pb_third_party::ListDiscordMessageResponse {
        discord_messages: pb_discord_messages,
        size,
    };

    Ok((StatusCode::OK, Json(resp)))
}

pub async fn list_discord_threads(
    payload: Option<Json<pb_third_party::ListDiscordThreadRequest>>,
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

    let list_discord_thread_payload =
        db_third_party::discord::ListDiscordThreadPayload { page_size };

    let db_discord_threads =
        match db_third_party::discord::DiscordThread::list(client, &list_discord_thread_payload)
            .await
        {
            Ok(result) => result,
            Err(error) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong when list discord message: {}", error),
                )
                    .into_response())
            }
        };

    let mut pb_discord_threads: Vec<pb_third_party::DiscordThread> = Vec::new();

    for i in db_discord_threads {
        pb_discord_threads.push(i.as_pb_type());
    }

    let size = pb_discord_threads.len() as i32;

    let resp = pb_third_party::ListDiscordThreadResponse {
        discord_threads: pb_discord_threads,
        size,
    };

    Ok((StatusCode::OK, Json(resp)))
}
