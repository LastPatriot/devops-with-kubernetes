use axum::{Router, response::Html, routing::get};
use std::env;
use std::net::SocketAddr;

async fn home() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Todo app</title>
</head>
<body>
<h1>Todo app</h1>
</body>
</html>
"#,
    )
}

#[tokio::main]
async fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_str.parse::<u16>().expect("PORT must be a number");
    let app = Router::new().route("/", get(home));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");
    println!("Server started in port {}", port);
    axum::serve(listener, app).await.expect("serve failed");
}
