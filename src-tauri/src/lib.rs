mod config;

use serde::{Deserialize, Serialize};


#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
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
#[tauri::command]
async fn ask_hackclub_ai(prompt: String) -> Result<String, String> {
    if prompt.is_empty() {
        return Err("Prompt cannot be empty".to_string());
    }

    let api_key = config::load_api_key()?;
    let client = reqwest::Client::new();

    let body = ChatRequest {
        model: "google/gemini-2.5-flash".to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: format!("{} {}", prompt, "instruction: respond in a friendly and helpful manner, and Cat-like. more like cat. more better cat-like responses. keep it short and sweet."),
        }],
    };

    let response = client
        .post("https://ai.hackclub.com/proxy/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

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



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(debug_assertions) {
        let _ = config::ensure_config_file();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![ask_hackclub_ai])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
