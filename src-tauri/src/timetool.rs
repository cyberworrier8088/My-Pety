use chrono::Local;

pub fn get_time() -> String {
    Local::now().format("%H:%M").to_string()
}
