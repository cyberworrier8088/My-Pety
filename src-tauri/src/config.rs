use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = ".mypety.env"; // this is the config file name where we saveing all things




#[tauri::command] // tauri command decorator. allows this function to be called from the frontend.
pub fn save_location(
    lat: f64, // latitude of user location
    lon: f64, // longitude of user location
) -> Result<(), String> { // saveing the location lat and lon to config. location saving
    let mut values = read_config_map().unwrap_or_default(); // load the config map first. map loading
    values.insert("LAT".to_string(), lat.to_string()); // insert lat value
    values.insert("LON".to_string(), lon.to_string()); // insert lon value

    write_config_map(&values)?; // write the map back to config file. writeing

    Ok(())
}

#[tauri::command] // tauri command decorator. allows this function to be called from the frontend.
pub fn save_config(api_key: String, name: String, password: String, pet_name: String, pet_type: String, pet_asset: String) -> Result<(), String> { // this func have all configs. saving it
    let password_hash = hash_password(&password)?; // hashing password for securty

    let pet_asset = normalize_pet_asset(&pet_asset); // normalize  value

    let mut values = read_config_map().unwrap_or_default(); // load current values

    values.insert("HACKCLUB_AI_KEY".to_string(), api_key); // insert  value

    values.insert("NAME".to_string(), name); // insert value

    values.insert("PASSWORD".to_string(), password_hash); // insert  value

    values.insert("PET_NAME".to_string(), pet_name); // insert  value

    values.insert("PET_TYPE".to_string(), pet_type); // insert  value

    values.insert("PET_ASSET".to_string(), pet_asset); // insert  value

    write_config_map(&values)?; // write  value to file

    Ok(())
}


pub fn load_location() -> Result<(f64, f64), String> { // load location from config. returns lat and lon in result
    let lat = load_config_value("LAT")?.parse::<f64>().map_err(|e| e.to_string())?; // parse to float

    let lon = load_config_value("LON")?.parse::<f64>().map_err(|e| e.to_string())?; // parse to float

    Ok((lat, lon))  // error handling 
}

#[tauri::command] // tauri command decorator. allows this function to be called from the frontend.


pub fn config_exists() -> bool { Path::new(CONFIG_FILE).exists() } // checking if config exists

pub fn load_api_key() -> Result<String, String> { load_config_value("HACKCLUB_AI_KEY") } // loading api key

pub fn load_name() -> Result<String, String> { load_config_value("NAME") } 

pub fn load_password() -> Result<String, String> { load_config_value("PASSWORD") }

pub fn verify_password(password: &str) -> Result<bool, String> { // verify if input password matches saved hash
    
    
    let saved_hash = load_password()?; // load saved hash first

    let parsed_hash = PasswordHash::new(&saved_hash).map_err(|e| e.to_string())?; // parsing hash string into hash struct

    Ok(Argon2::default().verify_password(password.as_bytes(),&parsed_hash).is_ok()) // verify using argon2, returns bool if it ok
}

pub fn pet_name() -> Result<String, String> { load_config_value("PET_NAME") }

pub fn load_pet_type() -> Result<String, String> { load_config_value("PET_TYPE") }

#[tauri::command] // tauri command decorator


pub fn load_pet_asset() -> String { read_config_map()
        .ok()
        .and_then(|values| values.get("PET_ASSET").cloned()) // get asset value
        .map(|asset| normalize_pet_asset(&asset)) // normalize it
        .unwrap_or_else(|| "ferris".to_string())
}

fn load_config_value(key: &str) -> Result<String, String> { // get a specific value from the config map by key
    let values = read_config_map()?; // load all config values map

    values.get(key).cloned().ok_or_else(|| format!("{key} not found in {CONFIG_FILE}"))
}

fn read_config_map() -> Result<HashMap<String, String>, String> { // read the env file and parse into a hash map of keys and values
    let content = fs::read_to_string(CONFIG_FILE).map_err(|e| e.to_string())?;

    let mut values = HashMap::new(); // create new hashmap

    for line in content.lines() { // loop through lines and split by = to get key value
        if let Some((key, value)) = line.split_once('=') { // split into two parts by =

            values.insert(key.trim().to_string(), value.trim().to_string()); // trim whitespace from key and value
        }
    }

    Ok(values)
}

fn write_config_map(values: &HashMap<String, String>) -> Result<(), String> { // write the hash map back to the env file
    let mut keys = values.keys().cloned().collect::<Vec<_>>(); // collect and sort keys so it ordered
    keys.sort(); // sort them

    let content = keys.into_iter().filter_map(|key| {
            values.get(&key).map(|value| {
                format!("{key}={value}") // format as KEY=VALUE
            })
        }).collect::<Vec<_>>().join("\n"); // join lines with newline

    fs::write(CONFIG_FILE, format!("{content}\n")).map_err(|e| e.to_string()) // write content to file

}

fn normalize_pet_asset(asset: &str) -> String { // make pet asset lowercase and match to dog, cat, or ferris
    match asset.trim().to_lowercase().as_str() { // check type of asset
        "dog" => "dog".to_string(),
        "cat" => "cat".to_string(),
        _ => "ferris".to_string(), // return ferris as default if no match
    }
}

fn hash_password(password: &str) -> Result<String, String> { // hash password using argon2. password must be at least 8 char long
    if password.len() < 8 { // check length is correct or too small
        return Err(
            "Password must be at least 8 characters long."
                .to_string()
        );
    }

    let salt = SaltString::generate(&mut OsRng); // generate random salt

    Argon2::default().hash_password(password.as_bytes(), &salt).map(|hash| hash.to_string()).map_err(|e| e.to_string()) // hash it using argon2
}
