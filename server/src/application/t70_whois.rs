use crate::application::app::counter;
use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::InteractionResponse;
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn whois(
        &self,
        data: ApplicationCommandInteractionData,
        _user: &User,
    ) -> Result<InteractionResponse, Error> {
        counter("whois");

        let option = match data.options.first() {
            None => return self.help().await,
            Some(option) => option,
        };
        let user_id: u64 = match option.value.parse() {
            Ok(id) => id,
            Err(err) => return Err(Error::Unexpected(err.to_string())),
        };
        let user_details = match self.whois_repo.get_by_discord_id(user_id).await? {
            Some(user) => user,
            None => return no_known_user(),
        };

        let message = format!(
            "_User lookup_\n<@{}>\nHuman name: {}\nHash name: {}",
            user_id, user_details.human_name, user_details.hash_name
        );
        Ok(InteractionResponse::channel_message_with_source_ephemeral(
            &message,
            vec![],
            vec![],
        ))
    }
}

fn no_known_user() -> Result<InteractionResponse, Error> {
    Ok(InteractionResponse::channel_message_with_source_ephemeral(
        "No user details available",
        vec![],
        vec![],
    ))
}
