use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::interactions::t32_settle_bet::close_message;
use crate::metric;
use crate::observe::Timer;
use crate::wager::WagerStatus;
use crate::wager_repository::WagerRepository;
use discord_api::interaction_request::{InteractionObject, MessageComponentInteractionData};
use discord_api::interaction_response::{Component, InteractionResponse};

pub async fn bet_selected<R: WagerRepository, C: DiscordClient>(
    data: MessageComponentInteractionData,
    request: InteractionObject,
    repo: &R,
    client: &C,
) -> Result<InteractionResponse, Error> {
    let _timer = Timer::new("t31_bet_selected_time");
    metric(|mut m| m.count("t31_bet_selected"));

    let wager_id = match data.values.get(0) {
        Some(wager_id) => wager_id,
        None => return Err("missing response to bet closing reason selection".into()),
    };
    let wager_id = match wager_id.parse::<i32>() {
        Ok(wager_id) => wager_id,
        Err(_) => {
            return Err("unable to parse a wager_id from the returned value".into());
        }
    };
    let wager = match repo.get(wager_id).await {
        Some(wager) => wager,
        None => return Err(Error::Invalid(format!("wager {} not found", wager_id))),
    };
    if wager.status != WagerStatus::Open {
        return Err(Error::Invalid(format!("wager {} is not open", wager_id)));
    }

    close_message(&request, client).await?;

    let offering_won = format!("{} won", wager.offering);
    let accepting_won = format!("{} won", wager.accepting);
    let content = format!("Closing: {}", wager);
    Ok(InteractionResponse::channel_message_with_source_ephemeral(
        &content,
        vec![Component::action_row(vec![
            Component::button(&offering_won, 1, format!("offering_{}", wager_id).as_str()),
            Component::button(
                &accepting_won,
                1,
                format!("accepting_{}", wager_id).as_str(),
            ),
            Component::button("No bet", 1, format!("nobet_{}", wager_id).as_str()),
            Component::button("Cancel", 2, format!("cancel_{}", wager_id).as_str()),
        ])],
    ))
}
