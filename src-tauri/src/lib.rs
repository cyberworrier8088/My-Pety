mod config;


use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;


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
    content: Option<String>, // incresed defensive programming
}






#[tauri::command]
async fn ask_hackclub_ai(prompt: String) -> Result<String, String> {
    if prompt.is_empty() {
        return Err("Prompt cannot be empty".to_string());
    } else if prompt.len() > 50 {
        return Err("Prompt too long".to_string());
    }

    // Get name from environment variable
    let name = config::load_name().unwrap_or_else(|_| "friend".to_string());

    let pet_name = config::pet_name().unwrap_or_else(|_| "Caty".to_string());


    let api_key = config::load_api_key()?;

    let pet_type = config::load_pet_type().unwrap_or_else(|_| "cat".to_string());
    
    


    let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .build()
    .map_err(|e| e.to_string())?;

    let body = ChatRequest {
        model: "google/gemini-2.5-flash".to_string(),
        messages: vec![
    ChatMessage {
        role: "system".into(),
        content: format!("Respond like a friendly {}. more {} style. short and small answers only.", pet_type, pet_type),
    },
    ChatMessage {
        role: "system".into(),
        content: format!("My name is {}", name),
    },
    ChatMessage {
        role: "system".into(),
        content: format!("Your name is {}", pet_name),
    },
    ChatMessage {
        role: "user".into(),
        content: prompt,
    },
]
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
        .and_then(|choice| choice.message.content)
        .ok_or_else(|| "No response from AI".to_string())
}





#[tauri::command]
fn check_password(password: String) -> Result<bool, String> {
    let saved = config::load_password()?;

    Ok(password == saved)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(debug_assertions) {
        if let Err(e) = config::ensure_config_file() {
            eprintln!("Config creation failed: {e}");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![ask_hackclub_ai, check_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
