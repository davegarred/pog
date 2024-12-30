use crate::error::Error;
use crate::gemini_dtos::{GenerateContentRequest, GenerateContentResponse};
use reqwest::header::HeaderMap;

pub async fn generate_content(key: &str, text: String) -> Result<GenerateContentResponse, Error> {
    let request = GenerateContentRequest::new(text);
    match reqwest::Client::new()
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro:generateContent?key={}", key))
        .headers(headers())
        .json(&request)
        .send()
        .await
    {
        Ok(result) => {
            if !result.status().is_success() {
                Err(Error::Gemini(format!("Gemini returned {} - {:?}", result.status(), result.text().await.unwrap_or(String::new()))))
            } else {
                Ok(result.json().await?)
            }
        },
        Err(err) => {
            Err(Error::Gemini(err.to_string()))
        }
    }
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/json; charset=UTF-8"
            .parse()
            .expect("add content-type header"),
    );
    headers
}
