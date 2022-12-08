use anyhow::bail;
use edgedb_derive::Queryable;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::db::model::curioucity::{Tag, Url};
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;
use crate::pb_gen::third_party::v1alpha as pb_third_party;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordGuild {
    pub id: String,
    pub guild_id: i64,
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
        id: value.id.clone(),
        guild_id: value.guild_id,
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
    pub id: String,
    pub thread_id: i64,
    #[edgedb(json)]
    pub full_messages_json: HashMap<String, String>,
    pub create_at: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub messages: Vec<DiscordMessage>,
    pub url: Url,
    pub created_timestamp_at_curioucity: String,
    pub created_timestamp_at_discord: String,
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
        id: value.id.clone(),
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
    pub id: String,
    pub message_id: i64,
    pub content: String,
    pub create_at: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub url: Url,
    pub created_timestamp_at_curioucity: String,
    pub created_timestamp_at_discord: String,
}

#[derive(Debug)]
pub struct CreateDiscordMessagePayload {
    pub message_id: i64,
    pub content: String,
    pub markdown_content: String,
    pub url: String,
    pub created_timestamp_at_curioucity: String,
    pub created_timestamp_at_discord: String,
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
        let query = "select (
            insert DiscordMessage {
                message_id := <str>$0
                content := <str>$1,
                markdown_content := <str>$2,
                created_timestamp_at_discord := <str>$3,
                created_timestamp_at_curioucity := <str>$4,
                url := (select Url filter .url = <str>$5)
            }
        ) {
            id,
            message_id,
            content,
            created_timestamp_at_discord,
            created_timestamp_at_curioucity,
            markdown_content,
            url,
            tags {
                id,
                name
            }
        };";

        let response = client
            .query_json(
                &query,
                &(
                    &payload.message_id,
                    &payload.content,
                    &payload.markdown_content,
                    &payload.created_timestamp_at_discord,
                    &payload.created_timestamp_at_curioucity,
                    &payload.url,
                ),
            )
            .await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<DiscordMessage>>(json.as_ref()).unwrap();
                Ok(result.into_iter().nth(0).unwrap())
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
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
        id: value.id.clone(),
        message_id: value.message_id,
        content: value.content.clone(),
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
        created_timestamp_at_discord: value.created_timestamp_at_discord.clone(),
        markdown_content: value.markdown_content.clone(),
        tags: pb_tags,
        url: Some(value.url.as_pb_type()),
    }
}
