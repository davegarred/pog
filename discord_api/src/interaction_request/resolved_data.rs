use crate::interaction_request::user::User;
use crate::InteractionError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-resolved-data-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ResolvedData {
    pub users: Option<HashMap<String, User>>,
}

impl ResolvedData {
    pub fn expect_user(&self, id: &str) -> Result<&User, InteractionError> {
        if let Some(user) = self.users.as_ref().and_then(|users| users.get(id)) {
            Ok(user)
        } else {
            Err(("ResolvedData", format!("user: {}", id).as_str()).into())
        }
    }
}
