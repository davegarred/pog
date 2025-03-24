use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use discord_api::interaction_request::{ModalSubmitInteractionData, User};
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
    pub async fn set_user(
        &self,
        data: ModalSubmitInteractionData,
        _user: &User,
    ) -> Result<InteractionResponse, Error> {
        let id_sec = &data.custom_id[8..];
        let id = match id_sec.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err("not a valid user id".into()),
        };
        let mut human_name = String::new();
        let mut hash_name = String::new();
        for component in data.components {
            if let Some(component) = component.components {
                if let Some(component) = component.first() {
                    if let Some(id) = &component.custom_id {
                        let value = component.value.clone();
                        let value = value.unwrap_or("".to_string());
                        match id.as_str() {
                            "human_name" => human_name = value,
                            "hash_name" => hash_name = value,
                            &_ => {}
                        }
                    }
                }
            }
        }

        self.whois_repo
            .set_user(id, &human_name, &hash_name)
            .await?;
        let message = format!("user <@{}> set", id);
        Ok(InteractionResponse::channel_message_with_source_ephemeral(
            message.as_str(),
            vec![],
            vec![],
        ))
    }
}
