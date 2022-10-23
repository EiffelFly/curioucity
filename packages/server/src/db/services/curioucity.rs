pub mod curioucity {
    use crate::db::source::curioucity::{get_resource_type, ResourceType, Tag, Url};
    use edgedb_tokio::Client;

    pub struct InsertOrSelectUrlPayload {
        url: String,
        resource_type: ResourceType,
    }

    pub async fn insert_or_select_url(
        client: Client,
        payload: &InsertOrSelectUrlPayload,
    ) -> Result<Option<Url>, anyhow::Error> {
        let query = "select (
			insert Url {
				url := <str>$0
                resource_type := <str>$1
			}
			unless conflict on .url
      	) {
			url,
			references,
            resource_type,
            resource
	  	}";

        let url = client
            .query_single::<Url, (&str, &str)>(
                &query,
                &(&payload.url, &get_resource_type(&payload.resource_type)),
            )
            .await?;

        Ok(url)
    }

    pub struct InsertOrSelectTagPayload {
        name: String,
    }

    pub async fn insert_or_select_tag(
        client: Client,
        payload: &InsertOrSelectTagPayload,
    ) -> Result<Option<Tag>, anyhow::Error> {
        let query = "select (
			insert Tag {
				name := <str>$0
			}
			unless conflict on .name
      	) {
			name,
	  	}";

        let tag = client
            .query_single::<Tag, (&str,)>(&query, &(&payload.name,))
            .await?;

        Ok(tag)
    }
}
