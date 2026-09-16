use axum::serve;
use notification_service::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8003")
        .await
        .expect("failed to bind TCP listener");

    println!(
        "notification-service listening on {}",
        listener.local_addr().unwrap()
    );

    serve(listener, app()).await.expect("server failed");
}
