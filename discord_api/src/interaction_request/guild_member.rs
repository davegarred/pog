use crate::interaction_request::user::User;
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/guild#guild-member-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GuildMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl GuildMember {
    pub fn expect_user(&self) -> Result<&User, crate::error::InteractionError> {
        match &self.user {
            Some(user) => Ok(user),
            None => Err(("GuildMember", "member").into()),
        }
    }
}
