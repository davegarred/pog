use pog_common::{discord_api_root, discord_headers, Authorization};

use crate::commands::ApplicationCommand;

pub struct DiscordClient {
    authorization: Authorization,
}

impl DiscordClient {
    pub fn new(application_id: String, application_token: String) -> Self {
        let authorization = Authorization {
            application_id,
            application_token,
        };
        Self { authorization }
    }

    pub async fn get_commands(&self) {
        let mut uri = discord_api_root();
        uri.push_str(
            format!(
                "/applications/{}/commands",
                self.authorization.application_id
            )
            .as_str(),
        );
        let response = reqwest::Client::new()
            .get(uri)
            .headers(discord_headers(&self.authorization))
            .send()
            .await;
        print_response(response).await;
    }
    pub async fn update_commands(&self) {
        let payload = vec![
            ApplicationCommand::create_bet(),
            ApplicationCommand::list_bets(),
            ApplicationCommand::settle(),
            ApplicationCommand::attendance(),
        ];
        let mut uri = discord_api_root();
        uri.push_str(
            format!(
                "/applications/{}/commands",
                self.authorization.application_id
            )
            .as_str(),
        );
        let response = reqwest::Client::new()
            .put(uri)
            .headers(discord_headers(&self.authorization))
            .json(&payload)
            .send()
            .await;
        print_response(response).await;
    }
}
async fn print_response(response: Result<reqwest::Response, reqwest::Error>) {
    match response {
        Ok(result) => {
            let result = result.bytes().await.unwrap();
            let body = std::str::from_utf8(result.as_ref()).unwrap();
            println!("{}", body);
        }
        Err(err) => panic!("{}", err),
    }
}
