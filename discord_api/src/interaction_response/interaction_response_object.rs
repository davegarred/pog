use crate::interaction_response::{Component, InteractionCallbackData};
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    response_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<InteractionCallbackData>,
}

impl InteractionResponse {
    pub fn ping_response() -> InteractionResponse {
        InteractionResponse {
            response_type: 1,
            data: None,
        }
    }

    pub fn message_response(content: String) -> InteractionResponse {
        Self::select_response(content, vec![])
    }

    pub fn select_response(content: String, components: Vec<Component>) -> InteractionResponse {
        InteractionResponse {
            response_type: 4,
            data: Some(InteractionCallbackData::message_callback(
                Some(content),
                components,
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
        InteractionResponse::message_response(value.to_string())
    }
}

impl From<String> for InteractionResponse {
    fn from(value: String) -> Self {
        InteractionResponse::message_response(value)
    }
}
