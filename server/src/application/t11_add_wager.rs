use crate::application::parse_date::parse_date;
use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use discord_api::interaction_request::{ModalSubmitInteractionData, User};
use discord_api::interaction_response::InteractionResponse;
use pog_common::discord_id::{split_combined_user_payload, DiscordId};
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};
use pog_common::wager::{Wager, WagerStatus};

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn add_wager(
        &self,
        data: ModalSubmitInteractionData,
        user: &User,
    ) -> Result<InteractionResponse, Error> {
        // let _timer = Timer::new("t11_add_wager_time");
        // metric(|mut m| m.count("t11_add_wager"));

        let offering = match &user.global_name {
            None => user.username.to_string(),
            Some(global_name) => global_name.to_string(),
        };
        let resolved_offering_user = DiscordId::from_raw_str(&user.id);
        let (accepting, resolved_accepting_user) = split_combined_user_payload(&data.custom_id);

        let components = data.collect_components()?;
        let (wager, outcome) = match (components.get("wager"), components.get("outcome")) {
            (Some(wager), Some(outcome)) => (wager.to_string(), outcome.to_string()),
            (_, _) => return Err("missing components needed to place wager".into()),
        };
        let time = chrono::Utc::now().to_rfc3339();
        let expected_settle_date = match components.get("settlement") {
            Some(c) => parse_date(c),
            None => None,
        };
        let wager = Wager {
            wager_id: 0,
            time,
            offering,
            resolved_offering_user,
            accepting,
            resolved_accepting_user,
            wager,
            outcome,
            status: WagerStatus::Open,
            expected_settle_date,
        };

        let response_message = wager.to_string();
        self.wager_repo.insert(wager).await?;
        Ok(response_message.into())
    }
}
