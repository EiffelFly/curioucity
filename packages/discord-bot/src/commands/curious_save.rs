use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::http::HttpBuilder;
use serenity::model::id::MessageId;
use serenity::model::prelude::command::{CommandOptionType, CommandType};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("curious-save")
        .kind(CommandType::ChatInput)
        .description(
            "To save specific thread you need to input the message id that open the thread",
        )
        .create_option(|option| {
            option
                .name("thread-id")
                .description("The message ID that open the thread")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn run(options: &[CommandDataOption], channel_id: ChannelId) -> String {
    let option = options
        .get(0)
        .expect("Expected to have the message id")
        .resolved
        .as_ref()
        .expect("Should input thread ID");

    println!("Option: {:#?}", option);
    println!("Channel: {:#?}", channel_id);

    if let CommandDataOptionValue::String(user_input) = option {
        match user_input.parse::<u64>() {
            Ok(message_id_num) => {
                let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
                let message_id = MessageId(message_id_num);
                let http = HttpBuilder::new(token).ratelimiter_disabled(true).build();
                let messages = channel_id
                    .messages(&http, |retriever| retriever.after(message_id).limit(25))
                    .await;
                println!("Messages: {:#?}", messages);
            }
            Err(_) => return "The thread id should be number".to_string(),
        }
    }

    "Hey, I'm alive!".to_string()
}
