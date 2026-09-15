use api_gateway::{AppState, app};
use axum::serve;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = AppState {
        client: reqwest::Client::new(),
        product_service_url: "http://0.0.0.0:8006".to_string(),
    };

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind TCP listener");

    println!("gateway listening on {}", listener.local_addr().unwrap());

    serve(listener, app(state)).await.expect("server failed");
}
