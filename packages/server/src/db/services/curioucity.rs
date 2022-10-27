pub mod curioucity {

    use crate::db::source::curioucity::{get_resource_type, ResourceType, Url};
    use anyhow::bail;
    use edgedb_tokio::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct CreateUrlPayload {
        pub url: String,
        pub resource_type: ResourceType,
    }

    pub async fn create_url(
        client: Client,
        payload: &CreateUrlPayload,
    ) -> Result<Url, anyhow::Error> {
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
            .query_json(
                &query,
                &(&payload.url, &get_resource_type(&payload.resource_type)),
            )
            .await;

        match response {
            Ok(json) => {
                let result = serde_json::from_str::<Vec<Url>>(json.as_ref()).unwrap();
                Ok(result.into_iter().nth(0).unwrap())
            }
            Err(error) => {
                println!("Error: {:?}", error);
                bail!("{}", error);
            }
        }
    }
}
