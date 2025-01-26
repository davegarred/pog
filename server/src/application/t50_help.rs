use discord_api::interaction_response::{
    Embed, EmbedField, InteractionCallbackData, InteractionResponse, MessageCallbackData,
};

use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;
use pog_common::repos::{AdminRepository, AttendanceRepository, WagerRepository, WhoisRepository};

const ATTENDANCE_DESCRIPTION: &str = r###"`/attendance` provides attendance data for others in the league.
- Specify a `manager` to see the attendance record for a manager
- Specify a `week` to see the attendance on any specific week
- Don't add anyting else to see your attendance (only you will see this)

This feature only works in the `the-league` channel (you must be in the Bleachers FF league to see the channel).
"###;

const PLACE_BET_DESCRIPTION: &str = r###"`/bet` allows you to record a bet against anyone.
After the command prompt, add the user that you are wagering against.
- If they are in our Discord server, use their handle starting with the @
- If not on our server, just use a simple name
Once you've submitted the request, a modal box will appear to fill out details including the amount, a description of the bet, and your best guess as to when it will be settled.

This feature only works in the `degenerate-gambling` channel.
"###;

const SHOW_BETS_DESCRIPTION: &str = r###"`/bets` provides a list of the current bets for a member of our Discord server.
You must also specify the bettor, this should be their Discord handle starting with an @

This feature only works in the `degenerate-gambling` channel.
"###;

const SETTLE_BET_DESCRIPTION: &str = r###"`/settle` allows you to settle a previously entered wager.
A modal will pop-up after the command is sent, select the wager that you wish to settle and an outcome.

This feature only works in the `degenerate-gambling` channel.
"###;

impl<WR, AR, SR, UR, C> Application<WR, AR, SR, UR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    SR: AdminRepository,
    UR: WhoisRepository,
    C: DiscordClient,
{
    pub async fn help(&self) -> Result<InteractionResponse, Error> {
        let mut embed = Embed::rich();
        embed.title = Some("POG help".to_string());
        embed.description =
            Some("Use the following commands to fit in within the POG-osphere".to_string());
        embed.fields = vec![
            EmbedField {
                name: "Place a bet".to_string(),
                value: PLACE_BET_DESCRIPTION.to_string(),
                inline: false,
            },
            EmbedField {
                name: "Show bets".to_string(),
                value: SHOW_BETS_DESCRIPTION.to_string(),
                inline: false,
            },
            EmbedField {
                name: "Settle a bet".to_string(),
                value: SETTLE_BET_DESCRIPTION.to_string(),
                inline: false,
            },
            EmbedField {
                name: "Check league attendance".to_string(),
                value: ATTENDANCE_DESCRIPTION.to_string(),
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
        let response = InteractionResponse::channel_message_with_source(
            InteractionCallbackData::Message(data),
        );
        Ok(response)
    }
}
