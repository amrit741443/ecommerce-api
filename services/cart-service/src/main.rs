use axum::serve;
use cart_service::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8002")
        .await
        .expect("failed to bind TCP listener");

    println!(
        "cart-service listening on {}",
        listener.local_addr().unwrap()
    );

    serve(listener, app()).await.expect("server failed");
}
