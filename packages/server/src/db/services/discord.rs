pub mod guild {
    use crate::db::source::{curioucity::Tag, discord::DiscordGuild};
    use edgedb_tokio::Client;
    pub struct UpsertDiscordGuildPayload {
        pub guild_id: i64,
        pub name: String,
        pub icon: String,
    }

    pub async fn upsert_discord_guild(
        client: Client,
        payload: &UpsertDiscordGuildPayload,
    ) -> Result<Option<DiscordGuild>, anyhow::Error> {
        let query = "select (
                insert DiscordGuild {
                    guild_id := <int64>$0,
                    name := <str>$1,
                    icon := <str>$2
                }
                unless conflict on .guild_id else DiscordGuild
                else (
                    update DiscordGuild 
                    set {
                        name := <str>$1,
                        icon := <str>$2
                    }
                )
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

    pub struct SelectOrInsertTagToDiscordGuildPayload {
        pub guild_id: i64,
        pub tag: Tag,
    }

    pub async fn select_or_insert_tag_to_discord_guild(
        client: Client,
        payload: &SelectOrInsertTagToDiscordGuildPayload,
    ) -> Result<Option<DiscordGuild>, anyhow::Error> {
        let query = "update DiscordGuild
            filter
                .guild_id := <int64>$0
            set {
                tags += (
                    select (
                        insert Tag {
                            name := <str>$1
                        }
                        unless conflict on .name
                    )
                )
            }
        ";

        let guild = client
            .query_single::<DiscordGuild, (&i64, &str)>(
                &query,
                &(&payload.guild_id, &payload.tag.name),
            )
            .await?;

        Ok(guild)
    }
}
