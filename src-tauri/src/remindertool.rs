use std::thread;
use std::time::Duration;
use notify_rust::Notification;

pub fn parse_reminder_input(input: &str) -> Result<(u64, String), String> {
    let trimmed = input.trim();
    let lower = trimmed.to_lowercase();

    let rest = lower
        .strip_prefix("remind me")
        .map(|_| trimmed["remind me".len()..].trim())
        .ok_or_else(|| {
            "Usage: remind me 30s stand up, remind me 5m drink water, remind me in 10 minutes check mail".to_string()
        })?;

    if rest.is_empty() {
        return Err(
            "Usage: remind me 30s stand up, remind me 5m drink water, remind me in 10 minutes check mail".to_string()
        );
    }

    if let Some(after_in) = rest.strip_prefix("in ") {
        return parse_in_style_reminder(after_in.trim());
    }

    parse_short_style_reminder(rest)
}

pub fn create_reminder(
    seconds: u64,
    message: String
) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds));

        let _ = Notification::new()
            .summary("Moxi")
            .body(&message)
            .show();
    });
}

fn parse_in_style_reminder(rest: &str) -> Result<(u64, String), String> {
    let parts = rest.split_whitespace().collect::<Vec<_>>();

    if parts.len() < 3 {
        return Err(
            "Usage: remind me in 10 minutes check mail".to_string()
        );
    }

    let amount = parts[0]
        .parse::<u64>()
        .map_err(|_| "Reminder time must start with a number.".to_string())?;

    let unit = parts[1];
    let message = parts[2..].join(" ");

    if message.trim().is_empty() {
        return Err("Please add a reminder message.".to_string());
    }

    let seconds = unit_to_seconds(amount, unit)?;
    Ok((seconds, message))
}

fn parse_short_style_reminder(rest: &str) -> Result<(u64, String), String> {
    let parts = rest.split_whitespace().collect::<Vec<_>>();

    if parts.len() < 2 {
        return Err(
            "Usage: remind me 30s stand up, remind me 5m drink water, remind me 1h sleep".to_string()
        );
    }

    let time_token = parts[0];
    let message = parts[1..].join(" ");

    if message.trim().is_empty() {
        return Err("Please add a reminder message.".to_string());
    }

    let seconds = if let Ok(amount) = time_token.parse::<u64>() {
        amount
    } else {
        parse_compact_duration(time_token)?
    };

    Ok((seconds, message))
}

fn parse_compact_duration(token: &str) -> Result<u64, String> {
    let split_index = token
        .find(|c: char| !c.is_ascii_digit())
        .ok_or_else(|| "Reminder time format is invalid.".to_string())?;

    let (amount_text, unit) = token.split_at(split_index);

    if amount_text.is_empty() || unit.is_empty() {
        return Err("Reminder time format is invalid.".to_string());
    }

    let amount = amount_text
        .parse::<u64>()
        .map_err(|_| "Reminder time must start with a number.".to_string())?;

    unit_to_seconds(amount, unit)
}

fn unit_to_seconds(amount: u64, unit: &str) -> Result<u64, String> {
    match unit.to_lowercase().as_str() {
        "s" | "sec" | "secs" | "second" | "seconds" => Ok(amount),
        "m" | "min" | "mins" | "minute" | "minutes" => Ok(amount * 60),
        "h" | "hr" | "hrs" | "hour" | "hours" => Ok(amount * 60 * 60),
        _ => Err("Supported reminder units: seconds, minutes, hours.".to_string()),
    }
}
