use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{
    Embed, EmbedField, InteractionCallbackData, InteractionResponse, MessageCallbackData,
};

use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn admin(
        &self,
        data: ApplicationCommandInteractionData,
        _user: &User,
    ) -> Result<InteractionResponse, Error> {
        let option = match data.options.first() {
            None => return admin_help(),
            Some(option) => option,
        };
        match option.name.as_ref() {
            "welcome_channel" => self.welcome_channel(&option.value).await,
            other => Err(Error::Unexpected(format!(
                "WARNING: Unrecognised option: {}",
                other
            ))),
        }
        // TODO: deal with more than one option returned
    }
    async fn welcome_channel(&self, channel: &str) -> Result<InteractionResponse, Error> {
        let mut settings = self.admin_repo.get().await?;
        settings.welcome_channel = channel.to_string();
        self.admin_repo.update(settings).await?;
        let message = format!("welcome channel updated to: <#{}>", channel);
        Ok(InteractionResponse::channel_message_with_source_ephemeral(
            message.as_str(),
            vec![],
            vec![],
        ))
    }
}

const WELCOME_CHANNEL_DESCRIPTION: &str = r###"`/welcome_channel` sets the expected landing page for new users.
"###;

fn admin_help() -> Result<InteractionResponse, Error> {
    let mut embed = Embed::rich();
    embed.title = Some("POG Admin help".to_string());
    embed.description = Some("Admin-only commands".to_string());
    embed.fields = vec![
        // EmbedField {
        //     name: "Place a bet".to_string(),
        //     value: WHOIS_DESCRIPTION.to_string(),
        //     inline: false,
        // },
        EmbedField {
            name: "Show bets".to_string(),
            value: WELCOME_CHANNEL_DESCRIPTION.to_string(),
            inline: false,
        },
    ];
    let flags: Option<u32> = Some(discord_api::interaction_response::message_flags::EPHEMERAL);
    let data = MessageCallbackData {
        tts: false,
        content: None,
        embeds: vec![embed],
        components: vec![],
        flags,
        allowed_mentions: vec![],
        attachments: vec![],
    };
    let response =
        InteractionResponse::channel_message_with_source(InteractionCallbackData::Message(data));
    Ok(response)
}
