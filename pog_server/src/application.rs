use std::collections::HashMap;

use crate::discord_client::DiscordClient;
use crate::discord_id::{combine_user_payload, split_combined_user_payload, DiscordId};
use crate::error::Error;
use crate::request::{
    DiscordRequest, DiscordUser, InteractionComponent, InteractionData, InteractionOption,
    RequestMessage,
};
use crate::response::{
    message_response, open_buy_modal, open_select_wager_for_close_choices, ping_response,
    DiscordResponse,
};
use crate::wager::{Wager, WagerStatus};
use crate::wager_repository::WagerRepository;

#[derive(Debug, Clone)]
pub struct Application<R: WagerRepository, C: DiscordClient> {
    pub repo: R,
    pub client: C,
}

impl<R, C> Application<R, C>
where
    R: WagerRepository,
    C: DiscordClient,
{
    pub fn new(repo: R, client: C) -> Self {
        Self { repo, client }
    }

    pub async fn request_handler(&self, request: DiscordRequest) -> Result<DiscordResponse, Error> {
        match request.response_type {
            1 => Ok(ping_response()),
            2 => self.command_handler(request).await,
            3 => self.select_choice_handler(request).await,
            5 => self.modal_response_handler(request).await,
            _ => Err(Error::Invalid(format!(
                "unknown response type: {}",
                request.response_type
            ))),
        }
    }

    pub async fn command_handler(&self, request: DiscordRequest) -> Result<DiscordResponse, Error> {
        let data = expect_data(&request)?;
        let name = match &data.name {
            None => return Err("data object is missing name field where expected".into()),
            Some(data) => data,
        };
        match name.as_str() {
            "bet" => self.initiate_bet(request),
            "bets" => self.list_bets(request).await,
            "payout" => self.pay_bet(request).await,
            &_ => Err(Error::Invalid(format!(
                "unknown interaction name: {}",
                name
            ))),
        }
    }

    pub async fn select_choice_handler(
        &self,
        request: DiscordRequest,
    ) -> Result<DiscordResponse, Error> {
        let data = expect_data(&request)?;
        if let Some(values) = &data.values {
            if let Some(wager_id) = values.get(0) {
                let wager_id = match wager_id.parse::<i32>() {
                    Ok(wager_id) => wager_id,
                    Err(_) => {
                        return Err("unable to parse a wager_id from the returned value".into());
                    }
                };

                let message_id = expect_request_message(&request)?.id.clone();
                let token = request.token;
                if let Err(Error::ClientFailure(msg)) =
                    self.client.delete_message(&message_id, &token).await
                {
                    println!("ERROR sending SNS: {}", msg);
                }

                self.repo.update_status(wager_id, WagerStatus::Paid).await?;
                let wager = match self.repo.get(wager_id).await {
                    Some(wager) => wager,
                    None => return Err(Error::Invalid(format!("wager {} not found", wager_id))),
                };
                let message = format!("Bet closed as paid: {}", wager.to_resolved_string());
                return Ok(message_response(message));
            }
        }
        Err("missing response to bet closing reason selection".into())
    }

    pub async fn modal_response_handler(
        &self,
        request: DiscordRequest,
    ) -> Result<DiscordResponse, Error> {
        let user = expect_member_user(&request)?;
        let offering = user.global_name.to_string();
        let resolved_offering_user = DiscordId::from_raw_str(&user.id);
        let data = expect_data(&request)?;
        let (accepting, resolved_accepting_user) = match &data.custom_id {
            Some(combined_user_data) => split_combined_user_payload(combined_user_data),
            None => return Err("custom_id needed for modal handler but not found".into()),
        };

        let components = collect_components(data)?;
        let (wager, outcome) = match (components.get("wager"), components.get("outcome")) {
            (Some(wager), Some(outcome)) => (wager.to_string(), outcome.to_string()),
            (_, _) => return Err("missing components needed to place wager".into()),
        };
        let time = chrono::Utc::now().to_rfc3339();
        let wager = Wager {
            wager_id: 0,
            time,
            offering,
            resolved_offering_user,
            accepting,
            resolved_accepting_user,
            wager,
            outcome,
            status: WagerStatus::Open,
        };

        let response_message = wager.to_resolved_string();
        self.repo.insert(wager).await?;
        Ok(message_response(response_message))
    }

    pub fn initiate_bet(&self, request: DiscordRequest) -> Result<DiscordResponse, Error> {
        let data = expect_data(&request)?;
        let option = expect_option_at(data, 0)?;
        let accepting = option.value.to_string();
        let accepting_user_payload: String = match DiscordId::attempt_from_str(&accepting) {
            Some(id) => {
                let user = expect_resolved_user(&id, &request)?;
                combine_user_payload(&user.global_name, Some(id))
            }
            None => accepting,
        };
        Ok(open_buy_modal(accepting_user_payload))
    }

    pub async fn list_bets(&self, request: DiscordRequest) -> Result<DiscordResponse, Error> {
        let data = expect_data(&request)?;
        let option = expect_option_at(data, 0)?;
        let user_id = match DiscordId::attempt_from_str(&option.value) {
            Some(id) => id,
            None => return Err(Error::UnresolvedDiscordUser),
        };
        let username = expect_resolved_user(&user_id, &request)?;
        let wagers = match DiscordId::attempt_from_str(&option.value) {
            Some(user_id) => self.repo.search_by_user_id(&user_id).await?,
            None => vec![],
        };
        if wagers.is_empty() {
            let message = format!("{} has no outstanding wagers", username.username);
            return Ok(message_response(message));
        }
        let mut message = format!(
            "{} has {} outstanding wagers:",
            username.username,
            wagers.len()
        );
        for wager in wagers {
            message.push_str(format!("\n- {}", wager).as_str());
        }
        Ok(message_response(message))
    }

    pub async fn pay_bet(&self, request: DiscordRequest) -> Result<DiscordResponse, Error> {
        let user = expect_member_user(&request)?;
        let wagers = match DiscordId::from_raw_str(&user.id) {
            Some(user_id) => self.repo.search_by_user_id(&user_id).await?,
            None => vec![],
        };
        if wagers.is_empty() {
            Ok(message_response("You have no open bets"))
        } else {
            Ok(open_select_wager_for_close_choices(wagers))
        }
    }
}

fn expect_data(request: &DiscordRequest) -> Result<&InteractionData, Error> {
    match &request.data {
        Some(data) => Ok(data),
        None => Err("command sent with no data".into()),
    }
}

fn expect_option_at(data: &InteractionData, index: usize) -> Result<&InteractionOption, Error> {
    match &data.options {
        Some(options) => match options.get(index) {
            Some(option) => Ok(option),
            None => Err("bet command sent with empty options".into()),
        },
        None => Err("bet command sent with no options".into()),
    }
}

fn collect_components(data: &InteractionData) -> Result<HashMap<String, String>, Error> {
    match &data.components {
        Some(components) => Ok(collect_components_recurse(components)?),
        None => Err("expected components but none found".into()),
    }
}

fn collect_components_recurse(
    components: &Vec<InteractionComponent>,
) -> Result<HashMap<String, String>, Error> {
    let mut result = HashMap::new();

    for component in components {
        match &component.components {
            Some(components) => {
                for (key, value) in collect_components_recurse(components)? {
                    result.insert(key, value);
                }
            }
            None => {
                if let (Some(key), Some(value)) = (&component.custom_id, &component.value) {
                    result.insert(key.to_string(), value.to_string());
                }
            }
        };
    }
    Ok(result)
}

fn expect_member_user(data: &DiscordRequest) -> Result<&DiscordUser, Error> {
    match &data.member {
        Some(member) => Ok(&member.user),
        None => Err("expected a member field on this request".into()),
    }
}

fn expect_request_message(data: &DiscordRequest) -> Result<&RequestMessage, Error> {
    match &data.message {
        Some(message) => Ok(message),
        None => Err("expected a request message on this request".into()),
    }
}

fn expect_resolved_user<'a>(
    id: &DiscordId,
    request: &'a DiscordRequest,
) -> Result<&'a DiscordUser, Error> {
    let data = expect_data(request)?;
    if let Some(resolved) = &data.resolved {
        if let Some(user) = resolved.users.get(&id.str_value()) {
            return Ok(user);
        }
    }
    Err(Error::UnresolvedDiscordUser)
}

#[test]
fn test_expect_resolved_user() {
    // TODO
    todo!()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::application::Application;
    use crate::discord_client::TestDiscordClient;
    use crate::request::DiscordRequest;
    use crate::response::{message_response, DiscordResponse};
    use crate::wager::{Wager, WagerStatus};
    use crate::wager_repository::{InMemWagerRepository, WagerRepository};

    #[tokio::test]
    async fn ping_request() {
        let request = expect_request_from("dto_payloads/ping_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(1, result.response_type);
    }

    #[tokio::test]
    async fn initialize_bet_request() {
        let request = expect_request_from("dto_payloads/initialize_bet_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(9, result.response_type);
    }

    #[tokio::test]
    async fn modal_response() {
        let request = expect_request_from("dto_payloads/bet_modal_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(4, result.response_type);
    }

    #[tokio::test]
    async fn list_bets_response() {
        let request = expect_request_from("dto_payloads/T20_list_bets_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(result, message_response("Harx has no outstanding wagers"))
    }

    #[tokio::test]
    async fn payout_response() {
        let request = expect_request_from("dto_payloads/T30_payout_request.json");
        let repository = InMemWagerRepository::default();
        repository
            .insert(Wager {
                wager_id: 1,
                time: "".to_string(),
                offering: "Harx".to_string(),
                resolved_offering_user: Some(695398918694895710.into()),
                accepting: "Woody".to_string(),
                resolved_accepting_user: None,
                wager: "$20".to_string(),
                outcome: "Raiders win out".to_string(),
                status: WagerStatus::Open,
            })
            .await
            .unwrap();
        let app = Application::new(repository, TestDiscordClient::default());
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"Close out a bet","components":[{"type":1,"components":[{"type":3,"custom_id":"bet","options":[{"label":"1","value":"1","description":"Harx vs Woody, $20 - Raiders win out"}],"placeholder":"Close which bet?"}]}]}}"#;
        assert_response(result, expected);
    }

    #[tokio::test]
    async fn payout_response_no_bet() {
        let request = expect_request_from("dto_payloads/T30_payout_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"You have no open bets"}}"#;
        assert_response(result, expected);
    }

    #[tokio::test]
    async fn select_wager_close_reason_response() {
        let request = expect_request_from("dto_payloads/T31_selected_bet_to_close_request.json");
        let repo = InMemWagerRepository::default();
        let client = TestDiscordClient::default();
        set_client_message(&client, Some("original message".to_string()));
        repo.insert(Wager {
            wager_id: 109,
            time: "".to_string(),
            offering: "----".to_string(),
            resolved_offering_user: Some(695398918694895710.into()),
            accepting: "Woody".to_string(),
            resolved_accepting_user: None,
            wager: "$20".to_string(),
            outcome: "Rangers repeat".to_string(),
            status: WagerStatus::Open,
        })
        .await
        .unwrap();
        let app = Application::new(repo, client.clone());
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"Bet closed as paid: <@695398918694895710> vs Woody, $20 - Rangers repeat"}}"#;
        assert_response(result, expected);
        assert_eq!(None, get_client_message(&client))
    }

    pub fn set_client_message(client: &TestDiscordClient, message: Option<String>) {
        *client.message.lock().unwrap() = message;
    }

    pub fn get_client_message(client: &TestDiscordClient) -> Option<String> {
        client.message.lock().unwrap().clone()
    }

    fn expect_request_from(filename: &str) -> DiscordRequest {
        let contents = fs::read_to_string(filename).unwrap();
        let request: DiscordRequest = serde_json::from_str(&contents).unwrap();
        request
    }

    fn assert_response(response: DiscordResponse, expected: &str) {
        let ser = serde_json::to_string(&response).unwrap();
        assert_eq!(ser.as_str(), expected);
    }
}
