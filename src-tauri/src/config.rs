use std::fs;
use std::io::{self, Write};

const CONFIG_FILE: &str = "Quotes.env";

pub fn ensure_config_file() -> Result<(), String> {
    if fs::metadata(CONFIG_FILE).is_ok() {
        return Ok(());
    }

    let api_key = ask("Enter your Hack Club AI API key").map_err(|e| e.to_string())?;
    let name = ask("Enter your name").map_err(|e| e.to_string())?;
    let pet_name = ask("Enter your pet's name").map_err(|e| e.to_string())?;
    let password = ask("Enter your password").map_err(|e| e.to_string())?;
    let pet_type = ask("Enter your pet's type (e.g., cat, dog, bird, etc)").map_err(|e| e.to_string())?;

    let config_text = format!("HACKCLUB_AI_KEY={}\nNAME={}\nPASSWORD={}\nPET_NAME={}\nPET_TYPE={}\n", api_key, name, password, pet_name, pet_type);
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

pub fn load_name() -> Result<String, String> {
    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| e.to_string())?;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("NAME=") {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Ok(trimmed);
            }
        }
    }

    Err("NAME not found in Quotes.env".to_string())
}
pub fn pet_name() -> Result<String, String> {
    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| e.to_string())?;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("PASSWORD=") {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Ok(trimmed);
            }
        }
    }

    Err("Pet name not found in Quotes.env".to_string())
}
pub fn load_password() -> Result<String, String> {
    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| e.to_string())?;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("PASSWORD=") {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Ok(trimmed);
            }
        }
    }

    Err("PASSWORD not found in Quotes.env".to_string())
}

pub fn load_pet_type() -> Result<String, String> {
    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| e.to_string())?;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("PET_TYPE=") {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Ok(trimmed);
            }
        }
    }

    Err("PET_TYPE not found in Quotes.env".to_string())
}




fn ask(label: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{}: ", label);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
