use std::thread::sleep;

use chrono::Local;
use cron_tab::Cron;
use notify_rust::Notification;

fn main() {
    println!("Started a cronjob at *sec 0min *hour *day *month *dayOfWeek *year");

    send_notification();

    loop {
        sleep(std::time::Duration::from_secs(10));
    }

    // Get current timezone and create a cron, with that timezone
    /*
    let local_tz = Local::now().timezone();
    let mut minutely_cron = Cron::new(local_tz);

    let _job_id = minutely_cron
        .add_fn("0 * * * * * *", || println!("Current time is {:?}", Local::now()))
        .unwrap();

    minutely_cron.start();

    loop {
        sleep(std::time::Duration::from_secs(10));
    }
    */
}

fn send_notification() {
    let notification = Notification::new()
        .summary("Its x o'clock")
        .show();
}

