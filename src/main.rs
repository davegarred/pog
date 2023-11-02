use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use lambda_http::http::HeaderMap;
use lambda_http::{run, Error};

use crate::application::Application;
use crate::postgres_repository::PostgresWagerRepo;
use crate::request::DiscordRequest;
use crate::response::{DiscordResponse, message_response};
use crate::verify::VerifyTool;

mod application;
mod discord_id;
mod error;
mod postgres_repository;
mod request;
mod response;
mod verify;
mod wager;
mod wager_repository;

pub const ADD_BET_PLACEHOLDER_TEXT: &str = "Jets beat the Giants this Sunday";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let public_key =
        std::env::var("DISCORD_PUBLIC_KEY").expect("finding public key from environment");
    let db_connection = std::env::var("DB_CONNECTION_STRING")
        .expect("finding db connection string from environment");
    let repo = PostgresWagerRepo::new(&db_connection).await;
    let application = Application::new(repo);
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
        //     let payload = serde_json::to_string(&response).unwrap();
        //     println!("success: {}", payload);
        //     let response = Response::builder()
        //         .status(StatusCode::OK)
        //         .header("Content-Type", "application/json")
        //         .body(payload)
        //         .unwrap();
        //     Ok(response.into_response())
        // }
        Err(err) => match err {
            error::Error::NotAuthorized => Ok((StatusCode::UNAUTHORIZED).into_response()),
            error::Error::Invalid(message) => {
                println!("unexpected error: {}", message);
                Ok((StatusCode::BAD_REQUEST).into_response())
            }
            error::Error::DatabaseFailure(message) => {
                println!("DATABASE FAILURE: {}", message);
                Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response())
            }
            error::Error::UnresolvedDiscordUser => Ok(message_response("not a user in this channel").str_response().into_response()),
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
    pub application: Application<PostgresWagerRepo>,
}
