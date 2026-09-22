use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::error::ApplicationError::{self, InvalidProductName};

#[derive(Debug)]
pub enum ApiError {
    Application(ApplicationError),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: &str) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        Self::Application(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Application(ApplicationError::ProductNotFound) => {
                into_error_response(StatusCode::NOT_FOUND, "product not found")
            }

            ApiError::Application(ApplicationError::MissingIdempotencyKey) => into_error_response(
                StatusCode::BAD_REQUEST,
                "Idempotency-Key  header is required",
            ),

            ApiError::Application(InvalidProductName) => {
                into_error_response(StatusCode::BAD_REQUEST, "product name cannot be empty")
            }

            ApiError::Application(ApplicationError::InvalidProductPrice) => {
                into_error_response(StatusCode::BAD_REQUEST, "product price cannot be negative")
            }

            ApiError::Application(ApplicationError::InvalidProductStock) => {
                into_error_response(StatusCode::BAD_REQUEST, "product stock cannot be negative")
            }

            ApiError::Application(ApplicationError::InsufficientStock) => {
                into_error_response(StatusCode::BAD_REQUEST, "insufficient stock")
            }

            ApiError::Application(ApplicationError::Repository(error)) => {
                eprintln!("Repository error: {error:?}");

                into_error_response(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        }
    }
}

fn into_error_response(status: StatusCode, message: &str) -> Response {
    (status, Json(ErrorResponse::new(message))).into_response()
}
