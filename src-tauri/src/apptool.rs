// src/apptool.rs

use std::process::Command; // command module

pub fn open_app(app: &str) -> Result<String, String> { // opening app function
    match app.to_lowercase().as_str() {

        // this all match use for opening app
        "notepad" => {
            Command::new("notepad").spawn().map_err(|e| e.to_string())?;
        }

        "calculator" | "calc" => {
            Command::new("calc").spawn().map_err(|e| e.to_string())?;
        }

        "paint" => {
            Command::new("mspaint").spawn().map_err(|e| e.to_string())?;
        }

        "cmd" => {
            Command::new("cmd").spawn().map_err(|e| e.to_string())?;
        }
        "chrome" => {Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe").spawn().map_err(|e| e.to_string())?;
}

        _ => {
            return Err(format!("Unknown app: {}", app)); // return error message
        }
    }

    Ok(format!("Opened {}", app)) // return success message
}
