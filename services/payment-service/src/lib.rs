use axum::{Router, routing::get};

async fn health() -> &'static str {
    "payment-service: OK"
}

pub fn app() -> Router {
    Router::new().route("/health", get(health))
}
