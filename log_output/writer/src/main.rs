use std::{env, fs, io::Write, path::Path, thread, time};
use std::fs::OpenOptions;
use std::time::Instant;
use chrono::{SecondsFormat, Utc};
use uuid::Uuid;

fn now_string() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn main() -> std::io::Result<()> {
    let env_path = env::var("LOG_OUTPUT").unwrap_or_else(|_| "/usr/src/app/files".to_string());
    let file_path = Path::new(&env_path).join("log.txt");
    let five_secs = time::Duration::from_secs(5);
    if let Some(dir) = file_path.parent() {
        fs::create_dir_all(dir)?;
    }
    let id = Uuid::new_v4();
    let mut file_handle = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    let mut next_tick = Instant::now();
    loop {
        next_tick += five_secs;
        let timestamp = now_string();
        writeln!(file_handle, "{}: {}", timestamp, id)?;
        println!("{}: {}", timestamp, id);
        thread::sleep(next_tick.saturating_duration_since(Instant::now()));
    }
}
