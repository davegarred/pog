mod discord_client;
mod gemini_client;
mod gemini_dtos;
mod snark;
mod tldr;

use crate::discord_client::{delete_message, update_message};
use crate::tldr::tldr;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use pog_common::DiscordMessage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(message_handler)).await?;
    Ok(())
}

pub(crate) async fn message_handler(event: LambdaEvent<DiscordMessage>) -> Result<(), Error> {
    if let Err(err) = payload_router(event).await {
        println!("ERROR: {}", err);
    }
    Ok(())
}
pub(crate) async fn payload_router(event: LambdaEvent<DiscordMessage>) -> Result<(), Error> {
    match event.payload {
        DiscordMessage::Delete(delete) => delete_message(delete).await?,
        DiscordMessage::Update(update) => update_message(update).await?,
        DiscordMessage::TlDr(message) => tldr(message).await?,
        DiscordMessage::Create(_) => {}
    }
    Ok(())
}
