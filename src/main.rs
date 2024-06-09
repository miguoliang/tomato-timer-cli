use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::Source;
use rodio::{source::SineWave, Sink};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Opt {
    #[clap(default_value = "25")]
    work_interval: u64,
    #[clap(default_value = "5")]
    short_break: u64,
}

fn main() {
    let opt = Opt::parse();
    println!("Work interval: {} minute(s)", opt.work_interval);
    println!("Short break: {} minute(s)", opt.short_break);

    loop {
        work_interval(opt.work_interval);
        break_interval(opt.short_break);
    }
}

fn work_interval(work_interval: u64) {
    let seconds = work_interval * 60;
    let bar = ProgressBar::new(seconds);
    bar.set_prefix(format!("{: >10}", "Work"));
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {bar:50.blue/gray} {msg}")
            .unwrap(),
    );
    for _ in 0..seconds {
        bar.inc(1);
        let left = seconds_to_minutes_seconds(seconds - bar.position());
        bar.set_message(left + " left");
        thread::sleep(Duration::from_secs(1));
    }
    bar.set_message("Well done!");
    bar.finish();
    play_sound();
}

fn break_interval(short_break: u64) {
    let seconds = short_break * 60;
    let bar = ProgressBar::new(seconds);
    bar.set_prefix(format!("{: >10}", "Break"));
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {bar:50.red/gray} {msg}")
            .unwrap(),
    );
    for _ in 0..seconds {
        bar.inc(1);
        let left = seconds_to_minutes_seconds(seconds - bar.position());
        bar.set_message(left + " left");
        thread::sleep(Duration::from_millis(1000));
    }
    bar.set_message("Time to work!");
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
