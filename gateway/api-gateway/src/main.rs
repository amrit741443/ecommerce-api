use api_gateway::{AppState, app};
use axum::serve;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let product_service_url =
        env::var("PRODUCT_SERVICE_URL").expect("PRODUCT_SERVICE_URL must be set");

    let state = AppState {
        client: reqwest::Client::new(),
        product_service_url,
    };

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind TCP listener");

    println!("gateway listening on {}", listener.local_addr().unwrap());

    serve(listener, app(state)).await.expect("server failed");
}
