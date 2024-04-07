use lambda_runtime::Error;
use serde::{Deserialize, Serialize};

use pog_common::{discord_headers, CreateMessage, DeleteMessage, UpdateMessage};

pub async fn delete_message(message: DeleteMessage) -> Result<(), Error> {
    match reqwest::Client::new()
        .delete(message.url())
        .headers(discord_headers(&message.authorization))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("ERROR calling Discord: {}", err);
            Err("unable to delete message".into())
        }
    }
}

pub async fn update_message(message: UpdateMessage) -> Result<(), Error> {
    let discord_request = DiscordRequest::new(&message.message);
    match reqwest::Client::new()
        .patch(message.url())
        .headers(discord_headers(&message.authorization))
        .json(&discord_request)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("ERROR calling Discord: {}", err);
            Err("unable to update message".into())
        }
    }
}

pub async fn create_message(message: CreateMessage) -> Result<(), Error> {
    let discord_request = DiscordRequest::new(&message.message);
    match reqwest::Client::new()
        .post(message.url())
        .headers(discord_headers(&message.authorization))
        .json(&discord_request)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("ERROR calling Discord: {}", err);
            Err("unable to update message".into())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
