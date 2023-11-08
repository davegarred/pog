use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/message-components
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MessageComponent {
    #[serde(rename = "type")]
    pub option_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<MessageComponent>>,
}
