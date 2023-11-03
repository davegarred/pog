mod discord_client;

use crate::discord_client::{delete_message, update_message};
use aws_lambda_events::sns::SnsEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use pog_common::DiscordMessage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(message_handler)).await?;
    Ok(())
}

pub(crate) async fn message_handler(event: LambdaEvent<SnsEvent>) -> Result<(), Error> {
    for record in event.payload.records {
        let payload = record.sns.message;
        let message: DiscordMessage = match serde_json::from_str(&payload) {
            Ok(message) => message,
            Err(err) => {
                println!("ERROR deserializing: {}\npayload: {}", err, payload);
                return Err("error deserializing".into());
            }
        };
        match message {
            DiscordMessage::Delete(delete) => delete_message(delete).await?,
            DiscordMessage::Update(update) => update_message(update).await?,
        }
    }
    Ok(())
}

#[test]
fn test() {}
