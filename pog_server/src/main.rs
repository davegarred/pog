use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use lambda_http::http::HeaderMap;
use lambda_http::{run, Error};

use crate::application::Application;
use crate::default_discord_client::DefaultDiscordClient;
use crate::postgres_repository::PostgresWagerRepo;
use crate::request::DiscordRequest;
use crate::response::{message_response, DiscordResponse};
use crate::verify::VerifyTool;

mod application;
mod default_discord_client;
mod discord_client;
mod discord_id;
mod error;
mod postgres_repository;
mod request;
mod response;
mod verify;
mod wager;
mod wager_repository;

pub const ADD_BET_PLACEHOLDER_TEXT: &str = "Jets beat the Chargers outright";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: move these to AWS secrets
    let db_user = std::env::var("DB_USER").expect("finding db user from environment");
    let db_pass = std::env::var("DB_PASS").expect("finding db pass from environment");
    let application_token = std::env::var("DISCORD_TOKEN").expect("finding token from environment");

    let public_key = std::env::var("DISCORD_PUBLIC_KEY").expect("finding public key from environment");
    let application_id = std::env::var("DISCORD_APPLICATION_ID").expect("finding application id from environment");
    let db_name = std::env::var("DB_NAME").expect("finding db name from environment");
    let db_host = std::env::var("DB_HOST").expect("finding db host from environment");
    let sns_topic = std::env::var("SNS_TOPIC").expect("finding sns topic");
    let db_connection = format!("postgresql://{}:{}@{}:5432/{}", db_user, db_pass, db_host, db_name);
    let repo = PostgresWagerRepo::new(&db_connection).await;
    let client = DefaultDiscordClient::new(application_id, application_token, sns_topic).await;
    let application = Application::new(repo, client);
    let state = AppState {
        verifier: VerifyTool::new(&public_key),
        application,
    };
    let routes = Router::new().route(
        "/interactions",
        get(lambda_query_handler).post(lambda_command_handler),
    );
    let app = Router::new().merge(routes).with_state(state);
    run(app).await?;
    Ok(())
}

pub async fn lambda_query_handler() -> Result<Response, (StatusCode, String)> {
    Ok(StatusCode::OK.into_response())
}

async fn lambda_command_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, (StatusCode, String)> {
    println!("POST body: {}", body);
    match route(state, headers, body).await {
        Ok(response) => Ok(response.str_response().into_response()),
        Err(err) => match err {
            error::Error::NotAuthorized => Ok((StatusCode::UNAUTHORIZED).into_response()),
            error::Error::Invalid(message) => {
                println!("ERROR unexpected error: {}", message);
                Ok((StatusCode::BAD_REQUEST).into_response())
            }
            error::Error::DatabaseFailure(message) => {
                println!("ERROR db connection failure: {}", message);
                Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
            }
            error::Error::ClientFailure(message) => {
                println!("ERROR Client failure: {}", message);
                Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
            }
            error::Error::UnresolvedDiscordUser => {
                Ok(message_response("not a user in this channel")
                    .str_response()
                    .into_response())
            }
        },
    }
}

async fn route(
    state: AppState,
    headers: HeaderMap,
    body: String,
) -> Result<DiscordResponse, error::Error> {
    state.verifier.validate(&headers, &body)?;
    let request: DiscordRequest = match serde_json::from_str::<DiscordRequest>(&body) {
        Ok(request) => request,
        Err(_) => return Err("unable to deserialize body".into()),
    };
    state.application.request_handler(request).await
}

#[derive(Debug, Clone)]
struct AppState {
    pub verifier: VerifyTool,
    pub application: Application<PostgresWagerRepo, DefaultDiscordClient>,
}
