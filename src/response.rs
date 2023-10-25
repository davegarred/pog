use serde::{Deserialize, Serialize};
use crate::ADD_BET_PLACEHOLDER_TEXT;

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DiscordResponse {
    #[serde(rename = "type")]
    pub response_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionComponent>,
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionComponent {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<InteractionComponent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
}

pub fn ping_response() -> DiscordResponse {
    DiscordResponse {
        response_type: 1,
        data: None,
    }
}

pub fn message_response<T: ToString>(message: T) -> DiscordResponse {
    DiscordResponse {
        response_type: 4,
        data: Some(InteractionComponent {
            content: Some(message.to_string()),
            custom_id: None,
            title: None,
            components: vec![],
            label: None,
            placeholder: None,
            style: None,
            min_length: None,
            response_type: None,
            max_length: None,
        }),
    }
}

pub fn open_buy_modal<T: ToString>(accepting: T) -> DiscordResponse {
    let wager_modal = InteractionComponent {
        response_type: Some(4),
        content: None,
        custom_id: Some("wager".to_string()),
        title: None,
        components: vec![],
        label: Some("How much are we wagering?".to_string()),
        placeholder: Some("$20".to_string()),
        style: Some(1),
        min_length: Some(2),
        max_length: Some(10),
    };
    let outcome_modal = InteractionComponent {
        response_type: Some(4),
        content: None,
        custom_id: Some("outcome".to_string()),
        title: None,
        components: vec![],
        label: Some("What is the bet on?".to_string()),
        placeholder: Some(ADD_BET_PLACEHOLDER_TEXT.to_string()),
        style: Some(2),
        min_length: Some(3),
        max_length: Some(100),
    };
    DiscordResponse {
        response_type: 9,
        data: Some(InteractionComponent {
            response_type: None,
            content: None,
            custom_id: Some(accepting.to_string()),
            title: Some("Place a bet".to_string()),
            components: vec![action_row(wager_modal), action_row(outcome_modal)],
            label: None,
            placeholder: None,
            style: None,
            min_length: None,
            max_length: None,
        }),
    }
}

pub fn action_row(modal: InteractionComponent) -> InteractionComponent {
    InteractionComponent {
        response_type: Some(1),
        content: None,
        custom_id: None,
        title: None,
        components: vec![modal],
        label: None,
        placeholder: None,
        style: None,
        min_length: None,
        max_length: None,
    }
}
#[cfg(test)]
mod test {
    use crate::response::{message_response, open_buy_modal, ping_response};

    #[test]
    fn test_ping() {
        let response = serde_json::to_string(&ping_response()).unwrap();
        assert_eq!(&response, r#"{"type":1}"#)
    }
    #[test]
    fn test_simple_response_message() {
        let response =
            serde_json::to_string(&message_response("this is a simple message")).unwrap();
        assert_eq!(
            &response,
            r#"{"type":4,"data":{"content":"this is a simple message"}}"#
        )
    }
    #[test]
    fn test_open_buy_modal() {
        let response = serde_json::to_string(&open_buy_modal("Woody")).unwrap();
        println!("{}", response);
        assert_eq!(
            &response,
            r#"{"type":9,"data":{"custom_id":"Woody","title":"Place a bet","components":[{"type":1,"components":[{"type":4,"custom_id":"wager","label":"How much are we wagering?","placeholder":"$20","style":1,"min_length":2,"max_length":10}]},{"type":1,"components":[{"type":4,"custom_id":"outcome","label":"What is the bet on?","placeholder":"Jets beat the Giants this Sunday","style":2,"min_length":3,"max_length":100}]}]}}"#
        )
    }
}
