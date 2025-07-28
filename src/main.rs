mod chatgpt;

fn main() {
    let prompt = "こんにちは、自己紹介してください";
    let response = chatgpt::fetch_chatgpt_response(prompt);
    println!("🧠 ChatGPTの返答: {response}");
}
