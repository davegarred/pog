use crate::discord_client::DiscordClient;
use pog_common::{discord_headers, Authorization, DeleteMessage, DiscordMessage};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::Error;

#[cfg(feature = "aws")]
#[derive(Debug, Clone)]
pub struct AwsDefaultDiscordClient {
    authorization: Authorization,
    client: aws_sdk_lambda::Client,
    client_function_name: String,
}

#[cfg(feature = "aws")]
impl AwsDefaultDiscordClient {
    pub async fn new(
        application_id: String,
        application_token: String,
        client_function_name: String,
    ) -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = aws_sdk_lambda::Client::new(&config);
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self {
            authorization,
            client,
            client_function_name,
        }
    }
}

#[cfg(feature = "aws")]
impl DiscordClient for AwsDefaultDiscordClient {
    async fn delete_message(&self, message_id: &str, request_token: &str) -> Result<(), Error> {
        let delete = pog_common::DeleteMessage {
            authorization: self.authorization.clone(),
            message_id: message_id.to_string(),
            request_token: request_token.to_string(),
        };
        let message = DiscordMessage::Delete(delete);
        let payload = serde_json::to_vec(&message).unwrap();

        let _timer = crate::observe::Timer::new("client_delete_time");
        match self
            .client
            .invoke()
            .function_name(&self.client_function_name)
            .set_invocation_type(Some(aws_sdk_lambda::types::InvocationType::Event))
            .payload(aws_sdk_lambda::primitives::Blob::new(payload.as_slice()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::ClientFailure(format!("found err: {:?}", err))),
        }
    }
}

#[cfg(feature = "gcp")]
#[derive(Debug, Clone)]
pub struct GcpDefaultDiscordClient {
    authorization: Authorization,
    messages: Arc<Mutex<Vec<DiscordMessage>>>,
}

impl GcpDefaultDiscordClient {
    pub async fn start(self) {
        loop {
            let mut messages = self.messages.lock().await;
            for message in messages.drain(..) {
                self.send(message).await;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}

#[cfg(feature = "gcp")]
impl GcpDefaultDiscordClient {
    pub async fn new(application_id: String, application_token: String) -> Self {
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self {
            authorization,
            messages: Arc::new(Default::default()),
        }
    }
    async fn queue(&self, message: DiscordMessage) {
        self.messages.lock().await.push(message);
    }

    async fn send(&self, message: DiscordMessage) {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json; charset=UTF-8".parse().unwrap(),
        );
        let delete_message = match &message {
            DiscordMessage::Delete(delete_message) => delete_message,
            _ => {
                println!("Processing unexpected message: {:?}", message);
                return;
            }
        };
        crate::application::app::counter("delete_discord_message");
        match reqwest::Client::new()
            .delete(delete_message.url())
            .headers(discord_headers(&self.authorization))
            .send()
            .await
        {
            Ok(_) => {}
            Err(err) => {
                println!("ERROR calling Discord: {}", err);
            }
        }
    }
}

#[cfg(feature = "gcp")]
impl DiscordClient for GcpDefaultDiscordClient {
    async fn delete_message(&self, message_id: &str, request_token: &str) -> Result<(), Error> {
        let message = DiscordMessage::Delete(DeleteMessage {
            authorization: self.authorization.clone(),
            message_id: message_id.to_string(),
            request_token: request_token.to_string(),
        });
        self.queue(message).await;
        Ok(())
    }
}
