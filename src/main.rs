use std::thread::sleep;

use chrono::{Local, Timelike};
use cron_tab::Cron;
use notify_rust::Notification;

fn main() {
    println!("Started a cronjob at 0sec *min *hour *day *month *dayOfWeek *year");

    // Get current timezone and create a cron, with that timezone
    let local_tz = Local::now().timezone();
    let mut minutely_cron = Cron::new(local_tz);

    let _job_id = minutely_cron
        .add_fn("0 * * * * * *", send_notification)
        .unwrap();

    minutely_cron.start();

    // Run an infinite loop so that only killing the
    loop {
        sleep(std::time::Duration::from_secs(10));
    }
}

fn send_notification() {
    // Get the current time
    let current_hour = Local::now().hour();
    let _notification = Notification::new()
        .summary(&format!("Its {} o'clock", current_hour))
        // This will auto dismiss the notification
        .timeout(100)
        .show();
}

