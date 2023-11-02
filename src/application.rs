use std::collections::HashMap;

use crate::discord_id::{combine_user_payload, DiscordId, split_combined_user_payload};
use crate::error::Error;
use crate::request::{
    DiscordRequest, DiscordUser, InteractionComponent, InteractionData, InteractionOption,
};
use crate::response::{
    DiscordResponse, message_response,
    open_buy_modal, open_select_wager_for_close_choices, ping_response,
};
use crate::wager::{Wager, WagerStatus};
use crate::wager_repository::WagerRepository;

#[derive(Debug, Clone)]
pub struct Application<R: WagerRepository> {
    pub repo: R,
}

impl<R: WagerRepository> Application<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
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
                self.repo.update_status(wager_id, WagerStatus::Paid).await?;
                return Ok(message_response("Bet closed as paid"));
            }
        }
        Err("missing response to bet closing reason selection".into())
    }

    pub async fn modal_response_handler(
        &self,
        request: DiscordRequest,
    ) -> Result<DiscordResponse, Error> {
        let user = expect_member_user(&request)?;
        // let offering = format!("<@{}>", user.global_name);
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
            None => return Err("user was not correctly resolved".into()),
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
        let mut message = format!("{} has {} outstanding wagers:", username.username, wagers.len());
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
        Ok(open_select_wager_for_close_choices(wagers))
        // Ok(message_response("this is not working yet"))
        // Ok(open_select_closing_reason_choices())
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
    Err("no resolved user found".into())
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
    use crate::request::DiscordRequest;
    use crate::response::message_response;
    use crate::wager_repository::InMemWagerRepository;

    #[tokio::test]
    async fn ping_request() {
        let request = expect_request_from("dto_payloads/ping_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(1, result.response_type);
    }

    #[tokio::test]
    async fn initialize_bet_request() {
        let request = expect_request_from("dto_payloads/initialize_bet_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(9, result.response_type);
    }

    #[tokio::test]
    async fn modal_response() {
        let request = expect_request_from("dto_payloads/bet_modal_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(4, result.response_type);
    }

    #[tokio::test]
    async fn list_bets_response() {
        let request = expect_request_from("dto_payloads/T20_list_bets_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(result, message_response("Harx has no outstanding wagers"))
    }

    #[tokio::test]
    async fn payout_response() {
        let request = expect_request_from("dto_payloads/payout_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        // TODO: fix this
        let ser = serde_json::to_string(&result).unwrap();
        println!("{}", ser);
        assert_eq!(4, result.response_type);
    }

    #[tokio::test]
    async fn select_wager_close_reason_response() {
        let request = expect_request_from("dto_payloads/T31_selected_bet_to_close_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        // TODO: fix this
        let ser = serde_json::to_string(&result).unwrap();
        println!("{}", ser);
        assert_eq!(4, result.response_type);
    }

    fn expect_request_from(filename: &str) -> DiscordRequest {
        let contents = fs::read_to_string(filename).unwrap();
        let request: DiscordRequest = serde_json::from_str(&contents).unwrap();
        request
    }
}
