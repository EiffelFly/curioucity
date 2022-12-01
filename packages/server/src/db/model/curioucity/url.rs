use anyhow::bail;
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

use super::ResourceUnion;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SingularUrl {
    pub id: Uuid,
    pub url: String,
    pub resource_type: db_curioucity::ResourceType,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FullUrl {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<SingularUrl>,
    pub resource_type: db_curioucity::ResourceType,
    pub resources: Option<Vec<ResourceUnion>>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUrlPayload {
    pub url: String,
    pub resource_type: db_curioucity::ResourceType,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteUrlPayload {
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetUrlPayload {
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct ListUrlPayload {
    pub page_size: i64,
}

impl FullUrl {
    pub async fn create(client: Client, payload: &CreateUrlPayload) -> Result<Self, anyhow::Error> {
        let query = "select (
            insert Url {
                url := <str>$0,
                resource_type := <str>$1
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
            resource
        };";

        let response = client
            .query_json(&query, &(&payload.url, &payload.resource_type.as_str()))
            .await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<FullUrl>>(json.as_ref()).unwrap();
                Ok(result.into_iter().nth(0).unwrap())
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
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
            resource
        } filter .url = <str>$0";

        let response = client.query_json(&query, &(&payload.url,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<FullUrl>>(json.as_ref()).unwrap();

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
            resource
        } order by .url
        limit <int64>$0";

        let response = client.query_json(&query, &(&payload.page_size,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<FullUrl>>(json.as_ref()).unwrap();
                Ok(result)
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
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
        };
        new_refs.push(singular_url);
    }

    return pb_curioucity::FullUrl {
        id: value.id.to_string(),
        url: value.url.clone(),
        resources: pb_resources,
        resource_type: value.resource_type.as_pb_num(),
        references: new_refs,
    };
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Url {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<SingularUrl>,
    pub resource_type: db_curioucity::ResourceType,
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
        };
        new_refs.push(singular_url);
    }

    return pb_curioucity::Url {
        id: value.id.to_string(),
        url: value.url.clone(),
        resource_type: value.resource_type.as_pb_num(),
        references: new_refs,
    };
}
