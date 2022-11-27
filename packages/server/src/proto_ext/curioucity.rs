use anyhow::bail;

use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

impl pb_curioucity::ResourceType {
    pub fn as_db_type(num: i32) -> Result<db_curioucity::ResourceType, anyhow::Error> {
        let resource_type = match pb_curioucity::ResourceType::from_i32(num) {
            Some(pb_curioucity::ResourceType::ResourceTypeDiscordguild) => {
                db_curioucity::ResourceType::DiscordGuild
            }
            Some(pb_curioucity::ResourceType::ResourceTypeDiscordthread) => {
                db_curioucity::ResourceType::DiscordThread
            }
            Some(pb_curioucity::ResourceType::ResourceTypeDiscordmessage) => {
                db_curioucity::ResourceType::DiscordMessage
            }
            Some(pb_curioucity::ResourceType::ResourceTypeWebsite) => {
                db_curioucity::ResourceType::Website
            }
            Some(pb_curioucity::ResourceType::ResourceTypeUnspecified) => {
                bail!("Resource type is not specified, you may forget to input this field")
            }
            None => bail!("Resource type is not specified, you may forget to input this field"),
        };

        Ok(resource_type)
    }

    pub fn as_num(&self) -> i32 {
        *self as i32
    }
}

impl From<db_curioucity::ResourceType> for i32 {
    fn from(value: db_curioucity::ResourceType) -> i32 {
        match value {
            db_curioucity::ResourceType::Website => 1,
            db_curioucity::ResourceType::DiscordGuild => 2,
            db_curioucity::ResourceType::DiscordThread => 3,
            db_curioucity::ResourceType::DiscordMessage => 4,
        }
    }
}
