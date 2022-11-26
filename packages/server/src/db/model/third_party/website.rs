use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;
use crate::pb_gen::third_party::v1alpha as pb_third_party;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Website {
    pub id: String,
    pub content: String,
    pub tags: Vec<db_curioucity::Tag>,
    pub url: db_curioucity::Url,
}

impl From<Website> for pb_third_party::Website {
    fn from(value: Website) -> Self {
        transform_website_to_pb(&value)
    }
}

impl Website {
    pub fn as_pb_type(&self) -> pb_third_party::Website {
        transform_website_to_pb(self)
    }
}

fn transform_website_to_pb(value: &Website) -> pb_third_party::Website {
    let mut pb_tags: Vec<pb_curioucity::Tag> = Vec::new();

    for i in &value.tags {
        pb_tags.push(i.as_pb_type());
    }

    pb_third_party::Website {
        id: value.id.clone(),
        content: value.content.clone(),
        tags: pb_tags,
        url: Some(value.url.as_pb_type()),
    }
}
