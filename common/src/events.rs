use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscordMessage {
    Create(CreateMessage),
    Update(UpdateMessage),
    Delete(DeleteMessage),
    TlDr(TlDrMessage),
}

// https://discord.com/developers/docs/resources/channel#create-message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateMessage {
    pub authorization: Authorization,
    pub channel_id: String,
    pub message: String,
    pub message_reference: Option<MessageReference>,
}

// https://discord.com/developers/docs/resources/channel#message-reference-object-message-reference-structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReference {
    pub message_id: String,
    pub channel_id: String,
}

impl CreateMessage {
    pub fn url(&self) -> String {
        create_message_url(self.channel_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMessage {
    pub authorization: Authorization,
    pub message_id: String,
    pub request_token: String,
}

impl DeleteMessage {
    pub fn url(&self) -> String {
        modify_message_url(&self.authorization, &self.request_token, &self.message_id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateMessage {
    pub authorization: Authorization,
    pub message_id: String,
    pub request_token: String,
    pub message: String,
}

impl UpdateMessage {
    pub fn url(&self) -> String {
        modify_message_url(&self.authorization, &self.request_token, &self.message_id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TlDrMessage {
    pub authorization: Authorization,
    pub original_message_id: String,
    pub channel_id: String,
    pub gemini_key: String,
    pub author: String,
    pub message: String,
}

fn modify_message_url(
    authorization: &Authorization,
    request_token: &str,
    message_id: &str,
) -> String {
    format!(
        "https://discord.com/api/v10/webhooks/{}/{}/messages/{}",
        authorization.application_id, request_token, message_id
    )
}

fn create_message_url(channel_id: &str) -> String {
    format!("https://discord.com/api/v10/channels/{channel_id}/messages")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    pub application_id: String,
    pub application_token: String,
}

impl Authorization {
    pub fn auth_header(&self) -> String {
        format!("Bot {}", self.application_token)
    }
}

#[cfg(test)]
mod test {
    use crate::{Authorization, DeleteMessage, UpdateMessage};
    use std::fs;

    #[test]
    fn serialization_delete() {
        let msg = sample_delete_message();
        let ser = serde_json::to_string(&msg).unwrap();
        let expected = fs::read_to_string("dto_payloads/delete_message.json").unwrap();
        assert_eq!(ser, expected);
        let des: DeleteMessage = serde_json::from_str(&ser).unwrap();
        assert_eq!(des, msg);
    }

    #[test]
    fn serialization_update() {
        let msg = sample_update_message();
        let ser = serde_json::to_string(&msg).unwrap();
        let expected = fs::read_to_string("dto_payloads/update_message.json").unwrap();
        assert_eq!(ser, expected);
        let des: UpdateMessage = serde_json::from_str(&ser).unwrap();
        assert_eq!(des, msg);
    }

    #[test]
    fn auth_header() {
        let msg = sample_delete_message();
        let found = msg.authorization.auth_header();
        assert_eq!(
            "Bot XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            found
        );
    }

    #[test]
    fn url_delete() {
        let msg = sample_delete_message();
        let url = msg.url();
        assert_eq!("https://discord.com/api/v10/webhooks/1111111111111111111/aW50ZXJhY3Rpb246MTE3MDAwNTUzNzUyMjQ2Njk0ODpsNWEwYjJPdlh4blQ0VFFZZmpoVzc5Y1h1aEIxdGFaeWxORVJmMDBwZjFJNUZucUpsNlNwV1hDallpNVlKNXV6TnpjeTg1NW1wQlI2dmFQT0lad2dCdzRLMWpYVW90VUo2V3VQcDZtRHdvbmNVTG9hQ0l6aE5hc0NOaFlwcjdPNw/messages/1170005526755688611", url);
    }

    #[test]
    fn url_update() {
        let msg = sample_update_message();
        let url = msg.url();
        assert_eq!("https://discord.com/api/v10/webhooks/1111111111111111111/aW50ZXJhY3Rpb246MTE3MDAwNTUzNzUyMjQ2Njk0ODpsNWEwYjJPdlh4blQ0VFFZZmpoVzc5Y1h1aEIxdGFaeWxORVJmMDBwZjFJNUZucUpsNlNwV1hDallpNVlKNXV6TnpjeTg1NW1wQlI2dmFQT0lad2dCdzRLMWpYVW90VUo2V3VQcDZtRHdvbmNVTG9hQ0l6aE5hc0NOaFlwcjdPNw/messages/1170005526755688611", url);
    }

    fn sample_delete_message() -> DeleteMessage {
        DeleteMessage {
            authorization: Authorization {
                application_id: "1111111111111111111".to_string(),
                application_token: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
            },
            message_id: "1170005526755688611".to_string(),
            request_token: "aW50ZXJhY3Rpb246MTE3MDAwNTUzNzUyMjQ2Njk0ODpsNWEwYjJPdlh4blQ0VFFZZmpoVzc5Y1h1aEIxdGFaeWxORVJmMDBwZjFJNUZucUpsNlNwV1hDallpNVlKNXV6TnpjeTg1NW1wQlI2dmFQT0lad2dCdzRLMWpYVW90VUo2V3VQcDZtRHdvbmNVTG9hQ0l6aE5hc0NOaFlwcjdPNw".to_string(),
        }
    }
    fn sample_update_message() -> UpdateMessage {
        UpdateMessage {
            authorization: Authorization {
                application_id: "1111111111111111111".to_string(),
                application_token: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
            },
            message_id: "1170005526755688611".to_string(),
            request_token: "aW50ZXJhY3Rpb246MTE3MDAwNTUzNzUyMjQ2Njk0ODpsNWEwYjJPdlh4blQ0VFFZZmpoVzc5Y1h1aEIxdGFaeWxORVJmMDBwZjFJNUZucUpsNlNwV1hDallpNVlKNXV6TnpjeTg1NW1wQlI2dmFQT0lad2dCdzRLMWpYVW90VUo2V3VQcDZtRHdvbmNVTG9hQ0l6aE5hc0NOaFlwcjdPNw".to_string(),
            message: "some message".to_string(),
        }
    }
}
