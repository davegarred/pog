use chrono::Local;
use std::sync::Mutex;

use crate::heartbeat::heartbeat;
use crate::inbound_payloads::GetGateway;
use crate::message_processor::MessageProcessor;
use futures_util::{future, pin_mut, StreamExt};
use pog_common::Authorization;

mod error;
mod gemini_client;
mod gemini_dtos;
mod heartbeat;
mod inbound_payloads;
mod message_processor;
mod payloads;
mod snark;
mod tldr;

const TLDR_MESSAGE_LENGTH: usize = 700;

#[tokio::main]
async fn main() {
    let application_id =
        std::env::var("DISCORD_APPLICATION_ID").expect("finding application id from environment");
    let discord_token =
        std::env::var("APPLICATION_TOKEN").expect("finding Discord token from environment");
    let authorization = Authorization {
        application_id,
        application_token: discord_token.clone(),
    };
    let gemini_token =
        std::env::var("GEMINI_TOKEN").expect("finding Gemini token from environment");

    println!("started at {}", Local::now().format("%Y-%m-%dT%H:%M:%S"));
    let resume_gateway = get_gateway().await;

    let (internal_tx, internal_rx) = futures_channel::mpsc::unbounded();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    let stdin_tx_heartbeat = stdin_tx.clone();
    tokio::spawn(heartbeat(internal_rx, stdin_tx_heartbeat));
    let message_processor = Mutex::new(MessageProcessor::new(
        resume_gateway.clone(),
        discord_token,
        authorization,
        gemini_token,
        stdin_tx,
        internal_tx,
    ));

    let (ws_stream, _) =
        tokio_tungstenite::connect_async(format!("{}/?v=10&encoding=json", resume_gateway))
            .await
            .expect("connect to discord gateway");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            message_processor
                .lock()
                .expect("unlock message processor mutex")
                .process(message)
                .await;
        })
    };

    // TODO: deal with restart/resume
    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn get_gateway() -> String {
    let gateway: GetGateway = reqwest::Client::new()
        .get(format!("{}{}", pog_common::DISCORD_API_ROOT, "/gateway"))
        .send()
        .await
        .expect("connect to discord gateway to retrieve resume url")
        .json()
        .await
        .expect("deserialize response payload to find resume url");
    gateway.url
}
