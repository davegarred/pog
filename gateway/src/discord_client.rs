use pog_common::{Authorization, DiscordMessage, TlDrMessage};

use crate::error::Error;

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
#[cfg(feature = "aws")]
pub struct AwsDiscordClient {
    authorization: Authorization,
    gemini_token: String,
    client: aws_sdk_lambda::Client,
    client_function_name: String,
}

#[cfg(feature = "aws")]
impl AwsDiscordClient {
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
#[cfg(feature = "aws")]
impl DiscordClient for AwsDiscordClient {
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

#[derive(Debug, Clone)]
#[cfg(feature = "gcp")]
pub struct GcpDiscordClient {
    authorization: Authorization,
    gemini_token: String,
    url: String,
}

#[cfg(feature = "gcp")]
impl GcpDiscordClient {
    pub fn new(
        application_id: String,
        application_token: String,
        gemini_token: String,
        client_function_name: String,
    ) -> Self {
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self {
            authorization,
            gemini_token,
            url: client_function_name,
        }
    }
}

#[async_trait::async_trait]
#[cfg(feature = "gcp")]
impl DiscordClient for GcpDiscordClient {
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
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json; charset=UTF-8".parse().unwrap(),
        );
        let url = format!("{}:8080/call", self.url);
        let response = reqwest::Client::new()
            .post(url)
            .headers(headers)
            .json(&message)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let msg = format!(
                "failure to call pog client with status {}: {}",
                response.status().to_string(),
                response.text().await.unwrap_or(String::new())
            );
            Err(Error::ClientFailure(msg))
        }
    }
}
