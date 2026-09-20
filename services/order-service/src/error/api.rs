use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::error::{ApplicationError, ProductClientError};

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
            ApiError::Application(ApplicationError::InsufficientStock) => {
                error_into_response(StatusCode::BAD_REQUEST, "insufficent stock")
            }

            ApiError::Application(ApplicationError::InvalidQuantity) => {
                error_into_response(StatusCode::BAD_REQUEST, "invalid quantity")
            }

            ApiError::Application(ApplicationError::InvalidUnitPrice) => {
                error_into_response(StatusCode::BAD_REQUEST, "invalid unit price")
            }

            ApiError::Application(ApplicationError::OrderMustContainItems) => {
                error_into_response(StatusCode::BAD_REQUEST, "Need atleast one item")
            }

            ApiError::Application(ApplicationError::Repository(error)) => {
                eprintln!("Repository error: {error:?}");

                error_into_response(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }

            ApiError::Application(ApplicationError::Database(error)) => {
                eprintln!("Database error: {error:?}");

                error_into_response(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }

            ApiError::Application(ApplicationError::ProductClient(error)) => match error {
                ProductClientError::ProductNotFound => {
                    error_into_response(StatusCode::NOT_FOUND, "product not found")
                }

                ProductClientError::RequestFailed(error) => {
                    eprintln!("Requestfailed: {error:?}");

                    error_into_response(StatusCode::BAD_GATEWAY, "unable to connect to resource")
                }
            },
        }
    }
}

fn error_into_response(status: StatusCode, message: &str) -> Response {
    (status, Json(ErrorResponse::new(message))).into_response()
}
