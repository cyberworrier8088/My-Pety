// src/lib.rs

// Enjoy your coding!


use serde::{Deserialize, Serialize}; // importing serde for serialization and deserialization


#[derive(Serialize)] // derive serialize for ChatMessage

// struct for chat message
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)] // derive deserialize for ChatResponse
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


// tauri command to ask hackclub ai
#[tauri::command]

// func for ask hack club ai
async fn ask_hackclub_ai(prompt: String) -> Result<String, String> { //
    let api_key = std::env::var("HACKCLUB_AI_KEY") // get api key from environment variable
        .map_err(|_| "Missing HACKCLUB_AI_KEY environment variable".to_string())?;

    let client = reqwest::Client::new(); // create http client

    let body = ChatRequest { // create chat request body
        model: "google/gemini-2.5-flash".to_string(), // model name
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: format!("{} {}", prompt, "instruction: respond in a friendly and helpful manner, and Cat-like. more like cat. more better cat-like responses. keep it short and sweet."),
        }], // user massage + instruction using formate
    };

    let response = client
        .post("https://ai.hackclub.com/proxy/v1/chat/completions") // post to hackclub ai
        .bearer_auth(api_key) // set bearer auth
        .json(&body) // set json body
        .send() // send request
        .await  // await response
        .map_err(|e| e.to_string())?; // map error

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    let data: ChatResponse = response.json().await.map_err(|e| e.to_string())?;

    data.choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| "No response from AI".to_string())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)] // if mobile, use mobile entry point

// run func
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build()) // add log plugin
        .invoke_handler(tauri::generate_handler![ask_hackclub_ai]) // add ask_hackclub_ai command
        .run(tauri::generate_context!()) // run tauri app
        .expect("error while running tauri application"); // expect error
}
