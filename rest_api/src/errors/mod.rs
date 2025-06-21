mod api_error;
mod service_error;

pub use api_error::ApiError;
pub use service_error::ServiceError;

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::NotFound(msg) => ApiError::NotFound(msg),
            ServiceError::BadRequest(msg) | ServiceError::InvalidId(msg) => {
                ApiError::BadRequest(msg)
            }
            ServiceError::HashingError(msg) | ServiceError::DatabaseError(msg) => {
                ApiError::InternalError(msg)
            }
            ServiceError::Conflict(msg) => ApiError::Conflict(msg),
            ServiceError::Unexpected(msg) => ApiError::InternalError(msg),
            ServiceError::Unauthorized(msg) => ApiError::Unauthorized(msg),
        }
    }
}
