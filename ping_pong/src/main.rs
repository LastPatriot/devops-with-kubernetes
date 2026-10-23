use std::{env, sync::Arc, sync::atomic::{AtomicU64, Ordering}};
use std::net::SocketAddr;
use axum::{Router, routing::get, extract::State};
async fn say_pong(State(counter): State<Arc<AtomicU64>>) -> String {
    let count = counter.fetch_add(1, Ordering::Relaxed);
    format!("pong {}", count)
}

#[tokio::main]
async fn main() {
    let port_string = env::var("PORT").unwrap_or_else(|_| "3003".to_string());
    let port = port_string.parse::<u16>().expect("Invalid Port number");
    let counter = Arc::new(AtomicU64::new(0));
    let app = Router::new().route("/pingpong", get(say_pong)).with_state(counter);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Could not bind to port");
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Could not start server");
}
