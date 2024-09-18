use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{auth::{get_hash_password, jwt::RegisteredUserSchema, response::{ErrorResponseBuilder, FilteredUser}}, models::user::User, query::check_user_exists, query::insert_user};
use crate::config::AppState;


fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        photo: user.photo.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}

pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisteredUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists = check_user_exists(State(data.clone()), body.email.to_string()).await.map_err(|e| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message(format!("Database Error {}", e))
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .build_json_value()
    })?;

    if let Some(exists) = user_exists {
        if exists {
            return Err(
                ErrorResponseBuilder::new()
                    .status("fail")
                    .error_message("User with that email already exists".to_string())
                    .status_code(StatusCode::CONFLICT)
                    .build_json_value()
            );
        }
    }

    let hash_password = get_hash_password(body.password).map_err(|e| {
        ErrorResponseBuilder::new()
            .status("fail")
            .error_message(format!("Error while hashing password: {:?}", e))
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .build_json_value()
    })?;

    let user = insert_user(State(data.clone()), body.name.to_string(), body.email.to_string(), hash_password).await
        .map_err(|e| {
            ErrorResponseBuilder::new()
                .status("fail")
                .error_message(format!("Database ERror {}", e))
                .status_code(StatusCode::INTERNAL_SERVER_ERROR)
                .build_json_value()
        })?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user)
    })});

    Ok(Json(user_response))

}