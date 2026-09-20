use thiserror::Error;

use crate::error::ProductClientError;

use super::RepositoryError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    ProductClient(#[from] ProductClientError),

    #[error("order must contain at least one item")]
    OrderMustContainItems,

    #[error("quantity must be greater than zero")]
    InvalidQuantity,

    #[error("unit price cannot be negative")]
    InvalidUnitPrice,

    #[error("insufficient product stock")]
    InsufficientStock,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
