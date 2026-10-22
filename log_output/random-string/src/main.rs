use uuid::Uuid;
use chrono::prelude::*;
use std::{time, env, net::SocketAddr};
use axum::{Router, routing::get, extract::State};

fn now_string() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
async fn status(State(id): State<Uuid>) -> String {
    let timestamp = now_string();
    format!("{}: {}", timestamp, id)
}
#[tokio::main]
async fn main() {
    let id = Uuid::new_v4();
    let five_secs = time::Duration::from_secs(5);
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_str.parse::<u16>().expect("PORT must be a number");
    let app = Router::new().route("/", get(status)).with_state(id);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind failed");
    println!("Server started in port {}", port);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(five_secs);
        loop {
            interval.tick().await;
            let timestamp = now_string();
            println!("{}: {}", timestamp, id);
        }
    });
    axum::serve(listener, app).await.expect("launch failed");
}
