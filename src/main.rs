mod event;

use chrono::{DateTime, Utc};
use clap::Parser;
use event::MixpanelClient;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::Source;
use rodio::{source::SineWave, Sink};
use std::collections::HashMap;
use std::error::Error;
use std::io::{stdout, Write};
use std::process::exit;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;
use termion::AsyncReader;
use tokio;

const TIMER_INTERVAL: u64 = 50;

#[derive(Parser)]
#[command(version, about, author, help_template = format!("
{{usage}}
  
{{all-args}}

{}Version:{} {{version}}
{}Authors:{} {{author}}

{{about}}

{}Report Bugs:{} {}{}/issues{}

Thanks for using! 😊
", 
termion::style::Bold, termion::style::Reset, 
termion::style::Bold, termion::style::Reset,
termion::style::Bold, termion::style::Reset,
termion::style::Underline,
get_repository_url("Cargo.toml").expect("Failed to get repository URL"),
termion::style::Reset))]
struct Opt {
    #[arg(short, long, default_value = "25", help = "Work interval in minutes")]
    work_interval: u64,
    #[arg(short, long, default_value = "5", help = "Short break in minutes")]
    short_break: u64,
    #[arg(
        short = 'L',
        long,
        default_value = "20",
        help = "Long break in minutes"
    )]
    long_break: u64,
    #[arg(short = 'I', long, default_value = "4", help = "Long break interval")]
    long_break_interval: u64,
}

struct Interval {
    label: &'static str,
    duration: u64,
    foreground_color: &'static str,
    background_color: &'static str,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    println!("💻 Work interval: {} minute(s)", opt.work_interval);
    println!("🍵 Short break: {} minute(s)", opt.short_break);
    println!("🍺 Long break: {} minute(s)", opt.long_break);
    println!(
        "Long break interval: {} work intervals",
        opt.long_break_interval
    );

    println!("");
    println!(
        "Press {}Ctrl+C{} or {}Q{} to stop the timer",
        termion::style::Bold,
        termion::style::Reset,
        termion::style::Bold,
        termion::style::Reset,
    );
    println!(
        "Press {}P{} to pause or resume a timer\n",
        termion::style::Bold,
        termion::style::Reset,
    );
    println!("");

    let token = String::from("d83029f8b9507a4f672fe96cc3fe9b2c");
    let distinct_id = std::env::var("MIXPANEL_DISTINCT_ID").ok();
    let mixpanel_client = create_mixpanel_client(token, distinct_id).await;
    send_start_event(mixpanel_client.clone(), &opt).await;

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || loop {
        let now = Utc::now();
        tx.send(now).unwrap();
        std::thread::sleep(Duration::from_millis(TIMER_INTERVAL));
    });

    let cycle = build_cycle(opt);

    let mut stdout = stdout().lock().into_raw_mode().unwrap();

    let mut stdin = termion::async_stdin().keys();

    for interval in cycle.iter().cycle() {
        if run_interval(mixpanel_client.clone(), interval, &rx, &mut stdin)
            .await
            .is_err()
        {
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
            label: "💻",
            duration: opt.work_interval,
            foreground_color: "blue",
            background_color: "blue",
        });

        if i == opt.long_break_interval - 1 {
            cycle.push(Interval {
                label: "🍺",
                duration: opt.long_break,
                foreground_color: "orange",
                background_color: "orange",
            });
        } else {
            cycle.push(Interval {
                label: "🍵",
                duration: opt.short_break,
                foreground_color: "green",
                background_color: "green",
            });
        }
    }
    cycle
}

async fn run_interval(
    mixpanel_client: Arc<MixpanelClient>,
    interval: &Interval,
    rx: &Receiver<DateTime<Utc>>,
    stdin: &mut Keys<AsyncReader>,
) -> Result<(), &'static str> {
    let len = interval.duration * 60;
    let mut start_timestamp = Utc::now();
    let mut paused = false;
    let bar = ProgressBar::new(len);
    let template = format!(
        "{{prefix}} [{{bar:60.{fg}/{bg}}}] {{msg}}",
        fg = interval.foreground_color,
        bg = interval.background_color
    );
    bar.set_style(
        ProgressStyle::default_bar()
            .template(template.as_str())
            .unwrap(),
    );
    bar.set_prefix(format!("{} {}", interval.label, format_interval(len)));
    let mut pause_timestamp: Option<DateTime<Utc>> = None;
    for recv in rx.iter() {
        let b = stdin.next();
        if let Some(Ok(key)) = b {
            match key {
                termion::event::Key::Char('q') | termion::event::Key::Ctrl('c') => {
                    bar.abandon();
                    send_quit_event(mixpanel_client.clone()).await;
                    return Err("User interrupted");
                }
                termion::event::Key::Char('p') => {
                    paused = !paused;
                    if paused {
                        pause_timestamp = Some(Utc::now());
                    } else {
                        let pause = pause_timestamp.unwrap();
                        let resume = Utc::now();
                        send_pause_event(mixpanel_client.clone(), pause, resume).await;
                    }
                }
                _ => {}
            }
        }

        if paused {
            start_timestamp += Duration::from_millis(TIMER_INTERVAL);
            bar.set_message(format!(
                "{}{}Paused{}",
                termion::style::Bold,
                termion::color::Fg(termion::color::Red),
                termion::style::Reset
            ));
            continue;
        }

        let pos = (recv - start_timestamp).num_seconds() as u64;
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

async fn create_mixpanel_client(token: String, distinct_id: Option<String>) -> Arc<MixpanelClient> {
    let client = MixpanelClient::new(token, distinct_id);
    Arc::new(client)
}

async fn send_event(
    mixpanel_client: Arc<MixpanelClient>,
    event_name: &str,
    properties: std::collections::HashMap<String, String>,
) {
    let mut event = event::Event::new(event_name);
    for (key, value) in properties {
        event.add_property(&key, &value);
    }

    mixpanel_client.send_event(&mut event).await;
}

async fn send_start_event(mixpanel_client: Arc<MixpanelClient>, opt: &Opt) {
    let mut properties = HashMap::new();
    properties.insert("work_interval".to_string(), opt.work_interval.to_string());
    properties.insert("short_break".to_string(), opt.short_break.to_string());
    properties.insert("long_break".to_string(), opt.long_break.to_string());
    properties.insert(
        "long_break_interval".to_string(),
        opt.long_break_interval.to_string(),
    );
    send_event(mixpanel_client, "start", properties).await;
}

async fn send_quit_event(mixpanel_client: Arc<MixpanelClient>) {
    send_event(mixpanel_client, "quit", HashMap::new()).await;
}

async fn send_pause_event(
    mixpanel_client: Arc<MixpanelClient>,
    pause: DateTime<Utc>,
    resume: DateTime<Utc>,
) {
    let mut properties = HashMap::new();
    properties.insert("pause".to_string(), pause.to_rfc3339());
    properties.insert("resume".to_string(), resume.to_rfc3339());
    send_event(mixpanel_client, "pause", properties).await;
}

fn get_repository_url(file_path: &str) -> Option<String> {
    let cargo_toml = fs::read_to_string(file_path).ok()?;
    let value: toml::Value = toml::from_str(&cargo_toml).ok()?;

    value
        .get("package")?
        .get("repository")?
        .as_str()
        .map(|s| s.to_string())
}
