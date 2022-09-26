use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("curious_save")
        .description(
            "To save specific thread you need to input the message id that open the thread",
        )
        .create_option(|option| {
            option
                .name("Thread ID")
                .description("The message ID that open the thread")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
