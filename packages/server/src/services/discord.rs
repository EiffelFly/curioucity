pub struct SelectOrInsertDiscordGuildPayload {
    pub guild_id: i64,
    pub name: String,
    pub icon: String,
}

pub async fn select_or_insert_discord_guild(
    guild: &SelectOrInsertDiscordGuildPayload,
) -> Result<(), anyhow::Error> {
    let conn = edgedb_tokio::create_client().await?;

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

    let guild = conn
        .query_json(&query, &(&guild.guild_id, &guild.name, &guild.icon))
        .await?;

    println!("Guild: {:?}", guild);

    Ok(())
}
