use discord_api::interaction_request::{
    ApplicationCommandInteractionData, InteractionData, InteractionObject,
    MessageComponentInteractionData, ModalSubmitInteractionData, User,
};
use discord_api::interaction_response::InteractionResponse;

use crate::discord_client::DiscordClient;
use crate::error::Error;
use crate::repos::{AttendanceRepository, WagerRepository};

#[derive(Debug, Clone)]
pub struct Application<WR: WagerRepository, AR: AttendanceRepository, C: DiscordClient> {
    pub wager_repo: WR,
    pub attendance_repo: AR,
    pub client: C,
}

impl<WR, AR, C> Application<WR, AR, C>
where
    WR: WagerRepository,
    AR: AttendanceRepository,
    C: DiscordClient,
{
    pub fn new(wager_repo: WR, attendance_repo: AR, client: C) -> Self {
        Self {
            wager_repo,
            attendance_repo,
            client,
        }
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
            pog_common::ADD_BET_COMMAND => self.initiate_bet(data).await,
            pog_common::LIST_BET_COMMAND => self.list_bets(data).await,
            pog_common::SETTLE_BET_COMMAND => self.pay_bet(data, user).await,
            pog_common::ATTENDANCE_BET_COMMAND => self.attendance(data, user).await,
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
            "offeri" | "accept" | "nobet_" | "cancel" => self.settle_bet(data, request).await,
            "settle" => self.bet_selected(data, request).await,
            &_ => Err("unknown component custom id".into()),
        }
    }

    pub async fn modal_response_handler(
        &self,
        data: ModalSubmitInteractionData,
        user: &User,
    ) -> Result<InteractionResponse, Error> {
        match &data.custom_id {
            &_ => self.add_wager(data, user).await,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use chrono::{Local, NaiveDate};

    use discord_api::interaction_request::InteractionObject;
    use discord_api::interaction_response::InteractionResponse;

    use crate::application::Application;
    use crate::discord_client::TestDiscordClient;
    use crate::discord_id::DiscordId;
    use crate::repos::{
        AttendanceRecords, InMemWagerRepository, InMemoryAttendanceRepository, WagerRepository,
    };
    use crate::wager::{Wager, WagerStatus};

    #[tokio::test]
    async fn ping_request() {
        let request = expect_request_from("dto_payloads/ping_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            InMemoryAttendanceRepository::default(),
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
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        let mut expected = r#"{"type":9,"data":{"custom_id":"1050119194533961860|Cisco","title":"Place a bet","components":[{"type":1,"components":[{"type":4,"custom_id":"wager","label":"How much are we wagering?","placeholder":"$20","style":1,"min_length":2,"max_length":10,"required":true}]},{"type":1,"components":[{"type":4,"custom_id":"outcome","label":"What is the bet on?","placeholder":"Jets beat the Chargers outright","style":2,"min_length":3,"max_length":100,"required":true}]},{"type":1,"components":[{"type":4,"custom_id":"settlement","label":"When will this bet settle?","placeholder":""#.to_string();
        expected += Local::now().format("%m/%d").to_string().as_str();
        expected += r#"","style":1,"min_length":3,"max_length":10,"required":false}]}]}}"#;
        assert_eq!(found, expected);
    }

    #[tokio::test]
    async fn t11_bet_modal_request() {
        let request = expect_request_from("dto_payloads/T11_bet_modal_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            InMemoryAttendanceRepository::default(),
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
    async fn t20_list_bets_request_no_bets() {
        let request = expect_request_from("dto_payloads/T20_list_bets_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            found,
            r#"{"type":4,"data":{"content":"Harx has no outstanding wagers","flags":64}}"#
        );
    }

    #[tokio::test]
    async fn t20_list_bets_request() {
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
        let app = Application::new(
            repo,
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            found,
            r#"{"type":4,"data":{"content":"Harx has 1 outstanding wagers:\n- ---- vs Woody, wager: $20 - Rangers repeat","flags":64}}"#
        );
    }

    #[tokio::test]
    async fn t20_list_bets_request_w_no_global_user() {
        let request =
            expect_request_from("dto_payloads/T20_list_bets_request_w_no_global_user.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            found,
            r#"{"type":4,"data":{"content":"johnanon has no outstanding wagers","flags":64}}"#
        );
    }

    #[tokio::test]
    async fn t30_payout_request() {
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
        let app = Application::new(
            repository,
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let expected = r#"{"type":4,"data":{"content":"Close out a bet","flags":64,"components":[{"type":1,"components":[{"type":3,"custom_id":"settle","options":[{"label":"1","value":"1","description":"Harx vs Woody, wager: $20 - Raiders win out"}],"placeholder":"Close which bet?"}]}]}}"#;
        assert_response(result, expected);
    }

    #[tokio::test]
    async fn t30_payout_request_no_bet() {
        let request = expect_request_from("dto_payloads/T30_payout_request.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let expected = r#"{"type":4,"data":{"content":"You have no open bets","flags":64}}"#;
        assert_response(result, expected);
    }

    #[tokio::test]
    async fn t31_selected_bet_to_close_request() {
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
        let app = Application::new(
            repo,
            InMemoryAttendanceRepository::default(),
            client.clone(),
        );

        let result = app.request_handler(request).await.unwrap();

        let expected = r#"{"type":4,"data":{"content":"Closing: ---- vs Woody, wager: $20 - Rangers repeat (settles: May  5)","flags":64,"components":[{"type":1,"components":[{"type":2,"style":1,"label":"---- won","custom_id":"offering_109","disabled":false},{"type":2,"style":1,"label":"Woody won","custom_id":"accepting_109","disabled":false},{"type":2,"style":1,"label":"No bet","custom_id":"nobet_109","disabled":false},{"type":2,"style":2,"label":"Cancel","custom_id":"cancel_109","disabled":false}]}]}}"#;
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
        let app = Application::new(
            repo,
            InMemoryAttendanceRepository::default(),
            client.clone(),
        );

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
        let app = Application::new(
            repo,
            InMemoryAttendanceRepository::default(),
            client.clone(),
        );

        let result = app.request_handler(request).await.unwrap();

        let expected = r#"{"type":4,"data":{"content":"Woody won: <@695398918694895710> vs Woody, wager: $20 - Rangers repeat"}}"#;
        assert_response(result, expected);
        assert_eq!(None, get_client_message(&client))
    }

    #[tokio::test]
    async fn t40_attendance_not_an_owner() {
        let request = expect_request_from("dto_payloads/T40_attendance_no_options.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            InMemoryAttendanceRepository::default(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            found,
            r#"{"type":4,"data":{"content":"no attendance records found, is <@695398918694895710> in the league?","flags":64}}"#
        );
    }

    #[tokio::test]
    async fn t40_attendance_no_options() {
        let request = expect_request_from("dto_payloads/T40_attendance_no_options.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            test_attendance_repo(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            found,
            r#"{"type":4,"data":{"embeds":[{"title":"Attendance through week 12","type":"rich","description":"<@695398918694895710>\nRanks in the top quarter, outstanding attendance!\n🤩","fields":[{"name":"Weekly attendance","value":"Attended 10 of 12 weeks","inline":false},{"name":"Game attendance","value":"Attended 30 games","inline":false}]}],"flags":64}}"#
        );
    }

    #[tokio::test]
    async fn t40_attendance_manager() {
        let request = expect_request_from("dto_payloads/T40_attendance_manager.json");
        let app = Application::new(
            InMemWagerRepository::default(),
            test_attendance_repo(),
            TestDiscordClient::default(),
        );

        let result = app.request_handler(request).await.unwrap();

        let found = serde_json::to_string(&result).unwrap();
        assert_eq!(
            found,
            r#"{"type":4,"data":{"embeds":[{"title":"Attendance through week 12","type":"rich","description":"<@1050119194533961860>\nRanks in the top quarter, outstanding attendance!\n🤩","fields":[{"name":"Weekly attendance","value":"Attended 7 of 12 weeks","inline":false},{"name":"Game attendance","value":"Attended 14 games","inline":false}]}]}}"#
        );
    }

    pub fn set_client_message(client: &TestDiscordClient, message: Option<String>) {
        *client.message.lock().unwrap() = message;
    }

    pub fn get_client_message(client: &TestDiscordClient) -> Option<String> {
        client.message.lock().unwrap().clone()
    }

    fn test_attendance_repo() -> InMemoryAttendanceRepository {
        let combined_attendance = AttendanceRecords(vec![
            (695398918694895710, 10, 30).into(),
            (431634941626023936, 10, 21).into(),
            (1048049562960539648, 7, 15).into(),
            (1050119194533961860, 7, 14).into(),
            (1054147659289600060, 7, 11).into(),
            (156425668270358529, 6, 9).into(),
            (689977564202401792, 6, 9).into(),
            (1045764168210448384, 5, 8).into(),
            (1045795671489380354, 4, 8).into(),
            (1046484657249718414, 4, 5).into(),
            (460972684986023937, 2, 4).into(),
            (885945439961108550, 0, 0).into(),
        ]);
        let mut weekly_attendance: Vec<(String, Vec<DiscordId>)> = Vec::default();
        weekly_attendance.push(("2023-11-23".to_string(), vec![695398918694895710.into()]));
        weekly_attendance.push((
            "2023-11-26".to_string(),
            vec![
                695398918694895710.into(),
                1048049562960539648.into(),
                431634941626023936.into(),
            ],
        ));
        weekly_attendance.push(("2023-11-27".to_string(), vec![695398918694895710.into()]));
        let weekly_attendance = weekly_attendance.into();
        InMemoryAttendanceRepository {
            combined_attendance,
            weekly_attendance,
        }
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
