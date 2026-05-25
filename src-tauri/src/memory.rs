use std::collections::HashMap;
use std::fs;

const MEMORY_FILE: &str = "memory.json";

pub fn save_memory(
    key: String,
    value: String
) -> Result<(), String> {
    let mut memories = load_memories();
    memories.insert(key, value);

    let json = serde_json::to_string_pretty(&memories).map_err(|e| e.to_string())?;

    fs::write(MEMORY_FILE, json).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_memories()
    -> HashMap<String,String>
{
    std::fs::read_to_string(
        MEMORY_FILE
    )
    .ok()
    .and_then(|content| {
        serde_json::from_str(
            &content
        ).ok()
    })
    .unwrap_or_default()
}

pub fn get_memory(key: &str) -> Option<String> {
    load_memories().get(key).cloned()
}
