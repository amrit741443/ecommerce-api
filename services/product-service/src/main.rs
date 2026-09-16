use axum::{Router, routing::get};
use product_service::{
    api::state::AppState, config::Config, infrastructure::database::postgres::create_pool,
};
use tokio::net::TcpListener;

async fn health() -> &'static str {
    "product-service: OK"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    // Initialize database pool
    let db = create_pool(&config.database_url).await;

    // Run pending migrations at startup

    println!("Connected to database");

    let state = AppState { db };

    // Build Axum router
    let app = Router::new()
        .route("/health", get(health))
        .with_state(state);

    // Fixed single colon binding
    let bind_addr = "0.0.0.0:8006";
    let listener = TcpListener::bind(bind_addr)
        .await
        .expect("Failed to bind to address");

    println!(
        "product-service listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app)
        .await
        .expect("Failed to serve application");
}
