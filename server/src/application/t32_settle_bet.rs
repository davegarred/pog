use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::repos::{AttendanceRepository, WagerRepository};
use crate::wager::WagerStatus;
use discord_api::interaction_request::{InteractionObject, MessageComponentInteractionData};
use discord_api::interaction_response::InteractionResponse;
use discord_api::InteractionError;

impl<WR, AR, C> Application<WR, AR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    C: DiscordClient,
{
    pub async fn settle_bet(
        &self,
        data: MessageComponentInteractionData,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        // let _timer = Timer::new("t32_settle_bet_time");
        // metric(|mut m| m.count("t32_settle_bet"));

        let (designator, wager_id) = split_custom_id(&data.custom_id)?;

        let wager_id = match wager_id.parse::<i32>() {
            Ok(wager_id) => wager_id,
            Err(_) => {
                return Err("unable to parse a wager_id from the returned value".into());
            }
        };
        let mut wager = match self.wager_repo.get(wager_id).await {
            Some(wager) => wager,
            None => return Err(Error::Invalid(format!("wager {} not found", wager_id))),
        };

        wager.status = match designator {
            Designation::Offering => WagerStatus::OfferingWon,
            Designation::Accepting => WagerStatus::AcceptingWon,
            Designation::NoBet => WagerStatus::NoBet,
            Designation::Cancel => {
                close_message(&request, &self.client).await?;
                return Ok(InteractionResponse::channel_message_with_source_ephemeral(
                    "No bets were settled",
                    vec![],
                    vec![],
                ));
            }
        };

        let message = match wager.status {
            WagerStatus::OfferingWon => {
                format!("{} won: {}", wager.offering, wager)
            }
            WagerStatus::AcceptingWon => {
                format!("{} won: {}", wager.accepting, wager)
            }
            WagerStatus::NoBet => format!("No bet: {}", wager),
            WagerStatus::Paid => format!("No bet: {}", wager),
            WagerStatus::Open => {
                return Err(Error::Invalid(format!("wager {} is still open", wager_id)))
            }
        };

        close_message(&request, &self.client).await?;
        self.wager_repo.update_status(wager_id, &wager).await?;

        Ok(message.into())
    }
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
    client.delete_message(&message_id, &token).await
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
