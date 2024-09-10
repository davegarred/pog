use crate::gemini_dtos::{GenerateContentRequest, GenerateContentResponse};
use aws_lambda_events::http::HeaderMap;
use lambda_runtime::Error;

pub async fn generate_content(key: &str, text: String) -> Result<GenerateContentResponse, Error> {
    let request = GenerateContentRequest::new(text);
    match reqwest::Client::new()
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", key))
        .headers(headers())
        .json(&request)
        .send()
        .await
    {
        Ok(result) => {
            Ok(result.json().await?)
        },
        Err(err) => {
            println!("ERROR calling Gemini: {}", err);
            Err("unable to update message".into())
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
