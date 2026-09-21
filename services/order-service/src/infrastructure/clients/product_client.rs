use reqwest::Client;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::time::Duration;
use uuid::Uuid;

use crate::error::ProductClientError;

#[derive(Debug, Deserialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
}

#[derive(Debug, Clone)]
pub struct ProductClient {
    client: Client,
    base_url: String,
}

impl ProductClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create product client");

        Self { client, base_url }
    }

    pub async fn get_product(
        &self,
        product_id: Uuid,
    ) -> Result<ProductResponse, ProductClientError> {
        let url = format!("{}/products/{}", self.base_url, product_id);
        let response = self.client.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ProductClientError::ProductNotFound);
        }

        let response = response.error_for_status()?;

        let product = response.json::<ProductResponse>().await?;

        Ok(product)
    }

    //reserve stock

    pub async fn reserve_stock(
        &self,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), ProductClientError> {
        let url = format!("{}/products/{}/reserve", self.base_url, product_id);

        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({"quantity":quantity}))
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ProductClientError::ProductNotFound);
        }

        response.error_for_status()?;

        Ok(())
    }

    // release stock
    pub async fn release_stock(
        &self,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), ProductClientError> {
        let url = format!("{}/products/{}/release", self.base_url, product_id);

        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "quantity": quantity }))
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ProductClientError::ProductNotFound);
        }

        response.error_for_status()?;

        Ok(())
    }
}
