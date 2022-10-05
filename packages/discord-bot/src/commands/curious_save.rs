use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::futures::future::BoxFuture;
use serenity::http::{Http, HttpBuilder};
use serenity::model::channel::ChannelType;
use serenity::model::prelude::command::{CommandOptionType, CommandType};
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::{ChannelId, Message, MessageId};
use serenity::FutureExt;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("curious-save")
        .kind(CommandType::ChatInput)
        .description(
            "To save specific thread you need to input the message id that open the thread",
        )
        .create_option(|option| {
            option
                .name("first-message-id")
                .description("This is the first message's id in the current thread.")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(options: &[CommandDataOption], channel_id: ChannelId) -> String {
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    let http = HttpBuilder::new(token).ratelimiter_disabled(true).build();

    let channel = match http.get_channel(*channel_id.as_u64()).await {
        Ok(channel) => channel,
        Err(error) => {
            println!("Error when retrieve channel info: {:#?}", error);
            return "Opps! Something went wrong, Error when retrieve channel \
            info"
                .to_string();
        }
    };

    let guild_channel = match channel.guild() {
        Some(guild_channel) => guild_channel,
        None => {
            println!("Error when retrieve guild channel");
            return "Opps! Something went wrong, Error when retrieve guild \
                        channel"
                .to_string();
        }
    };

    let kind = guild_channel.kind;
    let last_message_id = guild_channel.last_message_id;
    let message_count = guild_channel.message_count;
    let first_message_id = options.get(0);
    match kind {
        ChannelType::NewsThread => {
            return save_thread_message(
                http,
                channel_id,
                last_message_id,
                message_count,
                first_message_id,
            )
            .await
        }
        ChannelType::PrivateThread => {
            return save_thread_message(
                http,
                channel_id,
                last_message_id,
                message_count,
                first_message_id,
            )
            .await
        }
        ChannelType::PublicThread => {
            return save_thread_message(
                http,
                channel_id,
                last_message_id,
                message_count,
                first_message_id,
            )
            .await
        }
        _ => {
            return "Opps! You are not in a thread".to_string();
        }
    }
}

async fn save_thread_message(
    http: Http,
    channel_id: ChannelId,
    last_message_id_option: Option<MessageId>,
    message_count_option: Option<u8>,
    first_message_id_option: Option<&CommandDataOption>,
) -> String {
    match first_message_id_option {
        Some(_id) => {
            return "✨ We are processing your data, seat tight, it will be a fast journey"
                .to_string();
        }
        None => {
            let last_message_id = match last_message_id_option {
                Some(id) => id,
                None => {
                    println!("Error when retrieve last_message_id of the channel");
                    return "Opps! Something went wrong, Error when retrieve \
                    last_message_id of the channel"
                        .to_string();
                }
            };

            let message_count = match message_count_option {
                Some(count) => count,
                None => {
                    println!(
                        "This channel doesn't have proper message_counts, prompt \
                    user to enter the first message id"
                    );
                    return "Opps! this channel doesn't have proper message counts. \
                    To proper save this channel, please try curious_save again, but \
                    this time enter the first-message-id"
                        .to_string();
                }
            };

            let mut messages = Vec::new();

            get_thread_messages_with_counts(
                http,
                channel_id,
                last_message_id,
                message_count,
                &mut messages,
            )
            .await;

            println!("Messages: {:?}", messages);

            return "✨ We are processing your data, seat tight, it will be a fast \
            journey. But as a honest bot I need to inform you that due to the limit \
            of Discord API, we could only get around 255 messages without the help \
            from you. If you need a much more accurate result, please input the \
            first-message-id."
                .to_string();
        }
    }
}

/// We want to get all the messages in the thread.

fn get_thread_messages_with_counts(
    http: Http,
    channel_id: ChannelId,
    last_message_id: MessageId,
    message_count: u8,
    messages: &mut Vec<Message>,
) -> BoxFuture<'_, ()> {
    async move {
        let mut messages_res = match channel_id
            .messages(&http, |retriever| {
                retriever.before(last_message_id).limit(3)
            })
            .await
        {
            Ok(messages) => messages,
            Err(_error) => {
                print!("Something went wrong when try to get the messages");
                return ();
            }
        };

        messages.append(messages_res.as_mut());

        if messages.len() < message_count.into() {
            get_thread_messages_with_counts(
                http,
                channel_id,
                last_message_id,
                message_count,
                messages,
            )
            .await;
        }
    }
    .boxed()
}

// async fn get_thread_messages_with_first_message_id() -> Result<Vec<Message>, Error> {}
