// src/memory.rs


/// importtandt module in the project
/// feture imporve
/// coming soon


use std::collections::HashMap; 
use std::fs;

const MEMORY_FILE: &str = "memory.json"; /// memory file path

pub fn save_memory(key: String, value: String) -> Result<(), String> { // saving memory
    let mut memories = load_memories(); // loading existing memories
    memories.insert(key, value); // inserting new memory

    let json = serde_json::to_string_pretty(&memories).map_err(|e| e.to_string())?; // converting to json

    fs::write(MEMORY_FILE, json).map_err(|e| e.to_string())?; // writing to file

    Ok(())
}

pub fn load_memories() -> HashMap<String,String> { // loading memories
    std::fs::read_to_string(MEMORY_FILE) // reading file
    .ok() // handling error
    .and_then(|content| { // handling content
        serde_json::from_str(
            &content
        ).ok()
    })
    .unwrap_or_default()
}

pub fn get_memory(key: &str) -> Option<String> { // getting memory
    load_memories().get(key).cloned() // returning memory
}
