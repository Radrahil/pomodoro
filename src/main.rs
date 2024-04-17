use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let time = Instant::now();
    let work = Duration::from_secs(25 * 60);

    let mut stdout = stdout();

    stdout.execute(cursor::Hide).unwrap();

    println!("Basic pomodoro timer. CTRL-C to stop");

    loop {
        if time.elapsed() > work {
            Command::new("notify-send")
                .arg("Break time!")
                .output()
                .expect("failed to execute process");

            Command::new("spd-say")
                .arg("Break time!")
                .output()
                .expect("failed to execute process");

            write!(stdout, "\rTime left: 00:00\n").unwrap();
            stdout.flush().unwrap(); // Flush buffer to ensure immediate output
            println!("Break time!");

            break; // Exit loop once the condition is met
        } else {
            // Print time left
            let elapsed_time = time.elapsed().as_secs();
            let minutes_left = 25 - elapsed_time.div_euclid(60) - 1;
            let seconds_left = 60 - elapsed_time.rem_euclid(60) - 1;
            write!(
                stdout,
                "\rTime left: {:02}:{:02}",
                minutes_left, seconds_left
            )
            .unwrap();
            stdout.flush().unwrap(); // Flush buffer to ensure immediate output

            thread::sleep(Duration::from_millis(500)); // Sleep for a short duration
        }
    }

    stdout.execute(cursor::Show).unwrap();

    // Wait for 5 minutes
    thread::sleep(Duration::from_secs(5 * 60));
    Command::new("notify-send")
        .arg("Break over")
        .output()
        .expect("failed to execute process");
    println!("Break over");

    Command::new("spd-say")
        .arg("Break time!")
        .output()
        .expect("failed to execute process");
}
