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
            ApiError::Application(ApplicationError::ProductNotFound) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("product not found ")),
            )
                .into_response(),

            ApiError::Application(InvalidProductName) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("product name cannot be empty")),
            )
                .into_response(),

            ApiError::Application(ApplicationError::InvalidProductPrice) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("product price cannot be negative")),
            )
                .into_response(),

            ApiError::Application(ApplicationError::InvalidProductStock) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new("product stock cannot be negative")),
            )
                .into_response(),

            ApiError::Application(ApplicationError::Repository(error)) => {
                eprintln!("Repository error: {error:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("internal server error")),
                )
                    .into_response()
            }
        }
    }
}
