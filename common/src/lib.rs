pub use commands::{ADD_BET_COMMAND, ATTENDANCE_BET_COMMAND, LIST_BET_COMMAND, SETTLE_BET_COMMAND};
pub use discord_client::{discord_api_root, discord_headers};
pub use events::{
    Authorization, CreateMessage, DeleteMessage, DiscordMessage, MessageReference, TlDrMessage,
    UpdateMessage,
};

pub const DISCORD_API_ROOT: &str = "https://discord.com/api/v10";

mod commands;
mod discord_client;
mod events;
