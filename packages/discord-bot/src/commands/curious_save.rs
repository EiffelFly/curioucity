use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::http::HttpBuilder;
use serenity::model::channel::ChannelType;
use serenity::model::prelude::command::CommandType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::{ChannelId, MessageId};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("curious-save")
        .kind(CommandType::ChatInput)
        .description(
            "To save specific thread you need to input the message id that open the thread",
        )
}

pub async fn run(_options: &[CommandDataOption], channel_id: ChannelId) -> String {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = HttpBuilder::new(token).ratelimiter_disabled(true).build();
    let channel_res = http.get_channel(*channel_id.as_u64()).await;

    match channel_res {
        Ok(channel) => {
            let guild = channel.guild().unwrap();
            let kind = guild.kind;
            let last_message_id = guild.last_message_id.unwrap();
            match kind {
                ChannelType::NewsThread => {
                    return save_thread_message(channel_id, last_message_id).await
                }
                ChannelType::PrivateThread => {
                    return save_thread_message(channel_id, last_message_id).await
                }
                ChannelType::PublicThread => {
                    return save_thread_message(channel_id, last_message_id).await
                }
                _ => {
                    return "Opps! You are not in a thread".to_string();
                }
            }
        }
        Err(e) => {
            println!("Error when retrieve channel info: {:#?}", e);
            return "Opps! Something went wrong".to_string();
        }
    }
}

async fn save_thread_message(channel_id: ChannelId, last_message_id: MessageId) -> String {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = HttpBuilder::new(token).ratelimiter_disabled(true).build();
    let messages_res = channel_id
        .messages(&http, |retriever| {
            retriever.before(last_message_id).limit(25)
        })
        .await;

    match messages_res {
        Ok(messages) => {
            println!("Messages: {:#?}", messages);
            return "✨ We are processing your data, seat tight, it will be a fast journey"
                .to_string();
        }
        Err(e) => {
            println!("Error when retrieve messages: {:#?}", e);
            return "Opps! Something went wrong".to_string();
        }
    }
}
