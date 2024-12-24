use serde::{Deserialize, Serialize};

// https://ai.google.dev/api/rest/v1/models/generateContent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiContent {
    parts: Vec<GeminiPart>,
    role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPart {
    text: String,
}

impl GenerateContentRequest {
    pub fn new(text: String) -> Self {
        Self {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart { text }],
                role: None,
            }],
        }
    }
}

// https://ai.google.dev/api/rest/v1/GenerateContentResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentResponse {
    pub candidates: Vec<Candidate>,
    #[serde(rename = "promptFeedback")]
    pub prompt_feedback: Option<PromptFeedback>,
}

impl GenerateContentResponse {
    pub fn first_candidate(&self) -> String {
        if let Some(candidate) = self.candidates.first() {
            if let Some(content) = &candidate.content {
                if let Some(part) = content.parts.first() {
                    return part.text.clone();
                }
            }
        }
        "".to_string()
    }
}

// https://ai.google.dev/api/rest/v1/GenerateContentResponse#Candidate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Option<Content>,
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
    #[serde(rename = "citationMetadata")]
    pub citation_metadata: Option<CitationMetadata>,
    #[serde(rename = "tokenCount")]
    pub token_count: Option<u32>,
    pub index: Option<u32>,
}

// https://ai.google.dev/api/rest/v1/Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

// https://ai.google.dev/api/rest/v1/Content#Part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

// https://ai.google.dev/api/rest/v1/GenerateContentResponse#CitationMetadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationMetadata {
    #[serde(rename = "citationSources")]
    pub citation_sources: Vec<CitationSource>,
}

// https://ai.google.dev/api/rest/v1/GenerateContentResponse#CitationSource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationSource {
    #[serde(rename = "startIndex")]
    pub start_index: u32,
    #[serde(rename = "endIndex")]
    pub end_index: u32,
    pub uri: String,
    pub license: String,
}

// https://ai.google.dev/api/rest/v1/GenerateContentResponse#PromptFeedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptFeedback {
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Vec<SafetyRating>,
}

// https://ai.google.dev/api/rest/v1/GenerateContentResponse#SafetyRating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
    pub blocked: Option<bool>,
}

#[cfg(test)]
mod test {
    use crate::gemini_dtos::GenerateContentResponse;
    use std::fs;

    #[test]
    fn gemini_response() {
        let contents = fs::read_to_string("dto_payloads/gemini_response.json").unwrap();
        let payload: GenerateContentResponse = serde_json::from_str(&contents).unwrap();
        println!("{}", payload.first_candidate().as_str());
    }
}
