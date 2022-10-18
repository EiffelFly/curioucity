pub mod curioucity {
    use crate::db::source::curioucity::Url;
    use edgedb_tokio::Client;

    pub struct InsertOrSelectUrlPayload {
        url: String,
    }

    pub async fn insert_or_select_url(
        client: Client,
        payload: &InsertOrSelectUrlPayload,
    ) -> Result<Option<Url>, anyhow::Error> {
        let query = "select (
			insert Url {
				url := <str>$0
			}
			unless conflict on .url
      	) {
			url,
			references
	  	}";

        let url = client
            .query_single::<Url, (&str,)>(&query, &(&payload.url,))
            .await?;

        Ok(url)
    }
}
