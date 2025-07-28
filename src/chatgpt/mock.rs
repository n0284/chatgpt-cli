use std::fs;
use serde::Deserialize;

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

pub fn fetch_chatgpt_response(_prompt: &str) -> String {
    let json = fs::read_to_string("mock_chat_response.json")
        .expect("モックJSONの読み込みに失敗しました");

    let parsed: ChatResponse = serde_json::from_str(&json)
        .expect("モックJSONのパースに失敗しました");

    parsed.choices[0].message.content.clone()
}