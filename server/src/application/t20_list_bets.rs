use crate::application::app::counter;
use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use discord_api::interaction_request::ApplicationCommandInteractionData;
use discord_api::interaction_response::InteractionResponse;
use discord_api::InteractionError;
use pog_common::discord_id::DiscordId;
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn list_bets(
        &self,
        data: ApplicationCommandInteractionData,
    ) -> Result<InteractionResponse, Error> {
        counter("list_bets");

        let option = match data.options.first() {
            Some(option) => option,
            None => return Err("bet command sent with empty options".into()),
        };

        let user_id = match DiscordId::attempt_from_str(&option.value) {
            Some(id) => id,
            None => return Err(Error::UnresolvedDiscordUser),
        };
        let resolved_data = data
            .resolved
            .ok_or::<InteractionError>("missing resolved data".into())?;
        let username = resolved_data.expect_user(&user_id.str_value())?;
        let wagers = match DiscordId::attempt_from_str(&option.value) {
            Some(user_id) => self.wager_repo.search_by_user_id(&user_id).await?,
            None => vec![],
        };
        if wagers.is_empty() {
            let message = format!("{} has no outstanding wagers", username.username);
            return Ok(InteractionResponse::channel_message_with_source_ephemeral(
                &message,
                vec![],
                vec![],
            ));
        }
        let mut message = format!(
            "{} has {} outstanding wagers:",
            username.username,
            wagers.len()
        );
        for wager in wagers {
            message.push_str(format!("\n- {}", wager.simplified_string()).as_str());
        }
        let response =
            InteractionResponse::channel_message_with_source_ephemeral(&message, vec![], vec![]);
        Ok(response)
    }
}
