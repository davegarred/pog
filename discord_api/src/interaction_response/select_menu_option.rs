use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectMenuOption {
    label: String,
    value: String,
    description: String,
}

impl SelectMenuOption {
    pub fn new(label: String, value: String, description: String) -> Self {
        let description = if description.len() > 100 {
            description.chars().take(100).collect()
        } else {
            description
        };
        Self {
            label,
            value,
            description,
        }
    }
}
