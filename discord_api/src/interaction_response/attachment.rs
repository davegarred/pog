use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/channel#attachment-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Attachment {}
