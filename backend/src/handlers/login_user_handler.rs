use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{EncodingKey, Header};
use super::{build_success_response, util::build_cookie};

use crate::{auth::{check_is_valid_password, ErrorResponseBuilder, LoginUserSchema, TokenClaims}, config::AppState, query::get_user_by_email};


pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = get_user_by_email(State(data.clone()), body.email.to_string()).await.map_err(|e| {
        ErrorResponseBuilder::new()
            .status("error")
            .error_message(format!("Database error {:?}", e))
            .status_code(StatusCode::INTERNAL_SERVER_ERROR).build_json_value()
    })?
    .ok_or_else(|| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message("Invalid email or password".to_string())
            .status_code(StatusCode::BAD_REQUEST)
            .build_json_value()
    })?;

    let is_valid = check_is_valid_password(user.password);

    if !is_valid {
        return Err(
            ErrorResponseBuilder::new()
            .status("fail")
            .error_message("Invalid email or password".to_string())
            .status_code(StatusCode::BAD_REQUEST)
            .build_json_value()
        );
    }

    let claims = TokenClaims::new(user.id.to_string());
    let jwt_secret_ref = data.env.jwt_secret.as_ref();
    let token = claims.encode(&Header::default(), &EncodingKey::from_secret(jwt_secret_ref));

    let cookie = build_cookie(Some(token.clone()));

    let response = build_success_response(Some(token.clone()), cookie);
    Ok(response)
}