use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{CommandOptionType, CommandType};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected to have the message id")
        .resolved
        .as_ref();
    println!("Option: {:#?}", option);
    "Hey, I'm alive!".to_string()
}

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
