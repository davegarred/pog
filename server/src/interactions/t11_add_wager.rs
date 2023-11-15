use discord_api::interaction_request::{ModalSubmitInteractionData, User};
use discord_api::interaction_response::InteractionResponse;
use crate::discord_id::{DiscordId, split_combined_user_payload};
use crate::error::Error;
use crate::wager::{Wager, WagerStatus};
use crate::wager_repository::WagerRepository;

pub async fn add_wager<R: WagerRepository>(
    data: ModalSubmitInteractionData,
    user: &User,
    repo: &R,
) -> Result<InteractionResponse, Error> {
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
    };

    let response_message = wager.to_resolved_string();
    repo.insert(wager).await?;
    Ok(response_message.into())

}