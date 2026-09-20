use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub product_service_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("ORDER_DATABASE_URL").expect("ORDER_DATABASE_URL must be set");
        let product_service_url =
            env::var("PRODUCT_SERVICE_URL").expect("PRODUCT_SERVICE_URL must be set");
        Self {
            database_url,
            product_service_url,
        }
    }
}
