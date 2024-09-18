use axum::extract::State;
use std::sync::Arc;
use crate::models::User;

use uuid::Uuid;

use crate::config::AppState;

pub async fn get_user_by_id(State(data): State<Arc<AppState>>, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
    .fetch_optional(&data.db).await.map_err(|e| {
        e
    })
}

pub async fn get_user_by_email(State(data): State<Arc<AppState>>, email: String) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email.to_ascii_lowercase())
        .fetch_optional(&data.db).await.map_err(|e| {
            e
        })
}

pub async fn check_user_exists(State(data): State<Arc<AppState>>, email: String) -> Result<Option<bool>, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(email.to_ascii_lowercase())
        .fetch_one(&data.db)
        .await.map_err(|e| {
            e
        })
}


pub async fn insert_user(State(data): State<Arc<AppState>>, name: String, email: String, hashed_password: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *",
        name,
        email.to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await.map_err(|e| {
        e
    })
}