mod api_client;
mod config;
mod error;

use log::{info};
use std::io::{self, Write};
use anyhow::{Context, Result};
use crate::api_client::client::{ApiClient, ChatRequest, Message};
use crate::config::get_config_value;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let api_url = get_config_value("api").context("Failed to load API URL from config")?;
    let model = get_config_value("model").context("Failed to load model from config")?;

    info!("Using model: {}", model);

    let api_client = ApiClient::new(api_url);

    println!("CLI Chatbot active - type 'exit' to quit.");
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        if user_input.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_input.to_string(),
            },
        ];

        let chat_request = ChatRequest {
            model: model.clone(),
            messages,
            stream: true,
        };

        api_client.stream_response(&chat_request).await.context("Error during response streaming")?;
    }

    Ok(())
}
