use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/channel#embed-object
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
}

impl Embed {
    fn rich() -> Self {
        Self {
            title: None,
            embed_type: "rich".to_string(),
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: vec![],
        }
    }
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

// https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

// https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}
