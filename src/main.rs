use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::Source;
use rodio::{source::SineWave, Sink};
use std::io::{self, stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;

#[derive(Parser)]
struct Opt {
    #[clap(default_value = "25")]
    work_interval: u64,
    #[clap(default_value = "5")]
    short_break: u64,
    #[clap(default_value = "20")]
    long_break: u64,
    #[clap(default_value = "4")]
    long_break_interval: u64,
}

struct Interval {
    name: &'static str,
    color: &'static str,
    duration: u64,
    message_done: &'static str,
}

fn main() {
    let opt = Opt::parse();
    println!("Work interval: {} minute(s)", opt.work_interval);
    println!("Short break: {} minute(s)", opt.short_break);
    println!("Long break: {} minute(s)", opt.long_break);
    println!(
        "Long break interval: {} work intervals",
        opt.long_break_interval
    );
    println!("Press Ctrl+C to stop the timer\n");

    let mut long_break_counter = 0;
    loop {
        execute_interval(Interval {
            name: "Work",
            color: "blue",
            duration: opt.work_interval,
            message_done: "Well done!",
        });
        long_break_counter += 1;
        if long_break_counter == opt.long_break_interval {
            execute_interval(Interval {
                name: "Long break",
                color: "green",
                duration: opt.long_break,
                message_done: "Move on!",
            });
            long_break_counter = 0;
        } else {
            execute_interval(Interval {
                name: "Short break",
                color: "yellow",
                duration: opt.short_break,
                message_done: "Back to work!",
            });
        }
    }
}

fn execute_interval(
    Interval {
        name,
        color,
        duration,
        message_done,
    }: Interval,
) {
    let seconds = duration * 60;
    let bar = ProgressBar::new(seconds);
    bar.set_prefix(format!("{: >15}", name));
    bar.set_style(
        ProgressStyle::default_bar()
            .template(format!("{{prefix}} {{bar:50.{color}/gray}} {{msg}}").as_str())
            .unwrap(),
    );
    for _ in 0..seconds {
        bar.inc(1);
        let left = seconds_to_minutes_seconds(seconds - bar.position());
        bar.set_message(left + " left");
        thread::sleep(Duration::from_millis(1000));

        // // Handle the case where the user stops the timer
        // if let Ok(key) = get_user_input() {
        //     if key == 'c' {
        //         bar.finish();
        //         println!("Timer stopped by user");
        //         return;
        //     }
        // }
    }
    bar.set_message(message_done);
    bar.finish();
    play_sound();
}

fn seconds_to_minutes_seconds(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);

    // Extract minutes and seconds
    let minutes = duration.as_secs() / 60;
    let remaining_seconds = duration.as_secs() % 60;

    // Format the string with abbreviation (m for minutes, s for seconds)
    if minutes > 0 {
        format!("{}min {}sec", minutes, remaining_seconds)
    } else {
        format!("{}sec", remaining_seconds) // Handle cases where only seconds are present
    }
}

fn play_sound() {
    // Create a sink to play the sound
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Generate a sine wave sound source
    let source = SineWave::new(440.0).take_duration(Duration::from_secs(1));

    // Add the sound source to the sink and play it
    sink.append(source);
    sink.play();

    // Sleep for the duration of the sound
    thread::sleep(Duration::from_secs(1));
}

fn get_user_input() -> Result<char, std::io::Error> {
    // Disable terminal echo
    let mut mode = stdout().into_raw_mode()?;

    // Get a single keystroke
    let mut input = [0; 1];
    io::stdin().read_exact(&mut input)?;

    // Restore terminal echo
    mode.flush()?;

    Ok(input[0] as char)
}
