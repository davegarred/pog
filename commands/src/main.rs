use crate::client::DiscordClient;

mod client;
mod commands;

#[tokio::main]
async fn main() {
    let application_id = std::env::var("APPLICATION_ID")
        .expect("did not find expected environment variable APPLICATION_ID");
    let application_token = std::env::var("APPLICATION_TOKEN")
        .expect("did not find expected environment variable APPLICATION_TOKEN");
    let command = std::env::args().nth(1).expect("missing command");
    let client = DiscordClient::new(application_id, application_token);
    match command.as_str() {
        "get" => client.get_commands().await,
        "update" => client.update_commands().await,
        &_ => println!("unknown command"),
    }
}
