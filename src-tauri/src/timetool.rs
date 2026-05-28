use chrono::Local;

pub fn get_time() -> String {
    let now = Local::now();

    format!(
        "Time: {}\nDate: {}\nDay: {}\nTimezone: {}",
        now.format("%H:%M:%S"),
        now.format("%d-%m-%Y"),
        now.format("%A"),
        now.format("%Z")
    )
}