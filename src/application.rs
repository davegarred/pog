use crate::error::Error;
use crate::request::{
    DiscordRequest, DiscordUser, InteractionComponent, InteractionData, InteractionOption,
};
use crate::response::{message_response, open_buy_modal, ping_response, DiscordResponse};
use crate::wager::{Wager, WagerStatus};
use crate::wager_repository::WagerRepository;
use std::collections::HashMap;

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
            "bet" => self.initiate_bet(data),
            "bets" => self.list_bets(data).await,
            "payout" => self.pay_bet(request),
            &_ => Err(Error::Invalid(format!(
                "unknown interaction name: {}",
                name
            ))),
        }
    }

    pub async fn modal_response_handler(
        &self,
        request: DiscordRequest,
    ) -> Result<DiscordResponse, Error> {
        let user = expect_member_user(&request)?;
        let offering = format!("<@{}>", user.id);
        let data = expect_data(&request)?;
        let accepting = match &data.custom_id {
            Some(id) => id.to_string(),
            None => return Err("custom_id needed for modal handler but not found".into()),
        };
        let components = collect_components(data)?;
        let (wager, outcome) = match (components.get("wager"), components.get("outcome")) {
            (Some(wager), Some(outcome)) => (wager.to_string(), outcome.to_string()),
            (_, _) => return Err("missing components needed to place wager".into()),
        };
        let time = chrono::Utc::now().to_rfc3339();
        let wager = Wager {
            time,
            offering,
            accepting,
            wager,
            outcome,
            status: WagerStatus::Open,
        };

        let response_message = wager.to_string();
        self.repo.insert(wager).await?;
        Ok(message_response(response_message))
    }

    pub fn initiate_bet(&self, data: &InteractionData) -> Result<DiscordResponse, Error> {
        let option = expect_option_at(data, 0)?;
        let accepting = option.value.to_string();
        Ok(open_buy_modal(accepting))
    }

    pub async fn list_bets(&self, data: &InteractionData) -> Result<DiscordResponse, Error> {
        let option = expect_option_at(data, 0)?;
        let user = option.value.to_string();
        let wagers = self.repo.search_by_user(&user).await?;
        if wagers.is_empty() {
            let message = format!("{} has no outstanding wagers", user);
            return Ok(message_response(message));
        }
        let mut message = format!("{} has {} outstanding wagers:", user, wagers.len());
        for wager in wagers {
            message.push_str(format!("\n- {}", wager).as_str());
        }
        Ok(message_response(message))
    }
    pub fn pay_bet(&self, _request: DiscordRequest) -> Result<DiscordResponse, Error> {
        Ok(message_response("this is not working yet"))
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
            },
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

#[cfg(test)]
mod test {
    use crate::application::Application;
    use crate::request::DiscordRequest;
    use crate::wager_repository::InMemWagerRepository;
    use std::fs;

    #[tokio::test]
    async fn ping_request() {
        let request = expect_request_from("dto_payloads/ping_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(1, result.response_type);
    }

    // TODO: verify additional requests

    #[tokio::test]
    async fn modal_response() {
        let request = expect_request_from("dto_payloads/bet_modal_request.json");
        let app = Application::new(InMemWagerRepository::default());
        let result = app.request_handler(request).await.unwrap();
        assert_eq!(4, result.response_type);
    }

    fn expect_request_from(filename: &str) -> DiscordRequest {
        let contents = fs::read_to_string(filename).unwrap();
        let request: DiscordRequest = serde_json::from_str(&contents).unwrap();
        request
    }
}
