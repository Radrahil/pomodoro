use std::process::Command;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    // after 5 seconds sends a notification.
    loop {
        if now.elapsed().as_secs() > 5 {
            Command::new("notify-send")
                .arg("5 seconds have passed")
                .output()
                .expect("failed to execute process");
            Command::new("spd-say")
                .arg("5 seconds have passed")
                .output()
                .expect("failed to execute process");
            break;
        }
    }
}
