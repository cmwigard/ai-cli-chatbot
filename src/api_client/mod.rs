use std::io::Write;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use futures::StreamExt;
use log::{error};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

pub struct ApiClient {
    client: Client,
    api_url: String,
}

impl ApiClient {
    pub fn new(api_url: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create client");

        ApiClient { client, api_url }
    }

    pub async fn get_response(&self, chat_request: &ChatRequest) -> Result<reqwest::Response, reqwest::Error> {
        let response = self.client
            .post(&self.api_url)
            .json(chat_request)
            .send()
            .await?;

        Ok(response)
    }

    pub async fn stream_response(&self, chat_request: &ChatRequest) {
        match self.get_response(chat_request).await {
            Ok(response) => {
                let mut stream = response.bytes_stream();
                let mut buffer = Vec::new();

                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(data) => {
                            buffer.extend_from_slice(&data);
                            let text = String::from_utf8_lossy(&buffer);
                            let parts = text.split("}{");

                            for part in parts {
                                let part = if part.starts_with("{") { part.to_string() } else { format!("{{{}}}", part) };

                                match serde_json::from_str::<Value>(&part) {
                                    Ok(json) => {
                                        if let Some(message_obj) = json.get("message") {
                                            if let Some(content) = message_obj.get("content") {
                                                print!("{}", content.as_str().unwrap_or(""));
                                                std::io::stdout().flush().unwrap();
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        error!("Malformed JSON chunk received: {}", part);
                                    }
                                }
                            }

                            buffer.clear();
                        }
                        Err(err) => {
                            error!("Error streaming response: {}", err);
                            println!("Error streaming response. Please try again.");
                            break;
                        }
                    }
                }
                println!();
            }
            Err(err) => {
                if err.is_timeout() {
                    error!("Error: The request timed out.");
                    println!("The request timed out. Please try again.");
                } else if err.is_connect() {
                    error!("Error: Unable to connect to the server.");
                    println!("Unable to connect to the server. Please check if the model server is running.");
                } else {
                    error!("Error interacting with model: {}", err);
                    println!("Error interacting with model. Please check the model server.");
                }
            }
        }
    }
}
