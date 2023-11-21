use serde::{Deserialize, Serialize};

/// https://discord.com/developers/docs/resources/channel#allowed-mentions-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AllowedMention {
    pub parse: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
    #[serde(skip_serializing_if = "crate::interaction_response::is_false")]
    pub replied_user: bool,
}

impl AllowedMention {
    pub fn silence_all() -> Self {
        Self {
            parse: vec![],
            roles: vec![],
            users: vec![],
            replied_user: false,
        }
    }
    pub fn parse(parse: Vec<String>) -> Self {
        Self {
            parse,
            roles: vec![],
            users: vec![],
            replied_user: false,
        }
    }
}
