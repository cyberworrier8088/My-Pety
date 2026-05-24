use std::fs;
use std::io::{self, Write};

const CONFIG_FILE: &str = "Quotes.env";

pub fn ensure_config_file() -> Result<(), String> {
    if fs::metadata(CONFIG_FILE).is_ok() {
        return Ok(());
    }

    let api_key = ask("Enter your Hack Club AI API key")
        .map_err(|e| e.to_string())?;

    let config_text = format!("HACKCLUB_AI_KEY={}\n", api_key);
    fs::write(CONFIG_FILE, config_text).map_err(|e| e.to_string())?;

    println!("Saved setup in {}.", CONFIG_FILE);
    Ok(())
}

pub fn load_api_key() -> Result<String, String> {
    if let Ok(value) = std::env::var("HACKCLUB_AI_KEY") {
        let trimmed = value.trim().to_string();
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }

    ensure_config_file()?;

    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| e.to_string())?;
    for line in content.lines() {
        if let Some(value) = line.strip_prefix("HACKCLUB_AI_KEY=") {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Ok(trimmed);
            }
        }
    }

    Err("HACKCLUB_AI_KEY not found in Quotes.env".to_string())
}

fn ask(label: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{}: ", label);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
