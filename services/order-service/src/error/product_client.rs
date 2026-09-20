use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProductClientError {
    #[error("product not found")]
    ProductNotFound,

    #[error("product service request failed")]
    RequestFailed(#[from] reqwest::Error),
}
