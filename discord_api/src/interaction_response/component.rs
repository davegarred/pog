use crate::interaction_response::SelectMenuOption;
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/message-components#component-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Component {
    ActionRow(ActionRowComponent),
    Button(ButtonComponent),
    SelectMenu(SelectMenuComponent),
    SelectOption(SelectOptionComponent),
    TextInput(TextInputComponent),
}

// https://discord.com/developers/docs/interactions/message-components#action-rows
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ActionRowComponent {
    #[serde(rename = "type")]
    pub response_type: u8,
    pub components: Vec<Component>,
}

// https://discord.com/developers/docs/interactions/message-components#buttons
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ButtonComponent {
    #[serde(rename = "type")]
    pub response_type: u8,
    // TODO: https://discord.com/developers/docs/interactions/message-components#buttons
    pub style: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub disabled: bool,
}

// https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectMenuComponent {
    #[serde(rename = "type")]
    pub response_type: u8,
    pub custom_id: String,
    pub options: Vec<SelectMenuOption>,
    pub placeholder: Option<String>,
}

// https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectOptionComponent {
    pub label: String,
    pub value: String,
    pub description: String,
}
// https://discord.com/developers/docs/interactions/message-components#text-input-object-text-input-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TextInputComponent {
    #[serde(rename = "type")]
    pub response_type: u8,
    pub custom_id: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub style: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl Component {
    pub fn action_row(modals: Vec<Component>) -> Self {
        Self::ActionRow(ActionRowComponent {
            response_type: 1,
            components: modals,
        })
    }
    pub fn button(label: &str, style: u8, custom_id: &str) -> Self {
        Self::Button(ButtonComponent {
            response_type: 2,
            style,
            label: Some(label.to_string()),
            custom_id: Some(custom_id.to_string()),
            url: None,
            disabled: false,
        })
    }

    pub fn select_choice(
        custom_id: &str,
        placeholder: &str,
        options: Vec<SelectMenuOption>,
    ) -> Self {
        Self::SelectMenu(SelectMenuComponent {
            response_type: 3,
            custom_id: custom_id.to_string(),
            placeholder: Some(placeholder.to_string()),
            options,
        })
    }

    pub fn text_input(
        custom_id: &str,
        label: &str,
        placeholder: &str,
        style: u8,
        min_length: Option<u16>,
        max_length: Option<u16>,
        required: bool,
    ) -> Self {
        let placeholder = Some(placeholder.to_string());
        Self::TextInput(TextInputComponent {
            response_type: 4,
            custom_id: custom_id.to_string(),
            label: label.to_string(),
            placeholder,
            style,
            min_length,
            max_length,
            required: Some(required),
        })
    }
}
