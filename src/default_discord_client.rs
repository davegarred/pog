use async_trait::async_trait;
use reqwest::header::HeaderMap;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone)]
pub struct DefaultDiscordClient {
    application_id: String,
    headers: HeaderMap,
}

impl DefaultDiscordClient {
    pub fn new(application_id: String, application_token: String) -> Self {
        let mut headers = HeaderMap::new();
        let authorization = format!("Bot {}", application_token);
        headers.insert("Authorization", authorization.parse().unwrap());
        headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());
        headers.insert("User-Agent", "DiscordBot (https://github.com/davegarred/pog, 0.1.0)".parse().unwrap());
        Self {
            application_id,
            headers,
        }
    }
}

#[async_trait]
impl DiscordClient for DefaultDiscordClient {
    async fn set_message(&self, message_id: &str, request_token: &str, message: &str) -> Result<(), Error> {
        let endpoint = format!("https://discord.com/api/v10/webhooks/{}/{}/messages/{}", self.application_id, request_token, message_id);
        let discord_request = DiscordRequest::new(message);
        reqwest::Client::new()
            .patch(endpoint)
            .headers(self.headers.clone())
            .json(&discord_request)
            .send()
            .await
            .map(|_|Ok(()))
            .map_err(|e|Error::ClientFailure(e.to_string()))?
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
struct DiscordRequest {
    content: String,
    components: Vec<String>,
}

impl DiscordRequest {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            components: vec![],
        }
    }
}
