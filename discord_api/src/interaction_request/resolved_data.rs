use crate::interaction_request::user::User;
use crate::InteractionError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ResolvedData {
    pub users: HashMap<String, User>,
}

impl ResolvedData {
    pub fn expect_user(&self, id: &str) -> Result<&User, InteractionError> {
        match self.users.get(id) {
            Some(user) => Ok(user),
            None => Err(("ResolvedData", format!("user: {}", id).as_str()).into()),
        }
    }
}
