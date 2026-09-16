use axum::{Router, routing::get};

async fn health() -> &'static str {
    "auth-service: OK"
}

async fn check_product_service() -> String {
    match reqwest::get("http://product-service:8006/product/test").await {
        Ok(response) => match response.text().await {
            Ok(body) => format!("Product responded: {body}"),
            Err(_) => "Product responded but body could not be read".to_string(),
        },
        Err(err) => format!("Failed to reach product service: {}", err),
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/test/product", get(check_product_service))
}
