// src/errors/error.rs
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalError(String),

    #[error("Not Found")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
    code: u16,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, message) = match self {
            ApiError::InternalError(_) => (500, self.to_string()),
            ApiError::NotFound(_) => (404, self.to_string()),
            ApiError::BadRequest(_) => (400, self.to_string()),
            ApiError::Conflict(_) => (400, self.to_string()),
        };

        let response = ErrorResponse {
            status: "error",
            message,
            code: status_code,
        };

        HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(response)
    }
}
