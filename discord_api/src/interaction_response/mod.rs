pub use component::{
    ActionRowComponent, Component, SelectMenuComponent, SelectOptionComponent, TextInputComponent,
};
pub use select_menu_option::SelectMenuOption;

mod allowed_mention;
mod attachment;
mod component;
mod embed;
mod interaction_callback_data;
mod interaction_response_object;
pub mod message_flags;
mod select_menu_option;

pub use allowed_mention::AllowedMention;
pub use embed::*;
pub use interaction_callback_data::{
    AutocompleteCallbackData, InteractionCallbackData, MessageCallbackData, ModalCallbackData,
};
pub use interaction_response_object::InteractionResponse;

pub(crate) fn is_false(value: &bool) -> bool {
    !value
}
