use anyhow::bail;
use edgedb_derive::Queryable;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

use super::ResourceUnion;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FullTag {
    pub id: String,
    pub name: String,
    pub resources: Option<Vec<ResourceUnion>>,
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
    page_size: i64,
}

impl FullTag {
    pub async fn create(client: Client, payload: &CreateTagPayload) -> Result<Self, anyhow::Error> {
        let query = "select (
            insert Tag {
                name := <str>$0,
            }
        ) {
            id,
            name,
            resources
        };";

        let response = client.query_json(&query, &(&payload.name,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<FullTag>>(json.as_ref()).unwrap();
                Ok(result.into_iter().nth(0).unwrap())
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
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
            resources
        } filter .name = <str>$0";

        let response = client.query_json(&query, &(&payload.name,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<FullTag>>(json.as_ref()).unwrap();

                if result.is_empty() {
                    Ok(None)
                } else {
                    let url = result.into_iter().nth(0).unwrap();
                    Ok(Some(url))
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
    }

    pub async fn list(
        client: Client,
        payload: &ListTagPayload,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let query = "select Tag {
            id,
            name,
            resources
        } order by .name
        limit <i64>$0";

        let response = client.query_json(&query, &(&payload.page_size,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<FullTag>>(json.as_ref()).unwrap();
                Ok(result)
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
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
        id: value.id.clone(),
        name: value.name.clone(),
        resources: pb_resources,
    };
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

impl Tag {
    pub fn as_pb_type(&self) -> pb_curioucity::Tag {
        transform_tag_to_pb(self)
    }
}

fn transform_tag_to_pb(value: &Tag) -> pb_curioucity::Tag {
    pb_curioucity::Tag {
        id: value.id.clone(),
        name: value.name.clone(),
    }
}

impl From<Tag> for pb_curioucity::Tag {
    fn from(value: Tag) -> Self {
        transform_tag_to_pb(&value)
    }
}
