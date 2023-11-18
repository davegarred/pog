use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::metric;
use crate::observe::Timer;
use crate::wager::Wager;
use crate::wager_repository::WagerRepository;
use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{Component, InteractionResponse, SelectMenuOption};

pub async fn pay_bet<R: WagerRepository>(
    _data: ApplicationCommandInteractionData,
    user: &User,
    repo: &R,
) -> Result<InteractionResponse, Error> {
    let _timer = Timer::new("t30_pay_bet_time");
    metric(|mut m| m.count("t30_pay_bet"));

    let wagers = match DiscordId::from_raw_str(&user.id) {
        Some(user_id) => repo.search_by_user_id(&user_id).await?,
        None => vec![],
    };
    if wagers.is_empty() {
        Ok(InteractionResponse::channel_message_with_source_ephemeral(
            "You have no open bets",
            vec![],
        ))
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
    let close_bet = Component::select_choice("settle", "Close which bet?", options);
    InteractionResponse::select_response(
        "Close out a bet".to_string(),
        vec![Component::action_row(vec![close_bet])],
    )
}
