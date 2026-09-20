use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("invalid order status")]
    InvalidOrderStatus,
}
