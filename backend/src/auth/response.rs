use axum::{http::StatusCode, Json};
use chrono::prelude::*;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Default)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

impl From<ErrorResponse> for serde_json::Value {
    fn from(value: ErrorResponse) -> Self {
        serde_json::json!(value)     
    }
}

#[derive(Debug, Default)]
pub struct ErrorResponseBuilder {
    status_code: StatusCode,
    error_response: ErrorResponse
}

impl ErrorResponseBuilder {
    
    pub fn new() -> ErrorResponseBuilder {
        ErrorResponseBuilder::default()
    }

    pub fn status_code(mut self, code: StatusCode) -> ErrorResponseBuilder {
        self.status_code = code;
        self
    }

    pub fn error_response(mut self, error_response: ErrorResponse) -> ErrorResponseBuilder {
        self.error_response = error_response;
        self
    }

    pub fn error_message(mut self, message: String) -> ErrorResponseBuilder {
        self.error_response.message = message;
        self
    }

    pub fn status(mut self, status: &'static str) -> ErrorResponseBuilder {
        self.error_response.status = status;
        self
    }

    pub fn build(self) -> (StatusCode, ErrorResponse) {
        (self.status_code, self.error_response)
    }

    pub fn build_json(self) -> (StatusCode, Json<ErrorResponse>) {
        (self.status_code, Json(self.error_response))
    }

    pub fn build_json_value(self) -> (StatusCode, Json<serde_json::Value>) {
        (self.status_code, Json(self.error_response.into()))
    }
}
