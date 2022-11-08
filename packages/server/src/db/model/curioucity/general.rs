use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

use crate::gen::curioucity::v1alpha as pb_curioucity;

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
    pub fn as_str(&self) -> String {
        match self {
            ResourceType::Website => "Website".to_string(),
            ResourceType::DiscordGuild => "DiscordGuild".to_string(),
            ResourceType::DiscordThread => "DiscordThread".to_string(),
            ResourceType::DiscordMessage => "DiscordMessage".to_string(),
        }
    }

    pub fn as_pb_type(&self) -> pb_curioucity::ResourceType {
        match self {
            ResourceType::Website => pb_curioucity::ResourceType::ResourceTypeWebsite,
            ResourceType::DiscordGuild => pb_curioucity::ResourceType::ResourceTypeDiscordguild,
            ResourceType::DiscordThread => pb_curioucity::ResourceType::ResourceTypeDiscordthread,
            ResourceType::DiscordMessage => pb_curioucity::ResourceType::ResourceTypeDiscordmessage,
        }
    }

    pub fn as_pb_num(&self) -> i32 {
        match self {
            ResourceType::Website => 1,
            ResourceType::DiscordGuild => 2,
            ResourceType::DiscordThread => 3,
            ResourceType::DiscordMessage => 4,
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
