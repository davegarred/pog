use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-interaction-data-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionDataOption {
    #[serde(rename = "type")]
    pub option_type: u8,
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<InteractionDataOption>>,
}
