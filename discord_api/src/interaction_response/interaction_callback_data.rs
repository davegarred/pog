// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure

use crate::interaction_response::allowed_mention::AllowedMention;
use crate::interaction_response::attachment::Attachment;
use crate::interaction_response::embed::Embed;
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
    #[serde(skip_serializing_if = "crate::interaction_response::is_false")]
    pub tts: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_mentions: Vec<AllowedMention>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,
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
            tts: false,
            content,
            embeds: vec![],
            components,
            flags: None,
            allowed_mentions: vec![],
            attachments: vec![],
        })
    }
    pub fn ephemeral_message_callback(
        content: Option<String>,
        components: Vec<Component>,
        allowed_mentions: Vec<AllowedMention>,
    ) -> Self {
        Self::Message(MessageCallbackData {
            tts: false,
            content,
            embeds: vec![],
            components,
            flags: Some(message_flags::EPHEMERAL),
            allowed_mentions,
            attachments: vec![],
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
