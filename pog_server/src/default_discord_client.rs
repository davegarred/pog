use async_trait::async_trait;
use aws_sdk_sns::Client;
use pog_common::{Authorization, DeleteMessage, DiscordMessage};

use crate::discord_client::DiscordClient;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct DefaultDiscordClient {
    authorization: Authorization,
    client: Client,
    sns_topic: String,
}

impl DefaultDiscordClient {
    pub async fn new(application_id: String, application_token: String, sns_topic: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_sns::Client::new(&config);
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self {
            authorization,
            client,
            sns_topic,
        }
    }
}

#[async_trait]
impl DiscordClient for DefaultDiscordClient {
    async fn delete_message(&self, message_id: &str, request_token: &str) -> Result<(), Error> {
        let delete = DeleteMessage {
            authorization: self.authorization.clone(),
            message_id: message_id.to_string(),
            request_token: request_token.to_string(),
        };
        let message = DiscordMessage::Delete(delete);
        let payload = serde_json::to_string(&message).unwrap();
        match self
            .client
            .publish()
            .topic_arn(&self.sns_topic)
            .set_message(Some(payload))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::ClientFailure(format!("found err: {:?}", err))),
        }
    }

    async fn set_message(
        &self,
        _message_id: &str,
        _request_token: &str,
        _message: &str,
    ) -> Result<(), Error> {
        Ok(())
    }
}
