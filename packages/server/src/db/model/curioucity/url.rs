use anyhow::bail;
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

use crate::db::model::curioucity::general::{ResourceType, ResourceUnion};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Url {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<Url>,
    pub resource_type: ResourceType,
    pub resource: Option<ResourceUnion>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUrlPayload {
    pub url: String,
    pub resource_type: ResourceType,
}

impl Url {
    pub async fn create(client: Client, payload: &CreateUrlPayload) -> Result<Self, anyhow::Error> {
        let query = "select (
            insert Url {
                url := <str>$0,
                resource_type := <str>$1
            }
        ) {
            id,
            url,
            references,
            resource_type,
            resource
        };";

        let response = client
            .query_json(&query, &(&payload.url, &payload.resource_type.get_str()))
            .await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<Url>>(json.as_ref()).unwrap();
                Ok(result.into_iter().nth(0).unwrap())
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
    }
}
