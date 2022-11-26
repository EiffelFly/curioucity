use edgedb_derive::Queryable;
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
    let pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in value.tags {
        pb_tags.push(i.as_pb_type());
    }

    let pb_threads: Vec<pb_third_party::DiscordThread> = Vec::new();

    for i in value.threads {
        pb_threads.push(i.as_pb_type());
    }

    pb_third_party::DiscordGuild {
        id: value.id,
        guild_id: value.guild_id,
        name: value.name,
        icon: value.icon,
        threads: pb_threads,
        tags: pb_tags,
        url: Some(value.url.as_pb_type()),
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
    let pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in value.tags {
        pb_tags.push(i.as_pb_type());
    }

    let pb_messages: Vec<pb_third_party::DiscordMessage> = Vec::new();

    for i in value.messages {
        pb_messages.push(i.as_pb_type());
    }

    pb_third_party::DiscordThread {
        id: value.id,
        thread_id: value.thread_id,
        full_messages_json: value.full_messages_json,
        create_at: value.create_at,
        markdown_content: value.markdown_content,
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
}

impl From<DiscordMessage> for pb_third_party::DiscordMessage {
    fn from(value: DiscordMessage) -> Self {
        transform_discord_message_to_pb(&value)
    }
}

impl DiscordMessage {
    pub fn as_pb_type(&self) -> pb_third_party::DiscordMessage {
        transform_discord_message_to_pb(self)
    }
}

fn transform_discord_message_to_pb(value: &DiscordMessage) -> pb_third_party::DiscordMessage {
    let pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in value.tags {
        pb_tags.push(i.as_pb_type());
    }

    pb_third_party::DiscordMessage {
        id: value.id,
        message_id: value.message_id,
        content: value.content,
        create_at: value.create_at,
        markdown_content: value.markdown_content,
        tags: pb_tags,
        url: Some(value.url.as_pb_type()),
    }
}
