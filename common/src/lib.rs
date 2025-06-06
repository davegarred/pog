pub use commands::{
    ADD_BET_COMMAND, ADMIN_COMMAND, ATTENDANCE_BET_COMMAND, HELP_COMMAND, LIST_BET_COMMAND,
    SETTLE_BET_COMMAND, WHOIS_COMMAND,
};
pub use discord_client::{discord_api_root, discord_headers};
pub use events::{
    Authorization, CreateMessage, DeleteMessage, DiscordMessage, MessageReference, TlDrMessage,
    UpdateMessage,
};

pub const DISCORD_API_ROOT: &str = "https://discord.com/api/v10";

mod commands;
mod discord_client;
pub mod discord_id;
pub mod error;
mod events;
pub mod repos;
pub mod wager;
