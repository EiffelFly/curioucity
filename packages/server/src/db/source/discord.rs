use std::collections::HashMap;

use super::curioucity::Tag;

#[derive(edgedb_derive::Queryable)]
pub struct DiscordGuild {
    pub guild_id: i64,
    pub name: String,
    pub icon: String,
    pub threads: Vec<DiscordThread>,
    pub tags: Vec<Tag>,
}

#[derive(edgedb_derive::Queryable)]
pub struct DiscordThread {
    pub thread_id: i64,
    #[edgedb(json)]
    pub full_messages_json: HashMap<String, String>,
    pub create_at: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
    pub messages: Vec<DiscordMessage>,
}

#[derive(edgedb_derive::Queryable)]
pub struct DiscordMessage {
    pub message_id: i64,
    pub content: String,
    pub create_at: String,
    pub markdown_content: String,
    pub tags: Vec<Tag>,
}
