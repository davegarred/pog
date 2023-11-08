use crate::error::InteractionError;
use crate::interaction_request::interaction_data_option::InteractionDataOption;
use crate::interaction_request::message_component::MessageComponent;
use crate::interaction_request::resolved_data::ResolvedData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InteractionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<InteractionDataOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<MessageComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,
    // Id              string                 `json:"id"`
    // CustomId        string                 `json:"custom_id"`
    // Name            string                 `json:"name"`
    // Components      []InteractionComponent `json:"components"`
    // Options         []InteractionOption    `json:"options"`
    // InteractionType uint8                  `json:"type"`
}

impl InteractionData {
    pub fn expect_options(&self) -> Result<&Vec<InteractionDataOption>, InteractionError> {
        match &self.options {
            Some(options) => Ok(options),
            None => Err(("InteractionDataOption", "options").into()),
        }
    }

    pub fn expect_resolved_data(&self) -> Result<&ResolvedData, InteractionError> {
        match &self.resolved {
            Some(resolved_data) => Ok(resolved_data),
            None => Err(("InteractionData", "user").into()),
        }
    }

    pub fn collect_components(&self) -> Result<HashMap<String, String>, InteractionError> {
        match &self.components {
            Some(components) => Ok(Self::collect_components_recursively(components)?),
            None => Err(("InteractionDataOption", "components").into()),
        }
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
