mod config;
mod opensitetool;
mod readertool;
mod sysinfo;
mod timetool;
mod webtool;


use crate::config::config_exists;
use crate::config::save_config;
use crate::opensitetool::open_website;
use serde::{Deserialize, Serialize};
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
    content: Option<String>,
}

#[tauri::command]
async fn ask_hackclub_ai(prompt: String) -> Result<String, String> {
    let original_prompt = prompt.trim().to_string();
    if original_prompt.is_empty() {
        return Err("Prompt cannot be empty".to_string());
    }

    if original_prompt.len() > 120 {
        return Err("Prompt too long".to_string());
    }

    let lower = original_prompt.to_lowercase();

    if let Some(target) = lower.strip_prefix("open ") {
        let start = original_prompt.len() - target.len();
        return opensitetool::open_website(original_prompt[start..].trim().to_string()).await;
    }

    if lower.contains("time") {
        let result = timetool::get_time();
        return Ok(format!("The current time is {}.", result));
    }

    if lower.contains("ram") || lower.contains("memory") || lower.contains("system info") {
        return Ok(sysinfo::get_system_info());
    }

    if lower.contains("app") || lower.contains("application") {
        return Ok(sysinfo::get_app_list());
    }
    if lower.contains("close") || lower.contains("exit") || lower.contains("quit") {
        return Ok("CLOSE_APP".to_string());
    }

    let mut final_prompt = original_prompt.clone();

    if lower.contains("search")
        || lower.contains("web")
        || lower.contains("google")
        || lower.contains("duckduckgo")
        || lower.contains("bing")
        || lower.contains("today")
        || lower.contains("latest")
    {
        let search_result = webtool::duckduckgo_search(&original_prompt).await?;
        final_prompt = format!(
            "Search result:\n{}\n\nUser question:\n{}",
            search_result, original_prompt
        );
    }
    if lower.contains("read") || lower.contains("file") {
        let file_path = original_prompt.split(" ").nth(1).unwrap();
        return Ok(readertool::read_file(file_path.to_string()));
    }

    let name = config::load_name().unwrap_or_else(|_| "friend".to_string());
    let pet_name = config::pet_name().unwrap_or_else(|_| "Moxi".to_string());
    let pet_type = config::load_pet_type().unwrap_or_else(|_| "cat".to_string());
    let api_key = config::load_api_key()?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let body = ChatRequest {
        model: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: format!(
                    "Respond like a friendly {}. Use a short, cute style.",
                    pet_type
                ),
            },
            ChatMessage {
                role: "system".into(),
                content: format!("The user's name is {}.", name),
            },
            ChatMessage {
                role: "system".into(),
                content: format!("Your name is {}.", pet_name),
            },
            ChatMessage {
                role: "user".into(),
                content: final_prompt,
            },
        ],
    };

    let response = client
        .post("https://ai.hackclub.com/proxy/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}\n{}", status, body));
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
        println!("Debug mode");
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ask_hackclub_ai,
            check_password,
            open_website,
            save_config,
            config_exists,
            close_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



#[tauri::command]
fn close_app(app: tauri::AppHandle) -> Result<String, String> {
    app.exit(0);
    Ok("Closing application...".to_string())
}
