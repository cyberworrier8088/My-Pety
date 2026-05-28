use std::thread; // importing thread module
use std::time::Duration; // importing duration module
use notify_rust::Notification; // importing notification module

pub fn parse_reminder_input(input: &str) -> Result<(u64, String), String> {
    let trimmed = input.trim(); // trimming input
    let lower = trimmed.to_lowercase(); // converting to lowercase

    let rest = lower.strip_prefix("remind me") // stripping "remind me" from input
        .map(|_| trimmed["remind me".len()..].trim())
        .ok_or_else(|| {
            "Usage: remind me 30s stand up, remind me 5m drink water, remind me in 10 minutes check mail".to_string()
        })?;

    if rest.is_empty() { // 
        return Err(
            "Usage: remind me 30s stand up, remind me 5m drink water, remind me in 10 minutes check mail".to_string()
        );
    }

    if let Some(after_in) = rest.strip_prefix("in ") { // if rest starts with "in "
        return parse_in_style_reminder(after_in.trim());
    }

    parse_short_style_reminder(rest) // parsing short style reminder
}

pub fn create_reminder( // creating reminder func
    seconds: u64,
    message: String
) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds)); // sleeping for specified seconds

        let _ = Notification::new().summary("pet").body(&message).show(); // showing notification
    });
}

fn parse_in_style_reminder(rest: &str) -> Result<(u64, String), String> { // parsing in style reminder
    let parts = rest.split_whitespace().collect::<Vec<_>>(); // splitting rest into parts

    if parts.len() < 3 { // if parts length is less than 3
        return Err("Usage: remind me in 10 minutes check mail".to_string()); // returning error
    }

    let amount = parts[0].parse::<u64>().map_err(|_| "Reminder time must start with a number.".to_string())?; // parsing amount

    let unit = parts[1]; // getting unit
    let message = parts[2..].join(" "); // joining message

    if message.trim().is_empty() {
        return Err("Please add a reminder message.".to_string()); // returning error
    }

    let seconds = unit_to_seconds(amount, unit)?; // converting unit to seconds
    Ok((seconds, message)) // returning seconds and message
}

fn parse_short_style_reminder(rest: &str) -> Result<(u64, String), String> {
    let parts = rest.split_whitespace().collect::<Vec<_>>(); // splitting rest into parts

    if parts.len() < 2 { // if parts length is less than 2
        return Err(
            "Usage: remind me 30s stand up, remind me 5m drink water, remind me 1h sleep".to_string()
        );
    }

    let time_token = parts[0]; // getting time token
    let message = parts[1..].join(" "); // joining message

    if message.trim().is_empty() {
        return Err("Please add a reminder message.".to_string());
    }

    let seconds = if let Ok(amount) = time_token.parse::<u64>() { // parsing time token
        amount
    } else {
        parse_compact_duration(time_token)? // parsing compact duration
    };

    Ok((seconds, message)) // reyirn secoponds and msg
}

fn parse_compact_duration(token: &str) -> Result<u64, String> {
    let split_index = token.find(|c: char| !c.is_ascii_digit()).ok_or_else(|| "Reminder time format is invalid.".to_string())?; // finding split index

    let (amount_text, unit) = token.split_at(split_index); // splitting token

    if amount_text.is_empty() || unit.is_empty() { // checking if amount text or unit is empty
        return Err("Reminder time format is invalid.".to_string());
    }

    let amount = amount_text
        .parse::<u64>()
        .map_err(|_| "Reminder time must start with a number.".to_string())?; // parsing amount

    unit_to_seconds(amount, unit) // converting unit to seconds
}

fn unit_to_seconds(amount: u64, unit: &str) -> Result<u64, String> {
    match unit.to_lowercase().as_str() { // converting unit to lowercase
        "s" | "sec" | "secs" | "second" | "seconds" => Ok(amount), // returning amount
        "m" | "min" | "mins" | "minute" | "minutes" => Ok(amount * 60), // returning amount * 60
        "h" | "hr" | "hrs" | "hour" | "hours" => Ok(amount * 60 * 60), // returning amount * 60 * 60
        _ => Err("Supported reminder units: seconds, minutes, hours.".to_string()), // returning error
    }
}
