use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{Component, InteractionResponse, SelectMenuOption};
use crate::discord_id::DiscordId;
use crate::wager_repository::WagerRepository;
use crate::error::Error;
use crate::wager::Wager;

pub async fn pay_bet<R: WagerRepository>(
    _data: ApplicationCommandInteractionData,
    user: &User,
    repo: &R,
) -> Result<InteractionResponse, Error> {
    let wagers = match DiscordId::from_raw_str(&user.id) {
        Some(user_id) => repo.search_by_user_id(&user_id).await?,
        None => vec![],
    };
    if wagers.is_empty() {
        Ok("You have no open bets".into())
    } else {
        Ok(open_select_wager_for_close_choices(wagers))
    }
}

pub fn open_select_wager_for_close_choices(wagers: Vec<Wager>) -> InteractionResponse {
    let mut options: Vec<SelectMenuOption> = Default::default();
    for wager in wagers {
        let value = format!("{}", wager.wager_id);
        let description = wager.to_string();
        options.push(SelectMenuOption::new(value.clone(), value, description));
    }
    let close_bet = Component::select_choice_component("settle", "Close which bet?", options);
    InteractionResponse::select_response(
        "Close out a bet".to_string(),
        vec![Component::action_row(close_bet)],
    )
}
