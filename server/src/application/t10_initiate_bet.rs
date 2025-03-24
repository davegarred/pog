use chrono::Local;

use discord_api::interaction_request::ApplicationCommandInteractionData;
use discord_api::interaction_response::{Component, InteractionCallbackData, InteractionResponse};
use discord_api::InteractionError;

use crate::application::app::counter;
use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::ADD_BET_PLACEHOLDER_TEXT;
use pog_common::discord_id::{combine_user_payload, DiscordId};
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn initiate_bet(
        &self,
        data: ApplicationCommandInteractionData,
    ) -> Result<InteractionResponse, Error> {
        counter("initiate_bet");

        let option = match data.options.first() {
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
                format!("wager|{}", combine_user_payload(user_name, Some(id)))
            }
            None => accepting,
        };
        Ok(open_buy_modal(accepting_user_payload))
    }
}

pub fn open_buy_modal(accepting: String) -> InteractionResponse {
    let wager_modal = Component::text_input(
        "wager",
        "How much are we wagering?",
        "$20",
        1,
        Some(2),
        Some(10),
        true,
    );
    let outcome_modal = Component::text_input(
        "outcome",
        "What is the bet on?",
        ADD_BET_PLACEHOLDER_TEXT,
        2,
        Some(3),
        Some(100),
        true,
    );
    let today = Local::now().format("%m/%d").to_string();
    let settlement_date_modal = Component::text_input(
        "settlement",
        "When will this bet settle?",
        &today,
        1,
        Some(3),
        Some(10),
        false,
    );
    let modal_component = InteractionCallbackData::modal_callback_data(
        accepting,
        "Place a bet",
        vec![
            Component::action_row(vec![wager_modal]),
            Component::action_row(vec![outcome_modal]),
            Component::action_row(vec![settlement_date_modal]),
        ],
    );
    InteractionResponse::modal(modal_component)
}
