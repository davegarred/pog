use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Router;

use discord_api::interaction_request::InteractionObject;
use discord_api::interaction_response::InteractionResponse;

use crate::verify::VerifyTool;
use pog_common::repos::{
    new_db_pool, PostgresAdminRepository, PostgresAttendanceRepository, PostgresWagerRepo,
    PostgresWhoisRepository,
};

mod application;
mod default_discord_client;
mod discord_client;
mod error;
#[cfg(feature = "aws")]
mod observe;
mod response;
mod verify;

use crate::application::Application;
use crate::discord_client::DiscordClient;
use crate::error::Error;

#[cfg(feature = "aws")]
pub static POG_METRIC: once_cell::sync::OnceCell<
    std::sync::Arc<std::sync::Mutex<crate::observe::Metrics>>,
> = once_cell::sync::OnceCell::new();

pub const ADD_BET_PLACEHOLDER_TEXT: &str = "Raiders make the playoffs";
pub const CURRENT_FF_WEEK: u8 = 18;

#[cfg(feature = "aws")]
pub fn metric(f: impl Fn(std::sync::MutexGuard<crate::observe::Metrics>)) {
    let pog_metric = match POG_METRIC.get() {
        Some(metric) => metric.clone(),
        None => {
            println!("no metrics configured");
            return;
        }
    };
    let metric = pog_metric.lock().unwrap();
    f(metric);
}

#[cfg(feature = "aws")]
pub async fn reset_metric() {
    metric(|mut m| {
        let payload = m.finish("pog_dev");
        let payload = serde_json::to_string(&payload).unwrap();
        println!("{}", payload);
    });
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: move these to AWS secrets
    let db_user = std::env::var("DB_USER").expect("finding db user from environment");
    let db_pass = std::env::var("DB_PASS").expect("finding db pass from environment");
    let db_name = std::env::var("DB_NAME").expect("finding db name from environment");
    let db_host = std::env::var("DB_HOST").expect("finding db host from environment");
    let application_token = std::env::var("DISCORD_TOKEN").expect("finding token from environment");

    let public_key =
        std::env::var("DISCORD_PUBLIC_KEY").expect("finding public key from environment");
    let application_id =
        std::env::var("DISCORD_APPLICATION_ID").expect("finding application id from environment");
    let db_connection = format!(
        "postgresql://{}:{}@{}:5432/{}",
        db_user, db_pass, db_host, db_name
    );
    let db_pool = new_db_pool(&db_connection).await;
    let wager_repo = PostgresWagerRepo::new(db_pool.clone());
    let attendance_repo = PostgresAttendanceRepository::new(db_pool.clone());
    let admin_repo = PostgresAdminRepository::new(db_pool.clone());
    let whois_repo = PostgresWhoisRepository::new(db_pool.clone());

    #[cfg(feature = "aws")]
    {
        let lambda_name =
            std::env::var("AWS_LAMBDA_FUNCTION_NAME").expect("finding lambda function name");
        let client_lambda = std::env::var("CLIENT_LAMBDA").expect("finding client lambda name");
        let environment = std::env::var("ENVIRONMENT").expect("finding designated environment");
        let mut metrics = crate::observe::Metrics::default();
        metrics.dimension("lambda.name", &lambda_name);
        metrics.dimension("environment", &environment);
        POG_METRIC
            .set(std::sync::Arc::new(std::sync::Mutex::new(metrics)))
            .unwrap();
        let client = crate::default_discord_client::AwsDefaultDiscordClient::new(
            application_id,
            application_token,
            client_lambda,
        )
        .await;
        let application =
            Application::new(wager_repo, attendance_repo, admin_repo, whois_repo, client);
        let state = AppState {
            verifier: VerifyTool::new(&public_key),
            application,
        };
        let routes = Router::new().route(
            "/interactions",
            axum::routing::get(lambda_query_handler).post(lambda_command_handler),
        );
        let app = Router::new().merge(routes).with_state(state);

        if let Err(err) = lambda_http::run(app).await {
            panic!("{}", err);
        };
    }

    #[cfg(feature = "gcp")]
    {
        let client =
            default_discord_client::GcpDefaultDiscordClient::new(application_id, application_token)
                .await;
        let client_clone = client.clone();
        tokio::spawn(async move {
            client_clone.start().await;
        });
        let application =
            Application::new(wager_repo, attendance_repo, admin_repo, whois_repo, client);
        let state = AppState {
            verifier: VerifyTool::new(&public_key),
            application,
        };
        let router = Router::new()
            .route("/interactions", axum::routing::post(post_handler))
            .with_state(state);
        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
            .serve(router.into_make_service())
            .await
            .unwrap();
    }
    Ok(())
}

#[cfg(feature = "aws")]
pub async fn lambda_query_handler() -> Result<Response, (StatusCode, String)> {
    Ok(StatusCode::OK.into_response())
}

#[cfg(feature = "aws")]
const APPLICATION_FAILURE: &str = "application_failure";
#[cfg(feature = "aws")]
const UNEXPECTED_REQUEST_PAYLOAD: &str = "unexpected request payload";
#[cfg(feature = "aws")]
const SUCCESS: &str = "success";

#[cfg(feature = "aws")]
async fn lambda_command_handler<T: DiscordClient>(
    State(state): State<AppState<T>>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, (StatusCode, String)> {
    let result = post_handler(State(state), headers, body).await;
    reset_metric().await;
    Ok(result)
}
pub(crate) async fn post_handler<T: DiscordClient>(
    State(state): State<AppState<T>>,
    headers: HeaderMap,
    body: String,
) -> Response {
    println!("request: {}", body);
    match route(state, headers, body).await {
        Ok(response) => {
            // metric(|mut m| m.count(SUCCESS));
            str_response(response).into_response()
        }
        Err(err) => match err {
            Error::NotAuthorized => (StatusCode::UNAUTHORIZED).into_response(),
            Error::Invalid(message) => {
                println!("ERROR unexpected error: {}", message);
                // metric(|mut m| m.count(UNEXPECTED_REQUEST_PAYLOAD));
                StatusCode::BAD_REQUEST.into_response()
            }
            Error::DatabaseFailure(message) => {
                println!("ERROR db connection failure: {}", message);
                // metric(|mut m| m.count(APPLICATION_FAILURE));
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            Error::ClientFailure(message) => {
                println!("ERROR Client failure: {}", message);
                // metric(|mut m| m.count(APPLICATION_FAILURE));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Error::UnresolvedDiscordUser => {
                // metric(|mut m| m.count(UNEXPECTED_REQUEST_PAYLOAD));
                str_response("not a user in this channel".into()).into_response()
            }
            Error::Unexpected(message) => {
                println!("ERROR Client failure: {}", message);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
    }
}

fn str_response(response: InteractionResponse) -> axum::http::Response<String> {
    let payload = serde_json::to_string(&response).unwrap();
    println!("response: {}", payload);
    match axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(payload)
    {
        Ok(result) => result,
        Err(err) => {
            println!("error building response from discord response");
            panic!("{}", err)
        }
    }
}

async fn route<T: DiscordClient>(
    state: AppState<T>,
    headers: HeaderMap,
    body: String,
) -> Result<InteractionResponse, error::Error> {
    state.verifier.validate(&headers, &body)?;
    let request: InteractionObject = match serde_json::from_str::<InteractionObject>(&body) {
        Ok(request) => request,
        Err(_) => return Err("unable to deserialize body".into()),
    };
    state.application.request_handler(request).await
}

#[derive(Debug, Clone)]
struct AppState<T: DiscordClient> {
    pub verifier: VerifyTool,
    pub application: Application<
        PostgresWagerRepo,
        PostgresAttendanceRepository,
        PostgresAdminRepository,
        PostgresWhoisRepository,
        T,
    >,
}
