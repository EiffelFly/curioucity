use anyhow::bail;
use chrono::Utc;
use edgedb_derive::Queryable;
use edgedb_protocol::model::{Datetime, Uuid};
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::db::model::curioucity::{self as db_curioucity, FullUrl};
use crate::db::model::curioucity::{Tag, Url};
use crate::helper::time::get_edgedb_timestamp_from_2000_micros;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;
use crate::pb_gen::third_party::v1alpha::{self as pb_third_party};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordGuild {
    pub id: Uuid,
    pub kind: String,
    pub guild_id: String,
    pub name: String,
    pub icon: String,
    pub threads: Vec<DiscordThread>,
    pub tags: Vec<Tag>,
    pub url: Url,
    pub created_timestamp_at_curioucity: String,
    pub created_timestamp_at_discord: String,
}

impl From<DiscordGuild> for pb_third_party::DiscordGuild {
    fn from(value: DiscordGuild) -> pb_third_party::DiscordGuild {
        transform_discord_guild_to_pb(&value)
    }
}

impl DiscordGuild {
    pub fn as_pb_type(&self) -> pb_third_party::DiscordGuild {
        transform_discord_guild_to_pb(self)
    }
}

fn transform_discord_guild_to_pb(value: &DiscordGuild) -> pb_third_party::DiscordGuild {
    let mut pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in &value.tags {
        pb_tags.push(i.as_pb_type());
    }

    let mut pb_threads: Vec<pb_third_party::DiscordThread> = Vec::new();

    for i in &value.threads {
        pb_threads.push(i.as_pb_type());
    }

    pb_third_party::DiscordGuild {
        id: value.id.clone().to_string(),
        kind: 0,
        guild_id: value.guild_id.clone(),
        name: value.name.clone(),
        icon: value.icon.clone(),
        threads: pb_threads,
        tags: pb_tags,
        url: Some(value.url.as_pb_type()),
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
        created_timestamp_at_discord: value.created_timestamp_at_discord.clone(),
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordThread {
    pub id: Uuid,
    pub kind: String,
    pub thread_id: String,
    #[edgedb(json)]
    pub full_messages_json: HashMap<String, String>,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub messages: Vec<DiscordMessage>,
    pub url: Url,
    pub created_timestamp_at_curioucity: String,
    pub created_timestamp_at_discord: String,
}

#[derive(Debug)]
pub struct CreateDiscordThreadPayload {
    pub thread_id: String,
    pub markdown_content: String,
    pub url: String,
    pub created_timestamp_at_discord: Datetime,
    pub full_messages_json: String,
}

#[derive(Debug)]
pub struct DeleteDiscordThreadPayload {
    pub thread_id: String,
}

#[derive(Debug)]
pub struct GetDiscordThreadPayload {
    pub thread_id: String,
}

pub struct ListDiscordThreadPayload {
    pub page_size: i32,
}

impl DiscordThread {
    pub async fn create(
        client: Client,
        payload: &CreateDiscordThreadPayload,
    ) -> Result<Self, anyhow::Error> {
        let create_url_payload = db_curioucity::CreateUrlPayload {
            url: payload.url.clone(),
            resource_type: db_curioucity::ResourceType::DiscordThread,
        };

        match FullUrl::create(&client, &create_url_payload).await {
            Ok(_) => {}
            Err(error) => {
                println!("Error when create url: {:?}", error);
                bail!("Error when create url: {}", error)
            }
        }

        let create_discord_thread_query = "select (
            insert DiscordThread {
                kind := 'DISCORD_THREAD',
                thread_id := <str>$0,
                markdown_content := <str>$1,
                created_timestamp_at_discord := <datetime>$2,
                created_timestamp_at_curioucity := <datetime>$3,
                url := (select Url filter .url = <str>$4),
                full_messages_json := to_json(<str>$5),
            }
        ) {
            id,
            thread_id,
            kind,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            full_messages_json,
            url: {
                id,
                url,
                references,
                resource_type,
                created_timestamp_at_curioucity,
            },
            tags: {
                id,
                name
            },
            messages: {
                id
            }
        };";

        let created_timestamp_at_curioucity =
            match get_edgedb_timestamp_from_2000_micros(Utc::now().timestamp_micros()) {
                Ok(time) => time,
                Err(error) => {
                    println!("Time is out of range: {:?}", error);
                    bail!("{}", error)
                }
            };

        let response_json = match client
            .query_json(
                &create_discord_thread_query,
                &(
                    &payload.thread_id,
                    &payload.markdown_content,
                    &payload.created_timestamp_at_discord,
                    &created_timestamp_at_curioucity,
                    &payload.url,
                    &payload.full_messages_json,
                ),
            )
            .await
        {
            Ok(json) => json,
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        };

        let discord_threads =
            match serde_json::from_str::<Vec<DiscordThread>>(response_json.as_ref()) {
                Ok(result) => result,
                Err(error) => {
                    println!("Deserialize Error: {}", error);
                    bail!("Deserialize Error: {}", error)
                }
            };

        let discord_thread = match discord_threads.into_iter().nth(0) {
            Some(result) => result,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        Ok(discord_thread)
    }

    pub async fn delete(
        client: Client,
        payload: &DeleteDiscordThreadPayload,
    ) -> Result<(), anyhow::Error> {
        let query = "delete DiscordThread filter .thread_id = <str>$0";

        let response = client.query_json(&query, &(&payload.thread_id,)).await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
    }

    pub async fn get(
        client: Client,
        payload: &GetDiscordThreadPayload,
    ) -> Result<Option<Self>, anyhow::Error> {
        let query = "select DiscordThread {
            id,
            thread_id,
            kind,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            full_messages_json,
            url: {
                id,
                url,
                references,
                resource_type,
                created_timestamp_at_curioucity,
            },
            tags: {
                id,
                name
            },
            messages: {
                id
            }
        } filter .thread_id = <str>$0";

        let response_json = match client.query_json(&query, &(&payload.thread_id,)).await {
            Ok(result) => result,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let discord_threads =
            match serde_json::from_str::<Vec<DiscordThread>>(response_json.as_ref()) {
                Ok(result) => result,
                Err(error) => {
                    println!("Deserialize Error: {}", error);
                    bail!("Deserialize Error: {}", error)
                }
            };

        if discord_threads.is_empty() {
            return Ok(None);
        }

        let discord_thread = match discord_threads.into_iter().nth(0) {
            Some(result) => result,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        Ok(Some(discord_thread))
    }

    pub async fn list(
        client: Client,
        payload: &ListDiscordThreadPayload,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let query = "select DiscordThread {
            id,
            thread_id,
            kind,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            full_messages_json,
            url: {
                id,
                url,
                references,
                resource_type,
                created_timestamp_at_curioucity,
            },
            tags: {
                id,
                name
            },
            messages: {
                id
            }
        } order by .id limit <int32>$0";

        let response_json = match client.query_json(&query, &(&payload.page_size,)).await {
            Ok(result) => result,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let discord_threads =
            match serde_json::from_str::<Vec<DiscordThread>>(response_json.as_ref()) {
                Ok(result) => result,
                Err(error) => {
                    println!("Deserialize Error: {}", error);
                    bail!("Deserialize Error: {}", error)
                }
            };

        Ok(discord_threads)
    }
}

impl From<DiscordThread> for pb_third_party::DiscordThread {
    fn from(value: DiscordThread) -> Self {
        transform_discord_thread_to_pb(&value)
    }
}

impl DiscordThread {
    pub fn as_pb_type(&self) -> pb_third_party::DiscordThread {
        transform_discord_thread_to_pb(self)
    }
}

fn transform_discord_thread_to_pb(value: &DiscordThread) -> pb_third_party::DiscordThread {
    let mut pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in &value.tags {
        pb_tags.push(i.as_pb_type());
    }

    let mut pb_messages: Vec<pb_third_party::DiscordMessage> = Vec::new();

    for i in &value.messages {
        pb_messages.push(i.as_pb_type());
    }

    pb_third_party::DiscordThread {
        id: value.id.clone().to_string(),
        kind: 0,
        thread_id: value.thread_id.clone(),
        full_messages_json: value.full_messages_json.clone(),
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
        created_timestamp_at_discord: value.created_timestamp_at_discord.clone(),
        markdown_content: value.markdown_content.clone(),
        tags: pb_tags,
        messages: pb_messages,
        url: Some(value.url.as_pb_type()),
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordMessage {
    pub id: Uuid,
    pub kind: String,
    pub message_id: String,
    pub content: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub url: Url,
    pub created_timestamp_at_curioucity: String,
    pub created_timestamp_at_discord: String,
    pub order_in_thread: i32,
}

#[derive(Debug)]
pub struct CreateDiscordMessagePayload {
    pub message_id: String,
    pub content: String,
    pub markdown_content: String,
    pub url: String,
    pub created_timestamp_at_discord: Datetime,
    pub order_in_thread: i32,
}

pub struct DeleteDiscordMessagePayload {
    pub message_id: String,
}

pub struct GetDiscordMessagePayload {
    pub message_id: String,
}

pub struct ListDiscordMessagePayload {
    pub page_size: i32,
}

impl From<DiscordMessage> for pb_third_party::DiscordMessage {
    fn from(value: DiscordMessage) -> Self {
        transform_discord_message_to_pb(&value)
    }
}

impl DiscordMessage {
    pub async fn create(
        client: Client,
        payload: &CreateDiscordMessagePayload,
    ) -> Result<Self, anyhow::Error> {
        let create_url_payload = db_curioucity::CreateUrlPayload {
            url: payload.url.clone(),
            resource_type: db_curioucity::ResourceType::DiscordMessage,
        };

        match FullUrl::create(&client, &create_url_payload).await {
            Ok(_) => {}
            Err(error) => {
                println!("Error when create url: {:?}", error);
                bail!("Error when create url: {}", error)
            }
        }

        let create_discord_message_query = "select (
            insert DiscordMessage {
                message_id := <str>$0,
                kind := 'DISCORD_MESSAGE',
                content := <str>$1,
                markdown_content := <str>$2,
                created_timestamp_at_discord := <datetime>$3,
                created_timestamp_at_curioucity := <datetime>$4,
                url := (select Url filter .url = <str>$5),
                order_in_thread := <int32>$6,
            }
        ) {
            id,
            message_id,
            kind,
            content,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            order_in_thread,
            url: {
                id,
                url,
                references,
                resource_type,
                created_timestamp_at_curioucity,
            },
            tags: {
                id,
                name
            }
        };";

        let created_timestamp_at_curioucity =
            match get_edgedb_timestamp_from_2000_micros(Utc::now().timestamp_micros()) {
                Ok(time) => time,
                Err(error) => {
                    println!("Time is out of range: {:?}", error);
                    bail!("{}", error)
                }
            };

        let response_json = match client
            .query_json(
                &create_discord_message_query,
                &(
                    &payload.message_id,
                    &payload.content,
                    &payload.markdown_content,
                    &payload.created_timestamp_at_discord,
                    &created_timestamp_at_curioucity,
                    &payload.url,
                    &payload.order_in_thread,
                ),
            )
            .await
        {
            Ok(json) => json,
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        };

        let discord_messages =
            match serde_json::from_str::<Vec<DiscordMessage>>(response_json.as_ref()) {
                Ok(result) => result,
                Err(error) => {
                    println!("Deserialize Error: {}", error);
                    bail!("Deserialize Error: {}", error)
                }
            };

        let discord_message = match discord_messages.into_iter().nth(0) {
            Some(message) => message,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        Ok(discord_message)
    }

    pub async fn delete(
        client: Client,
        payload: &DeleteDiscordMessagePayload,
    ) -> Result<(), anyhow::Error> {
        let query = "delete DiscordMessage filter .message_id = <str>$0";

        let response = client.query_json(&query, &(&payload.message_id,)).await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
    }

    pub async fn get(
        client: Client,
        payload: &GetDiscordMessagePayload,
    ) -> Result<Option<Self>, anyhow::Error> {
        let query = "select DiscordMessage {
            id,
            message_id,
            kind,
            content,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            order_in_thread,
            url: {
                id,
                url,
                references,
                resource_type,
                created_timestamp_at_curioucity,
            },
            tags: {
                id,
                name
            }
        } filter .message_id = <str>$0";

        let response_json = match client.query_json(&query, &(&payload.message_id,)).await {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let discord_messages =
            match serde_json::from_str::<Vec<DiscordMessage>>(response_json.as_ref()) {
                Ok(result) => result,
                Err(error) => {
                    println!("Deserialize Error: {}", error);
                    bail!("Deserialize Error: {}", error)
                }
            };

        if discord_messages.is_empty() {
            return Ok(None);
        }

        let discord_message = match discord_messages.into_iter().nth(0) {
            Some(result) => result,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        Ok(Some(discord_message))
    }

    pub async fn list(
        client: Client,
        payload: &ListDiscordMessagePayload,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let query = "select DiscordMessage {
            id,
            message_id,
            kind,
            content,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            order_in_thread,
            url: {
                id,
                url,
                references,
                resource_type,
                created_timestamp_at_curioucity,
            },
            tags: {
                id,
                name
            }
        } order by .id
        limit <int32>$0";

        let response_json = match client.query_json(&query, &(&payload.page_size,)).await {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let discord_messages =
            match serde_json::from_str::<Vec<DiscordMessage>>(response_json.as_ref()) {
                Ok(result) => result,
                Err(error) => {
                    println!("Deserialize Error: {}", error);
                    bail!("Deserialize Error: {}", error)
                }
            };

        Ok(discord_messages)
    }

    pub fn as_pb_type(&self) -> pb_third_party::DiscordMessage {
        transform_discord_message_to_pb(self)
    }
}

fn transform_discord_message_to_pb(value: &DiscordMessage) -> pb_third_party::DiscordMessage {
    let mut pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in &value.tags {
        pb_tags.push(i.as_pb_type());
    }

    pb_third_party::DiscordMessage {
        id: value.id.clone().to_string(),
        kind: 1, // By default 0 will be unspecific, 1 will be its kind
        message_id: value.message_id.clone(),
        content: value.content.clone(),
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
        created_timestamp_at_discord: value.created_timestamp_at_discord.clone(),
        markdown_content: value.markdown_content.clone(),
        tags: pb_tags,
        url: Some(value.url.as_pb_type()),
        order_in_thread: value.order_in_thread.clone(),
    }
}
