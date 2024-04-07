use crate::error::Error;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::types::InvocationType;
use pog_common::{Authorization, DiscordMessage, TlDrMessage};

#[async_trait::async_trait]
pub trait DiscordClient: std::fmt::Debug {
    async fn tldr(
        &self,
        channel_id: &str,
        original_message_id: &str,
        author: &str,
        message: &str,
    ) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct DefaultDiscordClient {
    authorization: Authorization,
    gemini_token: String,
    client: aws_sdk_lambda::Client,
    client_function_name: String,
}

impl DefaultDiscordClient {
    pub async fn new(
        application_id: String,
        application_token: String,
        gemini_token: String,
        client_function_name: String,
    ) -> Self {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_lambda::Client::new(&config);
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self {
            authorization,
            client,
            client_function_name,
            gemini_token,
        }
    }
}

#[async_trait::async_trait]
impl DiscordClient for DefaultDiscordClient {
    async fn tldr(
        &self,
        channel_id: &str,
        original_message_id: &str,
        author: &str,
        message: &str,
    ) -> Result<(), Error> {
        let tldr = TlDrMessage {
            authorization: self.authorization.clone(),
            original_message_id: original_message_id.to_string(),
            channel_id: channel_id.to_string(),
            gemini_key: self.gemini_token.clone(),
            author: author.to_string(),
            message: message.to_string(),
        };
        let message = DiscordMessage::TlDr(tldr);
        let payload = serde_json::to_vec(&message).expect("serialize a tldr payload");

        match self
            .client
            .invoke()
            .function_name(&self.client_function_name)
            .set_invocation_type(Some(InvocationType::Event))
            .payload(Blob::new(payload.as_slice()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::ClientFailure(format!("found err: {:?}", err))),
        }
    }
}
