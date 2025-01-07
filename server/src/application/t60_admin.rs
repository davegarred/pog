use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::InteractionResponse;

use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::repos::{AttendanceRepository, WagerRepository};

impl<WR, AR, C> Application<WR, AR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    C: DiscordClient,
{
    pub async fn admin(
        &self,
        _data: ApplicationCommandInteractionData,
        _user: &User,
    ) -> Result<InteractionResponse, Error> {
        let message = format!("POG admin\nThis is the future home of admin functionality.");
        Ok(InteractionResponse::channel_message_with_source_ephemeral(
            &message,
            vec![],
            vec![],
        ))
    }
}
