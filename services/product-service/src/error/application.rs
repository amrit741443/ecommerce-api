use thiserror::Error;

use crate::error::RepositoryError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error("product not found")]
    ProductNotFound,

    #[error("product name cannot be empty")]
    InvalidProductName,

    #[error("product price cannot be negative")]
    InvalidProductPrice,

    #[error("product stock cannot be negative")]
    InvalidProductStock,
}
