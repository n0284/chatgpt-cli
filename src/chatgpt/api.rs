use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Clone, Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
}

// APIに送るための借用型（参照型）メッセージ
#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: String,
}

pub fn fetch_chatgpt_response(
    user_input: &str,
    api_key: &str,
    client: &Client,
    history: &mut Vec<Message>,
) -> String {
    // ユーザー入力を所有型Messageに変換して履歴に追加
    history.push(Message {
        role: "user".to_string(),
        content: user_input.to_string(),
    });

    // 履歴の所有型MessageからAPI送信用の借用型ChatMessageを生成
    let messages: Vec<ChatMessage> = history
        .iter()
        .map(|m| ChatMessage {
            role: &m.role,
            content: &m.content,
        })
        .collect();

    let request = ChatRequest {
        model: "gpt-4.1-nano",
        messages,
    };

    // println!("📜 APIに送る履歴:\n{:#?}", history);

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .unwrap()
        .json::<ChatResponse>()
        .unwrap();

    let reply = res.choices[0].message.content.trim().to_string();

    // アシスタントの応答も所有型Messageとして履歴に追加
    history.push(Message {
        role: "assistant".to_string(),
        content: reply.clone(),
    });

    reply
}
