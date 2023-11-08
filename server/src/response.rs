use discord_api::interaction_response::{
    Component, InteractionCallbackData, InteractionResponse, SelectMenuOption,
};

use crate::wager::Wager;
use crate::ADD_BET_PLACEHOLDER_TEXT;

pub fn open_buy_modal(accepting: String) -> InteractionResponse {
    let wager_modal = Component::modal_item(
        "wager",
        "How much are we wagering?",
        "$20",
        1,
        Some(2),
        Some(10),
    );
    let outcome_modal = Component::modal_item(
        "outcome",
        "What is the bet on?",
        ADD_BET_PLACEHOLDER_TEXT,
        2,
        Some(3),
        Some(100),
    );
    let modal_component = InteractionCallbackData::modal_callback_data(
        accepting,
        "Place a bet",
        vec![
            Component::action_row(wager_modal),
            Component::action_row(outcome_modal),
        ],
    );
    InteractionResponse::modal(modal_component)
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

pub fn open_select_wager_for_close_choices(wagers: Vec<Wager>) -> InteractionResponse {
    let mut options: Vec<SelectMenuOption> = Default::default();
    for wager in wagers {
        let value = format!("{}", wager.wager_id);
        // let value = format!("{}", wager.wager_id);
        let description = wager.to_string();
        options.push(SelectMenuOption::new(value.clone(), value, description));
    }
    let close_bet = Component::select_choice_component("bet", "Close which bet?", options);
    InteractionResponse::select_response(
        "Close out a bet".to_string(),
        vec![Component::action_row(close_bet)],
    )
}

#[cfg(test)]
mod test {
    use crate::response::open_buy_modal;
    use discord_api::interaction_response::InteractionResponse;

    #[test]
    fn test_ping() {
        let response = serde_json::to_string(&InteractionResponse::ping_response()).unwrap();
        assert_eq!(&response, r#"{"type":1}"#)
    }

    #[test]
    fn test_simple_response_message() {
        let response: InteractionResponse = "this is a simple message".into();
        let response = serde_json::to_string(&response).unwrap();
        assert_eq!(
            &response,
            r#"{"type":4,"data":{"content":"this is a simple message"}}"#
        )
    }

    #[test]
    fn test_open_buy_modal() {
        let response = serde_json::to_string(&open_buy_modal("Woody".to_string())).unwrap();
        assert_eq!(
            &response,
            r#"{"type":9,"data":{"custom_id":"Woody","title":"Place a bet","components":[{"type":1,"components":[{"type":4,"custom_id":"wager","label":"How much are we wagering?","placeholder":"$20","style":1,"min_length":2,"max_length":10}]},{"type":1,"components":[{"type":4,"custom_id":"outcome","label":"What is the bet on?","placeholder":"Jets beat the Chargers outright","style":2,"min_length":3,"max_length":100}]}]}}"#
        )
    }
}
