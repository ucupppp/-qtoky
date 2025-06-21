// src/errors/error.rs
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

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

    #[error("ValidationError: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
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
            ApiError::Conflict(_) => (409, self.to_string()),
            ApiError::ValidationError(_) => (422, self.to_string()),
            ApiError::Unauthorized(_) => (401, self.to_string()),
        };

        let response = ErrorResponse {
            status: "error",
            message,
            code: status_code,
        };

        HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(response)
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(err: ValidationErrors) -> Self {
        // Ubah ValidationErrors menjadi satu string yang readable
        let msg = err
            .field_errors()
            .iter()
            .map(|(_field, errs)| {
                let reasons = errs
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "tidak valid".to_string())
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{reasons}")
            })
            .collect::<Vec<_>>()
            .join(" | ");

        ApiError::ValidationError(msg)
    }
}
