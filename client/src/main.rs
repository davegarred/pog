mod discord_client;
mod error;
mod gemini_client;
mod gemini_dtos;
mod snark;
mod tldr;

use crate::discord_client::{delete_message, update_message};
use crate::tldr::tldr;
use pog_common::DiscordMessage;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    let state = GcpState::new();
    let state_copy = state.clone();
    tokio::spawn(async move {
        loop {
            let mut messages = state_copy.messages.lock().await;
            for message in messages.drain(..) {
                let name = format!("{:?}", message);
                match payload_router(message).await {
                    Ok(_) => {
                        println!("processed message: {}", name)
                    }
                    Err(err) => {
                        println!("failure: {:#?}\nprocessing: {}", err, name);
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });
    let router = axum::Router::new()
        .route("/call", axum::routing::post(post_handler))
        .with_state(state);
    println!(
        "started at {}",
        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S")
    );
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "gcp")]
pub async fn post_handler(
    axum::extract::State(state): axum::extract::State<GcpState>,
    axum::extract::Json(message): axum::extract::Json<DiscordMessage>,
) -> axum::response::Response {
    state.message(message).await;
    axum::response::IntoResponse::into_response(reqwest::StatusCode::OK)
}

#[derive(Clone)]
pub struct GcpState {
    messages: Arc<Mutex<Vec<DiscordMessage>>>,
}

impl GcpState {
    pub async fn message(&self, message: DiscordMessage) {
        println!("store message for processing: {:?}", &message);
        self.messages.lock().await.push(message);
    }

    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
