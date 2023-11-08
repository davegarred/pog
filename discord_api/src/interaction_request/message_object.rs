use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/channel#message-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MessageObject {
    pub id: String,
}
