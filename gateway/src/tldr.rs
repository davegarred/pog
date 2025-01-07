use crate::error::Error;
use crate::gemini_client::generate_content;
use crate::snark::random_snark;
use pog_common::{discord_headers, CreateMessage, MessageReference, TlDrMessage};
use serde::{Deserialize, Serialize};

pub async fn tldr(tldr: TlDrMessage) -> Result<(), Error> {
    let message = generate_response(&tldr).await?;
    let authorization = tldr.authorization;
    let create_message_data = CreateMessage {
        authorization,
        channel_id: tldr.channel_id.clone(),
        message,
        message_reference: Some(MessageReference {
            message_id: tldr.original_message_id,
            channel_id: tldr.channel_id,
        }),
    };
    create_message(create_message_data).await
}

pub async fn generate_response(tldr: &TlDrMessage) -> Result<String, Error> {
    let author = tldr.author.as_str();
    let snark = random_snark(author);

    let summarize_response =
        generate_summarization(tldr.gemini_key.as_str(), tldr.message.as_str()).await?;
    println!("{:?}", summarize_response);

    // TODO: deal with no/bad responses
    Ok(format!("{}\n_tldr:_\n{}", snark, summarize_response))
}

pub async fn generate_summarization(gemini_key: &str, message: &str) -> Result<String, Error> {
    for _iteration in 0..5 {
        let summarize_prompt = format!(
            "Your task is to summarize this opinion into 150 characters or less.
Opinion: {}",
            message
        );
        let summarize_response = generate_content(gemini_key, summarize_prompt).await?;
        if !summarize_response.first_candidate().is_empty() {
            return Ok(summarize_response.first_candidate());
        }
    }
    Err(Error::NoGeminiCandidatesReceived)
}

pub async fn create_message(message: CreateMessage) -> Result<(), Error> {
    let discord_request = DiscordRequest::new(&message.message);
    match reqwest::Client::new()
        .post(message.url())
        .headers(discord_headers(&message.authorization))
        .json(&discord_request)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("ERROR calling Discord: {}", err);
            Err(Error::ClientFailure(err.to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordRequest {
    content: String,
    components: Vec<String>,
}

impl DiscordRequest {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            components: vec![],
        }
    }
}
