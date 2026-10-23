use axum::{Router, routing::get, extract::State};
use std::{ env, net::SocketAddr, path::Path,};
use std::path::PathBuf;

async fn show_logs(State(path): State<PathBuf>) -> String {
    tokio::fs::read_to_string(&path).await.unwrap_or_else(|_| "No logs yet".to_string())
}

#[tokio::main]
async fn main() {
    let env_path = env::var("LOG_OUTPUT").unwrap_or_else(|_| "/usr/src/app/files".to_string());
    let file_path = Path::new(&env_path).join("log.txt");
    let port_string = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_string.parse::<u16>().expect("invalid port number");
    let app = Router::new().route("/", get(show_logs)).with_state(file_path);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind failed");
    println!("Server started in port {}", port);
    axum::serve(listener, app).await.expect("Server failed to start");

}
