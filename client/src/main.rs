mod discord_client;
mod error;
mod gemini_client;
mod gemini_dtos;
mod snark;
mod tldr;
use crate::discord_client::{delete_message, update_message};
use crate::tldr::tldr;
use pog_common::DiscordMessage;

#[tokio::main]
#[cfg(feature = "aws")]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(lambda_runtime::service_fn(message_handler)).await?;
    Ok(())
}

#[cfg(feature = "aws")]
pub(crate) async fn message_handler(
    event: lambda_runtime::LambdaEvent<DiscordMessage>,
) -> Result<(), lambda_runtime::Error> {
    if let Err(err) = payload_router(event.payload).await {
        println!("ERROR: {}", err);
    }
    Ok(())
}

pub(crate) async fn payload_router(message: DiscordMessage) -> Result<(), error::Error> {
    match message {
        DiscordMessage::Delete(delete) => delete_message(delete).await?,
        DiscordMessage::Update(update) => update_message(update).await?,
        DiscordMessage::TlDr(message) => tldr(message).await?,
        DiscordMessage::Create(_) => {}
    }
    Ok(())
}

#[cfg(feature = "gcp")]
#[tokio::main]
async fn main() {
    let router = axum::Router::new().route("/call", axum::routing::post(post_handler));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "gcp")]
pub async fn post_handler(
    axum::extract::Json(message): axum::extract::Json<DiscordMessage>,
) -> axum::response::Response {
    match payload_router(message).await {
        Ok(_) => axum::response::IntoResponse::into_response(reqwest::StatusCode::OK),
        Err(err) => {
            let message = format!("failure: {:#?}", err);
            println!("{}", message);
            axum::response::IntoResponse::into_response(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
