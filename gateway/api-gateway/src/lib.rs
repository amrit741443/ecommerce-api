use std::time::Duration;

use axum::{Router, extract::State, routing::get};

use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub product_service_url: String,
}

async fn get_product_test(State(state): State<AppState>) -> Result<String, String> {
    let url = format!("{}/products/test", state.product_service_url);

    let response = state
        .client
        .get(url)
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let body = response.text().await.map_err(|error| error.to_string())?;

    Ok(body)
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/api/products/test", get(get_product_test))
        .with_state(state)
}
