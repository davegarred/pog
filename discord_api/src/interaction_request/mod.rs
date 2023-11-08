pub use guild_member::GuildMember;
pub use interaction_data::{
    ApplicationCommandInteractionData, InteractionData, InteractionDataPayload,
    MessageComponentInteractionData, ModalSubmitInteractionData,
};
pub use interaction_data_option::InteractionDataOption;
pub use interaction_object::InteractionObject;
pub use message_component::MessageComponent;
pub use message_object::MessageObject;
pub use resolved_data::ResolvedData;
pub use user::User;

mod guild_member;
mod interaction_data;
mod interaction_data_option;
mod interaction_object;
mod message_component;
mod message_object;
mod resolved_data;
mod user;

#[cfg(test)]
mod test {
    use std::fs;

    use crate::interaction_request::interaction_object::InteractionObject;

    #[test]
    fn test_ping() {
        let contents = fs::read_to_string("dto_payloads/ping_request.json").unwrap();
        let _request: InteractionObject = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn bet_request() {
        let contents = fs::read_to_string("dto_payloads/interaction_request.json").unwrap();
        let _request: InteractionObject = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn test_bet_modal_request() {
        let contents = fs::read_to_string("dto_payloads/bet_modal_request.json").unwrap();
        let _request: InteractionObject = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn payout_request() {
        let contents = fs::read_to_string("dto_payloads/T30_payout_request.json").unwrap();
        let _request: InteractionObject = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn select_option_request() {
        let contents = fs::read_to_string("dto_payloads/select_option_request.json").unwrap();
        let _request: InteractionObject = serde_json::from_str(&contents).unwrap();
    }

    #[test]
    fn first_bug() {
        let contents =
            fs::read_to_string("dto_payloads/T20_list_bets_request_w_no_global_user.json").unwrap();
        let _request: InteractionObject = serde_json::from_str(&contents).unwrap();
    }
}
