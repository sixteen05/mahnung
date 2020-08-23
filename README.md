# mahnung
A simple script to send you hourly reminder

It uses Linux's inbuilt [notify-send](http://manpages.ubuntu.com/manpages/xenial/man1/notify-send.1.html)
to send system notifications every hour.

# Usage

 - Download the release
    ```bash
    wget https://github.com/sixteen05/mahnung/releases/download/<release_number>/mahnung
    ```
 - Make it executable
    ```bash
    chmod +x mahnung
    ```
 - Run the script in background.
   ```bash
   mahnung &
   ```
   This would print debug messages by default, so it is better to add `2>&1 >/dev/null` in the end before `&`
 - You will be notified every hour.
 - To stop the notifications, run
   ```bash
   mahnung stop
   ```

# Building

 - Clone the repo.
 - Make sure Cargo and Rust are installed.
 - Run ```cargo build```
 - To create a release ```cargo build --release```



