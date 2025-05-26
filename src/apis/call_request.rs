use std::env::{self, consts::OS};

use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

use crate::models::general::llm::{ApiResponse, ChatCompletion, Message};

// Call Large Language Model
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API keys
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in environment variables");

    let url = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion = ChatCompletion {
        model: "gpt-4o".to_string(),
        messages,
        temperature: 0.1,
    };

    let response: ApiResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(response.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give a short response.".to_string(),
        };

        let messages = vec![message];

        let result = call_gpt(messages).await;

        if let Ok(result_str) = result {
            dbg!(result_str);
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
