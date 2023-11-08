use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/user#user-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    pub avatar: String,
}
