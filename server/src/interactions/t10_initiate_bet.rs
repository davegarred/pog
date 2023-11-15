use discord_api::interaction_request::ApplicationCommandInteractionData;
use discord_api::interaction_response::{Component, InteractionCallbackData, InteractionResponse};
use discord_api::InteractionError;
use crate::ADD_BET_PLACEHOLDER_TEXT;
use crate::discord_id::{combine_user_payload, DiscordId};
use crate::error::Error;


pub fn initiate_bet(
    data: ApplicationCommandInteractionData,
) -> Result<InteractionResponse, Error> {
    let option = match data.options.get(0) {
        Some(option) => option,
        None => return Err("bet command sent with empty options".into()),
    };

    let accepting = option.value.to_string();
    let accepting_user_payload: String = match DiscordId::attempt_from_str(&accepting) {
        Some(id) => {
            let resolved_data = data
                .resolved
                .ok_or::<InteractionError>("missing resolved data".into())?;
            let user = resolved_data.expect_user(&id.str_value())?;
            let user_name = match &user.global_name {
                None => &user.username,
                Some(global_name) => global_name,
            };
            combine_user_payload(user_name, Some(id))
        }
        None => accepting,
    };
    Ok(open_buy_modal(accepting_user_payload))
}

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

#[test]
fn test_open_buy_modal() {
    let response = serde_json::to_string(&open_buy_modal("Woody".to_string())).unwrap();
    assert_eq!(
        &response,
        r#"{"type":9,"data":{"custom_id":"Woody","title":"Place a bet","components":[{"type":1,"components":[{"type":4,"custom_id":"wager","label":"How much are we wagering?","placeholder":"$20","style":1,"min_length":2,"max_length":10}]},{"type":1,"components":[{"type":4,"custom_id":"outcome","label":"What is the bet on?","placeholder":"Jets beat the Chargers outright","style":2,"min_length":3,"max_length":100}]}]}}"#
    )
}
