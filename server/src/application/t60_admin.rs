use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{
    Component, Embed, EmbedField, InteractionCallbackData, InteractionResponse, MessageCallbackData,
};

use crate::application::app::counter;
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
            "set_user" => self.set_user_initiate(&option.value).await,
            "welcome_channel" => self.welcome_channel(&option.value).await,
            other => Err(Error::Unexpected(format!(
                "WARNING: Unrecognised option: {}",
                other
            ))),
        }
        // TODO: deal with more than one option returned
    }
    async fn set_user_initiate(&self, user_id: &str) -> Result<InteractionResponse, Error> {
        counter("admin-set_user_initiate");

        Ok(open_set_user_modal(user_id))
    }
    async fn welcome_channel(&self, channel: &str) -> Result<InteractionResponse, Error> {
        counter("admin-welcome_channel");

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

const SET_USER_DESCRIPTION: &str = r###"`/pog_admin set_user` sets the expected landing page for new users.
After the command prompt add the user (do not use an '@' before the name here).
"###;
const WELCOME_CHANNEL_DESCRIPTION: &str = r###"`/pog_admin welcome_channel` sets the expected landing page for new users.
After the command prompt, add the desired welcome channel (do not use a '#' before the name here).

Any new users will see a welcome message on this channel when they arrive.
"###;

fn admin_help() -> Result<InteractionResponse, Error> {
    counter("admin-help");

    let mut embed = Embed::rich();
    embed.title = Some("POG Admin help".to_string());
    embed.description = Some("Admin-only commands".to_string());
    embed.fields = vec![
        EmbedField {
            name: "Add or update a user's human and hash names".to_string(),
            value: SET_USER_DESCRIPTION.to_string(),
            inline: false,
        },
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

pub fn open_set_user_modal(user_id: &str) -> InteractionResponse {
    let human_name = Component::text_input(
        "human_name",
        "Human name?",
        "Neil N. Bob",
        1,
        None,
        Some(30),
        false,
    );
    let hash_name = Component::text_input(
        "hash_name",
        "Hash name?",
        "Xena, the Warrior Princess",
        1,
        None,
        Some(30),
        false,
    );
    let custom_id = format!("setuser|{}", user_id);
    let modal_component = InteractionCallbackData::modal_callback_data(
        custom_id,
        "Add a user",
        vec![
            Component::action_row(vec![human_name]),
            Component::action_row(vec![hash_name]),
        ],
    );
    InteractionResponse::modal(modal_component)
}
