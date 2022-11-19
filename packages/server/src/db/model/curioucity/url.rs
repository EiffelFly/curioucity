use anyhow::bail;
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

use crate::db::model::curioucity as db_curioucity;
use crate::pb_gen::curioucity::v1alpha as pb_curioucity;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SingularUrl {
    pub id: Uuid,
    pub url: String,
    pub resource_type: db_curioucity::ResourceType,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Url {
    pub id: Uuid,
    pub url: String,
    pub references: Vec<SingularUrl>,
    pub resource_type: db_curioucity::ResourceType,
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
                let result = serde_json::from_str::<Vec<Url>>(json.as_ref()).unwrap();
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

    pub async fn get(client: Client, payload: &GetUrlPayload) -> Result<(), anyhow::Error> {
        let query = "select Url filter .url = <str>$0";

        let response = client.query_json(&query, &(&payload.url,)).await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error)
            }
        }
    }

    pub fn as_pb_type(&self) -> pb_curioucity::Url {
        let mut new_refs: Vec<pb_curioucity::SingularUrl> = Vec::new();

        for i in &self.references {
            let singular_url = pb_curioucity::SingularUrl {
                id: i.id.to_string(),
                url: i.url.clone(),
                resource_type: i.resource_type.as_pb_num(),
            };
            new_refs.push(singular_url);
        }

        return pb_curioucity::Url {
            id: self.id.to_string(),
            url: self.url.clone(),
            references: new_refs,
            resource_type: self.resource_type.as_pb_num(),
        };
    }
}
