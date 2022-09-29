use serenity::{builder::CreateApplicationCommand, http::HttpBuilder, model::prelude::GuildId};
use std::env;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("curious-list-active-thread")
        .description("To list all the active threads in the guild")
}

pub async fn run() -> String {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = HttpBuilder::new(token).ratelimiter_disabled(true).build();
    let guild_id = GuildId(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );
    let threads = guild_id.get_active_threads(http).await;

    match threads {
        Ok(e) => {
            let mut response = String::new();
            response.push_str(
                "✨ Here are the current active threads!
            ",
            );

            for thread in e.threads {
                let channel_id = thread.id;
                response.push_str("- <#");
                response.push_str(&channel_id.to_string());
                response.push_str(">");
                response.push_str(
                    "
                ",
                );
            }
            return response.to_string();
        }
        Err(_) => return "Opps! Something went wrong".to_string(),
    }
}
