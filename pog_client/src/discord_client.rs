use lambda_runtime::Error;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use pog_common::{Authorization, DeleteMessage, UpdateMessage};

pub async fn delete_message(message: DeleteMessage) -> Result<(), Error> {
    match reqwest::Client::new()
        .delete(message.url())
        .headers(headers(&message.authorization))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("ERROR calling Discord: {}", err);
            Err("".into())
        }
    }
}

pub async fn update_message(message: UpdateMessage) -> Result<(), Error> {
    let discord_request = DiscordRequest::new("");
    match reqwest::Client::new()
        .patch(message.url())
        .headers(headers(&message.authorization))
        .json(&discord_request)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("ERROR calling Discord: {}", err);
            Err("".into())
        }
    }
}

fn headers(authorization: &Authorization) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        authorization.auth_header().parse().unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/json; charset=UTF-8".parse().unwrap(),
    );
    headers.insert(
        "User-Agent",
        "DiscordBot (https://github.com/davegarred/pog_server, 0.1.0)"
            .parse()
            .unwrap(),
    );
    headers
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
