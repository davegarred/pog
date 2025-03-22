use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscordGatewayResponse {
    pub op: u32,
    pub d: DiscordGatewayResponsePayload,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiscordGatewayResponsePayload {
    Heartbeat(Option<u64>),
    Identify(DiscordGatewayIdentify),
    Resume(DiscordGatewayResume),
}

impl DiscordGatewayResponse {
    pub fn heartbeat(last_response: Option<u64>) -> Self {
        Self {
            op: 1,
            d: DiscordGatewayResponsePayload::Heartbeat(last_response),
        }
    }
    pub fn identify(token: &str) -> Self {
        Self {
            op: 2,
            d: DiscordGatewayResponsePayload::Identify(DiscordGatewayIdentify::new(token)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscordGatewayIdentify {
    pub token: String,
    pub properties: DiscordGatewayIdentifyConnectionProperties,
    pub compress: Option<bool>,
    pub intents: u32,
}

impl DiscordGatewayIdentify {
    pub fn new(token: &str) -> Self {
        let intents = (1 << 1)
            | (1 << 9)
            | (1 << 10)
            | (1 << 11)
            | (1 << 12)
            | (1 << 13)
            | (1 << 14)
            | (1 << 15);
        Self {
            token: token.to_string(),
            properties: Default::default(),
            compress: Some(false),
            intents,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscordGatewayIdentifyConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

impl Default for DiscordGatewayIdentifyConnectionProperties {
    fn default() -> Self {
        Self {
            os: "linux".to_string(),
            browser: "tungstenite".to_string(),
            device: "tungstenite".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscordGatewayResume {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}
