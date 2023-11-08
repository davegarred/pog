use crate::interaction_response::SelectMenuOption;
use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/message-components#component-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Component {
    ActionRow(ActionRowComponent),
    SelectMenu(SelectMenuComponent),
    SelectOption(SelectOptionComponent),
    TextInput(TextInputComponent),
}

// https://discord.com/developers/docs/interactions/message-components#action-rows
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ActionRowComponent {
    #[serde(rename = "type")]
    response_type: u8,
    components: Vec<Component>,
}

// https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectMenuComponent {
    #[serde(rename = "type")]
    response_type: u8,
    custom_id: String,
    options: Vec<SelectMenuOption>,
    placeholder: Option<String>,
}

// https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectOptionComponent {
    label: String,
    value: String,
    description: String,
}
// https://discord.com/developers/docs/interactions/message-components#text-input-object-text-input-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TextInputComponent {
    #[serde(rename = "type")]
    response_type: u8,
    custom_id: String,
    label: String,
    placeholder: Option<String>,
    style: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u16>,
}

impl Component {
    pub fn action_row(modal: Component) -> Self {
        Self::ActionRow(ActionRowComponent {
            response_type: 1,
            components: vec![modal],
        })
    }
    pub fn modal_item(
        custom_id: &str,
        label: &str,
        placeholder: &str,
        style: u8,
        min_length: Option<u16>,
        max_length: Option<u16>,
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
        })
    }

    pub fn select_choice_component(
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
}