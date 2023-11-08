use crate::interaction_request::user::User;
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/guild#guild-member-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    // TODO: https://discord.com/developers/docs/topics/permissions#role-object
    // pub roles: Role,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u32,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
}

impl GuildMember {
    pub fn expect_user(&self) -> Result<&User, crate::error::InteractionError> {
        match &self.user {
            Some(user) => Ok(user),
            None => Err(("GuildMember", "member").into()),
        }
    }
}
