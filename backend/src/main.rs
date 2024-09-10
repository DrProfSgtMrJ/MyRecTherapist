
use axum::{response::IntoResponse, routing::get, Json, Router};
mod config;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/heathchecker", get(health_checker_handler));

    println!("🚀 Server started successfully");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.expect("Failed to bind port 3030");
    axum::serve(listener, app).await.expect("Server failed to start");
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgress, and SQLX";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}