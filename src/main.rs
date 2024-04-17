use std::io::{stdout, Write};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let work = Instant::now();
    let five_secs = Duration::from_secs(62);

    let mut stdout = stdout();

    loop {
        if work.elapsed() > five_secs {
            Command::new("notify-send")
                .arg("5 seconds have passed")
                .output()
                .expect("failed to execute process");

            write!(stdout, "\rTime left: 00:00\n").unwrap();
            stdout.flush().unwrap(); // Flush buffer to ensure immediate output
            println!("Break time!");

            break; // Exit loop once the condition is met
        } else {
            // Print time left
            let elapsed_time = work.elapsed().as_secs();
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

    // Wait for 5 minutes
    thread::sleep(Duration::from_secs(5));
    Command::new("notify-send")
        .arg("Another 5 seconds have passed")
        .output()
        .expect("failed to execute process");
    println!("Break over");
}

// use std::{thread, time};
// use std::io::{Write, stdout};
// use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
//
// fn main() {
//     let mut stdout = stdout();
//
//     stdout.execute(cursor::Hide).unwrap();
//     for i in (1..30).rev() {
//         stdout.queue(cursor::SavePosition).unwrap();
//         stdout.write_all(format!("{}: FOOBAR ", i).as_bytes()).unwrap();
//         stdout.queue(cursor::RestorePosition).unwrap();
//         stdout.flush().unwrap();
//         thread::sleep(time::Duration::from_millis(100));
//
//         stdout.queue(cursor::RestorePosition).unwrap();
//         stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
//     }
//     stdout.execute(cursor::Show).unwrap();
//
//     println!("Done!");
// }
//
