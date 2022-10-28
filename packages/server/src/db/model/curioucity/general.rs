use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

use crate::db::model::third_party::{
    discord::{DiscordGuild, DiscordMessage, DiscordThread},
    website::Website,
};

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[edgedb(json)]
pub enum ResourceType {
    DiscordGuild,
    DiscordThread,
    DiscordMessage,
    Website,
}

impl ResourceType {
    pub fn get_str(&self) -> String {
        match self {
            ResourceType::DiscordGuild => "DiscordGuild".to_string(),
            ResourceType::DiscordThread => "DiscordThread".to_string(),
            ResourceType::DiscordMessage => "DiscordMessage".to_string(),
            ResourceType::Website => "Website".to_string(),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[edgedb(json)]
pub enum ResourceUnion {
    DiscordGuild(DiscordGuild),
    DiscordThread(DiscordThread),
    DiscordMessage(DiscordMessage),
    Website(Website),
}
