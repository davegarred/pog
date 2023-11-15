use discord_api::interaction_request::{InteractionObject, MessageComponentInteractionData};
use discord_api::interaction_response::InteractionResponse;
use discord_api::InteractionError;
use crate::discord_client::DiscordClient;
use crate::wager::WagerStatus;
use crate::error::Error;
use crate::wager_repository::WagerRepository;

pub async fn settle_bet<R: WagerRepository, C: DiscordClient>(
    data: MessageComponentInteractionData,
    request: InteractionObject,
    repo: &R,
    client: &C,
) -> Result<InteractionResponse, Error> {
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
    let mut wager = match repo.get(wager_id).await {
        Some(wager) => wager,
        None => return Err(Error::Invalid(format!("wager {} not found", wager_id))),
    };
    if wager.status != WagerStatus::Open {
        return Err(Error::Invalid(format!("wager {} is not open", wager_id)));
    }
    wager.status = WagerStatus::Paid;

    let message_id = request
        .message
        .ok_or::<InteractionError>("no message in request".into())?
        .id
        .clone();
    let token = request.token;
    if let Err(Error::ClientFailure(msg)) =
        client.delete_message(&message_id, &token).await
    {
        println!("ERROR sending SNS: {}", msg);
    }

    repo.update_status(wager_id, &wager).await?;
    let message = format!("Bet closed as paid: {}", wager.to_resolved_string());

    Ok(message.into())
}
