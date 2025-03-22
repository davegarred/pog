use crate::discord_client::DiscordClient;
use pog_common::{Authorization, DeleteMessage, DiscordMessage};

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
        let delete = DeleteMessage {
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

    async fn set_message(
        &self,
        _message_id: &str,
        _request_token: &str,
        _message: &str,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(feature = "gcp")]
#[derive(Debug, Clone)]
pub struct GcpDefaultDiscordClient {
    authorization: Authorization,
    url: String,
}

#[cfg(feature = "gcp")]
impl GcpDefaultDiscordClient {
    pub async fn new(application_id: String, application_token: String, url: String) -> Self {
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self { authorization, url }
    }

    pub async fn send(&self, message: DiscordMessage) -> Result<(), Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json; charset=UTF-8".parse().unwrap(),
        );
        let url = format!("{}/call", self.url);
        let response = reqwest::Client::new()
            .post(url)
            .headers(headers)
            .json(&message)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let status_code = response.status();
            let msg = format!("failure to call pog client with status {}", status_code);
            println!("client call failed with {}\n{}", status_code, msg);
            Err(Error::ClientFailure(msg))
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
        self.send(message).await
    }

    async fn set_message(
        &self,
        _message_id: &str,
        _request_token: &str,
        _message: &str,
    ) -> Result<(), Error> {
        // TODO: use this or nuke it
        Ok(())
    }
}
