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
 - You will be notified every hour.
 - To stop the notifications, run
   ```bash
   mahnung stop
   ```




