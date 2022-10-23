use serde::Deserialize;

#[derive(edgedb_derive::Queryable, serde::Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

#[derive(edgedb_derive::Queryable, serde::Deserialize, Debug)]
pub struct Url {
    pub url: String,
    pub references: Vec<Url>,
}

#[derive(Deserialize)]
pub enum ResourceType {
    DiscordGuild,
    DiscordThread,
    DiscordMessage,
}

pub fn get_resource_type(resource_type: &ResourceType) -> String {
    match resource_type {
        ResourceType::DiscordGuild => "DiscordGuild".to_string(),
        ResourceType::DiscordThread => "DiscordThread".to_string(),
        ResourceType::DiscordMessage => "DiscordMessage".to_string(),
    }
}
