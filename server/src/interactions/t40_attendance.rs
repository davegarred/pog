use discord_api::interaction_request::{ApplicationCommandInteractionData, User};
use discord_api::interaction_response::{
    Embed, EmbedField, InteractionCallbackData, InteractionResponse, MessageCallbackData,
};

use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::observe::Timer;
use crate::repos::AttendanceRepository;
use crate::{metric, CURRENT_FF_WEEK};

pub async fn attendance<R: AttendanceRepository>(
    data: ApplicationCommandInteractionData,
    user: &User,
    repo: &R,
) -> Result<InteractionResponse, Error> {
    let _timer = Timer::new("t40_attendance_time");
    metric(|mut m| m.count("t40_attendance"));

    let user_id = match data.options.get(0) {
        Some(option) => DiscordId::attempt_from_str(&option.value),
        None => DiscordId::from_raw_str(&user.id),
    };
    let user_id = match user_id {
        Some(user_id) => user_id,
        None => return Err("unable to parse discord id".into()),
    };

    let (overall_message, attendance) = match repo.attendance().await?.position_and_values(&user_id)
    {
        Some((position, attendance)) => (build_response_messages(position), attendance),
        None => {
            let content = format!("no attendance records found, is {} in the league?", user_id);
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
    embed.description = Some(format!("{}\n{}", user_id, overall_message));
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
    let data = MessageCallbackData {
        tts: false,
        content: None,
        embeds: vec![embed],
        components: vec![],
        flags: None,
        // flags: Some(discord_api::interaction_response::message_flags::EPHEMERAL),
        allowed_mentions: vec![],
        attachments: vec![],
    };
    let response =
        InteractionResponse::channel_message_with_source(InteractionCallbackData::Message(data));
    Ok(response)
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
