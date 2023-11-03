use crate::wager::Wager;
use crate::ADD_BET_PLACEHOLDER_TEXT;
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DiscordResponse {
    #[serde(rename = "type")]
    pub response_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionComponent>,
}

impl DiscordResponse {
    pub fn str_response(&self) -> axum::http::Response<String> {
        let payload = serde_json::to_string(self).unwrap();
        println!("response: {}", payload);
        match axum::response::Response::builder()
            .status(axum::http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(payload)
        {
            Ok(result) => result,
            Err(err) => {
                println!("error building response from discord response");
                panic!("{}", err)
            }
        }
    }
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<SelectMenuOption>,

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

// https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectMenuOption {
    pub label: String,
    pub value: String,
    pub description: String,
}

pub fn ping_response() -> DiscordResponse {
    DiscordResponse {
        response_type: 1,
        data: None,
    }
}

pub fn message_response<T: ToString>(content: T) -> DiscordResponse {
    select_response(content, vec![])
}

pub fn select_response<T: ToString>(
    content: T,
    components: Vec<InteractionComponent>,
) -> DiscordResponse {
    DiscordResponse {
        response_type: 4,
        data: Some(InteractionComponent {
            content: Some(content.to_string()),
            custom_id: None,
            title: None,
            components,
            options: vec![],
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
        options: vec![],
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
        options: vec![],
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
            options: vec![],
            label: None,
            placeholder: None,
            style: None,
            min_length: None,
            max_length: None,
        }),
    }
}

// pub fn open_select_closing_reason_choices() -> DiscordResponse {
//     let options = vec![
//         SelectMenuOption {
//             label: "Paid".to_string(),
//             value: "paid".to_string(),
//             description: "This bet was paid out".to_string(),
//         },
//         SelectMenuOption {
//             label: "No Bet".to_string(),
//             value: "nobet".to_string(),
//             description: "Push or the bet predicate never happened".to_string(),
//         },
//         SelectMenuOption {
//             label: "Cancel".to_string(),
//             value: "cancel".to_string(),
//             description: "This bet doesn't exist".to_string(),
//         },
//     ];
//     let close_reason = select_choice_component("reason", "Why is the bet closing?", options);
//     select_response("Close out a bet", vec![action_row(close_reason)])
// }

fn select_choice_component(
    custom_id: &str,
    placeholder: &str,
    options: Vec<SelectMenuOption>,
) -> InteractionComponent {
    InteractionComponent {
        response_type: Some(3),
        content: None,
        custom_id: Some(custom_id.to_string()),
        title: None,
        components: vec![],
        options,
        label: None,
        placeholder: Some(placeholder.to_string()),
        style: None,
        min_length: None,
        max_length: None,
    }
}

pub fn open_select_wager_for_close_choices(wagers: Vec<Wager>) -> DiscordResponse {
    let mut options: Vec<SelectMenuOption> = Default::default();
    for wager in wagers {
        let value = format!("{}", wager.wager_id);
        // let value = format!("{}", wager.wager_id);
        let description = wager.to_string();
        options.push(SelectMenuOption {
            label: value.clone(),
            value,
            description,
        });
    }
    let close_bet = select_choice_component("bet", "Close which bet?", options);
    select_response("Close out a bet", vec![action_row(close_bet)])
}

pub fn action_row(modal: InteractionComponent) -> InteractionComponent {
    InteractionComponent {
        response_type: Some(1),
        content: None,
        custom_id: None,
        title: None,
        components: vec![modal],
        options: vec![],
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
        assert_eq!(
            &response,
            r#"{"type":9,"data":{"custom_id":"Woody","title":"Place a bet","components":[{"type":1,"components":[{"type":4,"custom_id":"wager","label":"How much are we wagering?","placeholder":"$20","style":1,"min_length":2,"max_length":10}]},{"type":1,"components":[{"type":4,"custom_id":"outcome","label":"What is the bet on?","placeholder":"Jets beat the Chargers outright","style":2,"min_length":3,"max_length":100}]}]}}"#
        )
    }
}
