use std::process::exit;

use crate::client::DiscordClient;

mod client;
mod commands;


const ERROR_MSG: &str = r###"usage:
   commands <command>

       command
         one of: get, update
               get    - gets the current commands configured for the pog application
               update - sets the application commands to the latest

This expects APPLICATION_ID and APPLICATION_TOKEN environment variables to be set.

"###;
const RED: &str = "\x1b[31m";
const NC: &str = "\x1b[0m";
#[tokio::main]
async fn main() {
    let (application_id, application_token) = match (std::env::var("APPLICATION_ID"),std::env::var("APPLICATION_TOKEN")) {
        (Ok(id), Ok(token)) => (id,token),
        (_, _) => {
            println!("{RED}missing expected env variable{NC}");
            println!("{}", ERROR_MSG);
            exit(1)
        }
    };
    let command = match std::env::args().nth(1) {
        Some(command) => command,
        _ => {
            println!("{}", ERROR_MSG);
            exit(1)
        }
    };
    let client = DiscordClient::new(application_id, application_token);
    match command.as_str() {
        "get" => client.get_commands().await,
        "update" => client.update_commands().await,
        &_ => {
            println!("{RED}unknown command{NC}");
            println!("{}", ERROR_MSG);
            exit(1)
        },
    }
}
