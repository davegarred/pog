use crate::discord_id::{split_combined_user_payload, DiscordId};
use crate::error::Error;
use crate::interactions::parse_date::parse_date;
use crate::metric;
use crate::observe::Timer;
use crate::wager::{Wager, WagerStatus};
use crate::wager_repository::WagerRepository;
use discord_api::interaction_request::{ModalSubmitInteractionData, User};
use discord_api::interaction_response::InteractionResponse;

pub async fn add_wager<R: WagerRepository>(
    data: ModalSubmitInteractionData,
    user: &User,
    repo: &R,
) -> Result<InteractionResponse, Error> {
    let _timer = Timer::new("t11_add_wager_time");
    metric(|mut m| m.count("t11_add_wager"));

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
    repo.insert(wager).await?;
    Ok(response_message.into())
}
