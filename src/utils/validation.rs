use validator::Validate;

use crate::error::AppError;

pub fn validate_request<T: Validate>(payload: &T) -> Result<(), AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))
}
