use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::wager::WagerStatus;
use crate::wager_repository::WagerRepository;
use discord_api::interaction_request::{InteractionObject, MessageComponentInteractionData};
use discord_api::interaction_response::InteractionResponse;
use discord_api::InteractionError;

pub async fn settle_bet<R: WagerRepository, C: DiscordClient>(
    data: MessageComponentInteractionData,
    request: InteractionObject,
    repo: &R,
    client: &C,
) -> Result<InteractionResponse, Error> {
    let (designator, wager_id) = split_custom_id(&data.custom_id)?;

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

    wager.status = match designator {
        Designation::Offering => WagerStatus::OfferingWon,
        Designation::Accepting => WagerStatus::AcceptingWon,
        Designation::NoBet => WagerStatus::NoBet,
        Designation::Cancel => return Ok("No bets were settled".into()),
    };

    let message = match wager.status {
        WagerStatus::OfferingWon => {
            format!("{} won: {}", wager.offering, wager.to_resolved_string())
        }
        WagerStatus::AcceptingWon => {
            format!("{} won: {}", wager.accepting, wager.to_resolved_string())
        }
        WagerStatus::NoBet => format!("No bet: {}", wager.to_resolved_string()),
        WagerStatus::Paid => format!("No bet: {}", wager.to_resolved_string()),
        WagerStatus::Open => return Err(Error::Invalid(format!("wager {} is still open", wager_id)))
    };

    close_message(&request, client).await?;
    repo.update_status(wager_id, &wager).await?;

    Ok(message.into())
}

pub(crate) async fn close_message<C: DiscordClient>(
    request: &InteractionObject,
    client: &C,
) -> Result<(), Error> {
    let message_id = request
        .message
        .clone()
        .ok_or::<InteractionError>("no message in request".into())?
        .id
        .clone();
    let token = request.token.to_string();
    if let Err(Error::ClientFailure(msg)) = client.delete_message(&message_id, &token).await {
        println!("ERROR sending SNS: {}", msg);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum Designation {
    Offering,
    Accepting,
    NoBet,
    Cancel,
}

fn split_custom_id(custom_id: &str) -> Result<(Designation, String), Error> {
    let custom_id = custom_id.to_string();
    match custom_id.find('_') {
        Some(pos) => {
            let designator: String = custom_id.chars().take(pos).collect();
            let designator = match designator.as_str() {
                "offering" => Designation::Offering,
                "accepting" => Designation::Accepting,
                "nobet" => Designation::NoBet,
                "cancel" => Designation::Cancel,
                &_ => return Err("custom id was not recognized".into()),
            };
            let id: String = custom_id
                .chars()
                .skip(pos + 1)
                .take_while(|_| true)
                .collect();
            Ok((designator, id))
        }
        None => Err("custom id was not recognized".into()),
    }
}

#[test]
fn test_split_custom_id() {
    assert_eq!(
        Ok((Designation::Cancel, "127".to_string())),
        split_custom_id("cancel_127")
    );
    assert_eq!(
        Err("custom id was not recognized".into()),
        split_custom_id("not right")
    );
}
