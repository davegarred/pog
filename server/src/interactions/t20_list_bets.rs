use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::wager_repository::WagerRepository;
use discord_api::interaction_request::ApplicationCommandInteractionData;
use discord_api::interaction_response::InteractionResponse;
use discord_api::InteractionError;

pub async fn list_bets<R: WagerRepository>(
    data: ApplicationCommandInteractionData,
    repo: &R,
) -> Result<InteractionResponse, Error> {
    let option = match data.options.get(0) {
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
        Some(user_id) => repo.search_by_user_id(&user_id).await?,
        None => vec![],
    };
    if wagers.is_empty() {
        let message = format!("{} has no outstanding wagers", username.username);
        return Ok(message.as_str().into());
    }
    let mut message = format!(
        "{} has {} outstanding wagers:",
        username.username,
        wagers.len()
    );
    for wager in wagers {
        message.push_str(format!("\n- {}", wager).as_str());
    }
    Ok(message.into())
}
