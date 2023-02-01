use anyhow::bail;
use chrono::Utc;
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

use crate::db::model::curioucity as db_curioucity;
use crate::helper::time::get_edgedb_timestamp_from_2000_micros;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

use super::ResourceUnion;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SingularUrl {
    pub id: Uuid,
    pub url: String,
    pub resource_type: db_curioucity::ResourceType,
    pub created_timestamp_at_curioucity: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FullUrl {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<SingularUrl>,
    pub resource_type: db_curioucity::ResourceType,
    pub resources: Option<Vec<ResourceUnion>>,
    pub created_timestamp_at_curioucity: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Url {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<SingularUrl>,
    pub resource_type: db_curioucity::ResourceType,
    pub created_timestamp_at_curioucity: String,
}

pub struct CreateUrlPayload {
    pub url: String,
    pub resource_type: db_curioucity::ResourceType,
}

pub struct DeleteUrlPayload {
    pub url: String,
}

pub struct GetUrlPayload {
    pub url: String,
}

pub struct ListUrlPayload {
    pub page_size: i64,
}

impl FullUrl {
    pub async fn create(
        client: &Client,
        payload: &CreateUrlPayload,
    ) -> Result<Self, anyhow::Error> {
        let query = "select (
            insert Url {
                url := <str>$0,
                resource_type := <str>$1,
                created_timestamp_at_curioucity := <datetime>$2
            }
        ) {
            id,
            url,
            references: {
                id,
                url,
                resource_type
            },
            resource_type,
            resource,
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
            .query_json(
                &query,
                &(
                    &payload.url,
                    &payload.resource_type.as_str(),
                    &created_timestamp_at_curioucity,
                ),
            )
            .await
        {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let full_urls = match serde_json::from_str::<Vec<FullUrl>>(response_json.as_ref()) {
            Ok(result) => result,
            Err(error) => {
                println!("Deserialize Error: {}", error);
                bail!("Deserialize Error: {}", error)
            }
        };

        let full_url = match full_urls.into_iter().nth(0) {
            Some(url) => url,
            None => {
                println!("Deserialize Error, deserialized element not found");
                bail!("Deserialize Error, deserialized element not found")
            }
        };

        return Ok(full_url);
    }

    pub async fn delete(client: Client, payload: &DeleteUrlPayload) -> Result<(), anyhow::Error> {
        let query = "delete Url filter .url = <str>$0";

        let response = client.query_json(&query, &(&payload.url,)).await;

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
        payload: &GetUrlPayload,
    ) -> Result<Option<Self>, anyhow::Error> {
        let query = "select Url {
            id, 
            url,
            references: {
                id,
                url,
                resource_type
            },
            resource_type,
            resource,
            created_timestamp_at_curioucity
        } filter .url = <str>$0";

        let response_json = match client.query_json(&query, &(&payload.url,)).await {
            Ok(json) => json,
            Err(error) => {
                println!("Error occured when query database: {:?}", error);
                bail!("{}", error)
            }
        };

        let full_urls = match serde_json::from_str::<Vec<FullUrl>>(response_json.as_ref()) {
            Ok(result) => result,
            Err(error) => {
                println!("Deserialize Error: {}", error);
                bail!("Deserialize Error: {}", error)
            }
        };

        if full_urls.is_empty() {
            return Ok(None);
        } else {
            let full_url = match full_urls.into_iter().nth(0) {
                Some(url) => url,
                None => {
                    println!("Deserialize Error, deserialized element not found");
                    bail!("Deserialize Error, deserialized element not found")
                }
            };

            return Ok(Some(full_url));
        }
    }

    pub async fn list(
        client: Client,
        payload: &ListUrlPayload,
    ) -> Result<Vec<Self>, anyhow::Error> {
        let query = "select Url {
            id, 
            url,
            references: {
                id,
                url,
                resource_type
            },
            resource_type,
            resource,
            created_timestamp_at_curioucity
        } order by .url
        limit <int64>$0";

        let response_json = match client.query_json(&query, &(&payload.page_size,)).await {
            Ok(json) => json,
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        };

        let full_urls = match serde_json::from_str::<Vec<FullUrl>>(response_json.as_ref()) {
            Ok(result) => result,
            Err(error) => {
                println!("Deserialize Error: {}", error);
                bail!("Deserialize Error: {}", error)
            }
        };

        if full_urls.is_empty() {
            let empty_urls: Vec<FullUrl> = Vec::new();
            return Ok(empty_urls);
        }

        Ok(full_urls)
    }

    pub fn as_pb_type(&self) -> pb_curioucity::FullUrl {
        transform_full_url_to_pb(self)
    }
}

impl From<FullUrl> for pb_curioucity::FullUrl {
    fn from(value: FullUrl) -> Self {
        transform_full_url_to_pb(&value)
    }
}

fn transform_full_url_to_pb(value: &FullUrl) -> pb_curioucity::FullUrl {
    let mut pb_resources: Vec<pb_curioucity::ResourceUnion> = Vec::new();

    match &value.resources {
        Some(resources) => {
            for i in resources {
                pb_resources.push(i.as_pb_type());
            }
        }
        None => {}
    }

    let mut new_refs: Vec<pb_curioucity::SingularUrl> = Vec::new();

    for i in &value.references {
        let singular_url = pb_curioucity::SingularUrl {
            id: i.id.to_string(),
            url: i.url.clone(),
            resource_type: i.resource_type.as_pb_num(),
            created_timestamp_at_curioucity: i.created_timestamp_at_curioucity.clone(),
        };
        new_refs.push(singular_url);
    }

    return pb_curioucity::FullUrl {
        id: value.id.to_string(),
        url: value.url.clone(),
        resources: pb_resources,
        resource_type: value.resource_type.as_pb_num(),
        references: new_refs,
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
    };
}

impl Url {
    pub fn as_pb_type(&self) -> pb_curioucity::Url {
        transform_url_to_pb(self)
    }
}

impl From<Url> for pb_curioucity::Url {
    fn from(value: Url) -> Self {
        transform_url_to_pb(&value)
    }
}

fn transform_url_to_pb(value: &Url) -> pb_curioucity::Url {
    let mut new_refs: Vec<pb_curioucity::SingularUrl> = Vec::new();

    for i in &value.references {
        let singular_url = pb_curioucity::SingularUrl {
            id: i.id.to_string(),
            url: i.url.clone(),
            resource_type: i.resource_type.as_pb_num(),
            created_timestamp_at_curioucity: i.created_timestamp_at_curioucity.clone(),
        };
        new_refs.push(singular_url);
    }

    return pb_curioucity::Url {
        id: value.id.to_string(),
        url: value.url.clone(),
        resource_type: value.resource_type.as_pb_num(),
        references: new_refs,
        created_timestamp_at_curioucity: value.created_timestamp_at_curioucity.clone(),
    };
}
