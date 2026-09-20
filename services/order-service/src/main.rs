use order_service::{
    api::{routes::create_router, state::AppState},
    application::order_service::OrderService,
    config::Config,
    infrastructure::{
        clients::product_client::ProductClient, database::postgres::create_pool,
        repositories::order_repository::OrderRepository,
    },
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failed to load .env file");
    let config = Config::from_env();

    let db = create_pool(&config.database_url)
        .await
        .expect("failed to connect to order database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("failed to run order-service migrations");

    let repository = OrderRepository::new();

    let product_client = ProductClient::new(config.product_service_url);

    let order_service = OrderService::new(db, repository, product_client);

    let state = AppState { order_service };

    let app = create_router(state);

    println!("order-service database migrations completed");

    let listener = TcpListener::bind("0.0.0.0:8004")
        .await
        .expect("failed to bind TCP listener");

    println!(
        "order-service listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.expect("server error");
}
