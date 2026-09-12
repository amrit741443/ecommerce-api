use auth_service::app;
use axum::serve;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8001")
        .await
        .expect("failed to bind TCP listener");

    println!(
        "auth-service listening on {}",
        listener.local_addr().unwrap()
    );

    serve(listener, app()).await.expect("server failed");
}
