use axum::http::{header, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::json;


pub fn build_cookie(token: Option<String>) -> String {
    Cookie::build(("token", token.unwrap_or_default().to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .to_string()
}

pub fn build_success_response(token: Option<String>, cookie: String) -> Response<String> {
    let mut response = match token {
        Some(token) => Response::new(json!({"status": "success", "token": token}).to_string()),
        None => Response::new(json!({"status": "success"}).to_string())
    };
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.parse().unwrap());

    response
}