use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::env;

async fn todo_announce() -> &'static str {
     "Todo app"
}

#[tokio::main]
async fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_str.parse::<u16>().expect("PORT must be a number");
    let app = Router::new()
        .route("/", get(todo_announce));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind failed");
    println!("Server started in port {}", port);
    axum::serve(listener, app).await.expect("serve failed");
}
