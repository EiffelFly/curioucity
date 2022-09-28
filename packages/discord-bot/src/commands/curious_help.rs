use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("curious-help")
        .description("A curious bot help panel")
}

pub fn run(options: &[CommandDataOption]) -> String {
    println!("Option: {:#?}", options);
    "Hey, I'm alive!".to_string()
}
