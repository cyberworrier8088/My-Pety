use std::collections::HashMap;
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = "Quotes.env";

#[tauri::command]
pub fn save_config(
    api_key: String,
    name: String,
    password: String,
    pet_name: String,
    pet_type: String,
) -> Result<(), String> {

    let config_text = format!(
        "HACKCLUB_AI_KEY={}\nNAME={}\nPASSWORD={}\nPET_NAME={}\nPET_TYPE={}\n",
        api_key,
        name,
        password,
        pet_name,
        pet_type
    );

    std::fs::write(
        CONFIG_FILE,
        config_text
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn config_exists() -> bool {
    Path::new(CONFIG_FILE).exists()
}

pub fn load_api_key() -> Result<String, String> {
    load_config_value("HACKCLUB_AI_KEY")
}

pub fn load_name() -> Result<String, String> {
    load_config_value("NAME")
}

pub fn load_password() -> Result<String, String> {
    load_config_value("PASSWORD")
}

pub fn pet_name() -> Result<String, String> {
    load_config_value("PET_NAME")
}

pub fn load_pet_type() -> Result<String, String> {
    load_config_value("PET_TYPE")
}

fn load_config_value(key: &str) -> Result<String, String> {
    let values = read_config_map()?;

    values
        .get(key)
        .cloned()
        .ok_or_else(|| format!("{key} not found in {CONFIG_FILE}"))
}

fn read_config_map() -> Result<HashMap<String, String>, String> {
    let content =
        fs::read_to_string(CONFIG_FILE)
            .map_err(|e| e.to_string())?;

    let mut values = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) =
            line.split_once('=')
        {
            values.insert(
                key.trim().to_string(),
                value.trim().to_string(),
            );
        }
    }

    Ok(values)
}