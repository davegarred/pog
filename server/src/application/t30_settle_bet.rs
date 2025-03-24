use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{Component, InteractionResponse, SelectMenuOption};

use crate::application::app::counter;
use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use pog_common::discord_id::DiscordId;
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};
use pog_common::wager::Wager;

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn pay_bet(
        &self,
        _data: ApplicationCommandInteractionData,
        user: &User,
    ) -> Result<InteractionResponse, Error> {
        counter("pay_bet");

        let wagers = match DiscordId::from_raw_str(&user.id) {
            Some(user_id) => self.wager_repo.search_by_user_id(&user_id).await?,
            None => vec![],
        };
        if wagers.is_empty() {
            Ok(InteractionResponse::channel_message_with_source_ephemeral(
                "You have no open bets",
                vec![],
                vec![],
            ))
        } else {
            Ok(open_select_wager_for_close_choices(wagers))
        }
    }
}

pub fn open_select_wager_for_close_choices(wagers: Vec<Wager>) -> InteractionResponse {
    let mut options: Vec<SelectMenuOption> = Default::default();
    for wager in wagers {
        let value = format!("{}", wager.wager_id);
        let description = wager.simplified_string();
        options.push(SelectMenuOption::new(value.clone(), value, description));
    }
    let close_bet = Component::select_choice("settle", "Close which bet?", options);
    InteractionResponse::select_response(
        "Close out a bet".to_string(),
        vec![Component::action_row(vec![close_bet])],
    )
}
