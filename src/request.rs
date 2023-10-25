use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DiscordRequest {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub response_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<DiscordMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<DiscordUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<InteractionComponent>>,
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<InteractionOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<InteractionComponent>>,
    // Id              string                 `json:"id"`
    // CustomId        string                 `json:"custom_id"`
    // Name            string                 `json:"name"`
    // Components      []InteractionComponent `json:"components"`
    // Options         []InteractionOption    `json:"options"`
    // InteractionType uint8                  `json:"type"`
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-interaction-data-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionOption {
    #[serde(rename = "type")]
    pub option_type: u8,
    pub name: String,
    pub value: String,
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<InteractionOption>>,
}

// https://discord.com/developers/docs/interactions/message-components
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<InteractionComponent>>,
}

// https://discord.com/developers/docs/resources/guild#guild-member-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DiscordMember {
    pub user: DiscordUser,
}

// https://discord.com/developers/docs/resources/user#user-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::request::DiscordRequest;

    #[test]
    fn test_ping() {
        let contents = fs::read_to_string("dto_payloads/ping_request.json").unwrap();
        let _request: DiscordRequest = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn test_bet_request() {
        let contents = fs::read_to_string("dto_payloads/interaction_request.json").unwrap();
        let _request: DiscordRequest = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn test_bet_modal_request() {
        let contents = fs::read_to_string("dto_payloads/bet_modal_request.json").unwrap();
        let _request: DiscordRequest = serde_json::from_str(&contents).unwrap();
    }
}
