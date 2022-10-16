pub mod guild {
    use crate::db::source::discord::DiscordGuild;
    use edgedb_tokio::Client;
    pub struct SelectOrInsertDiscordGuildPayload {
        pub guild_id: i64,
        pub name: String,
        pub icon: String,
    }

    pub async fn select_or_insert_discord_guild(
        client: Client,
        payload: &SelectOrInsertDiscordGuildPayload,
    ) -> Result<Option<DiscordGuild>, anyhow::Error> {
        let query = "select (
                insert DiscordGuild {
                    guild_id := <int64>$0,
                    name := <str>$1,
                    icon := <str>$2
                }
                unless conflict on .guild_id else DiscordGuild
            ) {
                id,
                guild_id,
                name,
                icon,
                threads,
                tags
            }";

        let guild = client
            .query_single::<DiscordGuild, (&i64, &str, &str)>(
                &query,
                &(&payload.guild_id, &payload.name, &payload.icon),
            )
            .await?;

        Ok(guild)
    }
}
