#[tauri::command]
pub async fn open_website(url: String) -> Result<String, String> {
    let target = normalize_target(&url);

    open::that(&target).map_err(|e| e.to_string())?;

    Ok(format!("Opened {}", target))
}

fn normalize_target(input: &str) -> String {
    let value = input.trim();

    if value.starts_with("http://") || value.starts_with("https://") {
        return value.to_string();
    }

    if value.starts_with("localhost") || value.starts_with("127.0.0.1") {
        return format!("http://{}", value);
    }

    if value.contains(' ') {
        return format!(
            "https://duckduckgo.com/?q={}",
            urlencoding::encode(value)
        );
    }

    if value.contains('.') {
        return format!("https://{}", value);
    }

    format!("https://www.{}.com", value)
}
