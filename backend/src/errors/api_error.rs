// src/errors/error.rs
use actix_web::{
    Error as ActixError, HttpResponse, ResponseError,
    error::{JsonPayloadError, PayloadError, QueryPayloadError, UrlencodedError},
    http::StatusCode,
};
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

    #[error("Forbidden: {0}")]
    Forbidden(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
    code: u16,
}

// Response
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, message) = match self {
            ApiError::InternalError(_) => (500, self.to_string()),
            ApiError::NotFound(_) => (404, self.to_string()),
            ApiError::BadRequest(_) => (400, self.to_string()),
            ApiError::Conflict(_) => (409, self.to_string()),
            ApiError::ValidationError(_) => (422, self.to_string()),
            ApiError::Unauthorized(_) => (401, self.to_string()),
            ApiError::Forbidden(_) => (403, self.to_string()),
        };

        let response = ErrorResponse {
            status: "error",
            message,
            code: status_code,
        };

        HttpResponse::build(StatusCode::from_u16(status_code).unwrap()).json(response)
    }
}

// Validation
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
                            .unwrap_or_else(|| "tidak valid".into())
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

// Actix

impl From<ActixError> for ApiError {
    fn from(err: ActixError) -> Self {
        if let Some(json_err) = err.as_error::<JsonPayloadError>() {
            match json_err {
                JsonPayloadError::ContentType => ApiError::BadRequest(
                    "Konten harus berupa JSON (Content-Type: application/json)".into(),
                ),

                JsonPayloadError::Deserialize(_) => ApiError::BadRequest(
                    "Format JSON tidak valid atau ada field yang salah/tidak lengkap".into(),
                ),

                JsonPayloadError::Payload(_) => {
                    ApiError::BadRequest("Ukuran body terlalu besar atau rusak".into())
                }

                JsonPayloadError::Overflow { limit: _ } => {
                    ApiError::BadRequest("Nilai melebihi batas yang diperbolehkan".into())
                }

                _ => ApiError::BadRequest("Terjadi kesalahan saat memproses JSON".into()),
            }
        } else if let Some(payload_err) = err.as_error::<PayloadError>() {
            match payload_err {
                PayloadError::Overflow => {
                    ApiError::BadRequest("Ukuran permintaan terlalu besar".into())
                }

                PayloadError::Incomplete(_) => {
                    ApiError::BadRequest("Data tidak lengkap atau rusak".into())
                }

                _ => ApiError::BadRequest("Permintaan tidak dapat dibaca".into()),
            }
        } else if let Some(form_err) = err.as_error::<UrlencodedError>() {
            ApiError::BadRequest("Format form tidak valid".into())
        } else {
            ApiError::InternalError("Terjadi kesalahan internal saat memproses permintaan".into())
        }
    }
}
