use anyhow::bail;
use edgedb_derive::Queryable;
use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

pub struct CreateTagPayload {
    name: String,
}

pub struct DeleteTagPayload {
    name: String,
}

pub struct GetTagPayload {
    name: String,
}

impl Tag {
    pub async fn create(client: Client, payload: &CreateTagPayload) -> Result<Self, anyhow::Error> {
        let query = "select (
            insert Tag {
                name := <str>$0,
            }
        ) {
            id,
            name
        };";

        let response = client.query_json(&query, &(&payload.name,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<Tag>>(json.as_ref()).unwrap();
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
            name
        } filter .name = <str>$0";

        let response = client.query_json(&query, &(&payload.name,)).await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<Tag>>(json.as_ref()).unwrap();

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
}
