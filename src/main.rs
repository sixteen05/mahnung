use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::thread::sleep;

use chrono::{Local, Timelike};
use cron_tab::Cron;
use notify_rust::Notification;

const PID_FILE_NAME: &str = ".mahnung-pid";

fn main() {
    let all_args: Vec<String> = args().collect();
    let stop_command = String::from("stop");

    match all_args.get(1) {
        None => {
            start_cron();
            write_pid_to_file();

            // Run an infinite loop so that only killing the server would be an option
            loop {
                sleep(std::time::Duration::from_secs(10));
                if check_to_stop() {
                    break;
                }
            }
        }
        Some(cmd) => {
            if cmd.eq(&stop_command) {
                write_stop_to_file();
            } else {
                println!("This commands are not supported: {}", cmd);
            }
        }
    }
}

fn start_cron() {
    println!("Started a cronjob at 0sec *min *hour *day *month *dayOfWeek *year");
    // Get current timezone and create a cron, with that timezone
    let local_tz = Local::now().timezone();
    let mut minutely_cron = Cron::new(local_tz);

    let _job_id = minutely_cron
        .add_fn("0 0 * * * * *", send_notification)
        .unwrap();

    minutely_cron.start();
}

fn write_stop_to_file() {
    let file_result = File::create(PID_FILE_NAME);
    match file_result {
        Ok(mut f) => {
            match f.write_all(b"STOP") {
                Ok(_) => println!("Stop command sent, successfully"),
                Err(_) => println!("Could not send stop command"),
            }
        }
        Err(_) => println!("Could not write PID into file.")
    }
}

fn check_to_stop() -> bool {
    let file_result = File::open(PID_FILE_NAME);
    match file_result {
        Ok(mut f) => {
            let mut content = String::new();
            match f.read_to_string(&mut content) {
                Ok(_) => content.eq(&String::from("STOP")),
                Err(_) => false
            }
        }
        Err(_) => false
    }
}

fn write_pid_to_file() {
    let current_process_id = process::id();

    println!("Got current process ID as {}\nWriting it in .mahnung-pid", current_process_id);
    let file_result = File::create(".mahnung-pid");
    match file_result {
        Ok(mut f) => {
            match f.write_all(format!("{}", current_process_id).as_bytes()) {
                Ok(_) => (),
                Err(_) => println!("Could not write PID into file. Stop command will not work")
            }
            ()
        }
        Err(_) => println!("Could not write PID into file.")
    }
}

fn send_notification() {
    // Get the current time
    let current_hour = Local::now().hour();
    let current_min = Local::now().minute();
    // Only show notification if current time is around 10m, else skip
    // Happens when cron could not be triggered on time
    if current_min < 10 {
        let _notification = Notification::new()
            .summary(&format!("Its {} o'clock", current_hour))
            // This will auto dismiss the notification
            .timeout(100)
            .show();
    }
}

