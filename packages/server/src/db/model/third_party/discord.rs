use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::db::model::curioucity::{Tag, Url};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordGuild {
    pub guild_id: i64,
    pub name: String,
    pub icon: String,
    pub threads: Vec<DiscordThread>,
    pub tags: Vec<Tag>,
    pub url: Url,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordThread {
    pub thread_id: i64,
    #[edgedb(json)]
    pub full_messages_json: HashMap<String, String>,
    pub create_at: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub messages: Vec<DiscordMessage>,
    pub url: Url,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DiscordMessage {
    pub message_id: i64,
    pub content: String,
    pub create_at: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub url: Url,
}
