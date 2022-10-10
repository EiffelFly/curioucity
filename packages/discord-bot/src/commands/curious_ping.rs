use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("curious-ping").description("Test pong")
}

pub async fn run(_options: &[CommandDataOption]) -> String {
    let backend_host = env::var("BACKEND_HOST").expect("Expected backend host in the environment");
    let backend_port = env::var("BACKEND_PORT").expect("Expected backend port in the environment");
    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "http://{host}:{port}/ping",
            host = backend_host,
            port = backend_port
        ))
        .send()
        .await;

    match resp {
        Ok(response) => {
            println!("Response: {:?}", response.text().await);
        }
        Err(error) => {
            println!("Error: {:?}", error);
            return "Something went wrong".to_string();
        }
    }

    "Hey, I'm alive!".to_string()
}
