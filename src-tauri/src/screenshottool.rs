use crate::parse_chat_response_body; // importing parse_chat_response_body function
use screenshots::Screen; // importing Screen struct from screenshots crate
use serde::Serialize; // importing Serialize trait from serde crate
use tauri::Manager; // importing Manager trait from tauri crate
use base64::{engine::general_purpose, Engine as _}; // importing base64 engine

pub fn take_screenshot() -> Result<String, String> { // taking screenshot function
    let screens = Screen::all().map_err(|e| e.to_string())?; 

    let screen = screens.first().ok_or("No screen found")?;

    let image = screen.capture().map_err(|e| e.to_string())?;

    let path = "screenshot.png";

    image.save(path).map_err(|e| e.to_string())?; // saving screenshot to file

    Ok(path.to_string()) // returning path to screenshot
}



pub fn image_to_base64(path: &str) -> Result<String,String> { // converting image to base64

    let bytes = std::fs::read(path).map_err(|e| e.to_string())?; // reading image file

    Ok(general_purpose::STANDARD.encode(bytes)) // returning base64 encoded image
}








#[derive(Serialize)] // serialize image url
struct ImageUrl {url: String}

#[derive(Serialize)] // serialize content part
struct ContentPart {#[serde(rename="type")] kind: String, text: Option<String>, image_url: Option<ImageUrl>, // serializing content part
}

#[derive(Serialize)] // serialize vision message
struct VisionMessage {
    role: String,
    content: Vec<ContentPart>, // serializing vision message
}



pub async fn analyze_screen(window: tauri::AppHandle, api_key: String) -> Result<String, String> { // analyzing screen function

    let window = window.get_webview_window("main").ok_or("Window not found")?; // getting main window

    window.minimize().map_err(|e| e.to_string())?; // minimizing window

    tokio::time::sleep(std::time::Duration::from_millis(500)).await; // waiting for window to minimize

    let path = take_screenshot()?;

    window.unminimize().map_err(|e| e.to_string())?; // unminimizing window

    window.show().map_err(|e| e.to_string())?; // showing window

    let image = image_to_base64(&path)?; // converting image to base64

    let image_url = format!("data:image/png;base64,{}", image); // creating image url


    // creating vision message model giving instructions to ai
    // this same code and explantion in lib file have but some changes
    let messages = vec![
        VisionMessage { // creating vision message
            role: "user".into(),
            content: vec![
                ContentPart { // adding text content
                    kind: "text".into(),
                    text: Some(
                        "Describe my screen briefly. more short answer but evarthing included."
                            .into()
                    ),
                    image_url: None,
                },
                ContentPart { // adding image content
                    kind: "image_url".into(),
                    text: None,
                    image_url: Some(
                        ImageUrl {
                            url: image_url,
                        }
                    ),
                },
            ],
        },
    ];

    let body = serde_json::json!({
        "model": "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free", // this model can use image. 
        "messages": messages
    });

    let client = reqwest::Client::new();

    let response =
        client
            .post(
                "https://ai.hackclub.com/proxy/v1/chat/completions"
            )
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

    let result = async {

        if !response.status().is_success() { // checking if response is success

            let status =
                response.status();

            let body =
                response
                    .text()
                    .await
                    .unwrap_or_default();

            return Err(format!(
                "Vision API error: {}\n{}",
                status,
                body
            ));
        }

        let body =
            response
                .text()
                .await
                .map_err(|e| e.to_string())?;

        parse_chat_response_body(&body)

    }.await;

    let _ = std::fs::remove_file(&path); // removing image file

    result // returning result
}
