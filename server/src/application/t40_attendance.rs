use chrono::NaiveDate;

use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{
    Embed, EmbedField, InteractionCallbackData, InteractionResponse, MessageCallbackData,
};

use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::repos::{AttendanceRepository, WagerRepository};
use crate::CURRENT_FF_WEEK;

impl<WR, AR, C> Application<WR, AR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    C: DiscordClient,
{
    pub async fn attendance(
        &self,
        data: ApplicationCommandInteractionData,
        user: &User,
    ) -> Result<InteractionResponse, Error> {
        // let _timer = crate::observe::Timer::new("t40_attendance_time");
        // crate::metric(|mut m| m.count("t40_attendance"));

        let command_user = match DiscordId::from_raw_str(&user.id) {
            Some(user) => user,
            None => return Err("unable to parse discord id".into()),
        };

        let options = data.option_key_values();
        let manager_option = options.get("manager");
        match options.get("week") {
            Some(week) => self.weekly_attendance(week, manager_option).await,
            None => {
                self.individual_attendance(command_user, manager_option)
                    .await
            }
        }
    }

    async fn weekly_attendance(
        &self,
        week: &str,
        manager_option: Option<&String>,
    ) -> Result<InteractionResponse, Error> {
        let week: u8 = match week.parse::<u8>() {
            Ok(val) => val,
            Err(_) => return Err("unable to parse week".into()),
        };
        if !(1..=CURRENT_FF_WEEK).contains(&week) {
            let message = format!("No information for week {}", week);
            return Ok(InteractionResponse::channel_message_with_source_ephemeral(
                &message,
                vec![],
                vec![],
            ));
        }
        let manager_id: Option<DiscordId> = match manager_option {
            Some(manager) => Some(DiscordId::require_from_str(manager)?),
            None => None,
        };
        let weekly_results = self
            .attendance_repo
            .week_attendance(week, &manager_id)
            .await?;

        let mut embed = Embed::rich();
        let title = format!("Attendance for week {}", week);
        embed.title = Some(title);
        // TODO: refactor the repo call, shouldn't need a nested match here
        let description = match manager_id {
            Some(manager_id) => match weekly_results.interested_owner {
                Some(true) => format!("{} attended\n", manager_id),
                Some(false) => format!("{} did not attend\n", manager_id),
                None => "".to_string(),
            },
            None => "".to_string(),
        };
        embed.description = Some(description);

        let mut fields = vec![];
        for weekly_result in weekly_results.attendance {
            let attendees: Vec<String> = weekly_result.1.iter().map(|id| id.to_string()).collect();

            fields.push(EmbedField {
                name: format_date(&weekly_result.0),
                value: attendees.join(", "),
                inline: false,
            });
        }
        embed.fields = fields;
        let data = callback_data(embed, true);
        let response = InteractionResponse::channel_message_with_source(
            InteractionCallbackData::Message(data),
        );
        Ok(response)
    }
    async fn individual_attendance(
        &self,
        command_user: DiscordId,
        manager_option: Option<&String>,
    ) -> Result<InteractionResponse, Error> {
        let (ephemeral, manager_id) = match manager_option {
            Some(manager) => {
                let manager_id = DiscordId::require_from_str(manager)?;
                (false, manager_id)
            }
            None => (true, command_user),
        };
        let (overall_message, attendance) = match self
            .attendance_repo
            .combined_attendance()
            .await?
            .position_and_values(&manager_id)
        {
            Some((position, attendance)) => (build_response_messages(position), attendance),
            None => {
                let content = format!(
                    "no attendance records found, is {} in the league?",
                    manager_id
                );
                return Ok(InteractionResponse::channel_message_with_source_ephemeral(
                    &content,
                    vec![],
                    vec![],
                ));
            }
        };

        let mut embed = Embed::rich();
        let title = format!("Attendance through week {}", CURRENT_FF_WEEK);
        embed.title = Some(title);
        embed.description = Some(format!("{}\n{}", manager_id, overall_message));
        embed.fields = vec![
            EmbedField {
                name: "Weekly attendance".to_string(),
                value: format!("Attended {} of {} weeks", attendance.weeks, CURRENT_FF_WEEK),
                inline: false,
            },
            EmbedField {
                name: "Game attendance".to_string(),
                value: format!("Attended {} games", attendance.games),
                inline: false,
            },
        ];
        let data = callback_data(embed, ephemeral);
        let response = InteractionResponse::channel_message_with_source(
            InteractionCallbackData::Message(data),
        );
        Ok(response)
    }
}

fn build_response_messages(position: u8) -> String {
    if position < 4 {
        "Ranks in the top quarter, outstanding attendance!\n\u{1f929}".to_string()
    } else if position < 6 {
        "Ranks in the top half, great record!\n\u{1f600}".to_string()
    } else if position < 9 {
        "Ranks in the bottom half, needs to step it up!\n\u{1f610}".to_string()
    } else {
        "Attendance has been less than satisfactory\n\u{1f641}".to_string()
    }
}

fn callback_data(embed: Embed, ephemeral: bool) -> MessageCallbackData {
    let flags: Option<u32> = match ephemeral {
        true => Some(discord_api::interaction_response::message_flags::EPHEMERAL),
        false => None,
    };
    MessageCallbackData {
        tts: false,
        content: None,
        embeds: vec![embed],
        components: vec![],
        flags,
        allowed_mentions: vec![],
        attachments: vec![],
    }
}

fn format_date(date: &str) -> String {
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(err) => {
            println!("Error parsing date from db: {} - {}", date, err);
            return date.to_string();
        }
    };
    date.format("%a, %b %e").to_string()
}

#[test]
fn test_format_date() {
    assert_eq!("Mon, Nov 27", format_date("2023-11-27"));
    assert_eq!("Thu, Nov  2", format_date("2023-11-02"));
}
