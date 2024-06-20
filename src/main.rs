use chrono::{DateTime, Utc};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::Source;
use rodio::{source::SineWave, Sink};
use std::error::Error;
use std::io::{stdout, Write};
use std::process::exit;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;
use termion::AsyncReader;

#[derive(Parser)]
struct Opt {
    #[arg(short, long, default_value = "25")]
    work_interval: u64,
    #[arg(short, long, default_value = "5")]
    short_break: u64,
    #[arg(short = 'L', long, default_value = "20")]
    long_break: u64,
    #[arg(short = 'I', long, default_value = "4")]
    long_break_interval: u64,
}

struct Interval {
    label: &'static str,
    duration: u64,
    foreground_color: &'static str,
    background_color: &'static str,
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
    println!("Press Ctrl+C or q to stop the timer\n");

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || loop {
        let now = Utc::now();
        tx.send(now).unwrap();
        std::thread::sleep(Duration::from_millis(50));
    });

    let cycle = build_cycle(opt);

    let mut stdout = stdout().lock().into_raw_mode().unwrap();

    let mut stdin = termion::async_stdin().keys();

    let mut start_timestamp = Utc::now();
    for interval in cycle.iter().cycle() {
        if run_interval(interval, &rx, &mut start_timestamp, &mut stdin).is_err() {
            stdout.flush().unwrap();
            drop(stdout);
            exit(0);
        }
    }
}

fn build_cycle(opt: Opt) -> Vec<Interval> {
    let mut cycle = std::vec::Vec::new();
    for i in 0..opt.long_break_interval {
        cycle.push(Interval {
            label: "Work",
            duration: opt.work_interval,
            foreground_color: "blue",
            background_color: "blue",
        });

        if i == opt.long_break_interval - 1 {
            cycle.push(Interval {
                label: "Long Break",
                duration: opt.long_break,
                foreground_color: "orange",
                background_color: "orange",
            });
        } else {
            cycle.push(Interval {
                label: "Short Break",
                duration: opt.short_break,
                foreground_color: "green",
                background_color: "green",
            });
        }
    }
    cycle
}

fn run_interval(
    interval: &Interval,
    rx: &Receiver<DateTime<Utc>>,
    start_timestamp: &mut DateTime<Utc>,
    stdin: &mut Keys<AsyncReader>,
) -> Result<(), &'static str> {
    let len = interval.duration * 60;
    let bar = ProgressBar::new(len);
    let template = format!(
        "{{prefix}} [{{bar:40.{fg}/{bg}}}] {{msg}}",
        fg = interval.foreground_color,
        bg = interval.background_color
    );
    bar.set_style(
        ProgressStyle::default_bar()
            .template(template.as_str())
            .unwrap(),
    );
    bar.set_prefix(format!("{}: {}", interval.label, format_interval(len)));

    for recv in rx.iter() {
        let b = stdin.next();
        if let Some(Ok(key)) = b {
            match key {
                termion::event::Key::Char('q') | termion::event::Key::Ctrl('c') => {
                    bar.finish();
                    return Err("User interrupted");
                }
                _ => {}
            }
        }
        let pos = (recv - *start_timestamp).num_seconds() as u64;
        bar.set_position(pos);
        bar.set_message(format!("remaining {}", format_interval(len - pos)));

        if pos < len {
            continue;
        }

        bar.finish();
        if play_sound().is_err() {
            eprintln!("Failed to play sound.");
        }
        break;
    }
    Ok(())
}

fn format_interval(remaining_seconds: u64) -> String {
    // Extract minutes and seconds
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    // Format the string with abbreviation (m for minutes, s for seconds)
    if minutes > 0 && seconds > 0 {
        format!("{}m{}s", minutes, seconds)
    } else if minutes > 0 {
        format!("{}m", minutes) // Handle cases where only minutes are present
    } else {
        format!("{}s", seconds) // Handle cases where only seconds are present
    }
}

fn play_sound() -> Result<(), Box<dyn Error>> {
    // Create a sink to play the sound
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;

    // Try to create a new sink
    let sink = Sink::try_new(&stream_handle)?;

    // Generate a sine wave sound source
    let source = SineWave::new(440.0).take_duration(Duration::from_secs(1));

    // Add the sound source to the sink and play it
    sink.append(source);
    sink.play();

    // Sleep for the duration of the sound
    thread::sleep(Duration::from_secs(1));

    Ok(())
}
