use crate::heartbeat::heartbeat;
use crate::inbound_payloads::GetGateway;
use crate::message_processor::MessageProcessor;
use crate::summarizer::{summarize, ConversationList};
use chrono::Local;
use futures_util::{future, pin_mut, StreamExt};
use pog_common::repos::postgres_channel_comment_repository::PostgresConversationCommentRepository;
use pog_common::repos::{new_db_pool, AdminRepository, PostgresAdminRepository};
use pog_common::Authorization;
use std::sync::{Arc, Mutex};

mod error;
mod gemini_client;
mod gemini_dtos;
mod heartbeat;
mod inbound_payloads;
mod message_processor;
mod payloads;
mod snark;
mod summarizer;
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

    let db_user = std::env::var("DB_USER").expect("finding db user from environment");
    let db_pass = std::env::var("DB_PASS").expect("finding db pass from environment");
    let db_name = std::env::var("DB_NAME").expect("finding db name from environment");
    let db_host = std::env::var("DB_HOST").expect("finding db host from environment");
    let db_connection = format!(
        "postgresql://{}:{}@{}:5432/{}",
        db_user, db_pass, db_host, db_name
    );
    let db_pool = new_db_pool(&db_connection).await;
    let admin_repo = PostgresAdminRepository::new(db_pool.clone());
    let comment_repo = PostgresConversationCommentRepository::new(db_pool.clone());
    let settings = Arc::new(Mutex::new(
        admin_repo
            .get()
            .await
            .expect("unable to find admin settings"),
    ));
    let settings_copy = settings.clone();

    tokio::spawn(async move {
        loop {
            let settings = admin_repo
                .get()
                .await
                .expect("unable to find admin settings");
            *settings_copy.lock().expect("can't get settings mutex") = settings;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    println!(
        "started at {}, welcome channel - {}",
        Local::now().format("%Y-%m-%dT%H:%M:%S"),
        &settings.lock().unwrap().welcome_channel
    );

    let conversations: ConversationList = Default::default();
    tokio::spawn(summarize(conversations.clone(), comment_repo));
    loop {
        let resume_gateway = get_gateway().await;

        let (internal_tx, internal_rx) = futures_channel::mpsc::unbounded();
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        let stdin_tx_heartbeat = stdin_tx.clone();
        tokio::spawn(heartbeat(internal_rx, stdin_tx_heartbeat));
        let message_processor = Mutex::new(MessageProcessor::new(
            resume_gateway.clone(),
            discord_token.clone(),
            authorization.clone(),
            gemini_token.clone(),
            settings.clone(),
            stdin_tx,
            internal_tx,
            conversations.clone(),
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
                    .await
                    // TODO: deal with restart/resume
                    .expect("TODO: need to unwind error and continue");
            })
        };
        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }
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
