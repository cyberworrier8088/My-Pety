/// open web and real world info collection

#[tauri::command]


pub async fn open_website(url: String) -> Result<String, String> { // open website func
    let target = normalize_target(&url); // normalize target

    open::that(&target).map_err(|e| e.to_string())?; // open target

    Ok(format!("Opened {}", target)) // return success message
}

fn normalize_target(input: &str) -> String { // normalize target func
    let value = input.trim(); // trim input

    if value.starts_with("http://") || value.starts_with("https://") { // http or https
        return value.to_string();
    }

    if value.starts_with("localhost") || value.starts_with("127.0.0.1") { // localhost or 127.0.0.1
        return format!("http://{}", value);
    }

    if value.contains(' ') { // contains space
        return format!("https://duckduckgo.com/?q={}", urlencoding::encode(value)); // search on duckduckgo
    }

    if value.contains('.') { // contains dot
        return format!("https://{}", value);
    }

    format!("https://www.{}.com", value) // default
}
