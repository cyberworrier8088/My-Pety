use std::thread;
use std::time::Duration;
use notify_rust::Notification;

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
