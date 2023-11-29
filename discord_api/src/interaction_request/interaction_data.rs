use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::InteractionError;
use crate::interaction_request::interaction_data_option::InteractionDataOption;
use crate::interaction_request::message_component::MessageComponent;
use crate::interaction_request::resolved_data::ResolvedData;

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionDataPayload {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub interaction_type: Option<u8>,
    pub resolved: Option<ResolvedData>,
    pub options: Option<Vec<InteractionDataOption>>,
    pub guild_id: Option<String>,
    pub target_id: Option<String>,
    pub values: Option<Vec<String>>,
    pub custom_id: Option<String>,
    pub component_type: Option<u8>,
    pub components: Option<Vec<MessageComponent>>,
}

impl InteractionDataPayload {
    pub fn transform_data(
        &self,
        interaction_type: u8,
    ) -> Result<InteractionData, InteractionError> {
        match interaction_type {
            1 => Ok(InteractionData::Ping),
            2 => self.command_transform(),
            3 => self.message_transform(),
            4 => self.command_transform(),
            5 => self.modal_submit_transform(),
            _ => Err("unknown type".into()),
        }
    }

    fn command_transform(&self) -> Result<InteractionData, InteractionError> {
        let id = self
            .id
            .clone()
            .ok_or::<InteractionError>(("InteractionData", "id").into())?;
        let name = self
            .name
            .clone()
            .ok_or::<InteractionError>(("InteractionData", "name").into())?;
        let interaction_type = self
            .interaction_type
            .ok_or::<InteractionError>(("InteractionData", "type").into())?;
        Ok(InteractionData::Command(
            ApplicationCommandInteractionData {
                id,
                name,
                interaction_type,
                resolved: self.resolved.clone(),
                options: self.options.clone().unwrap_or(Vec::new()),
                guild_id: self.guild_id.clone(),
                target_id: self.target_id.clone(),
            },
        ))
    }
    fn message_transform(&self) -> Result<InteractionData, InteractionError> {
        let custom_id = self
            .custom_id
            .clone()
            .ok_or::<InteractionError>(("InteractionData", "custom_id").into())?;
        let component_type = self
            .component_type
            .ok_or::<InteractionError>(("InteractionData", "component_type").into())?;
        Ok(InteractionData::Message(MessageComponentInteractionData {
            custom_id,
            component_type,
            values: self.values.clone().unwrap_or(Vec::new()),
            resolved: self.resolved.clone(),
        }))
    }
    fn modal_submit_transform(&self) -> Result<InteractionData, InteractionError> {
        let custom_id = self
            .custom_id
            .clone()
            .ok_or::<InteractionError>(("InteractionData", "custom_id").into())?;
        Ok(InteractionData::ModalSubmit(ModalSubmitInteractionData {
            custom_id,
            components: self.components.clone().unwrap_or(Vec::new()),
        }))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum InteractionData {
    Ping,
    Command(ApplicationCommandInteractionData),
    Message(MessageComponentInteractionData),
    CommandAutocomplete(ApplicationCommandInteractionData),
    ModalSubmit(ModalSubmitInteractionData),
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-application-command-data-structure
#[derive(PartialEq, Clone, Debug)]
pub struct ApplicationCommandInteractionData {
    pub id: String,
    pub name: String,
    pub interaction_type: u8,
    pub resolved: Option<ResolvedData>,
    pub options: Vec<InteractionDataOption>,
    pub guild_id: Option<String>,
    pub target_id: Option<String>,
}

impl ApplicationCommandInteractionData {
    pub fn option_key_values(&self) -> HashMap<String, String> {
        ApplicationCommandInteractionData::recursive_option_key_values(&self.options)
    }

    fn recursive_option_key_values(
        options: &Vec<InteractionDataOption>,
    ) -> HashMap<String, String> {
        let mut result = HashMap::default();
        for option in options {
            result.insert(option.name.to_string(), option.value.to_string());
            if let Some(inner_options) = &option.options {
                result.extend(
                    ApplicationCommandInteractionData::recursive_option_key_values(inner_options),
                );
            }
        }
        result
    }
}

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-message-component-data-structure
#[derive(PartialEq, Clone, Debug)]
pub struct MessageComponentInteractionData {
    pub custom_id: String,
    pub component_type: u8,
    pub values: Vec<String>,
    pub resolved: Option<ResolvedData>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ModalSubmitInteractionData {
    pub custom_id: String,
    pub components: Vec<MessageComponent>,
}

impl ModalSubmitInteractionData {
    pub fn collect_components(&self) -> Result<HashMap<String, String>, InteractionError> {
        Self::collect_components_recursively(&self.components)
    }

    fn collect_components_recursively(
        components: &Vec<MessageComponent>,
    ) -> Result<HashMap<String, String>, InteractionError> {
        let mut result = HashMap::new();

        for component in components {
            match &component.components {
                Some(components) => {
                    for (key, value) in Self::collect_components_recursively(components)? {
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
}

#[cfg(test)]
mod test {
    use crate::interaction_request::interaction_data::{
        ApplicationCommandInteractionData, InteractionData, MessageComponentInteractionData,
        ModalSubmitInteractionData,
    };
    use crate::interaction_request::InteractionDataPayload;

    #[test]
    fn ping() {
        let data = InteractionDataPayload {
            id: None,
            name: None,
            interaction_type: None,
            resolved: None,
            options: None,
            guild_id: None,
            target_id: None,
            values: None,
            custom_id: None,
            component_type: None,
            components: None,
        }
        .transform_data(1)
        .unwrap();
        assert_eq!(data, InteractionData::Ping)
    }

    #[test]
    fn application_command() {
        let payload = r#"{
  "id": "1165494543471353916",
  "name": "bet",
  "type": 1
}"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(2)
            .unwrap();
        assert_eq!(
            data,
            InteractionData::Command(ApplicationCommandInteractionData {
                id: "1165494543471353916".to_string(),
                name: "bet".to_string(),
                interaction_type: 1,
                resolved: None,
                options: vec![],
                guild_id: None,
                target_id: None,
            })
        );
    }

    #[test]
    fn application_command_missing_name() {
        let payload = r#"{
  "id": "1165494543471353916",
  "type": 1
}"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(2)
            .unwrap_err();
        assert_eq!(data, ("InteractionData", "name").into());
    }

    #[test]
    fn message() {
        let payload = r#"{
    "component_type": 3,
    "custom_id": "bet",
    "values": [
        "109"
    ]
}"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(3)
            .unwrap();
        assert_eq!(
            data,
            InteractionData::Message(MessageComponentInteractionData {
                custom_id: "bet".to_string(),
                component_type: 3,
                values: vec!["109".to_string()],
                resolved: None,
            })
        );
    }
    #[test]
    fn message_no_values() {
        let payload = r#"{
    "component_type": 3,
    "custom_id": "bet"
}"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(3)
            .unwrap();
        assert_eq!(
            data,
            InteractionData::Message(MessageComponentInteractionData {
                custom_id: "bet".to_string(),
                component_type: 3,
                values: vec![],
                resolved: None,
            })
        );
    }
    #[test]
    fn message_missing_custom_id() {
        let payload = r#"{
    "component_type": 3,
    "values": [
        "109"
    ]
}"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(3)
            .unwrap_err();
        assert_eq!(data, ("InteractionData", "custom_id").into());
    }

    #[test]
    fn modal_submit() {
        let payload = r#"{
    "custom_id": "695398918694895710|Harx"
  }"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(5)
            .unwrap();
        assert_eq!(
            data,
            InteractionData::ModalSubmit(ModalSubmitInteractionData {
                custom_id: "695398918694895710|Harx".to_string(),
                components: vec![],
            })
        );
    }
    #[test]
    fn modal_submit_no_custom_id() {
        let payload = r#"{}"#;
        let data = serde_json::from_str::<InteractionDataPayload>(payload)
            .unwrap()
            .transform_data(5)
            .unwrap_err();
        assert_eq!(data, ("InteractionData", "custom_id").into());
    }
}
