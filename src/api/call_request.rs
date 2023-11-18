use crate::models::llm::{ApiResponse, ChatCompletion, Message};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {

    let url = "https://api.openai.com/v1/chat/completions";
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in the environement variables");

    // headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    // chat completion details (models, temp, messages)
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages: messages,
        temperature: 0.1,
    };

    // send req and get res
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let res: ApiResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_gpt() {
        let message1 = Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        };
        let message2 = Message {
            role: "user".to_string(),
            content: "Hi, can you help me".to_string(),
        };

        let messages = vec![message1, message2];
        let res = call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                println!("{}", res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
