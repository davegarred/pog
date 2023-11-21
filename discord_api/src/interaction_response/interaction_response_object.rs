use crate::interaction_response::allowed_mention::AllowedMention;
use crate::interaction_response::{Component, InteractionCallbackData};
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub response_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData>,
}

impl InteractionResponse {
    pub fn ping_response() -> InteractionResponse {
        InteractionResponse {
            response_type: 1,
            data: None,
        }
    }

    pub fn simple_message(content: String) -> InteractionResponse {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData::message_callback(
                Some(content),
                vec![],
            )),
        }
    }
    pub fn channel_message_with_source(
        content: &str,
        components: Vec<Component>,
    ) -> InteractionResponse {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData::message_callback(
                Some(content.to_string()),
                components,
            )),
        }
    }
    pub fn channel_message_with_source_ephemeral(
        content: &str,
        components: Vec<Component>,
        allowed_mentions: Vec<AllowedMention>,
    ) -> InteractionResponse {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData::ephemeral_message_callback(
                Some(content.to_string()),
                components,
                allowed_mentions,
            )),
        }
    }

    pub fn select_response(content: String, components: Vec<Component>) -> InteractionResponse {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData::ephemeral_message_callback(
                Some(content),
                components,
                vec![],
            )),
        }
    }

    pub fn modal(modal_collback_data: InteractionCallbackData) -> Self {
        Self {
            response_type: 9,
            data: Some(modal_collback_data),
        }
    }
}

impl From<&str> for InteractionResponse {
    fn from(value: &str) -> Self {
        InteractionResponse::simple_message(value.to_string())
    }
}

impl From<String> for InteractionResponse {
    fn from(value: String) -> Self {
        InteractionResponse::simple_message(value)
    }
}
