//! A sub-module to prescribe how each domain error gets converted to an HTTP response.
use warp::reply::{with_status, Reply};
use warp::http::{response::Response, status::StatusCode};

use crate::domain::hello::{HelloError};
use crate::response::ErrorResponse;

impl From<HelloError> for ErrorResponse {
    fn from(e: HelloError) -> ErrorResponse {
        match &e {
            HelloError::NotFound { .. } => ErrorResponse(
                with_status(
                    Response::new(e.to_string()), 
                    StatusCode::NOT_FOUND).into_response(),
            ),
            HelloError::DatabaseError(_) => {
                ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
        }
    }
}