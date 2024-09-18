use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json, body::Body,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use super::{jwt::TokenClaims, ErrorResponseBuilder};
use super::response::ErrorResponse;
use crate::config::AppState;

use crate::query::user::get_user_by_id;

pub async fn auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {

    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });
    let token = token.ok_or_else(|| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message("You are not logged in, please provide token".to_string())
            .status_code(StatusCode::UNAUTHORIZED)
            .build_json()
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message("Invalid token".to_string())
            .status_code(StatusCode::UNAUTHORIZED)
            .build_json()
    })?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message("Invalid token".to_string())
            .status_code(StatusCode::UNAUTHORIZED)
            .build_json()
    })?;

    let user = get_user_by_id(State(data), user_id).await.map_err(|e| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message(format!("Error fetching user from database: {:?}", e))
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .build_json()
    })?;

    let user = user.ok_or_else(|| {
        ErrorResponseBuilder::new()
            .error_message("The user belonging to this token no longer exists".to_string())
            .status_code(StatusCode::UNAUTHORIZED)
            .build_json()
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
