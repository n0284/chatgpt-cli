mod chatgpt;
use crate::chatgpt::Message;
use dotenvy::dotenv;
use reqwest::blocking::Client;
use std::env;
use std::io::{self, Write};

fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY がありません");
    let client = Client::new();
    let mut history: Vec<Message> = vec![];

    println!("🧠 ChatGPT CLI - 終了するには `exit` と入力");

    loop {
        print!("🧑 あなた: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let user_input = input.trim().to_string();

        if user_input == "exit" {
            break;
        }

        let response =
            chatgpt::fetch_chatgpt_response(&user_input, &api_key, &client, &mut history);
        println!("🤖 ChatGPT: {}\n", response);
    }
}
