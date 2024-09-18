use axum::{http::StatusCode, response::IntoResponse, Json};

use super::{build_cookie, build_success_response};


pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = build_cookie(None);
    let response = build_success_response(None, cookie);

    Ok(response)
}