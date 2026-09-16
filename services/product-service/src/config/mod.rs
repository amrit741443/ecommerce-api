use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url =
            env::var("PRODUCT_DATABASE_URL").expect("PRODUCT_DATABASE_URL must be set");
        Self { database_url }
    }
}
