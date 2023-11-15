


// pub fn open_select_closing_reason_choices() -> DiscordResponse {
//     let options = vec![
//         SelectMenuOption {
//             label: "Paid".to_string(),
//             value: "paid".to_string(),
//             description: "This bet was paid out".to_string(),
//         },
//         SelectMenuOption {
//             label: "No Bet".to_string(),
//             value: "nobet".to_string(),
//             description: "Push or the bet predicate never happened".to_string(),
//         },
//         SelectMenuOption {
//             label: "Cancel".to_string(),
//             value: "cancel".to_string(),
//             description: "This bet doesn't exist".to_string(),
//         },
//     ];
//     let close_reason = select_choice_component("reason", "Why is the bet closing?", options);
//     select_response("Close out a bet", vec![action_row(close_reason)])
// }

#[cfg(test)]
mod test {
    use discord_api::interaction_response::InteractionResponse;

    #[test]
    fn test_ping() {
        let response = serde_json::to_string(&InteractionResponse::ping_response()).unwrap();
        assert_eq!(&response, r#"{"type":1}"#)
    }

    #[test]
    fn test_simple_response_message() {
        let response: InteractionResponse = "this is a simple message".into();
        let response = serde_json::to_string(&response).unwrap();
        assert_eq!(
            &response,
            r#"{"type":4,"data":{"content":"this is a simple message"}}"#
        )
    }

}
