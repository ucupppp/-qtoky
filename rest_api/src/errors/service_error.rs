use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Invalid ID: {0}")]
    InvalidId(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Unexpected: {0}")]
    Unexpected(String),

    #[error("Hashing Error: {0}")]
    HashingError(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}
