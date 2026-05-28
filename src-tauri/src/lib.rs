// src/lib.rs

// this is the main lib file for the tauri app
// it contains all the commands and logic for the app
// calling other modules for specific func



// import modules ca;lling
////////////////////////
mod config;
mod opensitetool;
mod readertool;
mod sysinfo;
mod timetool;
mod webtool;
mod remindertool;
mod memory;
mod battery;
mod screenshottool;
mod apptool;
mod weather;
mod brightnesstool;
mod volumetool;
////////////////////////




// using crate modules and external libraries
////////////////////////////////////////////////////////////////
use crate::config::config_exists;
use crate::config::save_config;
use crate::opensitetool::open_website;
use serde::{Deserialize, Serialize}; // for json serialization
use std::time::Duration; // for timing
////////////////////////////////////////////////////////////////


// define structs for chat messages and requests
// derive serialize and deserialize for json. derive means it will automatically implement the serialize and deserialize traits
/////////////////////////////////
#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Deserialize)]
struct AssistantMessage {
    content: Option<String>,
}
/////////////////////////////////



// parse chat response body
////////////////////////////////////////////////////////////////////////////////
pub(crate) fn parse_chat_response_body(body: &str) -> Result<String, String> { // parse the response body from the ai api
    
    let data: ChatResponse = serde_json::from_str(body).map_err(|error| {
        // get a preview of the response body
        let preview = body.chars().take(240).collect::<String>();
        
        // returnig error msg
        format!("Could not read AI response. {}\nResponse preview: {}", error, preview)
    })?;

    data.choices.into_iter().next().and_then(|choice| choice.message.content).ok_or_else(|| "No response from AI :}".to_string())
}

////////////////////////////////////////////////////////////////////////////////




////////////////////////////////////////////////////////////////////////////////
// ask hackclub ai func                                                        /
////////////////////////////////////////////////////////////////////////////////
#[tauri::command] // tauri command decorator. allows this function to be called from the frontend.


async fn ask_hackclub_ai(app: tauri::AppHandle, prompt: String) -> Result<String, String> { // async function that takes a prompt and returns a result of a string or an error message
    
    let original_prompt = prompt.trim().to_string(); // trim the prompt and convert it to a string. orginal
    
    if original_prompt.is_empty() { // check if the prompt is empty
        return Err("Prompt cannot be empty".to_string());
    }

    // check if the prompt is too long, this method to reduce the token usage = cost reduction
    if original_prompt.len() > 120 { 
        return Err("Prompt too longg".to_string());
    }

    let lower = original_prompt.to_lowercase(); // convert the prompt to lowercase

    // this if else for using tools
    if lower.contains("time") { // check if the prompt contains "time"
        let result = timetool::get_time(); // get the current time using the timetool module
        return Ok(format!("The current time is {}.", result)); // return the current time
    }

    if lower.contains("ram") || lower.contains("system info") || lower.contains("system memory") || lower.contains("memory usage") { // check if the prompt contains any of these keywords
        // we using sysinfo module to get system info
        return Ok(sysinfo::get_system_info()); // return the system info
    }

    if lower.contains("apps") || lower.contains("application") { // check if the prompt contains "apps" or "application"
        return Ok(sysinfo::get_app_list()); // return the app list
    }
    if lower.contains("close") || lower.contains("exit") || lower.contains("quit") { // check if the prompt contains any of these keywords
        
        // this for closing the full pet app
        return Ok("CLOSE_APP".to_string());
    }
    if lower.starts_with("remind me") { // check if the prompt starts with "remind me"
        let (seconds, message) = remindertool::parse_reminder_input(&original_prompt)?; // parse the reminder input

        remindertool::create_reminder(seconds, message.clone()); // create the reminder

        return Ok(format!("Reminder set for {} seconds: {}", seconds, message)); // return the reminder
    }


    let mut final_prompt = original_prompt.clone(); // create a new string with the same value as the original prompt

    if lower.contains("search") || lower.contains("web") || lower.contains("google") || lower.contains("duckduckgo") || lower.contains("bing") || lower.contains("today") || lower.contains("latest") { // check if the prompt contains any of these keywords
        
        let search_result = webtool::duckduckgo_search(&original_prompt).await?; // search the web using duckduckgo, and await the result, webtool using
        
        final_prompt = format!("Search result:\n{}\n\nUser question:\n{}", search_result, original_prompt); // format the final prompt
    }


    // this function have probm
    if lower.contains("read") || lower.contains("file") { // check if the prompt contains "read" or "file"
        
        if let Some(file_path) = original_prompt.split_whitespace().nth(1) { // get the file path from the prompt
            
            let file_content = readertool::read_file(file_path.to_string()); // readertool::read_file reads the file
                
                final_prompt = format!("File content:\n{}\n\nUser question:\n{}", file_content, original_prompt); // format the final prompt
        } else {
            
            return Err("Please provide a file path.".to_string()); // return an error if no file path is provided
        }
    }




    if lower.starts_with("my favorite language is ") { // check if the prompt starts with that

    let value = original_prompt.trim_start_matches("my favorite language is ").to_string(); // trim the prompt and get the value

    memory::save_memory("favorite".into(), value.clone())?; // save the memory using the memory module

    return Ok(format!("I'll remember that! {}", value)); // return the response
    
    }

    if lower.contains("favorite language") { // check if the prompt contains this

    if let Some(favorite) = memory::get_memory("favorite") { // get the favorite language from the memory
        
        return Ok(format!("Your favorite {}", favorite)); // return the response
    }
}

    let memories = memory::load_memories(); // load the memories
    
    let memory_text = memories.iter().map(|(k,v)| { // map the memories to a string

        format!("{}: {}", k, v) // format the memory

    }).collect::<Vec<_>>().join("\n"); // join the memories with a new line



        if lower.starts_with("remember ") { // check if the prompt this

            let text = original_prompt.trim_start_matches("remember "); // trim the prompt

            if let Some((key,value)) = text.split_once('=') { // split the prompt into key and value
                memory::save_memory(key.trim().into(), value.trim().into())?; // save the memory
            }

            return Ok("Memory saved!".to_string()); // return the response
    }


    if lower == "show memories" { // show all memories

        let memories = memory::load_memories(); // load the memories

        return Ok(format!("{:#?}", memories)); // return the memories
    }

    if lower.contains("battery") || lower.contains("charge") || lower.contains("power") { // check if the prompt contains any of these
        return Ok(battery::get_battery_info()); // return the battery info
    }




    if lower.contains("screen") || lower.contains("screenshot") { // this checks
        let api_key = config::load_api_key()?; // load the api key

        return screenshottool::analyze_screen(app, api_key).await; // analyze the screen func
    }


    // Open applications
    if lower.starts_with("open app ") { // check

        let app_name = original_prompt.trim_start_matches("open app "); // trim the prompt

        return apptool::open_app(app_name); // open the app
}

    // Open websites
    if let Some(target) = original_prompt.strip_prefix("open ") { // check
        return opensitetool::open_website(target.trim().to_string()).await; // open the website
    }

    if lower.contains("weather") { // check

        let (lat, lon) = config::load_location().map_err(|_| {"Location is not set yet. Please allow location access and reopen the app, then try weather again.".to_string()})?; // load the location

        return weather::get_weather(lat, lon).await; // get the weather
    }


    if lower.starts_with("brightness ") { // check

        let value = original_prompt.split_once(' ').map(|(_, rest)| rest.trim()).ok_or_else(|| {"Invalid brightness level. Please provide a number between 0 and 100.".to_string()})?; // get the value

        let level = value.parse::<u8>().map_err(|_| {"Invalid brightness level. Please provide a number between 0 and 100.".to_string()})?; // parse the value
        return brightnesstool::set_brightness(level); // set the brightness
    }

    if lower.starts_with("volume ") {
        let value = original_prompt.split_once(' ').map(|(_, rest)| rest.trim()).ok_or_else(|| {"Invalid volume level. Please provide a number between 0 and 100.".to_string()})?; // get the value

        let level = value.parse::<u8>().map_err(|_| {"Invalid volume level. Please provide a number between 0 and 100.".to_string()})?; // parse the value
        return volumetool::set_volume(level); // set the volume
    }


    
    // this for config loading
    //////////////////////////////////////////////////////////////////////////////
    let name = config::load_name().unwrap_or_else(|_| "friend".to_string());
    let pet_name = config::pet_name().unwrap_or_else(|_| "Moxi".to_string());
    let pet_type = config::load_pet_type().unwrap_or_else(|_| "cat".to_string());
    let api_key = config::load_api_key()?;
    ///////////////////////////////////////////////////////////////////////////////


    let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build().map_err(|e| e.to_string())?; // build the client using reqwest

    let body = ChatRequest {
        model: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free".to_string(), // model name
        messages: vec![
            ChatMessage {
                role: "system".into(), // roll of system, system means instruction for the model
                content: format!("Respond like a friendly {}. Use a short, cute style.", pet_type),
            },
            ChatMessage {
                role: "system".into(), // roll of system
                content: format!("Memories:\n{}", memory_text),
            },
            ChatMessage {
                role: "system".into(), // roll of system
                content: format!("The user's name is {}.", name),
            },
            ChatMessage {
                role: "system".into(), // roll of system
                content: format!("Your name is {}.", pet_name),
            },
            ChatMessage {
                role: "user".into(), // roll of user
                content: final_prompt,
            },
        ],
    };

    ///////////////////////////////////////////////////////////////
    // send the request to the API
    //////////////////////////////////////////////////////////////
    let response = client
        .post("https://ai.hackclub.com/proxy/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    ////////////////////////////////////////////////////////////////

    // check if the response is successful
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}\n{}", status, body));
    }

    let body = response.text().await.map_err(|e| e.to_string())?;
    parse_chat_response_body(&body)
}





#[tauri::command]
fn check_password(password: String) -> Result<bool, String> {
    config::verify_password(&password)
}


#[tauri::command]
fn close_app(app: tauri::AppHandle) -> Result<String, String> {
    app.exit(0);
    Ok("Closing application...".to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(debug_assertions) {
        println!("Debug mode");
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ask_hackclub_ai,
            check_password,
            open_website,
            save_config,
            config_exists,
            close_app,
            config::load_pet_asset,
            config::save_location,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
