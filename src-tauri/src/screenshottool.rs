use crate::parse_chat_response_body;
use screenshots::Screen;
use serde::Serialize;
use tauri::Manager;

pub fn take_screenshot()
    -> Result<String, String>
{
    let screens =
        Screen::all()
            .map_err(|e| e.to_string())?;

    let screen =
        screens
            .first()
            .ok_or("No screen found")?;

    let image =
        screen
            .capture()
            .map_err(|e| e.to_string())?;

    let path = "screenshot.png";

    image
        .save(path)
        .map_err(|e| e.to_string())?;

    Ok(path.to_string())
}

use base64::{
    engine::general_purpose,
    Engine as _
};

pub fn image_to_base64(
    path: &str
) -> Result<String,String> {

    let bytes =
        std::fs::read(path)
            .map_err(|e| e.to_string())?;

    Ok(
        general_purpose::STANDARD
            .encode(bytes)
    )
}








#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize)]
struct ContentPart {
    #[serde(rename="type")]
    kind: String,

    text: Option<String>,

    image_url: Option<ImageUrl>,
}

#[derive(Serialize)]
struct VisionMessage {
    role: String,
    content: Vec<ContentPart>,
}



pub async fn analyze_screen(
    window: tauri::AppHandle,
    api_key: String,
) -> Result<String, String> {

    let window = window
        .get_webview_window("main")
        .ok_or("Window not found")?;

    window.minimize()
        .map_err(|e| e.to_string())?;

    tokio::time::sleep(
        std::time::Duration::from_millis(500)
    ).await;

    let path =
        take_screenshot()?;

    window.unminimize()
        .map_err(|e| e.to_string())?;

    window.show()
        .map_err(|e| e.to_string())?;

    let image =
        image_to_base64(
            &path
        )?;

    let image_url = format!(
        "data:image/png;base64,{}",
        image
    );

    let messages = vec![
        VisionMessage {
            role: "user".into(),
            content: vec![
                ContentPart {
                    kind: "text".into(),
                    text: Some(
                        "Describe my screen briefly."
                            .into()
                    ),
                    image_url: None,
                },
                ContentPart {
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
        "model":
            "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free",
        "messages":
            messages
    });

    let client =
        reqwest::Client::new();

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

        if !response.status().is_success() {

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

    let _ =
        std::fs::remove_file(
            &path
        );

    result
}