use std::io::Write;

use reqwest::{Client};
use serde::{Deserialize, Serialize};
use futures::StreamExt;
use log::{error};
use serde_json::Value;
use crate::error::AppError;

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

    pub async fn get_response(&self, chat_request: &ChatRequest) -> Result<reqwest::Response, AppError> {
        self.client
            .post(&self.api_url)
            .json(chat_request)
            .send()
            .await
            .map_err(|e| AppError::ApiRequestError(e.to_string()))
    }

    pub async fn stream_response(&self, chat_request: &ChatRequest) -> Result<(), AppError> {
        let response = self.get_response(chat_request).await?;

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
                                        std::io::stdout().flush()?;
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
                    return Err(AppError::UnexpectedError("Error streaming response".to_string()));
                }
            }
        }

        println!();
        Ok(())
    }
}
