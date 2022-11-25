use std::collections::HashMap;

use crate::db::model::curioucity::ResourceUnion;
use pbjson_types::{value::Kind, Struct, Value};

use crate::db::model::curioucity as db_curioucity;

impl From<ResourceUnion> for Value {
    fn from(values: ResourceUnion) -> Self {
        match values {
            ResourceUnion::DiscordGuild(DiscordGuild) => {
                let fields: HashMap<String, Value> = HashMap::new();
                fields.insert(
                    "guild_id".to_string(),
                    pbjson_types::Value::from(DiscordGuild.guild_id as f64),
                );

                fields.insert(
                    "name".to_string(),
                    pbjson_types::Value::from(DiscordGuild.name),
                );

                let tags_value: Vec<Value> = Vec::new();

                for i in &DiscordGuild.tags {
                    tags_value.push(pbjson_types::Value::from(i))
                }

                fields.insert("tags".to_string(), pbjson_types::Value::from(tags_value));

                return Value {
                    kind: Some(Kind::StructValue(Struct { fields })),
                };
            }
        }
    }
}

impl From<&db_curioucity::Tag> for Value {
    fn from(value: &db_curioucity::Tag) -> Self {
        let fields: HashMap<String, Value> = HashMap::new();

        fields.insert("name".to_string(), pbjson_types::Value::from(value.name));

        return Value {
            kind: Some(Kind::StructValue(Struct { fields })),
        };
    }
}
