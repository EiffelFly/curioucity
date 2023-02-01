use anyhow::bail;
use chrono::Utc;
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

use crate::{
    helper::time::get_edgedb_timestamp_from_2000_micros,
    pb_gen::curioucity::v1alpha as pb_curioucity,
};

use super::ResourceUnion;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FullTag {
    pub id: Uuid,
    pub name: String,
    pub resources: Option<Vec<ResourceUnion>>,
    pub created_timestamp_at_curioucity: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub created_timestamp_at_curioucity: String,
}

#[derive(Debug)]
pub struct CreateTagPayload {
    pub name: String,
}
#[derive(Debug)]
pub struct DeleteTagPayload {
    pub name: String,
}
#[derive(Debug)]
pub struct GetTagPayload {
    pub name: String,
}

#[derive(Debug)]
pub struct ListTagPayload {
    pub page_size: i64,
}

impl FullTag {
    pub async fn create(client: Client, payload: &CreateTagPayload) -> Result<Self, anyhow::Error> {
        let query = "select (
            insert Tag {
                name := <str>$0,
                created_timestamp_at_curioucity := <datetime>$1
            }
        ) {
            id,
            name,
            resources,
            created_timestamp_at_curioucity
        };";

        let created_timestamp_at_curioucity =
            match get_edgedb_timestamp_from_2000_micros(Utc::now().timestamp_micros()) {
                Ok(time) => time,
                Err(error) => {
                    println!("Time is out of range: {:?}", error);
                    bail!("{}", error)
                }
            };

        let response_json = match client
            .query_json(&query, &(&payload.name, &created_timestamp_at_curioucity))
            .await
        {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let full_tags = match serde_json::from_str::<Vec<FullTag>>(response_json.as_ref()) {
            Ok(result) => result,
            Err(error) => {
                println!("Deserialize Error: {}", error);
                bail!("Deserialize Error: {}", error)
            }
        };

        let full_tag = match full_tags.into_iter().nth(0) {
            Some(url) => url,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        Ok(full_tag)
    }

    pub async fn delete(client: Client, payload: &DeleteTagPayload) -> Result<(), anyhow::Error> {
        let query = "delete Tag filter .name = <str>$0";

        let response = client.query_json(&query, &(&payload.name,)).await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
    }

    pub async fn get(
        client: Client,
        payload: &GetTagPayload,
    ) -> Result<Option<Self>, anyhow::Error> {
        let query = "select Tag {
            id, 
            name,
            resources,
            created_timestamp_at_curioucity
        } filter .name = <str>$0";

        let response_json = match client.query_json(&query, &(&payload.name,)).await {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let full_tags = match serde_json::from_str::<Vec<FullTag>>(response_json.as_ref()) {
            Ok(result) => result,
            Err(error) => {
                println!("Deserialize Error: {}", error);
                bail!("Deserialize Error: {}", error)
            }
        };

        if full_tags.is_empty() {
            return Ok(None);
        }

        let full_tag = match full_tags.into_iter().nth(0) {
            Some(url) => url,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        Ok(Some(full_tag))
    }

    pub async fn list(
        client: Client,
        payload: &ListTagPayload,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let query = "select Tag {
            id,
            name,
            resources,
            created_timestamp_at_curioucity
        } order by .name
        limit <int64>$0";

        let response_json = match client.query_json(&query, &(&payload.page_size,)).await {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let full_tags = match serde_json::from_str::<Vec<FullTag>>(response_json.as_ref()) {
            Ok(result) => result,
            Err(error) => {
                println!("Deserialize Error: {}", error);
                bail!("Deserialize Error: {}", error)
            }
        };

        Ok(full_tags)
    }

    pub fn as_pb_type(&self) -> pb_curioucity::FullTag {
        transform_full_tag_to_pb(self)
    }
}

impl From<FullTag> for pb_curioucity::FullTag {
    fn from(value: FullTag) -> Self {
        transform_full_tag_to_pb(&value)
    }
}

fn transform_full_tag_to_pb(value: &FullTag) -> pb_curioucity::FullTag {
    let mut pb_resources: Vec<pb_curioucity::ResourceUnion> = Vec::new();

    match &value.resources {
        Some(resources) => {
            for i in resources {
                pb_resources.push(i.as_pb_type());
            }
        }
        None => {}
    }

    return pb_curioucity::FullTag {
        id: value.id.to_string(),
        name: value.name.clone(),
        resources: pb_resources,
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
    };
}

impl Tag {
    pub fn as_pb_type(&self) -> pb_curioucity::Tag {
        transform_tag_to_pb(self)
    }
}

fn transform_tag_to_pb(value: &Tag) -> pb_curioucity::Tag {
    pb_curioucity::Tag {
        id: value.id.to_string(),
        name: value.name.clone(),
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
    }
}

impl From<Tag> for pb_curioucity::Tag {
    fn from(value: Tag) -> Self {
        transform_tag_to_pb(&value)
    }
}
