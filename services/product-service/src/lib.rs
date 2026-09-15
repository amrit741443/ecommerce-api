use axum::{Router, routing::get};

async fn health() -> &'static str {
    "product-service: OK"
}

async fn product_info() -> &'static str {
    "Product service is responding"
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/products/test", get(product_info))
}
