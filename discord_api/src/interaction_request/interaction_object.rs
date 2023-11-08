use crate::error::InteractionError;
use crate::interaction_request::guild_member::GuildMember;
use crate::interaction_request::interaction_data::InteractionData;
use crate::interaction_request::message_component::MessageComponent;
use crate::interaction_request::message_object::MessageObject;
use crate::interaction_request::user::User;
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionObject {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub response_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<MessageComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MessageObject>,
    pub token: String,
}

impl InteractionObject {
    pub fn expect_data(&self) -> Result<&InteractionData, InteractionError> {
        match &self.data {
            Some(data) => Ok(data),
            None => Err(("DiscordRequest", "data").into()),
        }
    }
    pub fn expect_message(&self) -> Result<&MessageObject, InteractionError> {
        match &self.message {
            Some(message) => Ok(message),
            None => Err(("DiscordRequest", "message").into()),
        }
    }
    pub fn expect_member(&self) -> Result<&GuildMember, InteractionError> {
        match &self.member {
            Some(member) => Ok(member),
            None => Err(("DiscordRequest", "member").into()),
        }
    }
}
