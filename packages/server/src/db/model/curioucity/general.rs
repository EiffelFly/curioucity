use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

use crate::db::model::third_party as db_third_party;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[edgedb(json)]
pub enum ResourceType {
    DiscordGuild,
    DiscordThread,
    DiscordMessage,
    Website,
}

impl From<ResourceType> for String {
    fn from(value: ResourceType) -> String {
        match value {
            ResourceType::Website => "Website".to_string(),
            ResourceType::DiscordGuild => "DiscordGuild".to_string(),
            ResourceType::DiscordThread => "DiscordThread".to_string(),
            ResourceType::DiscordMessage => "DiscordMessage".to_string(),
        }
    }
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
    DiscordGuild(db_third_party::discord::DiscordGuild),
    DiscordThread(db_third_party::discord::DiscordThread),
    DiscordMessage(db_third_party::discord::DiscordMessage),
    Website(db_third_party::website::Website),
}

impl From<ResourceUnion> for pb_curioucity::ResourceUnion {
    fn from(value: ResourceUnion) -> Self {
        transform_resource_union_to_pb(&value)
    }
}

impl ResourceUnion {
    pub fn as_pb_type(&self) -> pb_curioucity::ResourceUnion {
        transform_resource_union_to_pb(self)
    }
}

fn transform_resource_union_to_pb(value: &ResourceUnion) -> pb_curioucity::ResourceUnion {
    match value {
        ResourceUnion::DiscordGuild(discord_guild) => pb_curioucity::ResourceUnion {
            resource: Some(pb_curioucity::resource_union::Resource::DiscordGuild(
                discord_guild.as_pb_type(),
            )),
        },
        ResourceUnion::DiscordThread(discord_thread) => pb_curioucity::ResourceUnion {
            resource: Some(pb_curioucity::resource_union::Resource::DiscordThread(
                discord_thread.as_pb_type(),
            )),
        },
        ResourceUnion::DiscordMessage(discord_message) => pb_curioucity::ResourceUnion {
            resource: Some(pb_curioucity::resource_union::Resource::DiscordMessage(
                discord_message.as_pb_type(),
            )),
        },
        ResourceUnion::Website(website) => pb_curioucity::ResourceUnion {
            resource: Some(pb_curioucity::resource_union::Resource::Website(
                website.as_pb_type(),
            )),
        },
    }
}
