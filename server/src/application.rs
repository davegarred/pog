use discord_api::interaction_request::{
    ApplicationCommandInteractionData, InteractionData, InteractionObject,
    MessageComponentInteractionData, ModalSubmitInteractionData, User,
};
use discord_api::interaction_response::InteractionResponse;

use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::interactions::{add_wager, bet_selected, initiate_bet, list_bets, pay_bet, settle_bet};
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
        match request.get_data()? {
            InteractionData::Ping => Ok(InteractionResponse::ping_response()),
            InteractionData::Command(data) => {
                self.command_handler(data, request.expect_member()?.expect_user()?)
                    .await
            }
            InteractionData::Message(data) => self.select_choice_handler(data, request).await,
            // InteractionData::CommandAutocomplete(data) => {}
            InteractionData::ModalSubmit(data) => {
                self.modal_response_handler(data, request.expect_member()?.expect_user()?)
                    .await
            }
            _ => Err(Error::Invalid(format!(
                "unknown response type: {}",
                request.interaction_type
            ))),
        }
    }

    pub async fn command_handler(
        &self,
        data: ApplicationCommandInteractionData,
        user: &User,
    ) -> Result<InteractionResponse, Error> {
        match data.name.as_str() {
            pog_common::ADD_BET_COMMAND => initiate_bet(data).await,
            pog_common::LIST_BET_COMMAND => list_bets(data, &self.repo).await,
            pog_common::SETTLE_BET_COMMAND => pay_bet(data, user, &self.repo).await,
            &_ => Err(Error::Invalid(format!(
                "unknown interaction name: {}",
                data.name
            ))),
        }
    }

    pub async fn select_choice_handler(
        &self,
        data: MessageComponentInteractionData,
        request: InteractionObject,
    ) -> Result<InteractionResponse, Error> {
        let ident: String = data.custom_id.chars().take(6).collect();
        match ident.as_str() {
            "offeri" | "accept" | "nobet_" | "cancel" => {
                settle_bet(data, request, &self.repo, &self.client).await
            }
            "settle" => bet_selected(data, request, &self.repo, &self.client).await,
            &_ => Err("unknown component custom id".into()),
        }
    }

    pub async fn modal_response_handler(
        &self,
        data: ModalSubmitInteractionData,
        user: &User,
    ) -> Result<InteractionResponse, Error> {
        match &data.custom_id {
            &_ => add_wager(data, user, &self.repo).await,
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
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
        let request = expect_request_from("dto_payloads/T10_initialize_bet_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();

        // TODO: improve this check
        assert_eq!(result.response_type, 9);
    }

    #[tokio::test]
    async fn modal_response() {
        let request = expect_request_from("dto_payloads/T11_bet_modal_request.json");
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
    async fn list_bets_no_bets() {
        let request = expect_request_from("dto_payloads/T20_list_bets_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(
            result,
            InteractionResponse::channel_message_with_source_ephemeral(
                "Harx has no outstanding wagers",
                vec![]
            )
        );
    }

    #[tokio::test]
    async fn list_bets() {
        let request = expect_request_from("dto_payloads/T20_list_bets_request.json");
        let repo = InMemWagerRepository::default();
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
            expected_settle_date: None,
        })
        .await
        .unwrap();
        let app = Application::new(repo, TestDiscordClient::default());
        let result = app.request_handler(request).await.unwrap();
        let expected =
            "Harx has 1 outstanding wagers:\n- ---- vs Woody, wager: $20 - Rangers repeat";
        assert_eq!(
            result,
            InteractionResponse::channel_message_with_source_ephemeral(expected, vec![])
        )
    }

    #[tokio::test]
    async fn list_bets_no_global_user() {
        let request =
            expect_request_from("dto_payloads/T20_list_bets_request_w_no_global_user.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            TestDiscordClient::default(),
        );
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(
            result,
            InteractionResponse::channel_message_with_source_ephemeral(
                "johnanon has no outstanding wagers",
                vec![]
            )
        );
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
                expected_settle_date: None,
            })
            .await
            .unwrap();
        let app = Application::new(repository, TestDiscordClient::default());
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"Close out a bet","components":[{"type":1,"components":[{"type":3,"custom_id":"settle","options":[{"label":"1","value":"1","description":"Harx vs Woody, wager: $20 - Raiders win out"}],"placeholder":"Close which bet?"}]}],"flags":64}}"#;
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
        let expected = r#"{"type":4,"data":{"content":"You have no open bets","flags":64}}"#;
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
            expected_settle_date: NaiveDate::from_ymd_opt(2024, 5, 5),
        })
        .await
        .unwrap();
        let app = Application::new(repo, client.clone());
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"Closing: ---- vs Woody, wager: $20 - Rangers repeat (settles: May  5)","components":[{"type":1,"components":[{"type":2,"style":1,"label":"---- won","custom_id":"offering_109","disabled":false},{"type":2,"style":1,"label":"Woody won","custom_id":"accepting_109","disabled":false},{"type":2,"style":1,"label":"No bet","custom_id":"nobet_109","disabled":false},{"type":2,"style":2,"label":"Cancel","custom_id":"cancel_109","disabled":false}]}],"flags":64}}"#;
        assert_response(result, expected);
        assert_eq!(None, get_client_message(&client))
    }

    #[tokio::test]
    async fn t32_reason_selected_cancel() {
        let request = expect_request_from("dto_payloads/T32a_reason_selected.json");
        let repo = InMemWagerRepository::default();
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
            expected_settle_date: NaiveDate::from_ymd_opt(2024, 5, 5),
        })
        .await
        .unwrap();
        let client = TestDiscordClient::default();
        set_client_message(&client, Some("original message".to_string()));
        let app = Application::new(repo, client.clone());
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"No bets were settled","flags":64}}"#;
        assert_response(result, expected);
        assert_eq!(
            Some("original message".to_string()),
            get_client_message(&client)
        )
    }

    #[tokio::test]
    async fn t32_reason_selected() {
        let request = expect_request_from("dto_payloads/T32_reason_selected.json");
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
            expected_settle_date: NaiveDate::from_ymd_opt(2024, 5, 5),
        })
        .await
        .unwrap();
        let app = Application::new(repo, client.clone());
        let result = app.request_handler(request).await.unwrap();
        let expected = r#"{"type":4,"data":{"content":"Woody won: <@695398918694895710> vs Woody, wager: $20 - Rangers repeat"}}"#;
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
