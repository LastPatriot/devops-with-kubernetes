use uuid::Uuid;
use chrono::prelude::*;
use std::{time, thread};

fn main() {
    let id = Uuid::new_v4();
    let five_secs = time::Duration::from_secs(5);
    loop {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        println!("{}: {}", timestamp, id);
        thread::sleep(five_secs);
    }
}
