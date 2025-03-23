use serde::{Deserialize, Serialize};

use crate::error::InteractionError;
use crate::interaction_request::guild_member::GuildMember;
use crate::interaction_request::interaction_data::{InteractionData, InteractionDataPayload};
use crate::interaction_request::message_object::MessageObject;
use crate::interaction_request::user::User;
use crate::interaction_request::Snowflake;

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionObject {
    pub id: Snowflake,
    pub application_id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub data: Option<InteractionDataPayload>,
    pub guild_id: Option<Snowflake>,
    // TODO: https://discord.com/developers/docs/resources/channel#channel-object
    // pub channel: Option<ChannelObject>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub version: u8,
    pub message: Option<MessageObject>,
    pub app_permissions: Option<String>,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    // TODO: https://discord.com/developers/docs/monetization/entitlements#entitlement-object
    // pub entitlements: Option<Vec<Entitlement>>,
}

impl InteractionObject {
    pub fn get_data(&self) -> Result<InteractionData, InteractionError> {
        match &self.data {
            Some(data) => data.transform_data(self.interaction_type),
            None => match self.interaction_type {
                1 => Ok(InteractionData::Ping),
                _ => Err("no interaction data found".into()),
            },
        }
    }

    pub fn expect_member(&self) -> Result<&GuildMember, InteractionError> {
        match &self.member {
            Some(member) => Ok(member),
            None => Err(("DiscordRequest", "member").into()),
        }
    }
}
