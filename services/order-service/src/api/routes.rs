use axum::{
    Router,
    routing::{get, post},
};

use crate::api::{handlers::order::create_order, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/orders", post(create_order))
        .with_state(state)
}

async fn health() -> &'static str {
    "order-service is healthy"
}
