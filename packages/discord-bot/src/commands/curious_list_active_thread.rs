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
    let thread = guild_id.get_active_threads(http).await;

    match thread {
        Ok(e) => println!("Threads: {:#?}", e),
        Err(err) => println!("Errors: {:#?}", err),
    }

    "Hey, I'm alive!".to_string()
}
