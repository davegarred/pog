use serde::{Deserialize, Serialize};

use pog_common::{ADD_BET_COMMAND, LIST_BET_COMMAND, SETTLE_BET_COMMAND};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "type")]
    command_type: u8,
    name: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<ApplicationCommandOptions>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOptions {
    #[serde(rename = "type")]
    command_type: u8,
    name: String,
    description: String,
    required: bool,
}

impl ApplicationCommand {
    pub fn create_bet() -> Self {
        Self {
            id: None,
            command_type: 1,
            name: ADD_BET_COMMAND.to_string(),
            description: "Create a bet".to_string(),
            options: Some(vec![ApplicationCommandOptions {
                command_type: 3,
                name: "who".to_string(),
                description: "Who are you betting?".to_string(),
                required: true,
            }]),
        }
    }
    pub fn list_bets() -> Self {
        Self {
            id: None,
            command_type: 1,
            name: LIST_BET_COMMAND.to_string(),
            description: "List bets".to_string(),
            options: Some(vec![ApplicationCommandOptions {
                command_type: 3,
                name: "bettor".to_string(),
                description: "Which bettor do you want listed?".to_string(),
                required: true,
            }]),
        }
    }
    pub fn settle() -> Self {
        Self {
            id: None,
            command_type: 1,
            name: SETTLE_BET_COMMAND.to_string(),
            description: "Close a bet".to_string(),
            options: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::commands::ApplicationCommand;

    pub type GlobalCommands = Vec<ApplicationCommand>;

    #[test]
    fn create_bet() {
        let command = serde_json::to_string(&ApplicationCommand::create_bet()).unwrap();
        assert_eq!(
            &command,
            r#"{"type":1,"name":"bet","description":"Create a bet","options":[{"type":3,"name":"who","description":"Who are you betting?","required":true}]}"#
        )
    }

    #[test]
    fn list_bets() {
        let command = serde_json::to_string(&ApplicationCommand::list_bets()).unwrap();
        assert_eq!(
            &command,
            r#"{"type":1,"name":"bets","description":"List bets","options":[{"type":3,"name":"bettor","description":"Which bettor do you want listed?","required":true}]}"#
        )
    }

    #[test]
    fn settle() {
        let command = serde_json::to_string(&ApplicationCommand::settle()).unwrap();
        assert_eq!(
            &command,
            r#"{"type":1,"name":"settle","description":"Close a bet"}"#
        )
    }

    #[test]
    fn get_serialization() {
        let contents = fs::read_to_string("dtos/global_commands.json").unwrap();
        let _request: GlobalCommands = serde_json::from_str(&contents).unwrap();
    }
}
