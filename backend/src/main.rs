mod config;
mod auth;
mod handlers;
mod models;
mod query;

use axum::{response::IntoResponse, routing::get, Json, Router};
use std::sync::Arc;
use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::config::{Config, AppState};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = setup_db(&config).await;

    let app = setup_router(&pool, &config).await;

    println!("🚀 Server started successfully");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.expect("Failed to bind port 3030");
    axum::serve(listener, app).await.expect("Server failed to start");
}

async fn setup_db(config: &Config) -> Pool<Postgres> {
    let pool = match PgPoolOptions::new().max_connections(10).connect(&config.database_url).await {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        },
        Err(e) => {
            println!("🔥 Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    pool
}

async fn setup_router(pool: &Pool<Postgres>, config: &Config) -> Router {
    let app_state = Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    });

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(app_state)
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgress, and SQLX";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}