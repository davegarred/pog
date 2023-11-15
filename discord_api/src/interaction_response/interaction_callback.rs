// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure

use crate::interaction_response::{message_flags, Component};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    Message(MessageCallbackData),
    Modal(ModalCallbackData),
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MessageCallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AutocompleteCallbackData {}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-modal
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ModalCallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
}

impl InteractionCallbackData {
    pub fn message_callback(content: Option<String>, components: Vec<Component>) -> Self {
        Self::Message(MessageCallbackData {
            content,
            components,
            flags: None,
        })
    }
    pub fn ephemeral_message_callback(content: Option<String>, components: Vec<Component>) -> Self {
        Self::Message(MessageCallbackData {
            content,
            components,
            flags: Some(message_flags::EPHEMERAL),
        })
    }
    // https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-modal
    pub fn modal_callback_data(custom_id: String, title: &str, components: Vec<Component>) -> Self {
        Self::Modal(ModalCallbackData {
            custom_id: Some(custom_id),
            title: Some(title.to_string()),
            components,
        })
    }
}
