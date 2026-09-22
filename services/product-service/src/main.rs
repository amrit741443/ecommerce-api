use product_service::{
    api::{routes::create_router, state::AppState},
    application::product_service::ProductService,
    config::Config,
    infrastructure::{
        database::postgres::create_pool, repositories::product_repository::ProductRepository,
    },
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    // Initialize database pool
    let db = create_pool(&config.database_url).await;

    //migrate
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("failed to run product-service migrations");

    // Run pending migrations at startup

    let product_repository = ProductRepository::new(db);

    let product_service = ProductService::new(product_repository);

    let state = AppState { product_service };

    let app = create_router(state);

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
