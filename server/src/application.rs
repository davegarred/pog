use discord_api::interaction_request::InteractionObject;
use discord_api::interaction_response::InteractionResponse;

use crate::discord_client::DiscordClient;
use crate::discord_id::{combine_user_payload, split_combined_user_payload, DiscordId};
use crate::error::Error;
use crate::response::{open_buy_modal, open_select_wager_for_close_choices};
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

    pub async fn request_handler(
        &self,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        match request.response_type {
            1 => Ok(InteractionResponse::ping_response()),
            2 => self.command_handler(request).await,
            3 => self.select_choice_handler(request).await,
            5 => self.modal_response_handler(request).await,
            _ => Err(Error::Invalid(format!(
                "unknown response type: {}",
                request.response_type
            ))),
        }
    }

    pub async fn command_handler(
        &self,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        let data = request.expect_data()?;
        let name = match &data.name {
            None => return Err("data object is missing name field where expected".into()),
            Some(data) => data,
        };
        match name.as_str() {
            pog_common::ADD_BET_COMMAND => self.initiate_bet(request),
            pog_common::LIST_BET_COMMAND => self.list_bets(request).await,
            pog_common::SETTLE_BET_COMMAND => self.pay_bet(request).await,
            &_ => Err(Error::Invalid(format!(
                "unknown interaction name: {}",
                name
            ))),
        }
    }

    pub async fn select_choice_handler(
        &self,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        let data = request.expect_data()?;
        if let Some(values) = &data.values {
            if let Some(wager_id) = values.get(0) {
                let wager_id = match wager_id.parse::<i32>() {
                    Ok(wager_id) => wager_id,
                    Err(_) => {
                        return Err("unable to parse a wager_id from the returned value".into());
                    }
                };
                let mut wager = match self.repo.get(wager_id).await {
                    Some(wager) => wager,
                    None => return Err(Error::Invalid(format!("wager {} not found", wager_id))),
                };
                if wager.status != WagerStatus::Open {
                    return Err(Error::Invalid(format!("wager {} is not open", wager_id)));
                }
                wager.status = WagerStatus::Paid;

                let message_id = request.expect_message()?.id.clone();
                let token = request.token;
                if let Err(Error::ClientFailure(msg)) =
                    self.client.delete_message(&message_id, &token).await
                {
                    println!("ERROR sending SNS: {}", msg);
                }

                self.repo.update_status(wager_id, &wager).await?;
                let message = format!("Bet closed as paid: {}", wager.to_resolved_string());
                return Ok(message.into());
            }
        }
        Err("missing response to bet closing reason selection".into())
    }

    pub async fn modal_response_handler(
        &self,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        let user = request.expect_member()?.expect_user()?;
        let offering = match &user.global_name {
            None => user.username.to_string(),
            Some(global_name) => global_name.to_string(),
        };
        let resolved_offering_user = DiscordId::from_raw_str(&user.id);
        let data = request.expect_data()?;
        let (accepting, resolved_accepting_user) = match &data.custom_id {
            Some(combined_user_data) => split_combined_user_payload(combined_user_data),
            None => return Err("custom_id needed for modal handler but not found".into()),
        };

        let components = data.collect_components()?;
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
        Ok(response_message.into())
    }

    pub fn initiate_bet(&self, request: InteractionObject) -> Result<InteractionResponse, Error> {
        let data = request.expect_data()?;
        let option = match data.expect_options()?.get(0) {
            Some(option) => option,
            None => return Err("bet command sent with empty options".into()),
        };

        let accepting = option.value.to_string();
        let accepting_user_payload: String = match DiscordId::attempt_from_str(&accepting) {
            Some(id) => {
                let user = request
                    .expect_data()?
                    .expect_resolved_data()?
                    .expect_user(&id.str_value())?;
                let user_name = match &user.global_name {
                    None => &user.username,
                    Some(global_name) => global_name,
                };
                combine_user_payload(user_name, Some(id))
            }
            None => accepting,
        };
        Ok(open_buy_modal(accepting_user_payload))
    }

    pub async fn list_bets(
        &self,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        let data = request.expect_data()?;
        let option = match data.expect_options()?.get(0) {
            Some(option) => option,
            None => return Err("bet command sent with empty options".into()),
        };

        let user_id = match DiscordId::attempt_from_str(&option.value) {
            Some(id) => id,
            None => return Err(Error::UnresolvedDiscordUser),
        };
        let username = request
            .expect_data()?
            .expect_resolved_data()?
            .expect_user(&user_id.str_value())?;
        let wagers = match DiscordId::attempt_from_str(&option.value) {
            Some(user_id) => self.repo.search_by_user_id(&user_id).await?,
            None => vec![],
        };
        if wagers.is_empty() {
            let message = format!("{} has no outstanding wagers", username.username);
            return Ok(message.as_str().into());
        }
        let mut message = format!(
            "{} has {} outstanding wagers:",
            username.username,
            wagers.len()
        );
        for wager in wagers {
            message.push_str(format!("\n- {}", wager).as_str());
        }
        Ok(message.into())
    }

    pub async fn pay_bet(&self, request: InteractionObject) -> Result<InteractionResponse, Error> {
        let user = request.expect_member()?.expect_user()?;
        let wagers = match DiscordId::from_raw_str(&user.id) {
            Some(user_id) => self.repo.search_by_user_id(&user_id).await?,
            None => vec![],
        };
        if wagers.is_empty() {
            Ok("You have no open bets".into())
        } else {
            Ok(open_select_wager_for_close_choices(wagers))
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use discord_api::interaction_request::InteractionObject;
    use discord_api::interaction_response::InteractionResponse;

    use crate::application::Application;
    use crate::discord_client::TestDiscordClient;
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

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(&found, r#"{"type":1}"#);
    }

    #[tokio::test]
    async fn initialize_bet_request() {
        let request = expect_request_from("dto_payloads/initialize_bet_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            &found,
            r#"{"type":9,"data":{"custom_id":"1050119194533961860|Cisco","title":"Place a bet","components":[{"type":1,"components":[{"type":4,"custom_id":"wager","label":"How much are we wagering?","placeholder":"$20","style":1,"min_length":2,"max_length":10}]},{"type":1,"components":[{"type":4,"custom_id":"outcome","label":"What is the bet on?","placeholder":"Jets beat the Chargers outright","style":2,"min_length":3,"max_length":100}]}]}}"#
        );
    }

    #[tokio::test]
    async fn modal_response() {
        let request = expect_request_from("dto_payloads/bet_modal_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            &found,
            r#"{"type":4,"data":{"content":"<@695398918694895710> vs <@695398918694895710>, wager: $20 - something something"}}"#
        );
    }

    #[tokio::test]
    async fn list_bets_response() {
        let request = expect_request_from("dto_payloads/T20_list_bets_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(result, "Harx has no outstanding wagers".into())
    }

    #[tokio::test]
    async fn first_bug() {
        let request =
            expect_request_from("dto_payloads/T20_list_bets_request_w_no_global_user.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(result, "johnanon has no outstanding wagers".into())
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
        let expected = r#"{"type":4,"data":{"content":"Close out a bet","components":[{"type":1,"components":[{"type":3,"custom_id":"bet","options":[{"label":"1","value":"1","description":"Harx vs Woody, wager: $20 - Raiders win out"}],"placeholder":"Close which bet?"}]}]}}"#;
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
        let expected = r#"{"type":4,"data":{"content":"Bet closed as paid: <@695398918694895710> vs Woody, wager: $20 - Rangers repeat"}}"#;
        assert_response(result, expected);
        assert_eq!(None, get_client_message(&client))
    }

    pub fn set_client_message(client: &TestDiscordClient, message: Option<String>) {
        *client.message.lock().unwrap() = message;
    }

    pub fn get_client_message(client: &TestDiscordClient) -> Option<String> {
        client.message.lock().unwrap().clone()
    }

    fn expect_request_from(filename: &str) -> InteractionObject {
        let contents = fs::read_to_string(filename).unwrap();
        let request: InteractionObject = serde_json::from_str(&contents).unwrap();
        request
    }

    fn assert_response(response: InteractionResponse, expected: &str) {
        let ser = serde_json::to_string(&response).unwrap();
        assert_eq!(ser.as_str(), expected);
    }
}
