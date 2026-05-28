use chrono::Local; // importing lib for time and dates

pub fn get_time() -> String {
    let now = Local::now(); // get current time

    format!(
        "Time: {}\nDate: {}\nDay: {}\nTimezone: {}",
        now.format("%H:%M:%S"), // format time
        now.format("%d-%m-%Y"), // format date
        now.format("%A"),       // format day
        now.format("%Z")        // format timezone
    ) // format output
}
