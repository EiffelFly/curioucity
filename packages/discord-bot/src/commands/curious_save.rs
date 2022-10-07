use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::futures::future::BoxFuture;
use serenity::http::{Http, HttpBuilder};
use serenity::model::channel::ChannelType;
use serenity::model::prelude::command::{CommandOptionType, CommandType};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
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
    let http = HttpBuilder::new(token).build();

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
    let message_count = guild_channel.message_count;
    match kind {
        ChannelType::NewsThread => {
            return save_thread_message(http, channel_id, message_count, options).await
        }
        ChannelType::PrivateThread => {
            return save_thread_message(http, channel_id, message_count, options).await
        }
        ChannelType::PublicThread => {
            return save_thread_message(http, channel_id, message_count, options).await
        }
        _ => {
            return "Opps! You are not in a thread".to_string();
        }
    }
}

async fn save_thread_message(
    http: Http,
    channel_id: ChannelId,
    message_count_option: Option<u8>,
    command_options: &[CommandDataOption],
) -> String {
    match command_options.get(0) {
        Some(option) => {
            let mut messages = Vec::new();

            if let CommandDataOptionValue::String(first_message_id) =
                option.resolved.as_ref().expect("Expected string object")
            {
                let parsed_id: u64 = match first_message_id.parse() {
                    Ok(id) => id,
                    Err(error) => {
                        println!("User input id is not number, error: {:?}", error);
                        return "Please input number".to_string();
                    }
                };

                let status = match get_thread_messages_with_first_message_id(
                    http,
                    channel_id,
                    MessageId(parsed_id),
                    None,
                    &mut messages,
                )
                .await
                {
                    Ok(_) => "Success".to_string(),
                    Err(_) => "Failed".to_string(),
                };

                println!("Status: {:?}", status);
                println!("Messages: {:?}", messages);

                return "✨ We are processing your data, seat tight, it will be a fast journey"
                    .to_string();
            } else {
                return "Please input number".to_string();
            }
        }
        None => {
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

            // At the very first time we don't pass in last_message_id
            // Because without last_message_id we will operate the request
            // without retriever.before() strategy. In this way discord will
            // give us the most recent messages.

            let status = match get_thread_messages_with_counts(
                http,
                channel_id,
                None,
                message_count,
                &mut messages,
            )
            .await
            {
                Ok(_) => "Success".to_string(),
                Err(_) => "Failed".to_string(),
            };

            println!("Status: {:?}", status);
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
    last_message_id: Option<MessageId>,
    message_count: u8,
    messages: &mut Vec<Message>,
) -> BoxFuture<'_, Result<String, String>> {
    async move {
        let mut messages_res = match channel_id
            .messages(&http, |retriever| match last_message_id {
                Some(id) => retriever.before(id).limit(5),
                None => retriever.limit(5),
            })
            .await
        {
            Ok(messages) => messages,
            Err(error) => {
                println!("Can't get the messages: {:#?} \n", error);
                return Err("Can't get the messages:".to_string());
            }
        };

        let new_last_message_id = match messages_res.last() {
            Some(message) => message.id,
            None => {
                print!(
                    "Can't retrieve the new last message id, this might be the \
                    end of the messages"
                );
                return Ok("Succeed".to_string());
            }
        };

        messages.append(messages_res.as_mut());

        if messages.len() < message_count.into() {
            return get_thread_messages_with_counts(
                http,
                channel_id,
                Some(new_last_message_id),
                message_count,
                messages,
            )
            .await;
        }

        Ok("Succeed".to_string())
    }
    .boxed()
}

fn get_thread_messages_with_first_message_id(
    http: Http,
    channel_id: ChannelId,
    first_message_id: MessageId,
    last_message_id: Option<MessageId>,
    messages: &mut Vec<Message>,
) -> BoxFuture<'_, Result<String, String>> {
    async move {
        let mut messages_res = match channel_id
            .messages(&http, |retriever| match last_message_id {
                Some(id) => retriever.before(id).limit(5),
                None => retriever.limit(5),
            })
            .await
        {
            Ok(messages) => messages,
            Err(error) => {
                println!("Can't get the messages: {:#?} \n", error);
                return Err("Can't get the messages:".to_string());
            }
        };

        // When retrieve the messages. The first messages of the thread is actually
        // a reference message referenced to the message that open the thread(The id user
        // give us) Although they are the same message on Discord UI but their id is
        // different.

        // In the meanwhile, because the first message we got from this request is a reference
        // message. Its content will be empty, we need to access the true content from
        // referenced_message

        // We check whether the last message of the current message response had
        // referenced_message

        let message_res_last_message = match messages_res.last().cloned() {
            Some(message) => message,
            None => {
                print!(
                    "Can't get the last message of dicord response, there may not have any \
                message exist."
                );
                return Ok("Succeed".to_string());
            }
        };

        //let message_res_last_message = messages_res.last().cloned();

        if let Some(referenced_message) = message_res_last_message.referenced_message {
            // We first pop the message_res tail from the list (This is actually the first message
            // of this thread.)
            messages_res.truncate(messages_res.len() - 1);
            messages.append(messages_res.as_mut());

            if referenced_message.id == first_message_id {
                messages.push(*referenced_message);
                print!("Successfully retrieve all messages");
                return Ok("Succeed".to_string());
            } else {
                messages.push(*referenced_message);
            }
        } else {
            // The last message of the current message response doesn't have referenced_message
            messages.append(messages_res.as_mut());
        }

        return get_thread_messages_with_first_message_id(
            http,
            channel_id,
            first_message_id,
            Some(message_res_last_message.id),
            messages,
        )
        .await;
    }
    .boxed()
}
