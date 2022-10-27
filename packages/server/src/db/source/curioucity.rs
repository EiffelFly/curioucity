use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use serde::{Deserialize, Serialize};

use super::discord::{DiscordGuild, DiscordMessage, DiscordThread};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Url {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<Url>,
    pub resource_type: ResourceType,
    pub resource: Option<ResourceUnion>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[edgedb(json)]
pub enum ResourceUnion {
    DiscordGuild(DiscordGuild),
    DiscordThread(DiscordThread),
    DiscordMessage(DiscordMessage),
    BlogPost(BlogPost),
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[edgedb(json)]
pub enum ResourceType {
    DiscordGuild,
    DiscordThread,
    DiscordMessage,
    BlogPost,
}

pub fn get_resource_type(resource_type: &ResourceType) -> String {
    match resource_type {
        ResourceType::DiscordGuild => "DiscordGuild".to_string(),
        ResourceType::DiscordThread => "DiscordThread".to_string(),
        ResourceType::DiscordMessage => "DiscordMessage".to_string(),
        ResourceType::BlogPost => "BlogPost".to_string(),
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct BlogPost {
    pub content: String,
}
