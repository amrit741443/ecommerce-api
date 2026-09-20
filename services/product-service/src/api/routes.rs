use axum::{
    Router,
    routing::{get, post},
};

use crate::api::{
    handlers::product::{
        create_product, delete_product, get_product, list_products, reserve_stock, update_product,
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/products", post(create_product).get(list_products))
        .route(
            "/products/{id}",
            get(get_product)
                .patch(update_product)
                .delete(delete_product),
        )
        .route("/products/{id}/reserve", post(reserve_stock))
        .with_state(state)
}

async fn health() -> &'static str {
    "product-service: OK"
}
