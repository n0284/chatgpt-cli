use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<MessageReq<'a>>,
}

#[derive(Serialize)]
struct MessageReq<'a> {
    role: &'a str,
    content: &'a str,
}

pub fn fetch_chatgpt_response(prompt: &str) -> String {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY が .env に定義されていません");
    let client = Client::new();
    let body = ChatRequest {
        model: "gpt-4.1-nano",
        messages: vec![MessageReq {
            role: "user",
            content: prompt,
        }],
    };

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .unwrap()
        .json::<ChatResponse>()
        .unwrap();

    res.choices[0].message.content.clone()
}
